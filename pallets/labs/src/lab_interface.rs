use sp_std::prelude::*;

/// Interface for Lab Pallet
/// Defines the functionalities of Lab Pallet 
pub trait LabInterface<T: frame_system::Config> {
    /// Error
    type Error;
    /// LabInfo Struct
    type LabInfo;

    /// Get lab by associated account_id
    fn lab_by_account_id(account_id: &T::AccountId) -> Option<Self::LabInfo>;
    /// Get the account_ids of labs in a location
    fn labs_by_country_city(country: &Vec<u8>, city: &Vec<u8>) -> Option<Vec<T::AccountId>>;

    /// Store A lab with its information
    fn create_lab(account_id: &T::AccountId, lab_info: &Self::LabInfo) -> Result<(), Self::Error>;
    /// Update a Lab information
    fn update_lab(account_id: &T::AccountId, lab_info: &Self::LabInfo) -> Result<(), Self::Error>;
    /// Delete Lab
    fn delete_lab(account_id: &T::AccountId) -> Result<(), Self::Error>;
}
