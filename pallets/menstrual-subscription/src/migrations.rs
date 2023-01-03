use crate::{
	AccountIdOf, Config, HashOf, MenstrualSubscription, MenstrualSubscriptionById, MomentOf, Pallet,
};
use frame_support::{
	pallet_prelude::{Decode, Encode},
	traits::Get,
	weights::Weight,
};
use primitives_duration::MenstrualSubscriptionDuration;
use primitives_menstrual_status::{MenstrualSubscriptionStatus, PaymentStatus};
use primitives_price_and_currency::CurrencyType;

pub fn migrate<T: Config>() -> Weight {
	use frame_support::traits::StorageVersion;

	let mut weight: Weight = 0;
	let mut version = StorageVersion::get::<Pallet<T>>();

	if version < 1 {
		weight = weight.saturating_add(version::v1::migrate::<T>());
		version = StorageVersion::new(1);
	}

	version.put::<Pallet<T>>();
	weight
}

mod version {
	use super::*;

	pub mod v1 {
		use super::*;

		pub fn migrate<T: Config>() -> Weight {
			let mut weight = T::DbWeight::get().writes(1);

			#[derive(Encode, Decode)]
			pub struct OldMenstrualSubscription<AccountId, Hash, Moment> {
				pub id: Hash,
				pub address_id: AccountId,
				pub duration: MenstrualSubscriptionDuration,
				pub price: u8,
				pub payment_status: PaymentStatus,
				pub status: MenstrualSubscriptionStatus,
				pub created_at: Moment,
				pub updated_at: Moment,
			}

			pub type OldMenstrualSubscriptionOf<T> =
				OldMenstrualSubscription<AccountIdOf<T>, HashOf<T>, MomentOf<T>>;

			MenstrualSubscriptionById::<T>::translate(
				|_key, old: OldMenstrualSubscriptionOf<T>| {
					weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

					Some(MenstrualSubscription {
						id: old.id,
						address_id: old.address_id,
						duration: old.duration,
						currency: CurrencyType::DBIO,
						payment_status: old.payment_status,
						status: old.status,
						created_at: old.created_at,
						updated_at: old.updated_at,
					})
				},
			);

			weight
		}
	}
}
