use super::*;

#[allow(unused)]
use crate::Pallet as MenstrualSubscription;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use primitives_menstrual_status::{MenstrualSubscriptionStatus, PaymentStatus};

benchmarks! {
	add_menstrual_subscription {
		let caller: T::AccountId = whitelisted_caller();
	}: add_menstrual_subscription(
		RawOrigin::Signed(caller),
		MenstrualSubscriptionDuration::default(),
		1,
		PaymentStatus::default(),
		MenstrualSubscriptionStatus::default()
	)

	change_menstrual_subscription_status {
		let caller: T::AccountId = whitelisted_caller();

		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let _add_menstrual_subscription = MenstrualSubscription::<T>::add_menstrual_subscription(
			caller_origin.clone(),
			MenstrualSubscriptionDuration::default(),
			1,
			PaymentStatus::default(),
			MenstrualSubscriptionStatus::default()
		);

		let _menstrual_subscription_ids = MenstrualSubscription::<T>::menstrual_subscription_by_address_id(
			caller.clone()
		).unwrap();
	}: change_menstrual_subscription_status(
		RawOrigin::Signed(caller),
		_menstrual_subscription_ids[0],
		MenstrualSubscriptionStatus::default()
	)

	set_menstrual_subscription_paid {
		let caller: T::AccountId = whitelisted_caller();

		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let _add_menstrual_subscription = MenstrualSubscription::<T>::add_menstrual_subscription(
			caller_origin.clone(),
			MenstrualSubscriptionDuration::default(),
			1,
			PaymentStatus::default(),
			MenstrualSubscriptionStatus::default(),
		);

		let _menstrual_subscription_ids = MenstrualSubscription::<T>::menstrual_subscription_by_address_id(
			caller.clone()
		).unwrap();
	}: set_menstrual_subscription_paid(
		RawOrigin::Signed(caller),
		_menstrual_subscription_ids[0]
	)
}

impl_benchmark_test_suite! {MenstrualSubscription, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
