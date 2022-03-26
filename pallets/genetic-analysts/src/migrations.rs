use crate::{Config, GeneticAnalysts, MomentOf, Pallet};
use frame_support::{
	dispatch::Weight,
	traits::{Get, GetStorageVersion, OnRuntimeUpgrade},
};
use sp_std::marker::PhantomData;

pub struct LookupReverseIndexMigration<T>(PhantomData<T>);

impl<T: Config> OnRuntimeUpgrade for LookupReverseIndexMigration<T> {
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<(), &'static str> {
		assert!(Pallet::<T>::on_chain_storage_version() < Pallet::<T>::current_storage_version());

		log::info!(
			"GeneticAnalysts pallet to storage version {:?} passes PRE migrate checks ✅",
			Pallet::<T>::current_storage_version()
		);

		Ok(())
	}

	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		// Account for the new storage version written below.
		let initial_weight = T::DbWeight::get().writes(1);

		let total_weight: Weight = GeneticAnalysts::<T>::iter().fold(
			initial_weight,
			|total_weight, (account_id, genetic_analyst)| {
				assert_eq!(genetic_analyst.unstake_at, MomentOf::<T>::default());
				assert_eq!(genetic_analyst.retrieve_unstake_at, MomentOf::<T>::default());

				GeneticAnalysts::<T>::insert(account_id, genetic_analyst);
				total_weight.saturating_add(T::DbWeight::get().reads_writes(1, 1))
			},
		);

		Pallet::<T>::current_storage_version().put::<Pallet<T>>();

		log::info!(
			"Completed GeneticAnalysts pallet migration to storage version {:?} ✅",
			Pallet::<T>::current_storage_version()
		);

		total_weight
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade() -> Result<(), &'static str> {
		assert_eq!(Pallet::<T>::on_chain_storage_version(), Pallet::<T>::current_storage_version());

		GeneticAnalysts::<T>::iter().for_each(|(account_id, genetic_analyst)| {
			assert_eq!(genetic_analyst.unstake_at, MomentOf::<T>::default());
			assert_eq!(genetic_analyst.retrieve_unstake_at, MomentOf::<T>::default());
		});

		log::info!(
			"GeneticAnalysts pallet to storage version {:?} passes POST migrate checks ✅",
			Pallet::<T>::current_storage_version()
		);

		Ok(())
	}
}
