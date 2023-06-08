// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use anyhow::{anyhow, bail, ensure, format_err, Context, Result};
use super::move_types::{HexEncodedBytes, MoveResource, MoveStructTag, MoveType, MoveValue};
use core::str::FromStr;
use move_binary_format::{layout::GetModule, CompiledModule};
use move_core_types::{
	account_address::AccountAddress,
	language_storage::{ModuleId, StructTag, TypeTag},
	resolver::{ModuleResolver, ResourceResolver},
	value::{MoveStructLayout, MoveTypeLayout},
};
use serde_json::Value;
use std::{
	convert::{TryFrom, TryInto},
	iter::IntoIterator,
};

pub fn parse_struct_tag_string3(tag: Vec<u8>) -> Result<(Vec<u8>, StructTag, Vec<u8>), Vec<u8>> {
	let tag_str = std::str::from_utf8(&tag).unwrap();
	let movestructtag: MoveStructTag = MoveStructTag::from_str(tag_str).unwrap();
	let struct_tag = StructTag::try_from(movestructtag).unwrap();
	let module_id = ModuleId::new(struct_tag.address.clone(), struct_tag.module.to_owned());
	Ok((
		bcs_alt::to_bytes(&struct_tag).unwrap(),
		struct_tag,
		bcs_alt::to_bytes(&module_id).unwrap(),
	))
}

pub fn parse_struct_tag_string(tag: Vec<u8>) -> Result<(Vec<u8>, StructTag, Vec<u8>), Vec<u8>> {
	let struct_tag: StructTag = bcs_alt::from_bytes(&tag).unwrap();
	let module_id = ModuleId::new(struct_tag.address.clone(), struct_tag.module.to_owned());
	Ok((tag, struct_tag, bcs_alt::to_bytes(&module_id).unwrap()))
}

pub struct StateView {
	bytes: Vec<u8>,
	module_bytes: Vec<u8>,
}
impl StateView {
	pub fn new(bytes: Vec<u8>, module_bytes: Vec<u8>) -> Self {
		Self { bytes, module_bytes }
	}
}
impl ModuleResolver for StateView {
	type Error = anyhow::Error;

	fn get_module(&self, _module_id: &ModuleId) -> anyhow::Result<Option<Vec<u8>>> {
		Ok(Some(self.module_bytes.clone()))
	}
}
impl ResourceResolver for StateView {
	type Error = anyhow::Error;

	fn get_resource(&self, _address: &AccountAddress, _tag: &StructTag) -> Result<Option<Vec<u8>>> {
		Ok(Some(self.bytes.clone()))
	}
}

impl GetModule for &StateView {
	type Error = anyhow::Error;
	// type Item = CompiledModule;

	fn get_module_by_id(&self, id: &ModuleId) -> Result<Option<CompiledModule>, Self::Error> {
		if let Some(bytes) = self.get_module(id)? {
			let module = CompiledModule::deserialize(&bytes)
				.map_err(|e| anyhow!("Failure deserializing module {:?}: {:?}", id, e))?;
			Ok(Some(module))
		} else {
			Ok(None)
		}
	}
}

pub fn struct_to_json(st: &StructTag, res: Vec<u8>, module: Vec<u8>) -> Result<MoveResource> {
	let view = StateView::new(res.clone(), module);
	// Internally produce FatStructType (with layout) for StructTag by
	// resolving & de-.. entire deps-chain.
	let annotator = move_resource_viewer::MoveValueAnnotator::new(&view);

	annotator.view_resource(&st, &res).and_then(|result| {
		result.try_into()
	})
}

