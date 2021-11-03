#![cfg_attr(not(feature = "std"), no_std)]

pub use scale_info::TypeInfo;

use sp_std::prelude::*;
use frame_support::pallet_prelude::*;

// LabVerificationStatus
#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum LabVerificationStatus {
    Unverified,
    Verified,
    Rejected,
    Revoked
}
impl Default for LabVerificationStatus {
    fn default() -> Self {
        Self::Unverified
    }
}

pub trait LabVerificationStatusTrait {
    fn is_verified(&self) -> bool;
    fn is_unverified(&self) -> bool;
    fn is_rejected(&self) -> bool;
    fn is_revoked(&self) -> bool;
}
impl LabVerificationStatusTrait for LabVerificationStatus {
    fn is_verified(&self) -> bool {
        match *self {
            LabVerificationStatus::Verified => true,
            _ => false
        }
    }
    fn is_unverified(&self) -> bool {
        match *self {
            LabVerificationStatus::Unverified => true,
            _ => false
        }
    }
    fn is_rejected(&self) -> bool {
        match *self {
            LabVerificationStatus::Rejected => true,
            _ => false
        }
    }
    fn is_revoked(&self) -> bool {
        match *self {
            LabVerificationStatus::Revoked => true,
            _ => false
        }
    }
}

/// Interface for Lab Pallet
/// Defines the functionalities of Lab Pallet
pub trait LabInterface<T: frame_system::Config> {
    type Error;
    type LabInfo;
    type Lab;
    type LabVerificationStatus: LabVerificationStatusTrait;

    /// Get lab by associated account_id
    fn lab_by_account_id(account_id: &T::AccountId) -> Option<Self::Lab>;
    /// Get lab verification status
    fn lab_verification_status(
        account_id: &T::AccountId
    ) -> Option<Self::LabVerificationStatus>;
    /// Get the account_ids of labs in a location
    fn labs_by_country_region_city(
        country_region_code: &Vec<u8>,
        city_code: &Vec<u8>,
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
        status: &Self::LabVerificationStatus,
    ) -> Result<Self::Lab, Self::Error>;
    /// Delete Lab
    fn delete_lab(account_id: &T::AccountId) -> Result<Self::Lab, Self::Error>;
}
