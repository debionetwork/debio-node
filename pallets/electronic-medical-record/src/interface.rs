//use sp_std::prelude::*;

pub trait ElectronicMedicalRecordInterface<T: frame_system::Config> {
    type Error;
    type ElectronicMedicalRecord;
    type ElectronicMedicalRecordInfo;

    fn upload_electronic_medical_record(owner_id: &T::AccountId, electronic_medical_record: &Self::ElectronicMedicalRecordInfo) -> Result<Self::ElectronicMedicalRecord, Self::Error>;
    fn remove_electronic_medical_record(owner_id: &T::AccountId) -> Result<Self::ElectronicMedicalRecord, Self::Error>;
    fn electronic_medical_record_by_owner_id(owner_id: &T::AccountId) -> Option<Self::ElectronicMedicalRecord>;
}
