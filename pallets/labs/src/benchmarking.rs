use super::*;

#[allow(unused)]
use crate::Pallet as Labs;
use crate::LabInfo;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, vec, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::vec::Vec;

use frame_support::sp_runtime::traits::Hash;
use frame_support::sp_std::convert::TryInto;

benchmarks! {
	register_lab {
		let lab = LabInfo {
            box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
            name: "DeBio Lab".as_bytes().to_vec(),
            email: "DeBio Email".as_bytes().to_vec(),
            country: "DeBio Country".as_bytes().to_vec(),
            region: "DeBio Region".as_bytes().to_vec(),
            city: "DeBio City".as_bytes().to_vec(),
            address: "DeBio Address".as_bytes().to_vec(),
            latitude: Some("DeBio Latitude".as_bytes().to_vec()),
            longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
            profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};
		let caller: T::AccountId = whitelisted_caller();
	}: register_lab(
        RawOrigin::Signed(caller),
        lab
    )
    
	update_lab {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let old_lab = LabInfo {
            box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
            name: "DeBio Lab".as_bytes().to_vec(),
            email: "DeBio Email".as_bytes().to_vec(),
            country: "DeBio Country".as_bytes().to_vec(),
            region: "DeBio Region".as_bytes().to_vec(),
            city: "DeBio City".as_bytes().to_vec(),
            address: "DeBio Address".as_bytes().to_vec(),
            latitude: Some("DeBio Latitude".as_bytes().to_vec()),
            longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
            profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};
		let _add_labs = Labs::<T>::register_lab(caller_origin.clone(), old_lab);

        let new_lab = LabInfo {
            box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
            name: "DeBio Lab 2".as_bytes().to_vec(),
            email: "DeBio Email 2".as_bytes().to_vec(),
            country: "DeBio Country 2".as_bytes().to_vec(),
            region: "DeBio Region 2".as_bytes().to_vec(),
            city: "DeBio City 2".as_bytes().to_vec(),
            address: "DeBio Address 2".as_bytes().to_vec(),
            latitude: Some("DeBio Latitude 2".as_bytes().to_vec()),
            longitude: Some("DeBio Longtitude 2".as_bytes().to_vec()),
            profile_image: Some("DeBio Profile Image owo".as_bytes().to_vec()),
		};
	}: update_lab(
        RawOrigin::Signed(caller), 
        new_lab
    )
    
	deregister_lab {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let lab = LabInfo {
            box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
            name: "DeBio Lab".as_bytes().to_vec(),
            email: "DeBio Email".as_bytes().to_vec(),
            country: "DeBio Country".as_bytes().to_vec(),
            region: "DeBio Region".as_bytes().to_vec(),
            city: "DeBio City".as_bytes().to_vec(),
            address: "DeBio Address".as_bytes().to_vec(),
            latitude: Some("DeBio Latitude".as_bytes().to_vec()),
            longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
            profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};

		let _add_labs = Labs::<T>::register_lab(caller_origin.clone(), lab);
	}: deregister_lab(
        RawOrigin::Signed(caller)
    )
}

impl_benchmark_test_suite! {Labs, crate::mock::ExternalityBuilder::build(), crate::mock::Test}