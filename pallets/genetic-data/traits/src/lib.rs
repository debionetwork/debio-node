#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;

pub trait GeneticData<T: Config> {
	fn get_id(&self) -> &T::Hash;
	fn get_owner_id(&self) -> &T::AccountId;
}

pub trait GeneticDataProvider<T: Config> {
	type Error;
	type GeneticData: GeneticData<T> + sp_std::fmt::Debug;

	fn genetic_data_by_id(id: &T::Hash) -> Option<Self::GeneticData>;
}