/// Retrieve table item for a specific ledger version
pub fn table_item_key(key_type: Vec<u8>, key: Vec<u8>, module_bytes: Vec<u8>) -> Result<Vec<u8>> {
	// Parse the key and value types for the table
	let key_type = MoveType::from_str(String::from_utf8(key_type).unwrap().as_str())
		.map_err(|err| format_err!("Failed to convert value_type from str {:?}", err))?;
	let key_type = key_type
		.try_into()
		.context("Failed to parse key_type")
		.map_err(|err| format_err!("Failed to parse key_type {:?}", err,))?;
	// Convert key to lookup version for DB
	let key: Value =
		serde_json::from_str::<serde_json::Value>(String::from_utf8(key).unwrap().as_str())
			.unwrap();
	let vm_key = MoveConverter::try_into_vm_value_from_tag(&key_type, key, module_bytes)
		.map_err(|err| format_err!("Failed to try into vm value {:?}", err,))?;
	vm_key
		.undecorate()
		.simple_serialize()
		.ok_or_else(|| format_err!("Failed to serialize table key"))
}

pub fn table_item_value_bytes(value_type: StructTag, bytes: Vec<u8>, module_bytes: Vec<u8>) -> Result<Option<Vec<u8>>> {
	let move_value = MoveConverter::try_into_move_value(&TypeTag::Struct(value_type), &bytes,module_bytes)
		.context("Failed to deserialize table item retrieved from DB")
		.map_err(|err| {
			format_err!("Failed to deserialize table item retrieved from DB {:?}", err,)
		})?;
	Ok(serde_json::to_vec(&move_value).ok())
}
/// The Move converter for converting Move types to JSON
///
/// This reads the underlying BCS types and ABIs to convert them into
/// JSON outputs
pub struct MoveConverter {
}

impl MoveConverter {
	pub fn try_into_vm_values(
		params: &[MoveTypeLayout],
		args: &[String],
	) -> Result<Vec<move_core_types::value::MoveValue>> {
		let arg_types = params
			.into_iter()
			.filter(|p| if let MoveTypeLayout::Signer = p { false } else { true })
			.collect::<Vec<_>>();
		ensure!(
			arg_types.len() == args.len(),
			"expected {} arguments [{:?}], but got {} ({:?})",
			arg_types.len(),
			arg_types,
			args.len(),
			args,
		);
		arg_types
			.into_iter()
			.zip(args.into_iter().map(|x| serde_json::from_str::<serde_json::Value>(x).unwrap()))
			.enumerate()
			.map(|(i, (arg_type, arg))| {
				Self::try_into_vm_value(&arg_type.clone().try_into()?, arg).map_err(|e| {
					format_err!(
						"parse arguments[{}] failed, expect {}, caused by error: {}",
						i,
						arg_type,
						e,
					)
				})
			})
			.collect::<Result<_>>()
	}

	// Converts JSON object to `MoveValue`, which can be bcs serialized into the same
	// representation in the DB.
	// Notice that structs are of the `MoveStruct::Runtime` flavor, matching the representation in
	// DB.
	pub fn try_into_vm_value_from_tag(
		type_tag: &TypeTag,
		val: Value,
		module_bytes: Vec<u8>,
	) -> Result<move_core_types::value::MoveValue> {
		let view = StateView::new(Vec::new(), module_bytes);
		// Internally produce FatStructType (with layout) for StructTag by
		// resolving & de-.. entire deps-chain.
		let annotator = move_resource_viewer::MoveValueAnnotator::new(&view);
		let layout = annotator.get_type_layout_with_types(type_tag)?;
		Self::try_into_vm_value_from_layout(&layout, val)
	}
	pub fn try_into_vm_value(
		layout: &MoveTypeLayout,
		val: Value,
	) -> Result<move_core_types::value::MoveValue> {
		Self::try_into_vm_value_from_layout(&layout, val)
	}

