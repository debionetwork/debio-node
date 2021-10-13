
//! Autogenerated weights for electronic_medical_record
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
// electronic-medical-record
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

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for electronic_medical_record.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> electronic_medical_record::WeightInfo for WeightInfo<T> {
	fn add_electronic_medical_record() -> Weight {
		(32_783_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn remove_electronic_medical_record() -> Weight {
		(29_591_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn add_electronic_medical_record_info() -> Weight {
		(54_553_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
}
