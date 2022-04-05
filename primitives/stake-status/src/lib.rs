#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	codec::{Decode, Encode},
	RuntimeDebug,
};
use scale_info::TypeInfo;

// StakeStatus
#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum StakeStatus {
	Staked,
	WaitingForUnstaked,
	Unstaked,
}
impl Default for StakeStatus {
	fn default() -> Self {
		StakeStatus::Unstaked
	}
}

pub trait StakeStatusTrait {
	fn is_staked(&self) -> bool;
	fn is_waiting_for_unstaked(&self) -> bool;
	fn is_unstaked(&self) -> bool;
}

impl StakeStatusTrait for StakeStatus {
	fn is_staked(&self) -> bool {
		matches!(*self, StakeStatus::Staked)
	}
	fn is_waiting_for_unstaked(&self) -> bool {
		matches!(*self, StakeStatus::WaitingForUnstaked)
	}
	fn is_unstaked(&self) -> bool {
		matches!(*self, StakeStatus::Unstaked)
	}
}
