
use move_binary_format::{
	access::ModuleAccess,
	file_format::{Ability, AbilitySet, SignatureToken, Visibility},
	CompiledModule,
};
use move_core_types::{account_address::AccountAddress, value::MoveTypeLayout};
use crate::constant::sig_to_ty;

#[derive(Debug)]
pub struct BytecodeInfo {
}

pub fn find_script_function(module: &CompiledModule, need_name: &str) -> Option<Script> {
	module
		.function_defs()
		.iter()
		.filter(|def| def.visibility == Visibility::Script)
		.find(|def| {
			let handle = module.function_handle_at(def.function);
			module.identifier_at(handle.name).as_str() == need_name
		})
		.map(|def| {
			let handle = module.function_handle_at(def.function);
			let parameters =
				module.signature_at(handle.parameters).0.iter().map(|p| make_type(p)).collect();

			let type_parameters = handle.type_parameters.iter().map(TypeAbilities::from).collect();
			let return_ = &module.signature_at(handle.return_).0;

			Script {
				name: String::from(module.identifier_at(handle.name).as_str()),
				parameters,
				type_parameters,
				returns: return_.iter().map(|st| make_type(st)).collect(),
			}
		})
}

#[derive(Debug)]
pub struct Script {
	pub name: String,
	pub parameters: Vec<MoveTypeLayout>,
	pub type_parameters: Vec<TypeAbilities>,
	pub returns: Vec<MoveTypeLayout>,
}

impl Script {
	pub fn type_params_count(&self) -> usize {
		self.type_parameters.len()
	}
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Type {
	Bool,
	U8,
	U64,
	U128,
	Address,
	Signer,
	Vector(Box<Type>),
	Struct(StructDef),
	Reference(Box<Type>),
	MutableReference(Box<Type>),
	TypeParameter(u16),
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct StructDef {
	pub address: AccountAddress,
	pub module_name: String,
	pub name: String,
	pub type_parameters: Vec<Type>,
}

fn make_type(tok: &SignatureToken) -> MoveTypeLayout {
	sig_to_ty(tok).unwrap()
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct TypeAbilities {
	pub abilities: Vec<TypeAbility>,
}

impl From<&AbilitySet> for TypeAbilities {
	fn from(val: &AbilitySet) -> Self {
		TypeAbilities {
			abilities: val
				.into_iter()
				.map(|a| match a {
					Ability::Copy => TypeAbility::Copy,
					Ability::Drop => TypeAbility::Drop,
					Ability::Store => TypeAbility::Store,
					Ability::Key => TypeAbility::Key,
				})
				.collect::<Vec<TypeAbility>>(),
		}
	}
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum TypeAbility {
	Copy,
	Drop,
	Store,
	Key,
}
