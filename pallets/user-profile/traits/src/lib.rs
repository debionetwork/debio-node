#![cfg_attr(not(feature = "std"), no_std)]

pub trait UserProfileProvider<T: frame_system::Config, EthAddress, ProfileRoles> {
	fn get_eth_address_by_account_id(account_id: &T::AccountId) -> Option<EthAddress>;
	fn get_registered_account_id(account_id: &T::AccountId) -> Option<bool>;
	fn get_account_profile_roles(account_id: &T::AccountId) -> Option<ProfileRoles>;
	fn set_account_profile_roles(account_id: &T::AccountId, roles: &ProfileRoles);
}
