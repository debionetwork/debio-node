#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	codec::{Decode, Encode},
	RuntimeDebug,
};
use scale_info::TypeInfo;

// PaymentStatus
#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum PaymentStatus {
	Unpaid,
	Paid,
}
impl Default for PaymentStatus {
	fn default() -> Self {
		PaymentStatus::Unpaid
	}
}

pub trait PaymentStatusTrait {
	fn is_unpaid(&self) -> bool;
	fn is_paid(&self) -> bool;
}

impl PaymentStatusTrait for PaymentStatus {
	fn is_unpaid(&self) -> bool {
		matches!(*self, PaymentStatus::Unpaid)
	}
	fn is_paid(&self) -> bool {
		matches!(*self, PaymentStatus::Paid)
	}
}

// MenstrualSubscriptionStatus
#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum MenstrualSubscriptionStatus {
	Inactive,
	Active,
	InQueue,
}
impl Default for MenstrualSubscriptionStatus {
	fn default() -> Self {
		MenstrualSubscriptionStatus::Inactive
	}
}

pub trait MenstrualSubscriptionStatusTrait {
	fn is_inactive(&self) -> bool;
	fn is_active(&self) -> bool;
	fn is_in_queue(&self) -> bool;
}

impl MenstrualSubscriptionStatusTrait for MenstrualSubscriptionStatus {
	fn is_inactive(&self) -> bool {
		matches!(*self, MenstrualSubscriptionStatus::Inactive)
	}
	fn is_active(&self) -> bool {
		matches!(*self, MenstrualSubscriptionStatus::Active)
	}
	fn is_in_queue(&self) -> bool {
		matches!(*self, MenstrualSubscriptionStatus::InQueue)
	}
}
