use codec::{self, Codec};
use std::{convert::From, sync::Arc};
use jsonrpsee::{
	core::{Error as JsonRpseeError, RpcResult as Result},
	proc_macros::rpc,
	types::error::{CallError, ErrorObject},
};

use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_mvm_rpc_runtime::{types::MVMApiEstimation, MVMApiRuntime};
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
// use frame_support::weights::Weight;
use fc_rpc_core::types::Bytes;
use serde::{Deserialize, Serialize};
pub mod api;
// pub mod api::addr;
// pub mod api::address;
// pub mod api::bytecode;
// pub mod api::constant;
// pub mod api::convert;
// pub mod api::fn_call;
// pub mod api::info;
// pub mod api::model;
// pub mod api::move_types;
// pub mod api::wrappers;
// pub mod api::api_state_view;
use crate::api::api_state_view::ApiStateView;
pub use crate::api::move_types::MoveModuleBytecode;
// pub struct ApiStateView<C, BlockHash, AccountId, Block> {
// 	client: Arc<C>,
// 	account_id: AccountId,
// 	at: Option<BlockHash>,
// 	_marker: std::marker::PhantomData<Block>,
// }
// impl<C, Block, AccountId> ApiStateView<C, <Block as BlockT>::Hash, AccountId, Block>
// where
// 	Block: BlockT,
// {
// 	pub fn new(client: Arc<C>, account_id: AccountId, at: Option<<Block as BlockT>::Hash>) -> Self {
// 		Self { client, account_id, at, _marker: Default::default() }
// 	}
// }
// impl<C, Block, AccountId> ModuleResolver
// 	for ApiStateView<C, <Block as BlockT>::Hash, AccountId, Block>
// where
// 	Block: BlockT,
// 	AccountId: Clone + std::fmt::Display + Codec,
// 	C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
// 	C::Api: MVMApiRuntime<Block, AccountId>,
// {
// 	type Error = anyhow::Error;

// 	fn get_module(&self, module_id: &ModuleId) -> anyhow::Result<Option<Vec<u8>>> {
// 		let api = self.client.runtime_api();
// 		let at = BlockId::hash(self.at.unwrap_or_else(||
// 			// If the block hash is not supplied assume the best block.
// 			self.client.info().best_hash));
// 		let bytes: Option<Vec<u8>> = api
// 			.get_module(&at, bcs_alt::to_bytes(module_id).unwrap())
// 			.map_err(runtime_error_into_rpc_err4)?
// 			.map_err(runtime_error_into_rpc_err5)?;
// 		Ok(bytes)
// 	}
// }
// impl<C, Block, AccountId> ResourceResolver
// 	for ApiStateView<C, <Block as BlockT>::Hash, AccountId, Block>
// where
// 	Block: BlockT,
// 	AccountId: Clone + std::fmt::Display + Codec,
// 	C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
// 	C::Api: MVMApiRuntime<Block, AccountId>,
// {
// 	type Error = anyhow::Error;

// 	fn get_resource(
// 		&self,
// 		_address: &AccountAddress,
// 		tag: &StructTag,
// 	) -> anyhow::Result<Option<Vec<u8>>> {
// 		let api = self.client.runtime_api();
// 		let at = BlockId::hash(self.at.unwrap_or_else(||
// 			// If the block hash is not supplied assume the best block.
// 			self.client.info().best_hash));
// 		let bytes: Option<Vec<u8>> = api
// 			.get_resource(&at, self.account_id.clone(), bcs_alt::to_bytes(tag).unwrap())
// 			.map_err(runtime_error_into_rpc_err4)?
// 			.map_err(runtime_error_into_rpc_err5)?;
// 		Ok(bytes)
// 	}
// }

// Estimation struct with serde.
#[derive(Serialize, Deserialize)]
pub struct Estimation {
	pub gas_used: u64,
	pub status_code: u64,
}

impl From<MVMApiEstimation> for Estimation {
	fn from(e: MVMApiEstimation) -> Self {
		Self { gas_used: e.gas_used, status_code: e.status_code }
	}
}

// RPC calls.
#[rpc(client, server)]
pub trait MVMApiRpc<BlockHash, AccountId> {
	#[method(name = "mvm_gasToWeight")]
	fn gas_to_weight(&self, gas: u64, at: Option<BlockHash>) -> Result<u64>;

