#![cfg_attr(not(feature = "std"), no_std)]

pub use scale_info::TypeInfo;

use sp_std::prelude::*;

use primitives_area_code::{CityCode, CountryRegionCode};
use primitives_verification_status::VerificationStatusTrait;

/// Interface for Lab Pallet
/// Defines the functionalities of Lab Pallet
pub trait LabInterface<T: frame_system::Config> {
	type Error;
	type Moment;
	type Balance;
	type LabInfo;
	type Lab;
	type VerificationStatus: VerificationStatusTrait;

	/// Get lab by associated account_id
	fn lab_by_account_id(account_id: &T::AccountId) -> Option<Self::Lab>;
	/// Get lab verification status
	fn lab_verification_status(account_id: &T::AccountId) -> Option<Self::VerificationStatus>;
	/// Get the account_ids of labs in a location
	fn labs_by_country_region_city(
		country_region_code: &CountryRegionCode,
		city_code: &CityCode,
	) -> Option<Vec<T::AccountId>>;

	/// Store A lab with its information
	fn create_lab(
		account_id: &T::AccountId,
		lab_info: &Self::LabInfo,
	) -> Result<Self::Lab, Self::Error>;
	/// Update a Lab information
	fn update_lab(
		account_id: &T::AccountId,
		lab_info: &Self::LabInfo,
	) -> Result<Self::Lab, Self::Error>;
	/// Update a Lab verification status
	fn update_lab_verification_status(
		lab_verifier_key: &T::AccountId,
		account_id: &T::AccountId,
		status: &Self::VerificationStatus,
	) -> Result<Self::Lab, Self::Error>;
	/// Stake Lab
	fn stake_lab(account_id: &T::AccountId) -> Result<Self::Lab, Self::Error>;
	/// Unstake Lab
	fn unstake_lab(account_id: &T::AccountId) -> Result<Self::Lab, Self::Error>;
	/// Retrieve Unstake Amount
	fn retrieve_unstake_amount(
		admin_key: &T::AccountId,
		account_id: &T::AccountId,
	) -> Result<Self::Lab, Self::Error>;
	/// Update Lab minimum stake amount
	fn update_minimum_stake_amount(
		account_id: &T::AccountId,
		amount: Self::Balance,
	) -> Result<(), Self::Error>;
	/// Update Lab unstake time
	fn update_unstake_time(
		account_id: &T::AccountId,
		moment: Self::Moment,
	) -> Result<(), Self::Error>;
	/// Update admin key
	fn update_admin_key(
		account_id: &T::AccountId,
		admin_key: &T::AccountId,
	) -> Result<(), Self::Error>;
	/// Delete Lab
	fn delete_lab(account_id: &T::AccountId) -> Result<Self::Lab, Self::Error>;
}
