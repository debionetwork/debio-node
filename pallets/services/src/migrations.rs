use crate::{AccountIdOf, BalanceOf, Config, HashOf, Pallet, Service, ServiceInfo, Services};
use frame_support::{
	pallet_prelude::{Decode, Encode},
	traits::Get,
	weights::Weight,
};
use primitives_duration::{DurationType, ExpectedDuration};
use primitives_price_and_currency::PriceByCurrency;
use sp_std::vec::Vec;
use traits_services::types::ServiceFlow;

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
			pub struct OldServiceInfo<Balance> {
				pub name: Vec<u8>,
				pub prices_by_currency: Vec<PriceByCurrency<Balance>>,
				pub expected_duration: OldExpectedDuration,
				pub category: Vec<u8>,
				pub description: Vec<u8>, // TODO: limit the length
				pub dna_collection_process: Vec<u8>,
				pub test_result_sample: Vec<u8>,
				pub long_description: Option<Vec<u8>>,
				pub image: Option<Vec<u8>>,
			}

			#[derive(Encode, Decode)]
			pub struct OldService<AccountId, Hash, Balance> {
				pub id: Hash,
				pub owner_id: AccountId,
				pub info: OldServiceInfo<Balance>,
				pub service_flow: ServiceFlow,
			}

			pub type OldServiceOf<T> = OldService<AccountIdOf<T>, HashOf<T>, BalanceOf<T>>;

			Services::<T>::translate(|_key, old_services: OldServiceOf<T>| {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

				let old_service_info = &old_services.info;

				let old_expected_duration = &old_service_info.expected_duration;
				let old_duration = old_expected_duration.duration;
				let old_duration_type = old_expected_duration.duration_type.clone();
				let expected_duration = ExpectedDuration {
					duration: old_duration as u64,
					duration_type: old_duration_type,
				};

				let service_info = ServiceInfo {
					name: old_service_info.name.clone(),
					prices_by_currency: old_service_info.prices_by_currency.clone(),
					expected_duration,
					category: old_service_info.category.clone(),
					description: old_service_info.description.clone(),
					dna_collection_process: old_service_info.dna_collection_process.clone(),
					test_result_sample: old_service_info.test_result_sample.clone(),
					long_description: old_service_info.long_description.clone(),
					image: old_service_info.image.clone(),
				};

				Some(Service {
					id: old_services.id,
					owner_id: old_services.owner_id,
					info: service_info,
					service_flow: old_services.service_flow,
				})
			});

			weight
		}
	}
}
