use crate::{
	AccountIdOf, BalanceOf, Config, GeneticAnalyst, GeneticAnalystInfo, GeneticAnalysts, HashOf,
	MomentOf, Pallet, Vec,
};
use frame_support::traits::{Get, GetStorageVersion, OnRuntimeUpgrade};
use primitives_availability_status::AvailabilityStatus;
use primitives_stake_status::StakeStatus;
use primitives_verification_status::VerificationStatus;
use sp_std::marker::PhantomData;

use frame_support::pallet_prelude::Decode;

pub struct LookupReverseIndexMigration<T>(PhantomData<T>);

#[derive(Decode)]
pub struct OldGeneticAnalyst<AccountId, Hash, Moment, Balance>
where
	Hash: PartialEq + Eq,
{
	pub account_id: AccountId,
	pub services: Vec<Hash>,
	pub qualifications: Vec<Hash>,
	pub info: GeneticAnalystInfo<Hash, Moment>,
	pub stake_amount: Balance,
	pub stake_status: StakeStatus,
	pub verification_status: VerificationStatus,
	pub availability_status: AvailabilityStatus,
}

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
		let mut weight = T::DbWeight::get().writes(1);

		<GeneticAnalysts<T>>::translate(
			|_key, old: OldGeneticAnalyst<AccountIdOf<T>, HashOf<T>, MomentOf<T>, BalanceOf<T>>| {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));
				Some(GeneticAnalyst {
					account_id: old.account_id,
					services: old.services,
					qualifications: old.qualifications,
					info: old.info,
					stake_amount: old.stake_amount,
					stake_status: old.stake_status,
					verification_status: old.verification_status,
					availability_status: old.availability_status,
					unstake_at: MomentOf::<T>::default(),
					retrieve_unstake_at: MomentOf::<T>::default(),
				})
			},
		);

		Pallet::<T>::current_storage_version().put::<Pallet<T>>();

		log::info!(
			"Completed GeneticAnalysts pallet migration to storage version {:?} ✅",
			Pallet::<T>::current_storage_version()
		);

		weight
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
