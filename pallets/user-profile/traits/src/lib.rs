#![cfg_attr(not(feature = "std"), no_std)]

pub trait UserProfileProvider<T: frame_system::Config, EthAddress, ProfileRoles> {
	type Error;
	type ProfileRoles;

	fn get_eth_address_by_account_id(account_id: &T::AccountId) -> Option<EthAddress>;
	fn get_registered_account_id(account_id: &T::AccountId) -> Option<bool>;

	fn get_account_profile_roles(account_id: &T::AccountId) -> Option<ProfileRoles>;
	fn set_account_profile_roles(account_id: &T::AccountId, roles: &ProfileRoles);

	fn set_account_profile_role_to_lab(
		account_id: &T::AccountId,
	) -> Result<Self::ProfileRoles, Self::Error>;
	fn set_account_profile_role_to_customer(
		account_id: &T::AccountId,
	) -> Result<Self::ProfileRoles, Self::Error>;
	fn set_account_profile_role_to_genetic_analyst(
		account_id: &T::AccountId,
	) -> Result<Self::ProfileRoles, Self::Error>;
	fn set_account_profile_role_to_doctor(
		account_id: &T::AccountId,
	) -> Result<Self::ProfileRoles, Self::Error>;
	fn set_account_profile_role_to_hospital(
		account_id: &T::AccountId,
	) -> Result<Self::ProfileRoles, Self::Error>;
}
