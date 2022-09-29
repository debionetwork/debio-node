use crate::*;

use frame_support::{pallet_prelude::*, traits::Currency};
use primitives_duration::ExpectedDuration;
use primitives_price_and_currency::PriceByCurrency;
use scale_info::TypeInfo;
use sp_std::vec::Vec;
use traits_services::{types::ServiceFlow, ServiceInfo as ServiceInfoT};

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type HashOf<T> = <T as frame_system::Config>::Hash;
pub type CurrencyOf<T> = <T as self::Config>::Currency;
pub type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;
pub type ServiceOf<T> = Service<AccountIdOf<T>, HashOf<T>, BalanceOf<T>>;
pub type ServiceInfoOf<T> = ServiceInfo<BalanceOf<T>>;
pub type ServiceIdOf<T> = HashOf<T>;

/// ServiceInfo struct
/// Information that is mutable by user
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct ServiceInfo<Balance> {
	pub name: Vec<u8>,
	pub prices_by_currency: Vec<PriceByCurrency<Balance>>,
	pub expected_duration: ExpectedDuration,
	pub category: Vec<u8>,
	pub description: Vec<u8>, // TODO: limit the length
	pub dna_collection_process: Vec<u8>,
	pub test_result_sample: Vec<u8>,
	pub long_description: Option<Vec<u8>>,
	pub image: Option<Vec<u8>>,
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct Service<AccountId, Hash, Balance> {
	pub id: Hash,
	pub owner_id: AccountId,
	pub info: ServiceInfo<Balance>,
	pub service_flow: ServiceFlow,
}
impl<AccountId, Hash, Balance> Service<AccountId, Hash, Balance> {
	pub fn new(
		id: Hash,
		owner_id: AccountId,
		info: ServiceInfo<Balance>,
		service_flow: ServiceFlow,
	) -> Self {
		Self { id, owner_id, info, service_flow }
	}

	pub fn get_id(&self) -> &Hash {
		&self.id
	}

	pub fn get_owner_id(&self) -> &AccountId {
		&self.owner_id
	}

	pub fn get_service_flow(&self) -> &ServiceFlow {
		&self.service_flow
	}

	pub fn get_prices_by_currency(&self) -> &Vec<PriceByCurrency<Balance>> {
		&self.info.prices_by_currency
	}
}

impl<T, AccountId, Hash, Balance> ServiceInfoT<T, Balance> for Service<AccountId, Hash, Balance>
where
	T: frame_system::Config<AccountId = AccountId, Hash = Hash>,
{
	fn get_id(&self) -> &Hash {
		self.get_id()
	}
	fn get_owner_id(&self) -> &AccountId {
		self.get_owner_id()
	}
	fn get_service_flow(&self) -> &ServiceFlow {
		self.get_service_flow()
	}
	fn get_prices_by_currency(&self) -> &Vec<PriceByCurrency<Balance>> {
		self.get_prices_by_currency()
	}
}
