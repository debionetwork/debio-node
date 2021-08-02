#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;
use sp_std::prelude::*;

pub mod types {
    use frame_support::codec::{Encode, Decode};
    use frame_support::pallet_prelude::*;
    use sp_std::prelude::*;

    #[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
    pub enum CurrencyType {
        Dai,
        Ethereum,
    }
    impl Default for CurrencyType {
        fn default() -> Self { CurrencyType::Dai }
    }

    #[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
    pub struct Price<Balance> {
        pub component: Vec<u8>,
        pub value: Balance,
    }

    #[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
    pub struct PriceByCurrency<Balance> {
        pub currency: CurrencyType,
        pub total_price: Balance,
        pub price_components: Vec<Price<Balance>>,
        pub additional_prices: Vec<Price<Balance>>,
    }

    #[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
    pub enum DurationType {
        WorkingDays,
        Hours,
        Days,
    }
    impl Default for DurationType {
        fn default() -> Self { DurationType::WorkingDays }
    }
    
    #[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
    pub struct ExpectedDuration {
        pub duration: i8,
        pub duration_type: DurationType,
    }
}

pub trait ServiceInfo<T: Config, Balance> {
    fn get_id(&self) -> &T::Hash;
    fn get_owner_id(&self) -> &T::AccountId;
    fn get_prices_by_currency(&self) -> &Vec<types::PriceByCurrency<Balance>>;
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
    fn associate(owner_id: &T::AccountId, service_id: &T::Hash) -> ();
    fn disassociate(owner_id: &T::AccountId, service_id: &T::Hash) -> ();
}

pub trait DaiHash<T: Config> {
    type DaiHash;

    fn dai_hash(&self) -> &Self::DaiHash;
}