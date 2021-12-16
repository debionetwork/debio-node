#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;
//use sp_std::prelude::*;

pub trait HospitalCertificationInfo<T: Config> {
    fn get_id(&self) -> &T::Hash;
    fn get_owner_id(&self) -> &T::AccountId;
}

pub trait HospitalCertificationsProvider<T: Config> {
    type Error;
    type HospitalCertification: HospitalCertificationInfo<T> + sp_std::fmt::Debug;

    fn delete_certification(
        owner_id: &T::AccountId,
        id: &T::Hash,
    ) -> Result<Self::HospitalCertification, Self::Error>;
    fn certification_by_id(id: &T::Hash) -> Option<Self::HospitalCertification>;
}

pub trait HospitalCertificationOwnerInfo<T: Config> {
    fn get_owner_id(&self) -> &T::AccountId;
}

pub trait HospitalCertificationOwner<T: Config> {
    type Owner: HospitalCertificationOwnerInfo<T> + sp_std::fmt::Debug;

    fn can_create_certification(id: &T::AccountId) -> bool;
    fn get_owner(id: &T::AccountId) -> Option<Self::Owner>;
    fn associate(owner_id: &T::AccountId, certification_id: &T::Hash);
    fn disassociate(owner_id: &T::AccountId, certification_id: &T::Hash);
}