	fn try_into_vm_value_from_layout(
		layout: &MoveTypeLayout,
		val: Value,
	) -> Result<move_core_types::value::MoveValue> {
		use move_core_types::value::MoveValue::*;
		//  use sp_core::U256;
		Ok(match layout {
			MoveTypeLayout::Bool => Bool(serde_json::from_value::<bool>(val)?),
			MoveTypeLayout::U8 => U8(serde_json::from_value::<u8>(val)?),
			// MoveTypeLayout::U16 => serde_json::from_value::<crate::move_types::U64>(val)?.into(),
			// MoveTypeLayout::U32 => serde_json::from_value::<crate::move_types::U64>(val)?.into(),
			MoveTypeLayout::U64 => U64(serde_json::from_value::<u64>(val)?),
			MoveTypeLayout::U128 => U128(serde_json::from_value::<u128>(val)?),
			// MoveTypeLayout::U256 =>
			// serde_json::from_value::<crate::move_types::U128>(val)?.into(),
			MoveTypeLayout::Address =>
				serde_json::from_value::<crate::api::address::Address>(val)?.into(),
			MoveTypeLayout::Vector(item_layout) =>
				Self::try_into_vm_value_vector(item_layout.as_ref(), val)?,
			MoveTypeLayout::Struct(struct_layout) =>
				Self::try_into_vm_value_struct(struct_layout, val)?,
			MoveTypeLayout::Signer => {
				bail!("unexpected move type {:?} for value {:?}", layout, val)
			},
		})
	}

	pub fn try_into_vm_value_vector(
		layout: &MoveTypeLayout,
		val: Value,
	) -> Result<move_core_types::value::MoveValue> {
		if matches!(layout, MoveTypeLayout::U8) {
			Ok(serde_json::from_value::<HexEncodedBytes>(val)?.into())
		} else if let Value::Array(list) = val {
			let vals = list
				.into_iter()
				.map(|v| Self::try_into_vm_value_from_layout(layout, v))
				.collect::<Result<_>>()?;

			Ok(move_core_types::value::MoveValue::Vector(vals))
		} else {
			bail!("expected vector<{:?}>, but got: {:?}", layout, val)
		}
	}

	pub fn try_into_vm_value_struct(
		layout: &MoveStructLayout,
		val: Value,
	) -> Result<move_core_types::value::MoveValue> {
		let field_layouts = if let MoveStructLayout::WithFields(fields) = layout {
			fields
		} else {
			bail!("Expecting `MoveStructLayout::WithTypes`, getting {:?}", layout);
		};
		let mut field_values = if let Value::Object(fields) = val {
			fields
		} else {
			bail!("Expecting a JSON Map for struct.");
		};
		let fields = field_layouts
			.iter()
			.map(|field_layout| {
				let name = field_layout.name.as_str();
				let value = field_values
					.remove(name)
					.ok_or_else(|| format_err!("field {} not found.", name))?;
				let move_value = Self::try_into_vm_value_from_layout(&field_layout.layout, value)?;
				Ok(move_value)
			})
			.collect::<Result<_>>()?;

		Ok(move_core_types::value::MoveValue::Struct(move_core_types::value::MoveStruct::Runtime(
			fields,
		)))
	}

	pub fn try_into_move_value(typ: &TypeTag, bytes: &[u8],module_bytes: Vec<u8>,) -> Result<MoveValue> {
		let view = StateView::new(bytes.to_vec(), module_bytes);
		// Internally produce FatStructType (with layout) for StructTag by
		// resolving & de-.. entire deps-chain.
		let annotator = move_resource_viewer::MoveValueAnnotator::new(&view);
		println!("try_into_move_value=in=834=annotator.view_value(typ, bytes)===={:?}=", annotator.view_value(typ, bytes));
		annotator.view_value(typ, bytes)?.try_into()
	}
}

pub fn new_vm_utf8_string(string: &str) -> move_core_types::value::MoveValue {
	use move_core_types::value::{MoveStruct, MoveValue};

	let byte_vector =
		MoveValue::Vector(string.as_bytes().iter().map(|byte| MoveValue::U8(*byte)).collect());
	let move_string = MoveStruct::Runtime(vec![byte_vector]);
	MoveValue::Struct(move_string)
}

