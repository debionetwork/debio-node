use super::*;

use crate::{
	Pallet as Rewards,
	RewarderKey
};
use pallet_sudo::Pallet as Sudo;
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

	add_total_reward_balance {
		let caller: T::AccountId = Sudo::<T>::key();
		let value = T::Currency::minimum_balance().saturating_mul(100u32.into());
	}: add_total_reward_balance(
        RawOrigin::Signed(caller.clone()),
        value
    )
}

impl_benchmark_test_suite! {Rewards, crate::mock::ExternalityBuilder::build(), crate::mock::Test}