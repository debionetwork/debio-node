//use sp_std::prelude::*;

use sp_std::vec::Vec;

pub trait ElectronicMedicalRecordInterface<T: frame_system::Config> {
    type Error;
    type ElectronicMedicalRecord;
    type ElectronicMedicalRecordInfoId;
    type ElectronicMedicalRecordInfo;
    
    fn generate_electronic_medical_record_info_id(owner_id: &T::AccountId, electronic_medical_record_info_count: u64) -> Self::ElectronicMedicalRecordInfoId;

    fn add_electronic_medical_record(owner_id: &T::AccountId) -> Result<Self::ElectronicMedicalRecord, Self::Error>;
    fn remove_electronic_medical_record(owner_id: &T::AccountId) -> Result<Self::ElectronicMedicalRecord, Self::Error>;
    fn electronic_medical_record_by_owner_id(owner_id: &T::AccountId) -> Option<Self::ElectronicMedicalRecord>;

    fn add_electronic_medical_record_info(owner_id: &T::AccountId, title: &mut Vec<u8>, description: &mut Vec<u8>, record_link: &mut Vec<u8>) -> Result<Self::ElectronicMedicalRecordInfo, Self::Error>;
    fn remove_electronic_medical_record_info(owner_id: &T::AccountId, electronic_medical_record_id: &Self::ElectronicMedicalRecordInfoId) -> Result<Self::ElectronicMedicalRecordInfo, Self::Error>;
    
    fn electronic_medical_record_info_count_by_owner(owner_id: &T::AccountId) -> u64;
    fn electronic_medical_record_info_by_id(electronic_medical_record_id: &Self::ElectronicMedicalRecordInfoId) -> Option<Self::ElectronicMedicalRecordInfo>;
}
