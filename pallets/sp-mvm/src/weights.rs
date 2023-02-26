// Copyright 2020-2021 Pontem Foundation LTD.
// This file is part of Pontem Network.
// Apache 2.0

//! Autogenerated weights for sp_mvm
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-12-02, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:

// ./target/release/pontem
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=sp-mvm
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/sp-mvm/src/weights.rs
// --template=./weight-template.hbs
// --extrinsic=*


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for sp_mvm.
pub trait WeightInfo {
	fn publish_module() -> Weight;
	fn execute() -> Weight;
	fn publish_empty_module() -> Weight;
	fn publish_many_deps_module() -> Weight;
	fn publish_s_module() -> Weight;
	fn publish_m_module() -> Weight;
	fn publish_l_module() -> Weight;
	fn execute_many_params() -> Weight;
	fn execute_store() -> Weight;
	fn execute_load() -> Weight;
	fn execute_store_event() -> Weight;
	fn execute_vec_input() -> Weight;
	fn execute_loop() -> Weight;
	
}

/// Weights for sp_mvm using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {	
	// Storage: Mvm VMStorage (r:2 w:1)
	fn publish_module() -> Weight {
Weight::from_ref_time((61_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64).ref_time())
			.saturating_add(T::DbWeight::get().writes(1_u64).ref_time()))
	}
	// Storage: Mvm VMStorage (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	fn execute() -> Weight {
Weight::from_ref_time((19_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64).ref_time()))		
	}
	// Storage: Mvm VMStorage (r:2 w:1)
	fn publish_empty_module() -> Weight {
Weight::from_ref_time((62_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64).ref_time())
			.saturating_add(T::DbWeight::get().writes(1_u64).ref_time()))
	}

	// Storage: Mvm VMStorage (r:30 w:1)
	fn publish_many_deps_module() -> Weight {
Weight::from_ref_time((519_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64).ref_time())
			.saturating_add(T::DbWeight::get().writes(1_u64).ref_time()))
	}
	
	
	// Storage: Mvm VMStorage (r:2 w:1)
	
	fn publish_s_module() -> Weight {
Weight::from_ref_time((121_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64).ref_time())
			.saturating_add(T::DbWeight::get().writes(1_u64).ref_time()))
	}
	// Storage: Mvm VMStorage (r:2 w:1)
	fn publish_m_module() -> Weight {
Weight::from_ref_time((4_253_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64).ref_time())
			.saturating_add(T::DbWeight::get().writes(1_u64).ref_time()))
	}
	// Storage: Mvm VMStorage (r:2 w:1)
	fn publish_l_module() -> Weight {
Weight::from_ref_time((68_396_000_000_u64)		
			.saturating_add(T::DbWeight::get().reads(1_u64).ref_time())
			.saturating_add(T::DbWeight::get().writes(1_u64).ref_time()))
	}
	// Storage: Mvm VMStorage (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	fn execute_many_params() -> Weight {
Weight::from_ref_time((116_000_000_u64)		
			.saturating_add(T::DbWeight::get().reads(1_u64).ref_time()))
	}
	
	// Storage: Mvm VMStorage (r:3 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	fn execute_store() -> Weight {
Weight::from_ref_time((84_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64).ref_time())
			.saturating_add(T::DbWeight::get().writes(1_u64).ref_time()))
	}
	// Storage: Mvm VMStorage (r:3 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	fn execute_load() -> Weight {
Weight::from_ref_time((49_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64).ref_time()))
	}
	// Storage: Mvm VMStorage (r:7 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	fn execute_store_event() -> Weight {
Weight::from_ref_time((563_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64).ref_time())
			.saturating_add(T::DbWeight::get().writes(1_u64).ref_time()))
	}
	
	// Storage: Mvm VMStorage (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	fn execute_vec_input() -> Weight {
Weight::from_ref_time((24_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64).ref_time()))
	}
	
	// Storage: Mvm VMStorage (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	fn execute_loop() -> Weight {
Weight::from_ref_time((179_783_000_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64).ref_time()))		
	}
	
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Mvm VMStorage (r:2 w:1)
	fn publish_module() -> Weight {
Weight::from_ref_time((61_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64).ref_time())
			.saturating_add(RocksDbWeight::get().writes(1_u64).ref_time()))
	}
	// Storage: Mvm VMStorage (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	fn execute() -> Weight {
Weight::from_ref_time((19_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64).ref_time()))
	}
	// Storage: Mvm VMStorage (r:2 w:1)
	fn publish_empty_module() -> Weight {
Weight::from_ref_time((62_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64).ref_time())
			.saturating_add(RocksDbWeight::get().writes(1_u64).ref_time()))
	}
	// Storage: Mvm VMStorage (r:30 w:1)
	fn publish_many_deps_module() -> Weight {
Weight::from_ref_time((519_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64).ref_time())
			.saturating_add(RocksDbWeight::get().writes(1_u64).ref_time()))
	}
	// Storage: Mvm VMStorage (r:2 w:1)
	fn publish_s_module() -> Weight {
Weight::from_ref_time((121_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64).ref_time())
			.saturating_add(RocksDbWeight::get().writes(1_u64).ref_time()))
	}
	// Storage: Mvm VMStorage (r:2 w:1)
	fn publish_m_module() -> Weight {
Weight::from_ref_time((4_253_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64).ref_time())
			.saturating_add(RocksDbWeight::get().writes(1_u64).ref_time()))
	}
	// Storage: Mvm VMStorage (r:2 w:1)
	fn publish_l_module() -> Weight {
Weight::from_ref_time((68_396_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64).ref_time())
			.saturating_add(RocksDbWeight::get().writes(1_u64).ref_time()))
	}
	// Storage: Mvm VMStorage (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	fn execute_many_params() -> Weight {
Weight::from_ref_time((116_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64).ref_time()))
	}
	// Storage: Mvm VMStorage (r:3 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	fn execute_store() -> Weight {
Weight::from_ref_time((84_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64).ref_time())
			.saturating_add(RocksDbWeight::get().writes(1_u64).ref_time()))
	}
	// Storage: Mvm VMStorage (r:3 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	fn execute_load() -> Weight {
Weight::from_ref_time((49_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64).ref_time()))
	}
	// Storage: Mvm VMStorage (r:7 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	fn execute_store_event() -> Weight {
Weight::from_ref_time((563_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64).ref_time())
			.saturating_add(RocksDbWeight::get().writes(1_u64).ref_time()))
	}
	// Storage: Mvm VMStorage (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	fn execute_vec_input() -> Weight {
Weight::from_ref_time((24_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64).ref_time()))
	}
	// Storage: Mvm VMStorage (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	fn execute_loop() -> Weight {
Weight::from_ref_time((179_783_000_000_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64).ref_time()))
	}
}
