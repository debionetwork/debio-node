use super::*;

#[allow(unused)]
use crate::Pallet as Rewards;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use frame_support::sp_runtime::traits::Saturating;

benchmarks! {
	reward_funds {
		let caller: T::AccountId = whitelisted_caller();
		let value = T::Currency::minimum_balance().saturating_mul(100u32.into());
        let _id = caller.clone();
	}: reward_funds(
        RawOrigin::Signed(caller.clone()),
        _id,
        value
    )

	slash_funds {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));

		let value = T::Currency::minimum_balance().saturating_mul(100u32.into());
        let _reward_funds = Rewards::<T>::reward_funds(
            caller_origin.clone(), 
            caller.clone(), 
            value
        );

        let _id = caller.clone();
	}: slash_funds(
        RawOrigin::Signed(caller.clone()),
        _id,
        value
    )
}

impl_benchmark_test_suite! {Labs, crate::mock::ExternalityBuilder::build(), crate::mock::Test}