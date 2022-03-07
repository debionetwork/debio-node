use super::*;

use crate::CustomerInfo;
#[allow(unused)]
use crate::Pallet as Customers;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	register_customer {
		let customer = CustomerInfo {
			name: "DeBio Customer".as_bytes().to_vec(),
			email: "DeBio Email".as_bytes().to_vec(),
			address: "DeBio Address".as_bytes().to_vec(),
			latitude: Some("DeBio Latitude".as_bytes().to_vec()),
			longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
			profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};
		let caller: T::AccountId = whitelisted_caller();
	}: register_customer(
		RawOrigin::Signed(caller),
		customer
	)

	update_customer {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let old_customer = CustomerInfo {
			name: "DeBio Customer".as_bytes().to_vec(),
			email: "DeBio Email".as_bytes().to_vec(),
			address: "DeBio Address".as_bytes().to_vec(),
			latitude: Some("DeBio Latitude".as_bytes().to_vec()),
			longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
			profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};
		let _add_customers = Customers::<T>::register_customer(caller_origin.clone(), old_customer);

		let new_customer = CustomerInfo {
			name: "DeBio Customer 2".as_bytes().to_vec(),
			email: "DeBio Email 2".as_bytes().to_vec(),
			address: "DeBio Address 2".as_bytes().to_vec(),
			latitude: Some("DeBio Latitude 2".as_bytes().to_vec()),
			longitude: Some("DeBio Longtitude 2".as_bytes().to_vec()),
			profile_image: Some("DeBio Profile Image owo".as_bytes().to_vec()),
		};
	}: update_customer(
		RawOrigin::Signed(caller),
		new_customer
	)

	deregister_customer {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let customer = CustomerInfo {
			name: "DeBio Customer".as_bytes().to_vec(),
			email: "DeBio Email".as_bytes().to_vec(),
			address: "DeBio Address".as_bytes().to_vec(),
			latitude: Some("DeBio Latitude".as_bytes().to_vec()),
			longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
			profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};

		let _add_customers = Customers::<T>::register_customer(caller_origin.clone(), customer);
	}: deregister_customer(
		RawOrigin::Signed(caller)
	)
}

impl_benchmark_test_suite! {Customers, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
