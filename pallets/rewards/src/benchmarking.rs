use super::*;

use crate::RewarderKey;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_support::sp_runtime::traits::Saturating;
use frame_system::RawOrigin;

benchmarks! {
	reward_funds {
		let caller: T::AccountId = RewarderKey::<T>::get();
		let value = T::Currency::minimum_balance().saturating_mul(100u32.into());
		let _id = caller.clone();
	}: reward_funds(
		RawOrigin::Signed(caller.clone()),
		_id,
		value
	)

	update_admin_key {
		let caller: T::AccountId = LabVerifierKey::<T>::get();
		let caller2: T::AccountId = whitelisted_caller();
	}: update_admin_key(
		RawOrigin::Signed(caller),
		caller2
	)
}

impl_benchmark_test_suite! {Rewards, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
