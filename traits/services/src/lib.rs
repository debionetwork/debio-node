#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;
use sp_std::prelude::*;

pub trait ServiceInfo<T: Config> {
    fn get_id(&self) -> &T::Hash;
    fn get_owner_id(&self) -> &T::AccountId;
}

pub trait ServicesProvider<T: Config> {
    type Error;
    type Balance;
    type Service: ServiceInfo<T> + sp_std::fmt::Debug;

    fn service_by_id(id: &T::Hash) -> Option<Self::Service>;
    fn delete_service(owner_id: &T::AccountId, id: &T::Hash) -> Result<Self::Service, Self::Error>;
}

pub trait ServiceOwnerInfo<T: Config> {
    fn get_id(&self) -> &T::AccountId;
    fn get_country(&self) -> &Vec<u8>;
    fn get_city(&self) -> &Vec<u8>;
}

pub trait ServiceOwner<T: Config> {
    type Owner: ServiceOwnerInfo<T> + sp_std::fmt::Debug;

    fn can_create_service(id: &T::AccountId) -> bool;
    fn get_owner(id: &T::AccountId) -> Option<Self::Owner>;
    fn associate(owner_id: &T::AccountId, service_id: &T::Hash) -> ();
    fn disassociate(owner_id: &T::AccountId, service_id: &T::Hash) -> ();
}

