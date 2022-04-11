use crate::{AccountIdOf, BalanceOf, Config, HashOf, Lab, LabInfo, Labs, MomentOf, Pallet, Vec};
use frame_support::traits::{Get, GetStorageVersion, OnRuntimeUpgrade};
use primitives_stake_status::StakeStatus;
use primitives_verification_status::VerificationStatus;
use sp_std::marker::PhantomData;

use frame_support::pallet_prelude::Decode;

pub struct LookupReverseIndexMigration<T>(PhantomData<T>);

#[derive(Decode)]
pub struct OldLab<AccountId, Hash>
where
	Hash: PartialEq + Eq,
{
	pub account_id: AccountId,
	pub services: Vec<Hash>,
	pub certifications: Vec<Hash>,
	pub verification_status: VerificationStatus,
	pub info: LabInfo<Hash>,
}

impl<T: Config> OnRuntimeUpgrade for LookupReverseIndexMigration<T> {
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<(), &'static str> {
		assert!(Pallet::<T>::on_chain_storage_version() < Pallet::<T>::current_storage_version());

		log::info!(
			"Labs pallet to storage version {:?} passes PRE migrate checks ✅",
			Pallet::<T>::current_storage_version()
		);

		Ok(())
	}

	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		// Account for the new storage version written below.
		let mut weight = T::DbWeight::get().writes(1);

		<Labs<T>>::translate(|_key, old: OldLab<AccountIdOf<T>, HashOf<T>>| {
			weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));
			Some(Lab {
				account_id: old.account_id,
				services: old.services,
				certifications: old.certifications,
				info: old.info,
				verification_status: old.verification_status,
				stake_amount: BalanceOf::<T>::default(),
				stake_status: StakeStatus::default(),
				unstake_at: MomentOf::<T>::default(),
				retrieve_unstake_at: MomentOf::<T>::default(),
			})
		});

		Pallet::<T>::current_storage_version().put::<Pallet<T>>();

		log::info!(
			"Completed Labs pallet migration to storage version {:?} ✅",
			Pallet::<T>::current_storage_version()
		);

		weight
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade() -> Result<(), &'static str> {
		assert_eq!(Pallet::<T>::on_chain_storage_version(), Pallet::<T>::current_storage_version());

		Labs::<T>::iter().for_each(|(account_id, lab)| {
			assert_eq!(lab.stake_amount, BalanceOf::<T>::default());
			assert_eq!(lab.stake_status, MomentOf::<T>::default());
			assert_eq!(lab.unstake_at, MomentOf::<T>::default());
			assert_eq!(lab.retrieve_unstake_at, MomentOf::<T>::default());
		});

		log::info!(
			"Labs pallet to storage version {:?} passes POST migrate checks ✅",
			Pallet::<T>::current_storage_version()
		);

		Ok(())
	}
}
