use super::*;

#[allow(unused)]
use crate::{
	Pallet as Rewards,
	RewarderKey
};
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use frame_support::sp_runtime::traits::Saturating;

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
}

impl_benchmark_test_suite! {Labs, crate::mock::ExternalityBuilder::build(), crate::mock::Test}