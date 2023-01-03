use crate::*;

use frame_support::{sp_runtime::traits::Saturating, traits::Currency};
use scale_info::TypeInfo;
use sp_std::vec::Vec;

pub type Country = Vec<u8>;
pub type Region = Vec<u8>;
pub type City = Vec<u8>;
pub type ServiceCategory = Vec<u8>;

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type CurrencyOf<T> = <T as self::Config>::Currency;
pub type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;
pub type HashOf<T> = <T as frame_system::Config>::Hash;
pub type RequestOf<T> = Request<AccountIdOf<T>, BalanceOf<T>, HashOf<T>>;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum RequestStatus {
	Open,
	WaitingForUnstaked,
	Unstaked,
	Claimed,
	Processed,
	Finalized,
	InLabList,
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
	pub service_id: Option<Hash>,
	pub order_id: Option<Hash>,
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
impl<AccountId, Balance, Hash> Request<AccountId, Balance, Hash>
where
	AccountId: Clone,
	Balance: Saturating,
{
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
			service_id: None,
			order_id: None,
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
