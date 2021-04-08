use sp_std::prelude::*;

/// Interface for Lab Pallet
/// Defines the functionalities of Lab Pallet 
pub trait LabInterface<T> {
    type Error;
    type Lab;

    fn register_lab() -> Result<(), Self::Error>;
    fn update_lab() -> Result<(), Self::Error>;
    fn delete_lab() -> Result<(), Self::Error>;

    fn labs_by_country_city() -> Option<Vec<Self::Lab>>;
    fn lab_by_account_id() -> Option<Self::Lab>;
}
