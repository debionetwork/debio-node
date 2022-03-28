#![cfg_attr(not(feature = "std"), no_std)]
mod mock;

#[allow(unused)]
use genetic_analysts::Pallet as GeneticAnalysts;
use genetic_analysts::{
	Call, Config as GeneticAnalystsConfig, GeneticAnalystInfo, GeneticAnalystVerifierKey,
};

pub struct Pallet<T: Config>(GeneticAnalysts<T>);

pub trait Config: GeneticAnalystsConfig {}

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{
	sp_runtime::{traits::Hash, SaturatedConversion},
	traits::Currency,
};
use frame_system::RawOrigin;

use primitives_availability_status::AvailabilityStatus;
use primitives_verification_status::VerificationStatus;

benchmarks! {
	register_genetic_analyst {
		let genetic_analyst = GeneticAnalystInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			first_name: "First Name".as_bytes().to_vec(),
			last_name: "Last Name".as_bytes().to_vec(),
			gender: "Gender".as_bytes().to_vec(),
			date_of_birth: <T as pallet_timestamp::pallet::Config>::Moment::default(),
			email: "Email".as_bytes().to_vec(),
			phone_number: "+6893026516".as_bytes().to_vec(),
			specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
			profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
			profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
		};
		let caller: T::AccountId = whitelisted_caller();
	}: register_genetic_analyst(
		RawOrigin::Signed(caller),
		genetic_analyst
	)

	update_genetic_analyst {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let old_genetic_analyst = GeneticAnalystInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			first_name: "First Name".as_bytes().to_vec(),
			last_name: "Last Name".as_bytes().to_vec(),
			gender: "Gender".as_bytes().to_vec(),
			date_of_birth: <T as pallet_timestamp::pallet::Config>::Moment::default(),
			email: "Email".as_bytes().to_vec(),
			phone_number: "+6893026516".as_bytes().to_vec(),
			specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
			profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
			profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
		};
		let _add_genetic_analysts = GeneticAnalysts::<T>::register_genetic_analyst(caller_origin, old_genetic_analyst);

		let new_genetic_analyst = GeneticAnalystInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			first_name: "First Name 2".as_bytes().to_vec(),
			last_name: "Last Name 2".as_bytes().to_vec(),
			gender: "Gender 2".as_bytes().to_vec(),
			date_of_birth: <T as pallet_timestamp::pallet::Config>::Moment::default(),
			email: "Email 2".as_bytes().to_vec(),
			phone_number: "+6893026516".as_bytes().to_vec(),
			specialization: "DeBio Genetic Analyst 2".as_bytes().to_vec(),
			profile_link: "DeBio Genetic Analyst profile_link 2".as_bytes().to_vec(),
			profile_image: Some("DeBio Genetic Analyst profile_image 2".as_bytes().to_vec()),
		};
	}: update_genetic_analyst(
		RawOrigin::Signed(caller),
		new_genetic_analyst
	)

	update_genetic_analyst_verification_status {
		let caller: T::AccountId = GeneticAnalystVerifierKey::<T>::get();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let old_genetic_analyst = GeneticAnalystInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			first_name: "First Name".as_bytes().to_vec(),
			last_name: "Last Name".as_bytes().to_vec(),
			gender: "Gender".as_bytes().to_vec(),
			date_of_birth: <T as pallet_timestamp::pallet::Config>::Moment::default(),
			email: "Email".as_bytes().to_vec(),
			phone_number: "+6893026516".as_bytes().to_vec(),
			specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
			profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
			profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
		};
		let _ = GeneticAnalysts::<T>::register_genetic_analyst(caller_origin.clone(), old_genetic_analyst);

		let _ = <T as genetic_analysts::Config>::Currency::deposit_creating(&caller, 60000000000000000000000u128.saturated_into());

		let _ = GeneticAnalysts::<T>::stake_genetic_analyst(caller_origin);
	}: update_genetic_analyst_verification_status(
		RawOrigin::Signed(caller),
		caller.clone(),
		VerificationStatus::default()
	)

	update_genetic_analyst_availability_status {
		let caller: T::AccountId = GeneticAnalystVerifierKey::<T>::get();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let old_genetic_analyst = GeneticAnalystInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			first_name: "First Name".as_bytes().to_vec(),
			last_name: "Last Name".as_bytes().to_vec(),
			gender: "Gender".as_bytes().to_vec(),
			date_of_birth: <T as pallet_timestamp::pallet::Config>::Moment::default(),
			email: "Email".as_bytes().to_vec(),
			phone_number: "+6893026516".as_bytes().to_vec(),
			specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
			profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
			profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
		};
		let _ = GeneticAnalysts::<T>::register_genetic_analyst(caller_origin, old_genetic_analyst);
	}: update_genetic_analyst_availability_status(
		RawOrigin::Signed(caller),
		AvailabilityStatus::Available
	)

	deregister_genetic_analyst {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let genetic_analyst = GeneticAnalystInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			first_name: "First Name".as_bytes().to_vec(),
			last_name: "Last Name".as_bytes().to_vec(),
			gender: "Gender".as_bytes().to_vec(),
			date_of_birth: <T as pallet_timestamp::pallet::Config>::Moment::default(),
			email: "Email".as_bytes().to_vec(),
			phone_number: "+6893026516".as_bytes().to_vec(),
			specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
			profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
			profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
		};

		let _add_genetic_analysts = GeneticAnalysts::<T>::register_genetic_analyst(caller_origin, genetic_analyst);
	}: deregister_genetic_analyst(
		RawOrigin::Signed(caller)
	)

	stake_genetic_analyst {
		let caller: T::AccountId = GeneticAnalystVerifierKey::<T>::get();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let _ = <T as genetic_analysts::Config>::Currency::deposit_creating(&caller, 60000000000000000000000u128.saturated_into());

		let old_genetic_analyst = GeneticAnalystInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			first_name: "First Name".as_bytes().to_vec(),
			last_name: "Last Name".as_bytes().to_vec(),
			gender: "Gender".as_bytes().to_vec(),
			date_of_birth: <T as pallet_timestamp::pallet::Config>::Moment::default(),
			email: "Email".as_bytes().to_vec(),
			phone_number: "+6893026516".as_bytes().to_vec(),
			specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
			profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
			profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
		};
		let _ = GeneticAnalysts::<T>::register_genetic_analyst(caller_origin, old_genetic_analyst);
	}: stake_genetic_analyst(
		RawOrigin::Signed(caller)
	)

	unstake_genetic_analyst {
		let caller: T::AccountId = GeneticAnalystVerifierKey::<T>::get();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let _ = <T as genetic_analysts::Config>::Currency::deposit_creating(&caller, 60000000000000000000000u128.saturated_into());

		let old_genetic_analyst = GeneticAnalystInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			first_name: "First Name".as_bytes().to_vec(),
			last_name: "Last Name".as_bytes().to_vec(),
			gender: "Gender".as_bytes().to_vec(),
			date_of_birth: <T as pallet_timestamp::pallet::Config>::Moment::default(),
			email: "Email".as_bytes().to_vec(),
			phone_number: "+6893026516".as_bytes().to_vec(),
			specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
			profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
			profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
		};
		let _ = GeneticAnalysts::<T>::register_genetic_analyst(caller_origin.clone(), old_genetic_analyst);

		let _ = GeneticAnalysts::<T>::stake_genetic_analyst(caller_origin);
	}: unstake_genetic_analyst(
		RawOrigin::Signed(caller)
	)

	retrieve_unstake_amount {
		let caller_admin: T::AccountId = GeneticAnalystVerifierKey::<T>::get();
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let _ = <T as genetic_analysts::Config>::Currency::deposit_creating(&caller, 60000000000000000000000u128.saturated_into());

		let old_genetic_analyst = GeneticAnalystInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			first_name: "First Name".as_bytes().to_vec(),
			last_name: "Last Name".as_bytes().to_vec(),
			gender: "Gender".as_bytes().to_vec(),
			date_of_birth: <T as pallet_timestamp::pallet::Config>::Moment::default(),
			email: "Email".as_bytes().to_vec(),
			phone_number: "+6893026516".as_bytes().to_vec(),
			specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
			profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
			profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
		};
		let _ = GeneticAnalysts::<T>::register_genetic_analyst(caller_origin.clone(), old_genetic_analyst);

		let _ = GeneticAnalysts::<T>::stake_genetic_analyst(caller_origin.clone());

		let _ = GeneticAnalysts::<T>::unstake_genetic_analyst(caller_origin);
	}: retrieve_unstake_amount(
		RawOrigin::Signed(caller_admin),
		caller
	)

	update_minimum_stake_amount {
		let caller: T::AccountId = GeneticAnalystVerifierKey::<T>::get();
	}: update_minimum_stake_amount(
		RawOrigin::Signed(caller),
		60000000000000000000000u128.saturated_into()
	)

	update_unstake_time {
		let caller: T::AccountId = GeneticAnalystVerifierKey::<T>::get();
	}: update_unstake_time(
		RawOrigin::Signed(caller),
		0u64.saturated_into()
	)

	update_admin_key {
		let caller: T::AccountId = GeneticAnalystVerifierKey::<T>::get();
		let caller2: T::AccountId = whitelisted_caller();
	}: update_admin_key(
		RawOrigin::Signed(caller),
		caller2
	)
}

impl_benchmark_test_suite! {GeneticAnalysts, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
