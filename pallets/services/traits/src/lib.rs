#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;
use primitives_price_and_currency::PriceByCurrency;
use sp_std::prelude::*;

pub mod types {
	use frame_support::{
		codec::{Decode, Encode},
		pallet_prelude::*,
	};
	use scale_info::TypeInfo;
	use sp_std::prelude::*;

	#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
	pub enum ServiceFlow {
		RequestTest,
		StakingRequestService,
	}
	impl Default for ServiceFlow {
		fn default() -> Self {
			ServiceFlow::RequestTest
		}
	}
}

pub trait ServiceInfo<T: Config, Balance> {
	fn get_id(&self) -> &T::Hash;
	fn get_owner_id(&self) -> &T::AccountId;
	fn get_prices_by_currency(&self) -> &Vec<PriceByCurrency<Balance>>;
	fn get_service_flow(&self) -> &types::ServiceFlow;
	fn is_service_owner(&self, account_id: &T::AccountId) -> bool;
}

pub trait ServicesProvider<T: Config, Balance> {
	type Error;
	type Service: ServiceInfo<T, Balance> + sp_std::fmt::Debug;

	fn delete_service(owner_id: &T::AccountId, id: &T::Hash) -> Result<Self::Service, Self::Error>;
	fn service_by_id(id: &T::Hash) -> Option<Self::Service>;
}

pub trait ServiceOwnerInfo<T: Config> {
	fn get_id(&self) -> &T::AccountId;
}

pub trait ServiceOwner<T: Config> {
	type Owner: ServiceOwnerInfo<T> + sp_std::fmt::Debug;

	fn can_create_service(id: &T::AccountId) -> bool;
	fn get_owner(id: &T::AccountId) -> Option<Self::Owner>;
	fn associate(owner_id: &T::AccountId, service_id: &T::Hash);
	fn disassociate(owner_id: &T::AccountId, service_id: &T::Hash);
}
