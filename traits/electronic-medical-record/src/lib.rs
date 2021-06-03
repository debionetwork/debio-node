#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;
//use sp_std::prelude::*;

pub trait ElectronicMedicalRecordInfo<T: Config> {
    fn get_id(&self) -> &T::Hash;
    fn get_owner_id(&self) -> &T::AccountId;
}

pub trait ElectronicMedicalRecordInfosProvider<T: Config> {
    type Error;
    type ElectronicMedicalRecordInfo: ElectronicMedicalRecordInfo<T> + sp_std::fmt::Debug;

    fn remove_electronic_medical_record_info(owner_id: &T::AccountId, electronic_medical_record_info_id: &T::Hash) -> Result<Self::ElectronicMedicalRecordInfo, Self::Error>;
    fn electronic_medical_record_info_by_id(electronic_medical_record_info_id: &T::Hash) -> Option<Self::ElectronicMedicalRecordInfo>;
}

pub trait ElectronicMedicalRecordInfoOwnerInfo<T: Config> {
    fn get_id(&self) -> &T::AccountId;
}

pub trait ElectronicMedicalRecordInfoOwner<T: Config> {
    type Owner: ElectronicMedicalRecordInfoOwnerInfo<T> + sp_std::fmt::Debug;

    fn can_create_electronic_medical_record_info(id: &T::AccountId) -> bool;
    fn get_owner(id: &T::AccountId) -> Option<Self::Owner>;
    fn associate(owner_id: &T::AccountId, electronic_medical_record_info_id: &T::Hash) -> ();
    fn disassociate(owner_id: &T::AccountId, electronic_medical_record_info_id: &T::Hash) -> ();
}