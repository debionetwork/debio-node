use super::*;

use crate::{
	GeneticAnalystInfo,
	StakeStatus,
};
#[allow(unused)]
use crate::Pallet as GeneticAnalysts;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

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
			stake_amount: 100,
			stake_status: StakeStatus::default(),
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
			stake_amount: 100,
			stake_status: StakeStatus::default(),
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
			stake_amount: 100,
			stake_status: StakeStatus::default(),
		};
	}: update_genetic_analyst(
		RawOrigin::Signed(caller),
		new_genetic_analyst
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
			stake_amount: 100,
			stake_status: StakeStatus::default(),
		};

		let _add_genetic_analysts = GeneticAnalysts::<T>::register_genetic_analyst(caller_origin.clone(), genetic_analyst);
	}: deregister_genetic_analyst(
		RawOrigin::Signed(caller)
	)
}

impl_benchmark_test_suite! {GeneticAnalysts, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
