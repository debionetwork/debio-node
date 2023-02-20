use crate::{
	AccountIdOf, BalanceOf, Config, GeneticAnalysisOrder, GeneticAnalysisOrderStatus,
	GeneticAnalysisOrders, HashOf, MomentOf, Pallet, PalletAccount, Weight,
};
use frame_support::{
	pallet_prelude::*,
	traits::{fungibles, Get},
};
use primitives_price_and_currency::{CurrencyType, Price};
use primitives_tracking_id::TrackingId;
use scale_info::prelude::string::String;
use sp_std::vec::Vec;

pub fn migrate<T: Config>() -> Weight {
	let mut weight: Weight = Weight::zero();
	let mut version = StorageVersion::get::<Pallet<T>>();

	if version == 1 {
		weight = weight.saturating_add(version::v2::migrate::<T>());
		version = StorageVersion::new(2);
	}

	if version == 2 {
		weight = weight.saturating_add(version::v3::migrate::<T>());
		version = StorageVersion::new(3);
	}

	version.put::<Pallet<T>>();
	weight
}

mod version {
	use super::*;

	pub mod v2 {
		use super::*;

		pub fn migrate<T: Config>() -> Weight {
			PalletAccount::<T>::put(<Pallet<T>>::get_pallet_id());

			T::DbWeight::get().writes(1)
		}
	}

	pub mod v3 {
		use super::*;

		pub fn migrate<T: Config>() -> Weight {
			let mut weight = T::DbWeight::get().writes(1);

			#[derive(Encode, Decode, Clone)]
			pub struct OldGeneticAnalysisOrder<Hash, AccountId, Balance, Moment> {
				pub id: Hash,
				pub service_id: Hash,
				pub customer_id: AccountId,
				pub customer_box_public_key: Hash,
				pub seller_id: AccountId,
				pub genetic_data_id: Hash,
				pub genetic_analysis_tracking_id: TrackingId,
				pub currency: CurrencyType,
				pub prices: Vec<Price<Balance>>,
				pub additional_prices: Vec<Price<Balance>>,
				pub total_price: Balance,
				pub status: GeneticAnalysisOrderStatus,
				pub created_at: Moment,
				pub updated_at: Moment,
				pub genetic_link: Vec<u8>,
			}

			pub type OldGeneticAnalysisOrderOf<T> =
				OldGeneticAnalysisOrder<HashOf<T>, AccountIdOf<T>, BalanceOf<T>, MomentOf<T>>;

			GeneticAnalysisOrders::<T>::translate(
				|_key, old_order: OldGeneticAnalysisOrderOf<T>| {
					weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

					let asset_id = if old_order.currency == CurrencyType::DBIO {
						None
					} else {
						let mut i = 0_u32;
						let mut asset_id: Option<u32> = None;

						while i <= 10 {
							let currency = old_order.currency.clone();
							let symbol =
								<T::Assets as fungibles::InspectMetadata<T::AccountId>>::symbol(&i);

							if let Ok(str_symbol) = String::from_utf8(symbol) {
								if currency.as_string().to_lowercase() == str_symbol.to_lowercase()
								{
									asset_id = Some(i);
									break
								}
							}

							i += 1;
						}

						asset_id
					};

					Some(GeneticAnalysisOrder {
						id: old_order.id,
						service_id: old_order.service_id,
						customer_id: old_order.customer_id,
						customer_box_public_key: old_order.customer_box_public_key,
						seller_id: old_order.seller_id,
						genetic_data_id: old_order.genetic_data_id,
						genetic_analysis_tracking_id: old_order.genetic_analysis_tracking_id,
						asset_id,
						currency: old_order.currency,
						prices: old_order.prices,
						additional_prices: old_order.additional_prices,
						total_price: old_order.total_price,
						status: old_order.status,
						created_at: old_order.created_at,
						updated_at: old_order.updated_at,
						genetic_link: old_order.genetic_link,
					})
				},
			);

			weight
		}
	}
}
