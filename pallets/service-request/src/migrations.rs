use crate::{
	AccountIdOf, BalanceOf, Config, HashOf, Pallet, ServiceInvoice, ServiceInvoiceById,
	ServiceInvoiceByOrderId, ServiceOffer, ServiceOfferById, ServicePrice,
};
use frame_support::{
	pallet_prelude::{Decode, Encode},
	traits::Get,
	weights::Weight,
};
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

			#[derive(Decode, Encode)]
			pub struct OldServiceOffer<AccountId, Balance, Hash> {
				request_hash: Hash,
				lab_address: AccountId,
				service_id: Hash,
				testing_price: Balance,
				qc_price: Balance,
			}

			pub type OldServiceOfferOf<T> =
				OldServiceOffer<AccountIdOf<T>, BalanceOf<T>, HashOf<T>>;

			ServiceOfferById::<T>::translate(|_key, old_service_offer: OldServiceOfferOf<T>| {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

				Some(ServiceOffer {
					request_hash: old_service_offer.request_hash,
					lab_address: old_service_offer.lab_address,
					service_id: old_service_offer.service_id,
					service_price: ServicePrice::new(
						b"native",
						old_service_offer.testing_price,
						old_service_offer.qc_price,
					),
				})
			});

			#[derive(Clone, Decode, Encode)]
			pub struct OldServiceInvoice<AccountId, Balance, Hash> {
				request_hash: Hash,
				order_id: Hash,
				service_id: Hash,
				customer_address: AccountId,
				seller_address: AccountId,
				dna_sample_tracking_id: Vec<u8>,
				testing_price: Balance,
				qc_price: Balance,
				pay_amount: Balance,
			}

			pub type OldServiceInvoiceOf<T> =
				OldServiceInvoice<AccountIdOf<T>, BalanceOf<T>, HashOf<T>>;

			ServiceInvoiceById::<T>::translate(
				|_key, old_service_invoice: OldServiceInvoiceOf<T>| {
					weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

					Some(ServiceInvoice {
						request_hash: old_service_invoice.request_hash,
						order_id: old_service_invoice.order_id,
						service_id: old_service_invoice.service_id,
						customer_address: old_service_invoice.customer_address,
						seller_address: old_service_invoice.seller_address,
						dna_sample_tracking_id: old_service_invoice.dna_sample_tracking_id,
						service_price: ServicePrice::new(
							b"native",
							old_service_invoice.testing_price,
							old_service_invoice.qc_price,
						),
					})
				},
			);

			ServiceInvoiceByOrderId::<T>::translate(
				|_key, old_service_invoice: OldServiceInvoiceOf<T>| {
					weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

					Some(ServiceInvoice {
						request_hash: old_service_invoice.request_hash,
						order_id: old_service_invoice.order_id,
						service_id: old_service_invoice.service_id,
						customer_address: old_service_invoice.customer_address,
						seller_address: old_service_invoice.seller_address,
						dna_sample_tracking_id: old_service_invoice.dna_sample_tracking_id,
						service_price: ServicePrice::new(
							b"native",
							old_service_invoice.testing_price,
							old_service_invoice.qc_price,
						),
					})
				},
			);

			weight
		}
	}
}
