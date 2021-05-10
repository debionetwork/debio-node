#![cfg_attr(not(feature = "std"), no_std)]

pub trait UserProfileProvider<T: frame_system::Config, EthAddress> {
    fn get_eth_address_by_account_id(account_id: &T::AccountId) -> Option<EthAddress>;
}
