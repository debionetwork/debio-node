#![cfg_attr(not(feature = "std"), no_std)]


pub mod structs {
    use frame_support::pallet_prelude::*;
    use sp_std::prelude::*;

    #[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
    pub struct Service<AccountId, Hash, Balance> {
        pub id: Hash,
        pub lab_id: AccountId,
        pub name: Vec<u8>,
        pub price: Balance,
        pub description: Vec<u8>, // TODO: limit the length
        pub long_description: Option<Vec<u8>>,
        pub image: Option<Vec<u8>>
    }
    impl<AccountId, Hash, Balance> Service<AccountId, Hash, Balance> {
        pub fn get_id(&self) -> &Hash {
            &self.id
        }

        pub fn get_lab_id(&self) -> &AccountId {
            &self.lab_id
        }

        pub fn get_price(&self) -> &Balance {
            &self.price
        }
    }
}

use structs::Service;
use frame_system::Config;

pub trait ServicesContainer<T: Config> {
    type Balance;

    fn service_by_id(id: &T::Hash) -> Option<Service<T::AccountId, T::Hash, Self::Balance>>;
}
