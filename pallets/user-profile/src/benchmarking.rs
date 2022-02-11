use super::*;

#[allow(unused)]
use crate::Pallet as UserProfile;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	set_eth_address {
		let eth_address = T::EthereumAddress::default();
		let caller: T::AccountId = whitelisted_caller();
	}: set_eth_address(
		RawOrigin::Signed(caller),
		eth_address
	)

	update_admin_key {
		let caller: T::AccountId = AdminKey::<T>::get();
		let caller2: T::AccountId = whitelisted_caller();
	}: update_admin_key(
		RawOrigin::Signed(caller),
		caller2
	)
}

impl_benchmark_test_suite! {
	UserProfile,
	crate::mock::ExternalityBuilder::build(),
	crate::mock::Test
}