	#[method(name = "mvm_weightToGas")]
	fn weight_to_gas(&self, weight: u64, at: Option<BlockHash>) -> Result<u64>;

	#[method(name = "mvm_estimateGasPublish")]
	fn estimate_gas_publish(
		&self,
		account: AccountId,
		module_bc: Bytes,
		gas_limit: u64,
		at: Option<BlockHash>,
	) -> Result<Estimation>;

	#[method(name = "mvm_estimateGasExecute")]
	fn estimate_gas_execute(
		&self,
		account: AccountId,
		tx_bc: Bytes,
		gas_limit: u64,
		at: Option<BlockHash>,
	) -> Result<Estimation>;

	#[method(name = "mvm_getResource")]
	fn get_resource(
		&self,
		account_id: AccountId,
		tag: Bytes,
		at: Option<BlockHash>,
	) -> Result<Option<Bytes>>;

	#[method(name = "mvm_getModuleABI")]
	fn get_module_abi(&self, module_id: Bytes, at: Option<BlockHash>) -> Result<Option<Bytes>>;

	#[method(name = "mvm_getModule")]
	fn get_module(&self, module_id: Bytes, at: Option<BlockHash>) -> Result<Option<Bytes>>;

	#[method(name = "mvm_encodeSubmission")]
	fn encode_submission(
		&self,
		function: Vec<Bytes>,
		arguments: Vec<Bytes>,
		type_parameters: Vec<Bytes>,
		at: Option<BlockHash>,
	) -> Result<Option<Bytes>>;

	#[method(name = "mvm_getModuleABIs")]
	fn get_module_abis(&self, module_id: Bytes, at: Option<BlockHash>) -> Result<Option<Bytes>>;

	// #[method(name = "mvm_getModuleABIs2")]
	// fn get_module_abis2(
	// 	&self,
	// 	module_id: Bytes,
	// 	at: Option<BlockHash>,
	// ) -> Result<Option<MoveModuleBytecode>>;

	#[method(name = "mvm_getResources")]
	fn get_resources(
		&self,
		account_id: AccountId,
		tag: Bytes,
		at: Option<BlockHash>,
	) -> Result<Option<Bytes>>;
	// #[method(name = "mvm_getResources2")]
	// fn get_resources2(
	// 	&self,
	// 	account_id: AccountId,
	// 	tag: Bytes,
	// 	at: Option<BlockHash>,
	// ) -> Result<Option<Bytes>>;
	// #[method(name = "mvm_getResources3")]
	// fn get_resources3(
	// 	&self,
	// 	account_id: AccountId,
	// 	tag: Bytes,
	// 	at: Option<BlockHash>,
	// ) -> Result<Option<Bytes>>;

	#[method(name = "mvm_getTableEntry")]
	fn get_table_entry(
		&self,
		handle: Bytes,
		key: Bytes,
		key_type: Bytes,
		value_type: Bytes,
		at: Option<BlockHash>,
	) -> Result<Option<Bytes>>;
}

pub struct MVMApi<C, P> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> MVMApi<C, P> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

