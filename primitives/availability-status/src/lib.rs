#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	codec::{Decode, Encode},
	RuntimeDebug,
};
use scale_info::TypeInfo;

// AvailabilityStatus
#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum AvailabilityStatus {
	Unavailable,
	Available,
}
impl Default for AvailabilityStatus {
	fn default() -> Self {
		Self::Available
	}
}

pub trait AvailabilityStatusTrait {
	fn is_available(&self) -> bool;
}
impl AvailabilityStatusTrait for AvailabilityStatus {
	fn is_available(&self) -> bool {
		matches!(*self, AvailabilityStatus::Available)
	}
}
