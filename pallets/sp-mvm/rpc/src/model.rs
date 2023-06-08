use anyhow::Error;
use move_core_types::{
	account_address::AccountAddress, identifier::Identifier, language_storage::TypeTag,
	value::MoveValue,
};
use move_vm::types::{Call, Signer, Transaction, TxV1};

/// Create a new function transaction.
pub fn new_func_tx(
	signers: Vec<Signer>,
	mod_address: AccountAddress,
	mod_name: Identifier,
	func_name: Identifier,
	args: Vec<MoveValue>,
	type_args: Vec<TypeTag>,
) -> Result<Vec<u8>, Error> {
	Ok(Transaction::V1(TxV1 {
		signers,
		call: Call::ScriptFunction { mod_address, func_name, mod_name },
		args: Transaction::args_to_vec(args).unwrap(),
		type_args,
	})
	.to_vec()
	.unwrap())
}

/// Script argument type.
#[derive(Debug, PartialEq, Eq)]
pub enum ScriptArg {
	/// u8
	U8(u8),
	/// u64
	U64(u64),
	/// u128
	U128(u128),
	/// bool
	Bool(bool),
	/// address
	Address(AccountAddress),
	/// vector<u8>
	VectorU8(Vec<u8>),
	/// vector<u64>
	VectorU64(Vec<u64>),
	/// vector<u128>
	VectorU128(Vec<u128>),
	/// vector<bool>
	VectorBool(Vec<bool>),
	/// vector<address>
	VectorAddress(Vec<AccountAddress>),
}

pub fn from_str(s: &str) -> Option<Signer> {
	Some(match s.to_lowercase().as_str() {
		"root" | "rt" | "dr" => Signer::Root,
		"_" => Signer::Placeholder,
		_ => Signer::Name(String::from(s)),
	})
}

#[derive(Debug, PartialEq)]
pub enum Signers {
	Explicit(Vec<AccountAddress>),
	Implicit(Vec<Signer>),
}

impl Signers {
	#[cfg(test)]
	pub fn len(&self) -> usize {
		match self {
			Signers::Explicit(v) => v.len(),
			Signers::Implicit(v) => v.len(),
		}
	}
}