impl<C, Block, AccountId> MVMApiRpcServer<<Block as BlockT>::Hash, AccountId> for MVMApi<C, Block>
where
	Block: BlockT,
	AccountId: Clone + std::fmt::Display + Codec,
	C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: MVMApiRuntime<Block, AccountId>,
{
	fn gas_to_weight(&self, gas: u64, at: Option<<Block as BlockT>::Hash>) -> Result<u64> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let res = api.gas_to_weight(&at, gas);

		res.map_err(runtime_error_into_rpc_err)
	}

	fn weight_to_gas(&self, weight: u64, at: Option<<Block as BlockT>::Hash>) -> Result<u64> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let res = api.weight_to_gas(&at, weight);

		res.map_err(runtime_error_into_rpc_err)
	}

	fn estimate_gas_publish(
		&self,
		account: AccountId,
		module_bc: Bytes,
		gas_limit: u64,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<Estimation> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let res = api
			.estimate_gas_publish(&at, account, module_bc.into_vec(), gas_limit)
			.map_err(runtime_error_into_rpc_err)?;

		let mvm_estimation = res.map_err(runtime_error_into_rpc_err2)?;

		Ok(Estimation::from(mvm_estimation))
	}

	fn estimate_gas_execute(
		&self,
		account: AccountId,
		tx_bc: Bytes,
		gas_limit: u64,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<Estimation> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let res = api
			.estimate_gas_execute(&at, account, tx_bc.into_vec(), gas_limit)
			.map_err(runtime_error_into_rpc_err)?;

		let mvm_estimation = res.map_err(runtime_error_into_rpc_err3)?;

		Ok(Estimation::from(mvm_estimation))
	}

	fn get_resource(
		&self,
		account_id: AccountId,
		tag: Bytes,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<Option<Bytes>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let f: Option<Vec<u8>> = api
			.get_resource(&at, account_id, tag.into_vec())
			.map_err(runtime_error_into_rpc_err4)?
			.map_err(runtime_error_into_rpc_err5)?;
		Ok(f.map(Into::into))
	}

	fn get_module_abi(
		&self,
		module_id: Bytes,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<Option<Bytes>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let f: Option<Vec<u8>> = api
			.get_module_abi(&at, module_id.into_vec())
			.map_err(runtime_error_into_rpc_err4)?
			.map_err(runtime_error_into_rpc_err5)?;
		Ok(f.map(Into::into))
	}

	fn get_module(
		&self,
		module_id: Bytes,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<Option<Bytes>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let f: Option<Vec<u8>> = api
			.get_module(&at, module_id.into_vec())
			.map_err(runtime_error_into_rpc_err4)?
			.map_err(runtime_error_into_rpc_err5)?;
		Ok(f.map(Into::into))
	}

	fn encode_submission(
		&self,
		function: Vec<Bytes>,
		arguments: Vec<Bytes>,
		type_parameters: Vec<Bytes>,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<Option<Bytes>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));
		let ff = function
			.into_iter()
			.map(|func| String::from_utf8(func.into_vec()).unwrap())
			.collect::<Vec<String>>();
		let ((module_id, module_address), module_name, func) = (
			crate::api::fn_call::parse_function_string(&ff[0], &ff[1]).unwrap(),
			ff[1].clone(),
			ff[2].clone(),
		);
		let f: Option<Vec<u8>> = api
			.get_module(&at, module_id.unwrap())
			.map_err(runtime_error_into_rpc_err4)?
			.map_err(runtime_error_into_rpc_err4)?;
		let f = crate::api::fn_call::make_function_call(
			&f.as_ref().unwrap(),
			module_address,
			module_name,
			func,
			type_parameters
				.into_iter()
				.map(|a| String::from_utf8(a.into_vec()).unwrap())
				.collect(),
			arguments
				.into_iter()
				.map(|a| String::from_utf8(a.into_vec()).unwrap())
				.collect(),
		)
		.map_err(runtime_error_into_rpc_err4)
		.ok();
		Ok(f.map(Into::into))
	}

	fn get_module_abis(
		&self,
		module_id: Bytes,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<Option<Bytes>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let f: Option<Vec<u8>> = api
			.get_module(&at, module_id.into_vec())
			.map_err(runtime_error_into_rpc_err4)?
			.map_err(runtime_error_into_rpc_err4)?;
        if f.is_none(){
            return Err(runtime_error_into_rpc_err7())
        }
		let f = crate::api::fn_call::make_abi(&f.as_ref().unwrap())
			.map_err(runtime_error_into_rpc_err4)
			.ok();
        if f.is_none(){
            return Err(runtime_error_into_rpc_err7())
        }
		let f = serde_json::to_vec(&f.as_ref().unwrap()).ok();
		Ok(f.map(Into::into))
	}

	// fn get_module_abis(
	// 	&self,
	// 	module_id: Bytes,
	// 	at: Option<<Block as BlockT>::Hash>,
	// ) -> Result<Option<MoveModuleBytecode>> {
	// 	let api = self.client.runtime_api();
	// 	let at = BlockId::hash(at.unwrap_or_else(||
	// 		// If the block hash is not supplied assume the best block.
	// 		self.client.info().best_hash));

	// 	let f: Option<Vec<u8>> = api
	// 		.get_module(&at, module_id.into_vec())
	// 		.map_err(runtime_error_into_rpc_err4)?
	// 		.map_err(runtime_error_into_rpc_err4)?;
	// 	let f = crate::fn_call::make_abi(&f.as_ref().unwrap())
	// 		.map_err(runtime_error_into_rpc_err4)
	// 		.ok();
	// 	Ok(f.map(Into::into))
	// }

	// fn get_resources(
	// 	&self,
	// 	account_id: AccountId,
	// 	tag: Bytes,
	// 	at: Option<<Block as BlockT>::Hash>,
	// ) -> Result<Option<Bytes>> {
	// 	let api = self.client.runtime_api();
	// 	let at = BlockId::hash(at.unwrap_or_else(||
	// 		// If the block hash is not supplied assume the best block.
	// 		self.client.info().best_hash));
	// 	let (tag_bcs, tag, module_id) = convert::parse_struct_tag_string(tag.into_vec()).unwrap();
	// 	let f: Option<Vec<u8>> = api
	// 		.get_resource(&at, account_id, tag_bcs)
	// 		.map_err(runtime_error_into_rpc_err4)?
	// 		.map_err(runtime_error_into_rpc_err5)?;
	// 	let module: Option<Vec<u8>> = api
	// 		.get_module(&at, module_id)
	// 		.map_err(runtime_error_into_rpc_err4)?
	// 		.map_err(runtime_error_into_rpc_err5)?;
	// 	let f = convert::struct_to_json(&tag, f.unwrap(), module.unwrap())
	// 		.map_err(runtime_error_into_rpc_err4)
	// 		.map_err(runtime_error_into_rpc_err6)?;
	// 	let ff = serde_json::to_vec(&f).ok();
	// 	println!("get_resources=result==={:?}=={:?}=", f, ff);
	// 	let f = ff;
	// 	Ok(f.map(Into::into))
	// }

	// fn get_resources2(
	// 	&self,
	// 	account_id: AccountId,
	// 	tag: Bytes,
	// 	at: Option<<Block as BlockT>::Hash>,
	// ) -> Result<Option<Bytes>> {
	// 	let api = self.client.runtime_api();
	// 	let att = BlockId::hash(at.unwrap_or_else(||
	// 		// If the block hash is not supplied assume the best block.
	// 		self.client.info().best_hash));
	// 	let (tag_bcs, tag, _module_id) = convert::parse_struct_tag_string(tag.into_vec()).unwrap();

	// 	let f: Option<Vec<u8>> = api
	// 		.get_resource(&att, account_id.clone(), tag_bcs)
	// 		.map_err(runtime_error_into_rpc_err4)?
	// 		.map_err(runtime_error_into_rpc_err5)?;
	// 	let view = ApiStateView::new(self.client.clone(), account_id.clone(), at);
	// 	// use move_resource_viewer::MoveValueAnnotator;
	// 	let annotator = move_resource_viewer::MoveValueAnnotator::new(&view);
	// 	use crate::move_types::MoveResource;

	// 	let f: MoveResource = annotator
	// 		.view_resource(&tag, &f.unwrap())
	// 		.and_then(|result| {
	// 			println!("=get_resources2===={:?}", result);
	// 			result.try_into()
	// 		})
	// 		.map_err(runtime_error_into_rpc_err5)?;

	// 	//  let module: Option<Vec<u8>> = api
	// 	//         .get_module(&at, module_id)
	// 	//         .map_err(runtime_error_into_rpc_err4)?
	// 	//         .map_err(runtime_error_into_rpc_err5)?;
	// 	// let f = convert::struct_to_json(&tag,f.unwrap(),module.unwrap()).
	// 	// map_err(runtime_error_into_rpc_err4).map_err(runtime_error_into_rpc_err6)?;
	// 	let ff = serde_json::to_vec(&f).ok();
	// 	println!("get_resources2=result==={:?}=={:?}=", f, ff);
	// 	let f = ff;
	// 	Ok(f.map(Into::into))
	// }

	fn get_resources(
		&self,
		account_id: AccountId,
		tag: Bytes,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<Option<Bytes>> {
		let api = self.client.runtime_api();
		let att = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));
		let (tag_bcs, tag, _module_id) = api::convert::parse_struct_tag_string3(tag.into_vec()).map_err(runtime_error_into_rpc_err4)?;

		let f: Option<Vec<u8>> = api
			.get_resource(&att, account_id.clone(), tag_bcs)
			.map_err(runtime_error_into_rpc_err4)?
			.map_err(runtime_error_into_rpc_err5)?;
        if f.is_none(){
            return Err(runtime_error_into_rpc_err7())
        }
		let view = ApiStateView::new(self.client.clone(), account_id.clone(), at);
		let annotator = move_resource_viewer::MoveValueAnnotator::new(&view);
		use crate::api::move_types::MoveResource;
		let f: MoveResource = annotator
			.view_resource(&tag, &f.unwrap())
			.and_then(|result| {
				result.try_into()
			})
			.map_err(runtime_error_into_rpc_err5)?;

		let f = serde_json::to_vec(&f).ok();
		Ok(f.map(Into::into))
	}

	fn get_table_entry(
		&self,
		handle: Bytes,
		key: Bytes,
		key_type: Bytes,
		value_type: Bytes,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<Option<Bytes>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));
		let (_tag_bcs, tag, module_id) =
			api::convert::parse_struct_tag_string3(value_type.clone().into_vec()).map_err(runtime_error_into_rpc_err4)?;
		let module: Option<Vec<u8>> = api
			.get_module(&at, module_id)
			.map_err(runtime_error_into_rpc_err4)?
			.map_err(runtime_error_into_rpc_err5)?;
        if module.is_none(){
            return Err(runtime_error_into_rpc_err7())
        }
		let raw_key = api::convert::table_item_key(key_type.into_vec(), key.into_vec(), module.clone().unwrap())
			.map_err(runtime_error_into_rpc_err5)?;
		let handle = std::str::from_utf8(&handle.into_vec()).unwrap().parse::<u128>().map_err(runtime_error_into_rpc_err5)?;
		let f: Option<Vec<u8>> = api
			.get_table_entry(&at, handle, raw_key)
			.map_err(runtime_error_into_rpc_err4)?
			.map_err(runtime_error_into_rpc_err5)?;
		let f: Option<Vec<u8>> =
			api::convert::table_item_value_bytes(tag, f.unwrap_or(vec![]),module.unwrap())?;
		Ok(f.map(Into::into))
	}
}
const RUNTIME_ERROR: i32 = 500;

