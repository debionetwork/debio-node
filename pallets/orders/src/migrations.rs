use crate::{
	AccountIdOf, BalanceOf, Config, CurrencyOf, HashOf, MomentOf, Order, OrderOf, OrderStatus,
	Orders, Pallet, PalletAccount,
};
use frame_support::{
	pallet_prelude::*,
	sp_runtime::traits::{AccountIdConversion, Zero},
	traits::{fungibles, Currency, ExistenceRequirement, Get},
	weights::Weight,
	PalletId,
};
use primitives_price_and_currency::{CurrencyType, Price};
use scale_info::prelude::string::String;
use sp_std::vec::Vec;
use traits_genetic_testing::DnaSampleTrackingId;
use traits_services::types::ServiceFlow;

pub fn migrate<T: Config>() -> Weight {
	let mut weight: Weight = Weight::zero();
	let mut version = StorageVersion::get::<Pallet<T>>();

	if version < 1 {
		weight = weight.saturating_add(version::v1::migrate::<T>());
		version = StorageVersion::new(1);
	}

	if version == 1 {
		weight = weight.saturating_add(version::v2::migrate::<T>());
		version = StorageVersion::new(2);
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

			#[derive(Encode, Decode, PartialEq, Eq)]
			pub struct OldOrder<Hash, AccountId, Balance, Moment> {
				pub id: Hash,
				pub service_id: Hash,
				pub customer_id: AccountId,
				pub customer_box_public_key: Hash,
				pub seller_id: AccountId,
				pub dna_sample_tracking_id: DnaSampleTrackingId,
				pub currency: CurrencyType,
				pub prices: Vec<Price<Balance>>,
				pub additional_prices: Vec<Price<Balance>>,
				pub status: OrderStatus,
				pub order_flow: ServiceFlow,
				pub created_at: Moment,
				pub updated_at: Moment,
			}

			pub type OldOrderOf<T> = OldOrder<HashOf<T>, AccountIdOf<T>, BalanceOf<T>, MomentOf<T>>;

			Orders::<T>::translate(|_key, old_order: OldOrderOf<T>| {
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
							if currency.as_string().to_lowercase() == str_symbol.to_lowercase() {
								asset_id = Some(i);
								break
							}
						}

						i += 1;
					}

					asset_id
				};

				let mut total_price = Zero::zero();

				for price in old_order.prices.iter() {
					total_price += price.value;
				}

				for addition_price in old_order.additional_prices.iter() {
					total_price += addition_price.value;
				}

				Some(Order {
					id: old_order.id,
					service_id: old_order.service_id,
					customer_id: old_order.customer_id,
					customer_box_public_key: old_order.customer_box_public_key,
					seller_id: old_order.seller_id,
					dna_sample_tracking_id: old_order.dna_sample_tracking_id,
					asset_id,
					currency: old_order.currency,
					prices: old_order.prices,
					additional_prices: old_order.additional_prices,
					total_price,
					status: old_order.status,
					order_flow: old_order.order_flow,
					created_at: old_order.created_at,
					updated_at: old_order.updated_at,
				})
			});

			weight
		}
	}

	pub mod v2 {
		use super::*;

		pub const PALLET_ID: PalletId = PalletId(*b"orders!!");

		pub fn migrate<T: Config>() -> Weight {
			let mut weight = T::DbWeight::get().writes(1);
			let receiver: T::AccountId = T::PalletId::get().into_account_truncating();

			PalletAccount::<T>::put(&receiver);

			Orders::<T>::translate(|order_id: HashOf<T>, order: OrderOf<T>| {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

				let sender: T::AccountId = PALLET_ID.into_sub_account_truncating(order_id);

				if order.currency.can_transfer() {
					if order.currency == CurrencyType::DBIO {
						let balance = T::Currency::free_balance(&sender);

						if !balance.is_zero() {
							let _ = CurrencyOf::<T>::transfer(
								&sender,
								&receiver,
								balance,
								ExistenceRequirement::AllowDeath,
							);
						}
					} else if let Some(asset_id) = &order.asset_id {
						let balance = <T::Assets as fungibles::Inspect<T::AccountId>>::balance(
							*asset_id, &sender,
						);

						if !balance.is_zero() {
							let _ = <T::Assets as fungibles::Transfer<T::AccountId>>::transfer(
								*asset_id, &sender, &receiver, balance, false,
							);
						}
					}
				}

				Some(order)
			});

			weight
		}
	}
}
