use super::*;

#[allow(unused)]
use crate::Pallet as MenstrualSubscription;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	add_menstrual_subscription {
		let caller: T::AccountId = whitelisted_caller();
	}: add_menstrual_subscription(
		RawOrigin::Signed(caller),
		"DeBio Menstrual Data".as_bytes().to_vec(),
		"DeBio Menstrual Data Document Description".as_bytes().to_vec(),
		"DeBio Menstrual Data Link".as_bytes().to_vec()
	)

	update_menstrual_subscription {
		let caller: T::AccountId = whitelisted_caller();

		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let _add_menstrual_subscription = MenstrualSubscription::<T>::add_menstrual_subscription(
			caller_origin.clone(),
			"DeBio Menstrual Data".as_bytes().to_vec(),
			"DeBio Menstrual Data Document Description".as_bytes().to_vec(),
			"DeBio Menstrual Data Link".as_bytes().to_vec()
		);

		let _menstrual_subscription_ids = MenstrualSubscription::<T>::menstrual_subscription_by_owner_id(
			caller.clone()
		).unwrap();
	}: update_menstrual_subscription(
		RawOrigin::Signed(caller),
		_menstrual_subscription_ids[0],
		"DeBio Menstrual Data 2".as_bytes().to_vec(),
		"DeBio Menstrual Data Document Description 2".as_bytes().to_vec(),
		"DeBio Menstrual Data Link 2".as_bytes().to_vec()
	)

	remove_menstrual_subscription {
		let caller: T::AccountId = whitelisted_caller();

		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let _add_menstrual_subscription = MenstrualSubscription::<T>::add_menstrual_subscription(
			caller_origin.clone(),
			"DeBio Menstrual Data".as_bytes().to_vec(),
			"DeBio Menstrual Data Document Description".as_bytes().to_vec(),
			"DeBio Menstrual Data Link".as_bytes().to_vec()
		);

		let _menstrual_subscription_ids = MenstrualSubscription::<T>::menstrual_subscription_by_owner_id(
			caller.clone()
		).unwrap();
	}: remove_menstrual_subscription(
		RawOrigin::Signed(caller),
		_menstrual_subscription_ids[0]
	)
}

impl_benchmark_test_suite! {MenstrualSubscription, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
