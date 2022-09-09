pub trait UserProfileInterface<T: frame_system::Config, EthAddress, ProfileRoles> {
	type Error;

	fn get_eth_address_by_account_id(account_id: &T::AccountId) -> Option<EthAddress>;
	fn set_eth_address_by_account_id(account_id: &T::AccountId, eth_address: &EthAddress);
	fn register_account_id(account_id: &T::AccountId);
	fn update_admin_key(
		account_id: &T::AccountId,
		admin_key: &T::AccountId,
	) -> Result<(), Self::Error>;

	fn set_account_profile_roles(
		account_id: &T::AccountId,
		role: &ProfileRoles,
	) -> Result<(), Self::Error>;
	fn set_account_profile_role_to_customer(
		account_id: &T::AccountId,
	) -> Result<ProfileRoles, Self::Error>;

	fn get_registered_account_id(account_id: &T::AccountId) -> Option<bool>;
	fn get_account_profile_roles(account_id: &T::AccountId) -> Option<ProfileRoles>;

	fn get_account_id_by_eth_address(eth_address: &EthAddress) -> Option<T::AccountId>;
}
