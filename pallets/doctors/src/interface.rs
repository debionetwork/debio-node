use sp_std::prelude::*;

/// Interface for Doctor Pallet
/// Defines the functionalities of Doctor Pallet 
pub trait DoctorInterface<T: frame_system::Config> {
    type Error;
    type DoctorInfo;
    type Doctor;

    /// Get doctor by associated account_id
    fn doctor_by_account_id(account_id: &T::AccountId) -> Option<Self::Doctor>;
    /// Get the account_ids of doctors in a location
    fn doctors_by_country_region_city(country_region_code: &Vec<u8>, city_code: &Vec<u8>) -> Option<Vec<T::AccountId>>;

    /// Store A doctor with its information
    fn create_doctor(account_id: &T::AccountId, doctor_info: &Self::DoctorInfo) -> Result<Self::Doctor, Self::Error>;
    /// Update a Doctor information
    fn update_doctor(account_id: &T::AccountId, doctor_info: &Self::DoctorInfo) -> Result<Self::Doctor, Self::Error>;
    /// Delete Doctor
    fn delete_doctor(account_id: &T::AccountId) -> Result<Self::Doctor, Self::Error>;
}
