//! Autogenerated weights for user_profile
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-10-13, STEPS: `[20, ]`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/debio-node
// benchmark
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// user-profile
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --raw
// --output
// ./runtime/src/weights

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for user_profile.
pub trait WeightInfo {
	fn set_eth_address() -> Weight;
}

/// Weight functions for user_profile.
pub struct DeBioWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for DeBioWeight<T> {
	fn set_eth_address() -> Weight {
		32_858_000_u64.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn set_eth_address() -> Weight {
		32_858_000_u64.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}
