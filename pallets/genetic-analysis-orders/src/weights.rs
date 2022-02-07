//! Autogenerated weights for genetic_analysis_orders
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-12-01, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/debio
// benchmark
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=genetic_analysis_orders
// --extrinsic=*
// --steps=20
// --repeat=10
// --heap-pages=4096
// --raw
// --output=./pallets/genetic_analysis_orders/src/weights.rs
// --template=./.maintain/pallet-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for genetic_analysis_orders.
pub trait WeightInfo {
	fn create_genetic_analysis_order() -> Weight;
	fn cancel_genetic_analysis_order() -> Weight;
	fn fulfill_genetic_analysis_order() -> Weight;
	fn set_genetic_analysis_order_paid() -> Weight;
	fn set_genetic_analysis_order_refunded() -> Weight;
}

/// Weights for genetic_analysis_orders using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Services Services (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	// Storage: GeneticTesting DnaSamples (r:1 w:1)
	// Storage: GeneticTesting DnaSamplesByOwner (r:1 w:1)
	// Storage: GeneticTesting DnaSamplesByLab (r:1 w:1)
	// Storage: Orders OrdersBySeller (r:1 w:1)
	// Storage: Orders OrdersByCustomer (r:1 w:1)
	// Storage: Orders Orders (r:0 w:1)
	// Storage: Orders LastOrderByCustomer (r:0 w:1)
	fn create_genetic_analysis_order() -> Weight {
		(102_701_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: Orders Orders (r:1 w:1)
	// Storage: GeneticTesting DnaSamples (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	fn cancel_genetic_analysis_order() -> Weight {
		(45_842_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Orders EscrowKey (r:1 w:0)
	// Storage: Orders Orders (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	fn set_genetic_analysis_order_paid() -> Weight {
		(38_903_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Orders Orders (r:1 w:1)
	// Storage: GeneticTesting DnaSamples (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	fn fulfill_genetic_analysis_order() -> Weight {
		(45_622_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Orders EscrowKey (r:1 w:0)
	// Storage: Orders Orders (r:1 w:1)
	// Storage: GeneticTesting DnaSamples (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	fn set_genetic_analysis_order_refunded() -> Weight {
		(50_135_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Services Services (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	// Storage: GeneticTesting DnaSamples (r:1 w:1)
	// Storage: GeneticTesting DnaSamplesByOwner (r:1 w:1)
	// Storage: GeneticTesting DnaSamplesByLab (r:1 w:1)
	// Storage: Orders OrdersBySeller (r:1 w:1)
	// Storage: Orders OrdersByCustomer (r:1 w:1)
	// Storage: Orders Orders (r:0 w:1)
	// Storage: Orders LastOrderByCustomer (r:0 w:1)
	fn create_genetic_analysis_order() -> Weight {
		(102_701_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
	// Storage: Orders Orders (r:1 w:1)
	// Storage: GeneticTesting DnaSamples (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	fn cancel_genetic_analysis_order() -> Weight {
		(45_842_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Orders EscrowKey (r:1 w:0)
	// Storage: Orders Orders (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	fn set_genetic_analysis_order_paid() -> Weight {
		(38_903_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Orders Orders (r:1 w:1)
	// Storage: GeneticTesting DnaSamples (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	fn fulfill_genetic_analysis_order() -> Weight {
		(45_622_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Orders EscrowKey (r:1 w:0)
	// Storage: Orders Orders (r:1 w:1)
	// Storage: GeneticTesting DnaSamples (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	fn set_genetic_analysis_order_refunded() -> Weight {
		(50_135_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}