use crate::{
	AccountIdOf, BalanceOf, Config, HashOf, Lab, LabInfo, Labs, MomentOf, Pallet, PalletAccount,
	Vec, Weight,
};
use frame_support::{pallet_prelude::Decode, traits::Get};
use primitives_stake_status::StakeStatus;
use primitives_verification_status::VerificationStatus;

pub fn migrate<T: Config>() -> Weight {
	use frame_support::traits::StorageVersion;

	let version = StorageVersion::get::<Pallet<T>>();
	let mut weight: Weight = 0;

	if version < 2 {
		weight = weight.saturating_add(v2::migrate::<T>());
		StorageVersion::new(2).put::<Pallet<T>>();
	}

	if version == 2 {
		weight = weight.saturating_add(v3::migrate::<T>());
		StorageVersion::new(3).put::<Pallet<T>>();
	}

	weight
}

mod v2 {
	use super::*;

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

	pub fn migrate<T: Config>() -> Weight {
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

		weight
	}
}

mod v3 {
	use super::*;

	pub fn migrate<T: Config>() -> Weight {
		PalletAccount::<T>::put(<Pallet<T>>::account_id());

		T::DbWeight::get().writes(1)
	}
}
