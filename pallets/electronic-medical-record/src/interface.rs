use sp_std::vec::Vec;

pub trait ElectronicMedicalRecordInterface<T: frame_system::Config> {
    type Error;
    type ElectronicMedicalRecordId;
    type ElectronicMedicalRecord;
    type ElectronicMedicalRecordFileId;
    type ElectronicMedicalRecordFile;
    type ElectronicMedicalRecordFileSubmission;

    fn generate_electronic_medical_record_id(
        owner_id: &T::AccountId,
        electronic_medical_record_file_count: u64,
    ) -> Self::ElectronicMedicalRecordId;

    fn generate_electronic_medical_record_file_id(
        electronic_medical_record_id: &T::Hash,
        electronic_medical_record_file_count: u64,
    ) -> Self::ElectronicMedicalRecordFileId;

    fn add_electronic_medical_record(
        owner_id: &T::AccountId,
        title: &[u8],
        category: &[u8],
        files: &Vec<Self::ElectronicMedicalRecordFileSubmission>
    ) -> Result<Self::ElectronicMedicalRecord, Self::Error>;

    fn remove_electronic_medical_record(
        owner_id: &T::AccountId,
        electronic_medical_record_id: &T::Hash,
    ) -> Result<Self::ElectronicMedicalRecord, Self::Error>;

    fn electronic_medical_record_count_by_owner(
        owner_id: &T::AccountId
    ) -> u64;

    fn electronic_medical_record_by_owner_id(
        owner_id: &T::AccountId,
    ) -> Option<Vec<T::Hash>>;

    fn electronic_medical_record_by_id(
        electronic_medical_record_id: &Self::ElectronicMedicalRecordId,
    ) -> Option<Self::ElectronicMedicalRecord>;

    fn electronic_medical_record_file_by_id(
        electronic_medical_record_id: &Self::ElectronicMedicalRecordFileId,
    ) -> Option<Self::ElectronicMedicalRecordFile>;
}
