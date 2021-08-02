//use sp_std::prelude::*;

pub trait UserProfileInterface<T: frame_system::Config, EthAddress> {

    fn get_eth_address_by_account_id(account_id: &T::AccountId) -> Option<EthAddress>;
    fn set_eth_address_by_account_id(account_id: &T::AccountId, eth_address: &EthAddress) -> (); 

    fn get_account_id_by_eth_address(eth_address: &EthAddress) -> Option<T::AccountId>;
}
