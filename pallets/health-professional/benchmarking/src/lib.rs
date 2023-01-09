#![cfg_attr(not(feature = "std"), no_std)]
mod mock;

#[allow(unused)]
use health_professional::{
	Config as HealthProfessionalConfig, HealthProfessionalInfo, HealthProfessionalVerifierKey,
	MinimumStakeAmount, Pallet as HealthProfessional, UnstakeTime,
};

#[allow(unused)]
use health_professional_qualification::{
	Certification, Config as HealthProfessionalQualificationConfig, Experience,
	Pallet as HealthProfessionalQualification,
};

use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_support::{
	sp_runtime::{traits::Hash, SaturatedConversion},
	traits::Currency,
};
use frame_system::RawOrigin;

use pallet_timestamp::{Config as TimestampConfig, Now};

pub struct Pallet<T: Config>(HealthProfessional<T>);

pub trait Config:
	HealthProfessionalConfig + HealthProfessionalQualificationConfig + TimestampConfig
{
}

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type CurrencyOf<T> = <T as health_professional::Config>::Currency;
pub type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;

use health_professional::Call;
use primitives_availability_status::AvailabilityStatus;
use primitives_verification_status::VerificationStatus;

benchmarks! {
	register {
		let caller: T::AccountId = whitelisted_caller();
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
	}: register(RawOrigin::Signed(caller), health_professional_info)

	update_info {
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

		let updated_info = HealthProfessionalInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			first_name: b"First Name".to_vec(),
			last_name: b"Last Name".to_vec(),
			myriad_username: b"myriaduser".to_vec(),
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
	}: update_info(RawOrigin::Signed(caller), updated_info)

	update_availability_status {
		let caller: T::AccountId = whitelisted_caller();
		let doctor_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));

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

		let _ = HealthProfessional::<T>::register(doctor_origin, health_professional_info);
	}: update_availability_status(RawOrigin::Signed(caller), AvailabilityStatus::Unavailable)

	update_verification_status {
		let caller: T::AccountId = HealthProfessionalVerifierKey::<T>::get().unwrap();
		let doctor: T::AccountId = whitelisted_caller();
		let doctor_origin = T::Origin::from(RawOrigin::Signed(doctor.clone()));

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

		let _ = HealthProfessional::<T>::register(doctor_origin, health_professional_info);
	}: update_verification_status(RawOrigin::Signed(caller), doctor, VerificationStatus::Unverified)

	deregister {
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
	}: deregister(RawOrigin::Signed(caller))

	stake {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));

		let init_balance = 2_000_000_000_000_000_000u128.saturated_into();
		let min_stake_amount = 1_000_000_000_000_000_000u128.saturated_into::<BalanceOf<T>>
		();

		let _ = <T as health_professional::Config>::Currency::deposit_creating(&caller, init_balance);

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

		MinimumStakeAmount::<T>::put(min_stake_amount);
	}: stake(RawOrigin::Signed(caller))

	unstake {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));

		let init_balance = 2_000_000_000_000_000_000u128.saturated_into();
		let min_stake_amount = 1_000_000_000_000_000_000u128.saturated_into::<BalanceOf<T>>();

		let _ = <T as health_professional::Config>::Currency::deposit_creating(&caller, init_balance);

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

		MinimumStakeAmount::<T>::put(min_stake_amount);

		let _ = HealthProfessional::<T>::stake(caller_origin);
	}: unstake(RawOrigin::Signed(caller))

	retrieve_unstaked_amount {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));

		let init_balance = 2_000_000_000_000_000_000u128.saturated_into();
		let min_stake_amount = 1_000_000_000_000_000_000u128.saturated_into::<BalanceOf<T>>();

		let _ = <T as health_professional::Config>::Currency::deposit_creating(&caller, init_balance);

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

		MinimumStakeAmount::<T>::put(min_stake_amount);
		UnstakeTime::<T>::put(10_000_000_000);

		let _ = HealthProfessional::<T>::stake(caller_origin.clone());
		let _ = HealthProfessional::<T>::unstake(caller_origin);

		let now = 10_000_000_000u128.saturated_into::<<T as pallet_timestamp::Config>::Moment>();

		Now::<T>::put(now);
	}: retrieve_unstaked_amount(RawOrigin::Signed(caller))

	update_stake_amount {
		let caller: T::AccountId = HealthProfessionalVerifierKey::<T>::get().unwrap();
		let amount = 100_000_000_000_000u128.saturated_into();
	}: update_stake_amount(RawOrigin::Signed(caller), amount)

	update_unstake_time {
		let caller: T::AccountId = HealthProfessionalVerifierKey::<T>::get().unwrap();
		let moment = 100_000_000_000_000u128;
	}: update_unstake_time(RawOrigin::Signed(caller), moment)

	update_verifier_key {
		let caller: T::AccountId = HealthProfessionalVerifierKey::<T>::get().unwrap();
		let other_admin: T::AccountId = whitelisted_caller();
	}: update_verifier_key(RawOrigin::Signed(caller), other_admin)
}
