#![cfg_attr(not(feature = "std"), no_std)]


pub mod structs {
    use frame_support::pallet_prelude::*;
    use sp_std::prelude::*;

    #[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
    pub struct ServiceInfo<Balance> {
        name: Vec<u8>,
        price: Balance,
        description: Vec<u8>, // TODO: limit the length
        long_description: Option<Vec<u8>>,
        image: Option<Vec<u8>>
    }

    #[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
    pub struct Service<AccountId, Hash, Balance> {
        pub id: Hash,
        pub owner_id: AccountId,
        pub info: ServiceInfo<Balance>,
    }
    impl<AccountId, Hash, Balance> Service<AccountId, Hash, Balance> {
        pub fn get_id(&self) -> &Hash {
            &self.id
        }

        pub fn get_owner_id(&self) -> &AccountId {
            &self.owner_id
        }

        pub fn get_price(&self) -> &Balance {
            &self.info.price
        }
    }
}

use structs::Service;
use frame_system::Config;

pub trait ServicesContainer<T: Config> {
    type Balance;

    fn service_by_id(id: &T::Hash) -> Option<Service<T::AccountId, T::Hash, Self::Balance>>;
}
