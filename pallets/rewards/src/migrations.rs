use crate::{Config, Pallet, PalletAccount, Weight};
use frame_support::traits::Get;

pub fn migrate<T: Config>() -> Weight {
	use frame_support::traits::StorageVersion;

	let version = StorageVersion::get::<Pallet<T>>();
	let mut weight: Weight = 0;

	if version < 2 {
		weight = weight.saturating_add(v2::migrate::<T>());
		StorageVersion::new(2).put::<Pallet<T>>();
	}

	weight
}

mod v2 {
	use super::*;

	pub fn migrate<T: Config>() -> Weight {
		PalletAccount::<T>::put(<Pallet<T>>::account_id());

		T::DbWeight::get().writes(1)
	}
}
