use super::*;

#[allow(unused)]
use crate::Pallet as Labs;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, vec, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::vec::Vec;

use frame_support::sp_runtime::traits::Hash;
use frame_support::sp_std::convert::TryInto;

benchmarks! {    
	set_eth_address {
        let eth_address = T::EthereumAddress::default();
		let caller: T::AccountId = whitelisted_caller();
	}: set_eth_address(
        RawOrigin::Signed(caller),
        eth_address
    )
}

impl_benchmark_test_suite! {Labs, crate::mock::ExternalityBuilder::build(), crate::mock::Test}