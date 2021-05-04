//use sp_std::prelude::*;

pub trait UserProfileInterface<T: frame_system::Config> {
    type UsdtAddress;

    fn get_usdt_address_by_account_id(account_id: &T::AccountId) -> Option<Self::UsdtAddress>;
    fn set_usdt_address_by_account_id(account_id: &T::AccountId, usdt_address: &Self::UsdtAddress) -> (); 
}
