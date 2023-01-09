#![cfg_attr(not(feature = "std"), no_std)]
mod mock;

#[allow(unused)]
use health_professional::{
	Config as HealthProfessionalConfig, HealthProfessionalInfo, Pallet as HealthProfessional,
};

#[allow(unused)]
use health_professional_qualification::{
	Certification, Config as HealthProfessionalQualificationConfig, Experience,
	Pallet as HealthProfessionalQualification,
};

use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_support::sp_runtime::traits::Hash;
use frame_system::RawOrigin;

use pallet_timestamp::Config as TimestampConfig;

pub struct Pallet<T: Config>(HealthProfessionalQualification<T>);

pub trait Config:
	HealthProfessionalConfig + HealthProfessionalQualificationConfig + TimestampConfig
{
}

use health_professional_qualification::Call;
use sp_std::vec;

benchmarks! {
	create {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));

		let health_professional_info = HealthProfessionalInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			first_name: b"First Name".to_vec(),
			last_name: b"Last Name".to_vec(),
			myriad_username: b"debiouser".to_vec(),
			gender: b"Gender".to_vec(),
			date_of_birth: <T as pallet_timestamp::pallet::Config>::Moment::default(),
			email: b"Email".to_vec(),
			phone_number: b"+6893026516".to_vec(),
			role: b"doctor".to_vec(),
			category: b"Mental Health".to_vec(),
			profile_link: Some(b"DeBio Genetic Analyst profile_link".to_vec()),
			profile_image: Some(b"DeBio Genetic Analyst profile_image".to_vec()),
			anonymous: false,
		};

		let _ = HealthProfessional::<T>::register(caller_origin, health_professional_info);

		let experience = Experience { title: b"DeBio title".to_vec() };
		let certification = Certification {
			title: b"DeBio title".to_vec(),
			issuer: b"DeBio issuer".to_vec(),
			month: b"DeBio month".to_vec(),
			year: b"DeBio year".to_vec(),
			description: b"DeBio description".to_vec(),
			supporting_document: Some(b"DeBio Profile Image uwu".to_vec()),
		};
	}: create(RawOrigin::Signed(caller), vec![experience], vec![certification])

	update {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));

		let health_professional_info = HealthProfessionalInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			first_name: b"First Name".to_vec(),
			last_name: b"Last Name".to_vec(),
			myriad_username: b"debiouser".to_vec(),
			gender: b"Gender".to_vec(),
			date_of_birth: <T as pallet_timestamp::pallet::Config>::Moment::default(),
			email: b"Email".to_vec(),
			phone_number: b"+6893026516".to_vec(),
			role: b"doctor".to_vec(),
			category: b"Mental Health".to_vec(),
			profile_link: Some(b"DeBio Genetic Analyst profile_link".to_vec()),
			profile_image: Some(b"DeBio Genetic Analyst profile_image".to_vec()),
			anonymous: false,
		};

		let _ = HealthProfessional::<T>::register(caller_origin.clone(), health_professional_info);

		let experience = Experience { title: b"DeBio title".to_vec() };
		let certification = Certification {
			title: b"DeBio title".to_vec(),
			issuer: b"DeBio issuer".to_vec(),
			month: b"DeBio month".to_vec(),
			year: b"DeBio year".to_vec(),
			description: b"DeBio description".to_vec(),
			supporting_document: Some(b"DeBio Profile Image uwu".to_vec()),
		};
		let _ = HealthProfessionalQualification::<T>::create(caller_origin, vec![experience], vec![certification]);

		let result = HealthProfessional::<T>::health_professional_by_account_id(caller.clone());
		let health_professional = result.unwrap();
		let qualification_id = health_professional.qualifications()[0];

		let experience = Experience { title: b"Myriad title".to_vec() };
		let certification = Certification {
			title: b"Myriad title".to_vec(),
			issuer: b"DeBio issuer".to_vec(),
			month: b"DeBio month".to_vec(),
			year: b"DeBio year".to_vec(),
			description: b"DeBio description".to_vec(),
			supporting_document: Some(b"DeBio Profile Image uwu".to_vec()),
		};
	}: update(RawOrigin::Signed(caller), qualification_id, Some(vec![experience]), Some(vec![certification]))

	delete {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));

		let health_professional_info = HealthProfessionalInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			first_name: b"First Name".to_vec(),
			last_name: b"Last Name".to_vec(),
			myriad_username: b"debiouser".to_vec(),
			gender: b"Gender".to_vec(),
			date_of_birth: <T as pallet_timestamp::pallet::Config>::Moment::default(),
			email: b"Email".to_vec(),
			phone_number: b"+6893026516".to_vec(),
			role: b"doctor".to_vec(),
			category: b"Mental Health".to_vec(),
			profile_link: Some(b"DeBio Genetic Analyst profile_link".to_vec()),
			profile_image: Some(b"DeBio Genetic Analyst profile_image".to_vec()),
			anonymous: false,
		};

		let _ = HealthProfessional::<T>::register(caller_origin.clone(), health_professional_info);
		let experience = Experience { title: b"DeBio title".to_vec() };
		let certification = Certification {
			title: b"DeBio title".to_vec(),
			issuer: b"DeBio issuer".to_vec(),
			month: b"DeBio month".to_vec(),
			year: b"DeBio year".to_vec(),
			description: b"DeBio description".to_vec(),
			supporting_document: Some(b"DeBio Profile Image uwu".to_vec()),
		};
		let _ = HealthProfessionalQualification::<T>::create(caller_origin, vec![experience], vec![certification]);

		let result = HealthProfessional::<T>::health_professional_by_account_id(caller.clone());
		let health_professional = result.unwrap();
		let qualification_id = health_professional.qualifications()[0];
	}: delete(RawOrigin::Signed(caller), qualification_id)
}
