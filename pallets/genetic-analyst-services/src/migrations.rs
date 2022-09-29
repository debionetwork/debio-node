use crate::{
	AccountIdOf, BalanceOf, Config, GeneticAnalystService, GeneticAnalystServiceInfo,
	GeneticAnalystServices, HashOf, Pallet,
};
use frame_support::{
	pallet_prelude::{Decode, Encode},
	traits::Get,
	weights::Weight,
};
use primitives_duration::{DurationType, ExpectedDuration};
use primitives_price_and_currency::PriceByCurrency;
use sp_std::vec::Vec;

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
			pub struct OldExpectedDuration {
				pub duration: i8,
				pub duration_type: DurationType,
			}

			#[derive(Encode, Decode)]
			pub struct OldGeneticAnalystServiceInfo<Balance> {
				pub name: Vec<u8>,
				pub prices_by_currency: Vec<PriceByCurrency<Balance>>,
				pub expected_duration: OldExpectedDuration,
				pub description: Vec<u8>,
				pub test_result_sample: Vec<u8>,
			}

			#[derive(Encode, Decode)]
			pub struct OldGeneticAnalystService<AccountId, Hash, Balance> {
				pub id: Hash,
				pub owner_id: AccountId,
				pub info: OldGeneticAnalystServiceInfo<Balance>,
			}

			pub type OldGeneticAnalystServiceOf<T> =
				OldGeneticAnalystService<AccountIdOf<T>, HashOf<T>, BalanceOf<T>>;

			GeneticAnalystServices::<T>::translate(
				|_key, old_services: OldGeneticAnalystServiceOf<T>| {
					weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

					let old_service_info = &old_services.info;

					let old_expected_duration = &old_service_info.expected_duration;
					let old_duration = old_expected_duration.duration;
					let old_duration_type = old_expected_duration.duration_type.clone();
					let expected_duration = ExpectedDuration {
						duration: old_duration as u64,
						duration_type: old_duration_type,
					};

					let service_info = GeneticAnalystServiceInfo {
						name: old_service_info.name.clone(),
						prices_by_currency: old_service_info.prices_by_currency.clone(),
						expected_duration,
						description: old_service_info.description.clone(),
						test_result_sample: old_service_info.test_result_sample.clone(),
					};

					Some(GeneticAnalystService {
						id: old_services.id,
						owner_id: old_services.owner_id,
						info: service_info,
					})
				},
			);

			weight
		}
	}
}
