//use sp_std::prelude::*;

use sp_std::vec::Vec;

pub trait ElectronicMedicalRecordInterface<T: frame_system::Config> {
    type Error;
    type ElectronicMedicalRecord;
    type ElectronicMedicalRecordFileId;
    type ElectronicMedicalRecordFile;

    fn generate_electronic_medical_record_file_id(
        owner_id: &T::AccountId,
        electronic_medical_record_file_count: u64,
    ) -> Self::ElectronicMedicalRecordFileId;

    fn add_electronic_medical_record(
        owner_id: &T::AccountId,
        title: &Vec<u8>,
        category: &Vec<u8>,
    ) -> Result<Self::ElectronicMedicalRecord, Self::Error>;
    fn remove_electronic_medical_record(
        owner_id: &T::AccountId,
    ) -> Result<Self::ElectronicMedicalRecord, Self::Error>;
    fn electronic_medical_record_by_owner_id(
        owner_id: &T::AccountId,
    ) -> Option<Self::ElectronicMedicalRecord>;

    fn add_electronic_medical_record_file(
        owner_id: &T::AccountId,
        title: &mut Vec<u8>,
        description: &mut Vec<u8>,
        record_link: &mut Vec<u8>,
    ) -> Result<Self::ElectronicMedicalRecordFile, Self::Error>;
    fn remove_electronic_medical_record_file(
        owner_id: &T::AccountId,
        electronic_medical_record_id: &Self::ElectronicMedicalRecordFileId,
    ) -> Result<Self::ElectronicMedicalRecordFile, Self::Error>;

    fn electronic_medical_record_file_count_by_owner(owner_id: &T::AccountId) -> u64;
    fn electronic_medical_record_file_by_id(
        electronic_medical_record_id: &Self::ElectronicMedicalRecordFileId,
    ) -> Option<Self::ElectronicMedicalRecordFile>;
}
