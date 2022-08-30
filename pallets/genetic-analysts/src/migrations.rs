use crate::{
	AccountIdOf, BalanceOf, Config, GeneticAnalyst, GeneticAnalystInfo, GeneticAnalysts, HashOf,
	MomentOf, Pallet, PalletAccount, Vec, Weight,
};
use frame_support::{pallet_prelude::Decode, traits::Get};
use primitives_availability_status::AvailabilityStatus;
use primitives_stake_status::StakeStatus;
use primitives_verification_status::VerificationStatus;

pub fn migrate<T: Config>() -> Weight {
	use frame_support::traits::StorageVersion;

	let mut version = StorageVersion::get::<Pallet<T>>();
	let mut weight: Weight = 0;

	if version < 2 {
		weight = weight.saturating_add(v2::migrate::<T>());
		StorageVersion::new(2).put::<Pallet<T>>();
	}

	version = StorageVersion::get::<Pallet<T>>();

	if version == 2 {
		weight = weight.saturating_add(v3::migrate::<T>());
		StorageVersion::new(3).put::<Pallet<T>>();
	}

	weight
}

mod v2 {
	use super::*;

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

	pub fn migrate<T: Config>() -> Weight {
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

		weight
	}
}

mod v3 {
	use super::*;

	pub fn migrate<T: Config>() -> Weight {
		PalletAccount::<T>::put(<Pallet<T>>::get_pallet_id());

		T::DbWeight::get().writes(1)
	}
}
