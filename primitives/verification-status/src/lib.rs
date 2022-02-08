#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	codec::{Decode, Encode},
	RuntimeDebug,
};
use scale_info::TypeInfo;

// VerificationStatus
#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum VerificationStatus {
	Unverified,
	Verified,
	Rejected,
	Revoked,
}
impl Default for VerificationStatus {
	fn default() -> Self {
		Self::Unverified
	}
}

pub trait VerificationStatusTrait {
	fn is_verified(&self) -> bool;
	fn is_unverified(&self) -> bool;
	fn is_rejected(&self) -> bool;
	fn is_revoked(&self) -> bool;
}
impl VerificationStatusTrait for VerificationStatus {
	fn is_verified(&self) -> bool {
		matches!(*self, VerificationStatus::Verified)
	}
	fn is_unverified(&self) -> bool {
		matches!(*self, VerificationStatus::Unverified)
	}
	fn is_rejected(&self) -> bool {
		matches!(*self, VerificationStatus::Rejected)
	}
	fn is_revoked(&self) -> bool {
		matches!(*self, VerificationStatus::Revoked)
	}
}
