use super::*;

use crate::{Config, PalletAccount, RewarderKey};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::sp_runtime::SaturatedConversion;
use frame_system::RawOrigin;

const SEED: u32 = 0;

benchmarks! {
	reward_funds {
		let caller: T::AccountId = RewarderKey::<T>::get().unwrap();
		let pallet_id: T::AccountId = PalletAccount::<T>::get().unwrap();
		let receiver: T::AccountId = account("receiver", 0, SEED);

		let init_balance = 1000000000000000000000u128.saturated_into();
		let reward = 1000000000000000000u128.saturated_into();

		let _ = <T as Config>::Currency::deposit_creating(
			&pallet_id,
			init_balance
		);
	}: reward_funds(RawOrigin::Signed(caller), receiver, reward)

	update_admin_key {
		let caller: T::AccountId = RewarderKey::<T>::get().unwrap();
		let caller2: T::AccountId = whitelisted_caller();
	}: update_admin_key(RawOrigin::Signed(caller), caller2)
}

impl_benchmark_test_suite! {Rewards, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
