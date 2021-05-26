#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;
//use sp_std::prelude::*;

pub trait ElectricalMedicalRecordInfo<T: Config> {
    fn get_owner_id(&self) -> &T::AccountId;
}

pub trait ElectricalMedicalRecordsProvider<T: Config> {
    type Error;
    type ElectricalMedicalRecord: ElectricalMedicalRecordInfo<T> + sp_std::fmt::Debug;

    fn remove_electrical_medical_record(owner_id: &T::AccountId) -> Result<Self::ElectricalMedicalRecord, Self::Error>;
    fn electrical_medical_record_by_owner_id(owner_id: &T::AccountId) -> Option<Self::ElectricalMedicalRecord>;
}

