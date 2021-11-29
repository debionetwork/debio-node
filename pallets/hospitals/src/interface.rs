use primitives_area_code::{CityCode, CountryRegionCode};
use sp_std::prelude::*;

/// Interface for Hospital Pallet
/// Defines the functionalities of Hospital Pallet
pub trait HospitalInterface<T: frame_system::Config> {
	type Error;
	type HospitalInfo;
	type Hospital;

	/// Get hospital by associated account_id
	fn hospital_by_account_id(account_id: &T::AccountId) -> Option<Self::Hospital>;
	/// Get the account_ids of hospitals in a location
	fn hospitals_by_country_region_city(
		country_region_code: &CountryRegionCode,
		city_code: &CityCode,
	) -> Option<Vec<T::AccountId>>;

	/// Store A hospital with its information
	fn create_hospital(
		account_id: &T::AccountId,
		hospital_info: &Self::HospitalInfo,
	) -> Result<Self::Hospital, Self::Error>;
	/// Update a Hospital information
	fn update_hospital(
		account_id: &T::AccountId,
		hospital_info: &Self::HospitalInfo,
	) -> Result<Self::Hospital, Self::Error>;
	/// Delete Hospital
	fn delete_hospital(account_id: &T::AccountId) -> Result<Self::Hospital, Self::Error>;
}
