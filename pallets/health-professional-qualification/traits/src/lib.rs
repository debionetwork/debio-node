#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;

pub trait HealthProfessionalQualificationOwner<T: Config> {
	type Owner: sp_std::fmt::Debug;

	fn can_create_qualification(id: &T::AccountId) -> bool;
	fn get_owner(id: &T::AccountId) -> Option<Self::Owner>;
	fn associate(owner_id: &T::AccountId, qualification_id: &T::Hash);
	fn disassociate(owner_id: &T::AccountId, qualification_id: &T::Hash);
}

pub trait HealthProfessionalQualificationProvider<T: Config> {
	type Error;

	fn delete_qualifications(account_id: &T::AccountId, hash_ids: &[T::Hash]);
}

pub trait HealthProfessionalQualificationCountT<T: Config> {
	fn add_health_professional_qualification_count(value: u64);
	fn substract_health_professional_qualification_count(value: u64);
	fn add_health_professional_qualification_count_by_owner(account_id: &T::AccountId, value: u64);
	fn substract_health_professional_qualification_count_by_owner(
		account_id: &T::AccountId,
		value: u64,
	);
}