/// Converts a runtime trap into an RPC error.
fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> JsonRpseeError {
	CallError::Custom(ErrorObject::owned(
		RUNTIME_ERROR,
		"Error during requesting Runtime API",
		Some(format!("{:?}", err)),
	))
	.into()
}

fn runtime_error_into_rpc_err2(err: impl std::fmt::Debug) -> JsonRpseeError {
	CallError::Custom(ErrorObject::owned(
		RUNTIME_ERROR,
		"Error during publishing module for estimation",
		Some(format!("{:?}", err)),
	))
	.into()
}

fn runtime_error_into_rpc_err3(err: impl std::fmt::Debug) -> JsonRpseeError {
	CallError::Custom(ErrorObject::owned(
		RUNTIME_ERROR,
		"Error during script execution  for estimation",
		Some(format!("{:?}", err)),
	))
	.into()
}

fn runtime_error_into_rpc_err4(err: impl std::fmt::Debug) -> JsonRpseeError {
	CallError::Custom(ErrorObject::owned(RUNTIME_ERROR, "ABI error", Some(format!("{:?}", err))))
		.into()
}

fn runtime_error_into_rpc_err5(err: impl std::fmt::Debug) -> JsonRpseeError {
	CallError::Custom(ErrorObject::owned(
		RUNTIME_ERROR,
		"Error from method",
		Some(format!("{:?}", err)),
	))
	.into()
}

// fn runtime_error_into_rpc_err6(err: impl std::fmt::Debug) -> JsonRpseeError {
// 	CallError::Custom(ErrorObject::owned(
// 		RUNTIME_ERROR,
// 		"Error from struct tag json",
// 		Some(format!("{:?}", err)),
// 	))
// 	.into()
// }
fn runtime_error_into_rpc_err7() -> JsonRpseeError {
	CallError::Custom(ErrorObject::owned(
		RUNTIME_ERROR,
		"Error from None ",
		Some(""),
	))
	.into()
}