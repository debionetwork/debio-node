#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;

pub trait GeneticAnalystsProvider<T: Config> {
	fn is_genetic_analyst_available(owner_id: &T::AccountId) -> Option<bool>;
}
