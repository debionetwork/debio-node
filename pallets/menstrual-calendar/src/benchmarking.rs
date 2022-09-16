use super::*;

#[allow(unused)]
use crate::Pallet as MenstrualCalendar;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	add_menstrual_calendar {
		let caller: T::AccountId = whitelisted_caller();
	}: add_menstrual_calendar(
		RawOrigin::Signed(caller),
		"DeBio Menstrual Data".as_bytes().to_vec(),
		"DeBio Menstrual Data Document Description".as_bytes().to_vec(),
		"DeBio Menstrual Data Link".as_bytes().to_vec()
	)

	update_menstrual_calendar {
		let caller: T::AccountId = whitelisted_caller();

		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let _add_menstrual_calendar = MenstrualCalendar::<T>::add_menstrual_calendar(
			caller_origin.clone(),
			"DeBio Menstrual Data".as_bytes().to_vec(),
			"DeBio Menstrual Data Document Description".as_bytes().to_vec(),
			"DeBio Menstrual Data Link".as_bytes().to_vec()
		);

		let _menstrual_calendar_ids = MenstrualCalendar::<T>::menstrual_calendar_by_address_id(
			caller.clone()
		).unwrap();
	}: update_menstrual_calendar(
		RawOrigin::Signed(caller),
		_menstrual_calendar_ids[0],
		"DeBio Menstrual Data 2".as_bytes().to_vec(),
		"DeBio Menstrual Data Document Description 2".as_bytes().to_vec(),
		"DeBio Menstrual Data Link 2".as_bytes().to_vec()
	)

	remove_menstrual_calendar {
		let caller: T::AccountId = whitelisted_caller();

		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let _add_menstrual_calendar = MenstrualCalendar::<T>::add_menstrual_calendar(
			caller_origin.clone(),
			"DeBio Menstrual Data".as_bytes().to_vec(),
			"DeBio Menstrual Data Document Description".as_bytes().to_vec(),
			"DeBio Menstrual Data Link".as_bytes().to_vec()
		);

		let _menstrual_calendar_ids = MenstrualCalendar::<T>::menstrual_calendar_by_address_id(
			caller.clone()
		).unwrap();
	}: remove_menstrual_calendar(
		RawOrigin::Signed(caller),
		_menstrual_calendar_ids[0]
	)
}

impl_benchmark_test_suite! {MenstrualCalendar, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
