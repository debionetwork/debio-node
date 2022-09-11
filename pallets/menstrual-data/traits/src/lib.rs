#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;

pub trait MenstrualData<T: Config> {
	fn get_id(&self) -> &T::Hash;
	fn get_owner_id(&self) -> &T::AccountId;
}

pub trait MenstrualDataProvider<T: Config> {
	type Error;
	type MenstrualData: MenstrualData<T> + sp_std::fmt::Debug;

	fn menstrual_data_by_id(id: &T::Hash) -> Option<Self::MenstrualData>;
}
