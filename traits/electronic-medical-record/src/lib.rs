#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;
//use sp_std::prelude::*;

pub trait ElectronicMedicalRecordInfo<T: Config> {
    fn get_owner_id(&self) -> &T::AccountId;
}

pub trait ElectronicMedicalRecordsProvider<T: Config> {
    type Error;
    type ElectronicMedicalRecord: ElectronicMedicalRecordInfo<T> + sp_std::fmt::Debug;

    fn remove_electronic_medical_record(owner_id: &T::AccountId) -> Result<Self::ElectronicMedicalRecord, Self::Error>;
    fn electronic_medical_record_by_owner_id(owner_id: &T::AccountId) -> Option<Self::ElectronicMedicalRecord>;
}

