#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;
use pallet_timestamp::Config as TimeConfig;
//use sp_std::prelude::*;

pub trait ElectronicMedicalRecordFile<T: Config + TimeConfig> {
    fn get_id(&self) -> &T::Hash;
    fn get_uploaded_at(&self) -> &T::Moment;
    fn get_electronic_medical_record_id(&self) -> &T::Hash;
}

pub trait ElectronicMedicalRecordFilesProvider<T: Config + TimeConfig> {
    type Error;
    type Moment;
    type ElectronicMedicalRecordFile: ElectronicMedicalRecordFile<T> + sp_std::fmt::Debug;

    fn remove_electronic_medical_record_file(
        owner_id: &T::AccountId,
        electronic_medical_record_file_id: &T::Hash,
    ) -> Result<Self::ElectronicMedicalRecordFile, Self::Error>;
    fn electronic_medical_record_file_by_id(
        electronic_medical_record_file_id: &T::Hash,
    ) -> Option<Self::ElectronicMedicalRecordFile>;
}

pub trait ElectronicMedicalRecordFileOwnerInfo<T: Config> {
    fn get_electronic_medical_record_id(&self) -> &T::Hash;
}

pub trait ElectronicMedicalRecordFileByElectronicMedicalRecord<T: Config> {
    type ElectronicMedicalRecord: ElectronicMedicalRecordFileOwnerInfo<T> + sp_std::fmt::Debug;

    fn associate(electronic_medical_record_id: &T::Hash, electronic_medical_record_file_id: &T::Hash) -> ();
    fn disassociate(electronic_medical_record_id: &T::Hash, electronic_medical_record_file_id: &T::Hash) -> ();
}
