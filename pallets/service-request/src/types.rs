use crate::*;

use frame_support::traits::Currency;
use scale_info::TypeInfo;
use sp_std::vec::Vec;

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type CurrencyOf<T> = <T as self::Config>::Currency;
pub type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;
pub type HashOf<T> = <T as frame_system::Config>::Hash;
pub type AdminOf<T> = AccountIdOf<T>;
pub type RequestOf<T> = Request<AccountIdOf<T>, BalanceOf<T>, HashOf<T>>;
pub type ServiceOfferOf<T> = ServiceOffer<AccountIdOf<T>, BalanceOf<T>, HashOf<T>>;
pub type ServiceInvoiceOf<T> = ServiceInvoice<AccountIdOf<T>, BalanceOf<T>, HashOf<T>>;
pub type RequestIdOf<T> = HashOf<T>;
pub type RequesterIdOf<T> = AccountIdOf<T>;
pub type LabIdOf<T> = AccountIdOf<T>;
pub type CountryOf = Vec<u8>;
pub type RegionOf = Vec<u8>;
pub type CityOf = Vec<u8>;
pub type ServiceCategoryOf = Vec<u8>;
pub type ServiceIdOf<T> = HashOf<T>;
pub type OrderIdOf<T> = HashOf<T>;
pub type DNASampleTrackingIdOf = Vec<u8>;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum RequestStatus {
	Open,
	WaitingForUnstaked,
	Unstaked,
	Claimed,
	Processed,
	Finalized,
}
impl Default for RequestStatus {
	fn default() -> Self {
		RequestStatus::Open
	}
}

#[derive(Clone, Decode, Default, Encode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct Request<AccountId, Balance, Hash> {
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
#[allow(clippy::too_many_arguments)]
impl<AccountId: Clone, Balance, Hash> Request<AccountId, Balance, Hash> {
	pub fn new(
		hash: Hash,
		requester_address: &AccountId,
		country: &[u8],
		region: &[u8],
		city: &[u8],
		service_category: &[u8],
		staking_amount: Balance,
		created_at: u128,
	) -> Self {
		Self {
			hash,
			requester_address: requester_address.clone(),
			lab_address: None,
			country: country.to_vec(),
			region: region.to_vec(),
			city: city.to_vec(),
			service_category: service_category.to_vec(),
			staking_amount,
			status: RequestStatus::default(),
			created_at,
			updated_at: None,
			unstaked_at: None,
		}
	}

	pub fn get_lab_address(&self) -> &Option<AccountId> {
		&self.lab_address
	}

	pub fn get_requester_address(&self) -> &AccountId {
		&self.requester_address
	}
}

#[derive(Clone, Decode, Encode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct ServiceOffer<AccountId, Balance, Hash> {
	pub request_hash: Hash,
	pub lab_address: AccountId,
	pub service_id: Hash,
	pub testing_price: Balance,
	pub qc_price: Balance,
}
impl<AccountId: Clone, Balance, Hash> ServiceOffer<AccountId, Balance, Hash> {
	pub fn new(
		request_hash: Hash,
		lab_address: &AccountId,
		service_id: Hash,
		testing_price: Balance,
		qc_price: Balance,
	) -> Self {
		Self { request_hash, lab_address: lab_address.clone(), service_id, testing_price, qc_price }
	}
}

#[derive(Clone, Decode, Encode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct ServiceInvoice<AccountId, Balance, Hash> {
	pub request_hash: Hash,
	pub order_id: Hash,
	pub service_id: Hash,
	pub customer_address: AccountId,
	pub seller_address: AccountId,
	pub dna_sample_tracking_id: Vec<u8>,
	pub testing_price: Balance,
	pub qc_price: Balance,
	pub pay_amount: Balance,
}
#[allow(clippy::too_many_arguments)]
impl<AccountId, Balance, Hash> ServiceInvoice<AccountId, Balance, Hash> {
	pub fn new(
		request_hash: Hash,
		order_id: Hash,
		service_id: Hash,
		customer_address: AccountId,
		seller_address: AccountId,
		dna_sample_tracking_id: Vec<u8>,
		testing_price: Balance,
		qc_price: Balance,
		pay_amount: Balance,
	) -> Self {
		Self {
			request_hash,
			order_id,
			service_id,
			customer_address,
			seller_address,
			dna_sample_tracking_id,
			testing_price,
			qc_price,
			pay_amount,
		}
	}
}
