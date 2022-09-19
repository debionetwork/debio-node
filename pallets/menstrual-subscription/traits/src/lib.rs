#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;

pub trait MenstrualSubscription<T: Config> {
	fn get_id(&self) -> &T::Hash;
	fn get_address_id(&self) -> &T::AccountId;
}

pub trait MenstrualSubscriptionProvider<T: Config> {
	type Error;
	type MenstrualSubscription: MenstrualSubscription<T> + sp_std::fmt::Debug;

	fn menstrual_subscription_by_id(id: &T::Hash) -> Option<Self::MenstrualSubscription>;
}
