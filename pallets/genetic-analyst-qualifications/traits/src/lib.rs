#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;
//use sp_std::prelude::*;

pub trait GeneticAnalystQualificationInfo<T: Config> {
	fn get_id(&self) -> &T::Hash;
	fn get_owner_id(&self) -> &T::AccountId;
}

pub trait GeneticAnalystQualificationsProvider<T: Config> {
	type Error;
	type GeneticAnalystQualification: GeneticAnalystQualificationInfo<T> + sp_std::fmt::Debug;

	fn delete_qualification(
		owner_id: &T::AccountId,
		id: &T::Hash,
	) -> Result<Self::GeneticAnalystQualification, Self::Error>;
	fn qualification_by_id(id: &T::Hash) -> Option<Self::GeneticAnalystQualification>;
}

pub trait GeneticAnalystQualificationOwnerInfo<T: Config> {
	fn get_owner_id(&self) -> &T::AccountId;
}

pub trait GeneticAnalystQualificationOwner<T: Config> {
	type Owner: GeneticAnalystQualificationOwnerInfo<T> + sp_std::fmt::Debug;

	fn can_create_qualification(id: &T::AccountId) -> bool;
	fn get_owner(id: &T::AccountId) -> Option<Self::Owner>;
	fn associate(owner_id: &T::AccountId, qualification_id: &T::Hash);
	fn disassociate(owner_id: &T::AccountId, qualification_id: &T::Hash);
}
