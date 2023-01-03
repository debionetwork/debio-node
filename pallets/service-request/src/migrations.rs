use crate::{
	AccountIdOf, BalanceOf, Config, HashOf, Pallet, Request as NewRequest, RequestById,
	RequestByOrderId, RequestStatus,
};
use frame_support::{
	generate_storage_alias,
	pallet_prelude::{Decode, Encode},
	traits::Get,
	weights::Weight,
	Blake2_128Concat,
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

			#[derive(Decode, Encode, Clone)]
			pub struct ServicePrice<Balance> {
				asset_id: Vec<u8>,
				qc_price: Balance,
				testing_price: Balance,
			}
			impl<Balance: Clone> ServicePrice<Balance> {
				pub fn new(asset_id: &[u8], qc_price: Balance, testing_price: Balance) -> Self {
					Self { asset_id: asset_id.to_vec(), qc_price, testing_price }
				}
			}

			#[derive(Decode, Encode, Clone)]
			pub struct OldServiceOffer<AccountId, Balance, Hash> {
				request_hash: Hash,
				lab_address: AccountId,
				service_id: Hash,
				testing_price: Balance,
				qc_price: Balance,
			}

			#[derive(Decode, Encode, Clone)]
			pub struct ServiceOffer<AccountId, Balance, Hash> {
				request_hash: Hash,
				lab_address: AccountId,
				service_id: Hash,
				service_price: ServicePrice<Balance>,
			}

			pub type ServiceOfferOf<T> = ServiceOffer<AccountIdOf<T>, BalanceOf<T>, HashOf<T>>;
			pub type OldServiceOfferOf<T> =
				OldServiceOffer<AccountIdOf<T>, BalanceOf<T>, HashOf<T>>;

			generate_storage_alias!(
				ServiceRequest,
				ServiceOfferById<T: Config> => Map<(Blake2_128Concat, HashOf<T>), ServiceOfferOf<T>>
			);

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
			pub struct ServiceInvoice<AccountId, Balance, Hash> {
				request_hash: Hash,
				order_id: Hash,
				service_id: Hash,
				customer_address: AccountId,
				seller_address: AccountId,
				dna_sample_tracking_id: Vec<u8>,
				service_price: ServicePrice<Balance>,
			}

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

			pub type ServiceInvoiceOf<T> = ServiceInvoice<AccountIdOf<T>, BalanceOf<T>, HashOf<T>>;
			pub type OldServiceInvoiceOf<T> =
				OldServiceInvoice<AccountIdOf<T>, BalanceOf<T>, HashOf<T>>;

			generate_storage_alias!(
				ServiceRequest,
				ServiceInvoiceById<T: Config> => Map<(Blake2_128Concat, HashOf<T>), ServiceInvoiceOf<T>>
			);

			generate_storage_alias!(
				ServiceRequest,
				ServiceInvoiceByOrderId<T: Config> => Map<(Blake2_128Concat, HashOf<T>), ServiceInvoiceOf<T>>
			);

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

	pub mod v2 {
		use super::*;

		pub fn migrate<T: Config>() -> Weight {
			let mut weight = T::DbWeight::get().writes(1);

			#[derive(Clone, Decode, Encode)]
			pub struct OldRequest<AccountId, Balance, Hash> {
				pub hash: Hash,
				pub requester_address: AccountId,
				pub lab_address: Option<AccountId>,
				pub country: Vec<u8>,
				pub region: Vec<u8>,
				pub city: Vec<u8>,
				pub service_category: Vec<u8>,
				pub staking_amount: Balance,
				pub status: RequestStatus,
				pub created_at: u128,
				pub updated_at: Option<u128>,
				pub unstaked_at: Option<u128>,
			}

			#[derive(Clone, Decode, Encode)]
			pub struct ServicePrice<Balance> {
				asset_id: Vec<u8>,
				qc_price: Balance,
				testing_price: Balance,
			}

			#[derive(Clone, Decode, Encode)]
			pub struct ServiceInvoice<AccountId, Balance, Hash> {
				request_hash: Hash,
				order_id: Hash,
				service_id: Hash,
				customer_address: AccountId,
				seller_address: AccountId,
				dna_sample_tracking_id: Vec<u8>,
				service_price: ServicePrice<Balance>,
			}

			#[derive(Decode, Encode)]
			pub struct ServiceOffer<AccountId, Balance, Hash> {
				request_hash: Hash,
				lab_address: AccountId,
				service_id: Hash,
				service_price: ServicePrice<Balance>,
			}

			pub type OldRequestOf<T> = OldRequest<AccountIdOf<T>, BalanceOf<T>, HashOf<T>>;
			pub type ServiceInvoiceOf<T> = ServiceInvoice<AccountIdOf<T>, BalanceOf<T>, HashOf<T>>;
			pub type ServiceOfferOf<T> = ServiceOffer<AccountIdOf<T>, BalanceOf<T>, HashOf<T>>;

			generate_storage_alias!(
				ServiceRequest,
				ServiceInvoiceById<T: Config> => Map<(Blake2_128Concat, HashOf<T>), ServiceInvoiceOf<T>>
			);

			generate_storage_alias!(
				ServiceRequest,
				ServiceInvoiceByOrderId<T: Config> => Map<(Blake2_128Concat, HashOf<T>), ServiceInvoiceOf<T>>
			);

			generate_storage_alias!(
				ServiceRequest,
				ServiceOfferById<T: Config> => Map<(Blake2_128Concat, HashOf<T>), ServiceOfferOf<T>>
			);

			RequestById::<T>::translate(|request_id: HashOf<T>, request: OldRequestOf<T>| {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

				let service_invoice_opt = ServiceInvoiceById::<T>::take(&request_id);
				let service_offer_opt = ServiceOfferById::<T>::take(&request_id);

				let (mut service_id, order_id) = if let Some(service_invoice) = service_invoice_opt
				{
					(Some(service_invoice.service_id), Some(service_invoice.order_id))
				} else {
					(None, None)
				};

				if let Some(order_id) = order_id {
					RequestByOrderId::<T>::insert(&order_id, &request_id);
				}

				if service_id.is_none() {
					if let Some(service_offer) = service_offer_opt {
						service_id = Some(service_offer.service_id);
					}
				}

				let new_request = NewRequest {
					hash: request.hash,
					requester_address: request.requester_address,
					lab_address: request.lab_address,
					service_id,
					order_id,
					country: request.country,
					region: request.region,
					city: request.city,
					service_category: request.service_category,
					staking_amount: request.staking_amount,
					status: request.status,
					created_at: request.created_at,
					updated_at: request.updated_at,
					unstaked_at: request.unstaked_at,
				};

				Some(new_request)
			});

			ServiceOfferById::<T>::translate(|_request_id, _service_offer: ServiceOfferOf<T>| {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));
				None
			});

			ServiceInvoiceByOrderId::<T>::translate(
				|_order_id, _service_invoice: ServiceInvoiceOf<T>| {
					weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));
					None
				},
			);

			ServiceInvoiceById::<T>::translate(
				|_request_id, _service_invoice: ServiceInvoiceOf<T>| {
					weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));
					None
				},
			);

			weight
		}
	}
}
