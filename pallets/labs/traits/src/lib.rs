#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;
use primitives_verification_status::VerificationStatus;

pub trait LabsProvider<T: Config> {
	fn lab_verification_status(account_id: &T::AccountId) -> Option<VerificationStatus>;
}
