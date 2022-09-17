#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	codec::{Decode, Encode},
	RuntimeDebug,
};
use scale_info::TypeInfo;

// MenstrualCycleLog
#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct MenstrualCycleLog {
	date: u64,
	menstruation: bool,
	symptoms: Vec<Vec<u8>>,
}
impl Default for MenstrualCycleLog {
	fn default() -> Self { MenstrualCycleLog {
		date: 1,
		menstruation: false,
		symptoms: vec![vec![]],
	} }
}