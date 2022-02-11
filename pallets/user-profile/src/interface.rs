pub trait UserProfileInterface<T: frame_system::Config, EthAddress> {
	type Error;

	fn get_eth_address_by_account_id(account_id: &T::AccountId) -> Option<EthAddress>;
	fn set_eth_address_by_account_id(account_id: &T::AccountId, eth_address: &EthAddress);

	fn update_admin_key(
		account_id: &T::AccountId,
		admin_key: &T::AccountId,
	) -> Result<(), Self::Error>;

	fn get_account_id_by_eth_address(eth_address: &EthAddress) -> Option<T::AccountId>;
}
