#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;
use primitives_price_and_currency::PriceByCurrency;
use sp_std::prelude::*;

pub trait GeneticAnalystServiceInfo<T: Config, Balance> {
	fn get_id(&self) -> &T::Hash;
	fn get_owner_id(&self) -> &T::AccountId;
	fn get_prices_by_currency(&self) -> &Vec<PriceByCurrency<Balance>>;
}

pub trait GeneticAnalystServicesProvider<T: Config, Balance> {
	type Error;
	type GeneticAnalystService: GeneticAnalystServiceInfo<T, Balance> + sp_std::fmt::Debug;

	fn delete_genetic_analyst_service(
		owner_id: &T::AccountId,
		id: &T::Hash,
	) -> Result<Self::GeneticAnalystService, Self::Error>;
	fn genetic_analyst_service_by_id(id: &T::Hash) -> Option<Self::GeneticAnalystService>;
}

pub trait GeneticAnalystServiceOwnerInfo<T: Config> {
	fn get_id(&self) -> &T::AccountId;
}

pub trait GeneticAnalystServiceOwner<T: Config> {
	type Owner: GeneticAnalystServiceOwnerInfo<T> + sp_std::fmt::Debug;

	fn can_create_genetic_analyst_service(id: &T::AccountId) -> bool;
	fn get_owner(id: &T::AccountId) -> Option<Self::Owner>;
	fn associate(owner_id: &T::AccountId, genetic_analyst_service_id: &T::Hash);
	fn disassociate(owner_id: &T::AccountId, genetic_analyst_service_id: &T::Hash);
}
