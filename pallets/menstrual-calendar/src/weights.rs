//! Autogenerated weights for menstrual_calendar
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-18, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/debio
// benchmark
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=menstrual-calendar
// --extrinsic=*
// --steps=20
// --repeat=10
// --heap-pages=4096
// --raw
// --output=./pallets/menstrual-calendar/src/weights.rs
// --template=./.maintain/pallet-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for menstrual_calendar.
pub trait WeightInfo { 
	fn add_menstrual_calendar() -> Weight; 
	fn update_menstrual_calendar() -> Weight; 
	fn remove_menstrual_calendar() -> Weight; 
}

/// Weights for menstrual_calendar using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>); 
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> { 
	// Storage: MentrualCalendar MentrualCalendarCountByOwner (r:1 w:1) 
	// Storage: Timestamp Now (r:1 w:0) 
	// Storage: MentrualCalendar MentrualCalendarByOwner (r:1 w:1) 
	// Storage: MentrualCalendar MentrualCalendarCount (r:1 w:1) 
	// Storage: MentrualCalendar MentrualCalendarById (r:0 w:1) 
	fn add_menstrual_calendar() -> Weight { 
		89_688_000_u64 
			.saturating_add(T::DbWeight::get().reads(4_u64)) 
			.saturating_add(T::DbWeight::get().writes(4_u64)) 
	}
	// Storage: MentrualCalendar MentrualCalendarById (r:1 w:1) 
	// Storage: Timestamp Now (r:1 w:0) 
	fn update_menstrual_calendar() -> Weight { 
		107_069_000_u64 
			.saturating_add(T::DbWeight::get().reads(2_u64)) 
			.saturating_add(T::DbWeight::get().writes(1_u64)) 
	}
	// Storage: MentrualCalendar MentrualCalendarById (r:1 w:1) 
	// Storage: MentrualCalendar MentrualCalendarByOwner (r:1 w:1) 
	// Storage: MentrualCalendar MentrualCalendarCount (r:1 w:1) 
	// Storage: MentrualCalendar MentrualCalendarCountByOwner (r:1 w:1) 
	fn remove_menstrual_calendar() -> Weight { 
		47_647_000_u64 
			.saturating_add(T::DbWeight::get().reads(4_u64)) 
			.saturating_add(T::DbWeight::get().writes(4_u64)) 
	}
}

// For backwards compatibility and tests
impl WeightInfo for () { 
	// Storage: MentrualCalendar MentrualCalendarCountByOwner (r:1 w:1) 
	// Storage: Timestamp Now (r:1 w:0) 
	// Storage: MentrualCalendar MentrualCalendarByOwner (r:1 w:1) 
	// Storage: MentrualCalendar MentrualCalendarCount (r:1 w:1) 
	// Storage: MentrualCalendar MentrualCalendarById (r:0 w:1) 
	fn add_menstrual_calendar() -> Weight { 
		89_688_000_u64
			.saturating_add(RocksDbWeight::get().reads(4_u64)) 
			.saturating_add(RocksDbWeight::get().writes(4_u64)) 
	} 
	// Storage: MentrualCalendar MentrualCalendarById (r:1 w:1) 
	// Storage: Timestamp Now (r:1 w:0) 
	fn update_menstrual_calendar() -> Weight { 
		107_069_000_u64
			.saturating_add(RocksDbWeight::get().reads(2_u64)) 
			.saturating_add(RocksDbWeight::get().writes(1_u64)) 
	} 
	// Storage: MentrualCalendar MentrualCalendarById (r:1 w:1) 
	// Storage: MentrualCalendar MentrualCalendarByOwner (r:1 w:1) 
	// Storage: MentrualCalendar MentrualCalendarCount (r:1 w:1) 
	// Storage: MentrualCalendar MentrualCalendarCountByOwner (r:1 w:1) 
	fn remove_menstrual_calendar() -> Weight { 
		47_647_000_u64
			.saturating_add(RocksDbWeight::get().reads(4_u64)) 
			.saturating_add(RocksDbWeight::get().writes(4_u64)) 
	} 
}
