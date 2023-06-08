use std::str::FromStr;
use anyhow::{bail, Error, Result};
use move_core_types::{
	account_address::AccountAddress,
	identifier::Identifier,
	language_storage::ModuleId as InternalModuleId,
	value::{MoveTypeLayout, MoveValue},
};

use crate::{
	addr,
	info::find_script_function,
	model::{from_str, new_func_tx, Signers},
	move_types::MoveModuleBytecode,
};
use move_vm::types::Signer;
use codec::Encode;
use move_binary_format::CompiledModule;
pub fn parse_function_string(
	addr: &String,
	module: &String,
) -> Result<(Option<Vec<u8>>, AccountAddress), Vec<u8>> {
	let owner_address = pontem_parse_address(addr).unwrap();
	let module = Identifier::from_str(module).unwrap();
	Ok((
		Some(bcs_alt::to_bytes(&InternalModuleId::new(owner_address, module)).unwrap()),
		owner_address,
	))
}

pub fn move_module_id_to_module_id<AccountId: Encode>(
	owner: &AccountId,
	module: Vec<u8>,
) -> Result<Option<Vec<u8>>, Vec<u8>> {
	Ok(Some(
		InternalModuleId::new(
			addr::account_to_account_address(&owner),
			Identifier::from_utf8(module).unwrap(),
		)
		.access_vector(),
	))
}
#[allow(unused)]
fn diem_root_address() -> AccountAddress {
	AccountAddress::from_hex_literal("0xA550C18")
		.expect("Parsing valid hex literal should always succeed")
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn make_abi(
	module: &Vec<u8>,
) -> Result<MoveModuleBytecode, Error> {
	// .context("Failed to parse move module ABI")
	MoveModuleBytecode::new(module.clone())
		.try_parse_abi()
		.map_err(|err| anyhow::anyhow!(" Failed to parse abi. Error:'{:?}'", err))
}
#[allow(clippy::too_many_arguments)]
pub(crate) fn make_function_call(
	module: &Vec<u8>,
	addr: AccountAddress,
	module_name: String,
	func_name: String,
	type_tag: Vec<String>,
	args: Vec<String>,
) -> Result<Vec<u8>, Error> {
	let module = CompiledModule::deserialize(module).unwrap();
	let func_name = Identifier::from_str(func_name.as_str()).unwrap();
	let module_name = Identifier::from_str(module_name.as_str()).unwrap();
	let (signers, args) = select_function(&module, &func_name, &args, &type_tag)?;
	if let Signers::Implicit(signers) = signers {
		new_func_tx(signers, addr, module_name, func_name, args, vec![])
	} else {
		new_func_tx(vec![], addr, module_name, func_name, args, vec![])
	}
}

fn select_function(
	module: &CompiledModule,
	name: &Identifier,
	args: &[String],
	type_tag: &[String],
) -> Result<(Signers, Vec<MoveValue>), Error> {
	if let Some(script) = find_script_function(module, name.as_str()) {
		if type_tag.len() != script.type_params_count() {
			return Err(anyhow::anyhow!(
				"Unable to parse AccountAddress. Maximum address length is {}.  Actual {}",
				type_tag.len(),
				script.type_params_count()
			))
		}
		prepare_function_signature(&script.parameters[..], args)
	} else {
		Err(anyhow::anyhow!(
			"Unable to parse AccountAddress. Maximum address length is {}. ",
			type_tag.len()
		))
	}

}

fn prepare_function_signature(
	code_args: &[MoveTypeLayout],
	call_args: &[String],
) -> Result<(Signers, Vec<MoveValue>), Error> {
	let signers_count = code_args
		.iter()
		.take_while(|tp| if let MoveTypeLayout::Signer = **tp { true } else { false })
		.count();
	let params_count = code_args.len() - signers_count;
	if call_args.len() < params_count {
		bail!("The function accepts {} parameters, {} are passed", params_count, call_args.len());
	}
	let args_index = call_args.len() - params_count;

	let params = crate::convert::MoveConverter::try_into_vm_values(
		&code_args[signers_count..],
		&call_args[args_index..],
	)?;
	let mut signers = (0..signers_count)
		.take_while(|i| *i < args_index)
		.map(|i| from_str(&call_args[i]))
		.take_while(|s| s.is_some())
		.flatten()
		.collect::<Vec<_>>();
	let explicit_signers = signers.len();
	for _ in explicit_signers..signers_count {
		signers.push(Signer::Placeholder);
	}
	Ok((Signers::Implicit(signers), params))
}

pub fn pontem_parse_address(addr: &str) -> Result<AccountAddress> {
	if !addr.starts_with("0x") {
		// first try ss58 parsing
		use sp_core::crypto::Ss58Codec;
		let signer = sp_core::sr25519::Public::from_ss58check_with_version(addr).unwrap().0;
		let address = crate::addr::account_to_account_address(&signer);
		Ok(address)
	} else {
		let addr = addr.to_string();
		// try parsing hex diem/aptos address with optional 0x prefix
		let max_hex_len = AccountAddress::LENGTH * 2 + 2;
		if addr.len() > max_hex_len {
			return Err(anyhow::anyhow!(
				"Unable to parse AccountAddress. Maximum address length is {}.  Actual {}",
				max_hex_len,
				addr
			))
		}
		use anyhow::Context;
		AccountAddress::from_hex_literal(&addr)
			.with_context(|| format!("Address {:?} is not a valid diem/pont address", addr))
	}
}
#[allow(unused)]
fn parse_address(
	arg_value: &str,
) -> Result<AccountAddress, Error> {
	match pontem_parse_address(arg_value) {
		Ok(addr) => Ok(addr),
		Err(_) => bail!("Failed to find address with name:{}", arg_value),
	}
}

#[cfg(test)]
mod call_tests {
	use move_core_types::{
		account_address::AccountAddress,
		language_storage::CORE_CODE_ADDRESS,
		value::{MoveTypeLayout, MoveValue},
	};
	use crate::fn_call::prepare_function_signature;

	fn s(v: &str) -> String {
		v.to_string()
	}

	fn addr(v: &str) -> AccountAddress {
		AccountAddress::from_hex_literal(v).unwrap()
	}

	#[test]
	fn test_args_types() {
		let (signers, args) = prepare_function_signature(&[], &[]).unwrap();
		assert_eq!(signers.len(), 0);
		assert_eq!(args.len(), 0);

		let (signers, args) = prepare_function_signature(&[MoveTypeLayout::U8], &[s("1")]).unwrap();
		assert_eq!(signers.len(), 0);
		assert_eq!(args, vec![MoveValue::U8(1)]);

		let (signers, args) = prepare_function_signature(
			&[MoveTypeLayout::Bool, MoveTypeLayout::Bool],
			&[s("true"), s("false")],
		)
		.unwrap();
		assert_eq!(signers.len(), 0);
		assert_eq!(args, vec![MoveValue::Bool(true), MoveValue::Bool(false)]);

		let (signers, args) = prepare_function_signature(
			&[MoveTypeLayout::U64, MoveTypeLayout::U64, MoveTypeLayout::U128],
			&[s("0"), s("1000000000"), s("10000000000000000")],
		)
		.unwrap();
		assert_eq!(signers.len(), 0);
		assert_eq!(
			args,
			vec![MoveValue::U64(0), MoveValue::U64(1000000000), MoveValue::U128(10000000000000000),]
		);

		let (signers, args) =
			prepare_function_signature(&[MoveTypeLayout::Address], &[s("\"0x1\"")]).unwrap();
		assert_eq!(signers.len(), 0);
		assert_eq!(args, vec![MoveValue::Address(CORE_CODE_ADDRESS)]);

		let (signers, args) = prepare_function_signature(
			&[
				MoveTypeLayout::Vector(Box::new(MoveTypeLayout::Bool)),
				MoveTypeLayout::Vector(Box::new(MoveTypeLayout::U8)),
				MoveTypeLayout::Vector(Box::new(MoveTypeLayout::U8)),
				MoveTypeLayout::Vector(Box::new(MoveTypeLayout::U8)),
				MoveTypeLayout::Vector(Box::new(MoveTypeLayout::U64)),
				MoveTypeLayout::Vector(Box::new(MoveTypeLayout::U128)),
				MoveTypeLayout::Vector(Box::new(MoveTypeLayout::Address)),
			],
			&[
				s("[true, false]"),
				s("\"0x1000\""),
				s("\"\""),
				s("\"0x0102\""),
				s("[1000, 0]"),
				s("[0]"),
				s("[\"0x1\",\"0x2\"]"),
			],
		)
		.unwrap();
		assert_eq!(signers.len(), 0);
		assert_eq!(
			args,
			vec![
				MoveValue::Vector(vec![MoveValue::Bool(true), MoveValue::Bool(false)]),
				MoveValue::Vector(vec![MoveValue::U8(16), MoveValue::U8(0)]),
				MoveValue::Vector(vec![]),
				MoveValue::Vector(vec![MoveValue::U8(1), MoveValue::U8(2)]),
				MoveValue::Vector(vec![MoveValue::U64(1000), MoveValue::U64(0)]),
				MoveValue::Vector(vec![MoveValue::U128(0)]),
				MoveValue::Vector(vec![
					MoveValue::Address(addr("0x1")),
					MoveValue::Address(addr("0x2"))
				]),
			]
		);
	}
}
