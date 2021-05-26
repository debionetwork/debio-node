//use sp_std::prelude::*;

pub trait ElectricalMedicalRecordInterface<T: frame_system::Config> {
    type Error;
    type ElectricalMedicalRecord;
    type ElectricalMedicalRecordInfo;

    fn upload_electrical_medical_record(owner_id: &T::AccountId, electrical_medical_record: &Self::ElectricalMedicalRecordInfo) -> Result<Self::ElectricalMedicalRecord, Self::Error>;
    fn remove_electrical_medical_record(owner_id: &T::AccountId) -> Result<Self::ElectricalMedicalRecord, Self::Error>;
    fn electrical_medical_record_by_owner_id(owner_id: &T::AccountId) -> Option<Self::ElectricalMedicalRecord>;
}
