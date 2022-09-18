#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	codec::{Decode, Encode},
	inherent::Vec,
	RuntimeDebug,
};
use scale_info::{prelude::vec, TypeInfo};

// Symptom
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct Symptom {
	name: Vec<u8>,
}

// MenstrualCycleLog
#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct MenstrualCycleLog {
	date: u64,
	menstruation: bool,
	symptoms: Vec<Symptom>,
}
impl Default for MenstrualCycleLog {
	fn default() -> Self {
		MenstrualCycleLog { date: 1, menstruation: false, symptoms: vec![Symptom::default()] }
	}
}
