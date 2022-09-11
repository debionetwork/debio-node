use super::*;

#[allow(unused)]
use crate::Pallet as MenstrualData;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	add_menstrual_data {
		let caller: T::AccountId = whitelisted_caller();
	}: add_menstrual_data(
		RawOrigin::Signed(caller),
		"DeBio Menstrual Data".as_bytes().to_vec(),
		"DeBio Menstrual Data Document Description".as_bytes().to_vec(),
		"DeBio Menstrual Data Link".as_bytes().to_vec()
	)

	update_menstrual_data {
		let caller: T::AccountId = whitelisted_caller();

		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let _add_menstrual_data = MenstrualData::<T>::add_menstrual_data(
			caller_origin.clone(),
			"DeBio Menstrual Data".as_bytes().to_vec(),
			"DeBio Menstrual Data Document Description".as_bytes().to_vec(),
			"DeBio Menstrual Data Link".as_bytes().to_vec()
		);

		let _menstrual_data_ids = MenstrualData::<T>::menstrual_data_by_owner_id(
			caller.clone()
		).unwrap();
	}: update_menstrual_data(
		RawOrigin::Signed(caller),
		_menstrual_data_ids[0],
		"DeBio Menstrual Data 2".as_bytes().to_vec(),
		"DeBio Menstrual Data Document Description 2".as_bytes().to_vec(),
		"DeBio Menstrual Data Link 2".as_bytes().to_vec()
	)

	remove_menstrual_data {
		let caller: T::AccountId = whitelisted_caller();

		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let _add_menstrual_data = MenstrualData::<T>::add_menstrual_data(
			caller_origin.clone(),
			"DeBio Menstrual Data".as_bytes().to_vec(),
			"DeBio Menstrual Data Document Description".as_bytes().to_vec(),
			"DeBio Menstrual Data Link".as_bytes().to_vec()
		);

		let _menstrual_data_ids = MenstrualData::<T>::menstrual_data_by_owner_id(
			caller.clone()
		).unwrap();
	}: remove_menstrual_data(
		RawOrigin::Signed(caller),
		_menstrual_data_ids[0]
	)
}

impl_benchmark_test_suite! {MenstrualData, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
