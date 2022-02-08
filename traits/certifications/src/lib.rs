#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;
//use sp_std::prelude::*;

pub trait CertificationInfo<T: Config> {
	fn get_id(&self) -> &T::Hash;
	fn get_owner_id(&self) -> &T::AccountId;
}

pub trait CertificationsProvider<T: Config> {
	type Error;
	type Certification: CertificationInfo<T> + sp_std::fmt::Debug;

	fn delete_certification(
		owner_id: &T::AccountId,
		id: &T::Hash,
	) -> Result<Self::Certification, Self::Error>;
	fn certification_by_id(id: &T::Hash) -> Option<Self::Certification>;
}

pub trait CertificationOwnerInfo<T: Config> {
	fn get_owner_id(&self) -> &T::AccountId;
}

pub trait CertificationOwner<T: Config> {
	type Owner: CertificationOwnerInfo<T> + sp_std::fmt::Debug;

	fn can_create_certification(id: &T::AccountId) -> bool;
	fn get_owner(id: &T::AccountId) -> Option<Self::Owner>;
	fn associate(owner_id: &T::AccountId, certification_id: &T::Hash);
	fn disassociate(owner_id: &T::AccountId, certification_id: &T::Hash);
}
