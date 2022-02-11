use super::*;

#[allow(unused)]
use crate::Pallet as GeneticAnalysts;
use crate::{GeneticAnalystInfo, GeneticAnalystVerifierKey};
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use primitives_verification_status::VerificationStatus;

benchmarks! {
	register_genetic_analyst {
		let genetic_analyst = GeneticAnalystInfo {
			first_name: "First Name".as_bytes().to_vec(),
			last_name: "Last Name".as_bytes().to_vec(),
			gender: "Gender".as_bytes().to_vec(),
			date_of_birth: <T as pallet_timestamp::pallet::Config>::Moment::default(),
			email: "Email".as_bytes().to_vec(),
			phone_number: "+6893026516".as_bytes().to_vec(),
			specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
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
			first_name: "First Name".as_bytes().to_vec(),
			last_name: "Last Name".as_bytes().to_vec(),
			gender: "Gender".as_bytes().to_vec(),
			date_of_birth: <T as pallet_timestamp::pallet::Config>::Moment::default(),
			email: "Email".as_bytes().to_vec(),
			phone_number: "+6893026516".as_bytes().to_vec(),
			specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
		};
		let _add_genetic_analysts = GeneticAnalysts::<T>::register_genetic_analyst(caller_origin.clone(), old_genetic_analyst);

		let new_genetic_analyst = GeneticAnalystInfo {
			first_name: "First Name 2".as_bytes().to_vec(),
			last_name: "Last Name 2".as_bytes().to_vec(),
			gender: "Gender 2".as_bytes().to_vec(),
			date_of_birth: <T as pallet_timestamp::pallet::Config>::Moment::default(),
			email: "Email 2".as_bytes().to_vec(),
			phone_number: "+6893026516".as_bytes().to_vec(),
			specialization: "DeBio Genetic Analyst 2".as_bytes().to_vec(),
		};
	}: update_genetic_analyst(
		RawOrigin::Signed(caller),
		new_genetic_analyst
	)

	update_genetic_analyst_verification_status {
		let caller: T::AccountId = GeneticAnalystVerifierKey::<T>::get();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let old_genetic_analyst = GeneticAnalystInfo {
			first_name: "First Name".as_bytes().to_vec(),
			last_name: "Last Name".as_bytes().to_vec(),
			gender: "Gender".as_bytes().to_vec(),
			date_of_birth: <T as pallet_timestamp::pallet::Config>::Moment::default(),
			email: "Email".as_bytes().to_vec(),
			phone_number: "+6893026516".as_bytes().to_vec(),
			specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
		};
		let _ = GeneticAnalysts::<T>::register_genetic_analyst(caller_origin.clone(), old_genetic_analyst);
	}: update_genetic_analyst_verification_status(
		RawOrigin::Signed(caller),
		caller.clone(),
		VerificationStatus::default()
	)

	deregister_genetic_analyst {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let genetic_analyst = GeneticAnalystInfo {
			first_name: "First Name".as_bytes().to_vec(),
			last_name: "Last Name".as_bytes().to_vec(),
			gender: "Gender".as_bytes().to_vec(),
			date_of_birth: <T as pallet_timestamp::pallet::Config>::Moment::default(),
			email: "Email".as_bytes().to_vec(),
			phone_number: "+6893026516".as_bytes().to_vec(),
			specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
		};

		let _add_genetic_analysts = GeneticAnalysts::<T>::register_genetic_analyst(caller_origin.clone(), genetic_analyst);
	}: deregister_genetic_analyst(
		RawOrigin::Signed(caller)
	)

	stake_genetic_analyst {
		let caller: T::AccountId = GeneticAnalystVerifierKey::<T>::get();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let _ = <T as pallet::Config>::Currency::deposit_creating(&caller, 60000000000000000000000u128.saturated_into());

		let old_genetic_analyst = GeneticAnalystInfo {
			first_name: "First Name".as_bytes().to_vec(),
			last_name: "Last Name".as_bytes().to_vec(),
			gender: "Gender".as_bytes().to_vec(),
			date_of_birth: <T as pallet_timestamp::pallet::Config>::Moment::default(),
			email: "Email".as_bytes().to_vec(),
			phone_number: "+6893026516".as_bytes().to_vec(),
			specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
		};
		let _ = GeneticAnalysts::<T>::register_genetic_analyst(caller_origin.clone(), old_genetic_analyst);
	}: stake_genetic_analyst(
		RawOrigin::Signed(caller)
	)

	update_minimum_stake_amount {
		let caller: T::AccountId = GeneticAnalystVerifierKey::<T>::get();
	}: update_minimum_stake_amount(
		RawOrigin::Signed(caller),
		60000000000000000000000u128.saturated_into()
	)

	update_admin_key {
		let caller: T::AccountId = GeneticAnalystVerifierKey::<T>::get();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));
	}: update_admin_key(
		RawOrigin::Signed(caller),
		caller_origin.clone()
	)
}

impl_benchmark_test_suite! {GeneticAnalysts, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
