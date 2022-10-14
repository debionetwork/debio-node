use super::*;

#[allow(unused)]
use crate::{Pallet as MenstrualCalendar, Symptom};
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::sp_runtime::SaturatedConversion;
use frame_system::RawOrigin;

benchmarks! {
	add_menstrual_calendar {
		let caller: T::AccountId = whitelisted_caller();
	}: add_menstrual_calendar(RawOrigin::Signed(caller), 16)

	update_menstrual_calendar {
		let caller: T::AccountId = whitelisted_caller();

		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let _ = MenstrualCalendar::<T>::add_menstrual_calendar(
			caller_origin.clone(),
			16
		);

		let menstrual_ids = MenstrualCalendar::<T>::menstrual_calendar_by_owner(
			caller.clone()
		).unwrap();
	}: update_menstrual_calendar(RawOrigin::Signed(caller), menstrual_ids[0], 20)

	add_menstrual_cycle_log {
		let caller: T::AccountId = whitelisted_caller();

		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let _ = MenstrualCalendar::<T>::add_menstrual_calendar(
			caller_origin.clone(),
			16u8.saturated_into()
		);

		let menstrual_ids = MenstrualCalendar::<T>::menstrual_calendar_by_owner(
			caller.clone()
		).unwrap();
	}: add_menstrual_cycle_log(
		RawOrigin::Signed(caller),
		menstrual_ids[0],
		0u128.saturated_into(),
		vec![Symptom::from(b"pain")],
		true
	)

	update_menstrual_cycle_log {
		let caller: T::AccountId = whitelisted_caller();

		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let _ = MenstrualCalendar::<T>::add_menstrual_calendar(
			caller_origin.clone(),
			16
		);

		let menstrual_ids = MenstrualCalendar::<T>::menstrual_calendar_by_owner(
			caller.clone()
		).unwrap();

		let _ = MenstrualCalendar::<T>::add_menstrual_cycle_log(
			caller_origin.clone(),
			menstrual_ids[0],
			0u128.saturated_into(),
			vec![Symptom::from(b"pain")],
			true,
		);

		let cycle_log_ids = MenstrualCalendar::<T>::menstrual_cycle_log_by_owner_id(menstrual_ids[0]).unwrap();
	}: update_menstrual_cycle_log(
		RawOrigin::Signed(caller),
		menstrual_ids[0],
		cycle_log_ids[0],
		1u128.saturated_into(),
		vec![Symptom::from(b"headache")],
		false
	)

	remove_menstrual_cycle_log {
		let caller: T::AccountId = whitelisted_caller();

		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let _ = MenstrualCalendar::<T>::add_menstrual_calendar(
			caller_origin.clone(),
			16
		);

		let menstrual_ids = MenstrualCalendar::<T>::menstrual_calendar_by_owner(
			caller.clone()
		).unwrap();

		let _ = MenstrualCalendar::<T>::add_menstrual_cycle_log(
			caller_origin.clone(),
			menstrual_ids[0],
			0u128.saturated_into(),
			vec![Symptom::from(b"pain")],
			true,
		);

		let cycle_log_ids = MenstrualCalendar::<T>::menstrual_cycle_log_by_owner_id(menstrual_ids[0]).unwrap();
	}: remove_menstrual_cycle_log(
		RawOrigin::Signed(caller),
		menstrual_ids[0],
		cycle_log_ids[0]
	)
}

impl_benchmark_test_suite! {MenstrualCalendar, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
