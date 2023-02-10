#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;
use pallet_timestamp::Config as TimeConfig;
use sp_std::vec::Vec;

pub trait ElectronicMedicalRecordFile<T: Config + TimeConfig> {
	fn get_id(&self) -> &T::Hash;
	fn get_uploaded_at(&self) -> &T::Moment;
	fn get_electronic_medical_record_id(&self) -> &T::Hash;
}

pub trait ElectronicMedicalRecordFilesProvider<T: Config + TimeConfig> {
	type ElectronicMedicalRecordFile: ElectronicMedicalRecordFile<T> + sp_std::fmt::Debug;

	fn electronic_medical_record_file_by_id(
		electronic_medical_record_file_id: &T::Hash,
	) -> Option<Self::ElectronicMedicalRecordFile>;
	fn valid_electronic_medical_record_id(
		account_id: &T::AccountId,
		electronical_medical_record_ids: &[T::Hash],
	) -> Vec<T::Hash>;
}

pub trait ElectronicMedicalRecordFileOwnerInfo<T: Config> {
	fn get_electronic_medical_record_id(&self) -> &T::Hash;
}

pub trait ElectronicMedicalRecordFileByElectronicMedicalRecord<T: Config> {
	type ElectronicMedicalRecord: ElectronicMedicalRecordFileOwnerInfo<T> + sp_std::fmt::Debug;

	fn associate(
		electronic_medical_record_id: &T::Hash,
		electronic_medical_record_file_id: &T::Hash,
	);
	fn disassociate(
		electronic_medical_record_id: &T::Hash,
		electronic_medical_record_file_id: &T::Hash,
	);
}
