use super::*;

#[allow(unused)]
use crate::Pallet as Hospitals;
use crate::HospitalInfo;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, vec, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::vec::Vec;

use frame_support::sp_runtime::traits::Hash;
use frame_support::sp_std::convert::TryInto;

benchmarks! {
	register_hospital {
		let hospital = HospitalInfo {
            name: "DeBio Hospital".as_bytes().to_vec(),
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
	}: register_hospital(
        RawOrigin::Signed(caller),
        hospital
    )
    
	update_hospital {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let old_hospital = HospitalInfo {
            name: "DeBio Hospital".as_bytes().to_vec(),
            email: "DeBio Email".as_bytes().to_vec(),
            country: "DeBio Country".as_bytes().to_vec(),
            region: "DeBio Region".as_bytes().to_vec(),
            city: "DeBio City".as_bytes().to_vec(),
            address: "DeBio Address".as_bytes().to_vec(),
            latitude: Some("DeBio Latitude".as_bytes().to_vec()),
            longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
            profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};
		let _add_hospitals = Hospitals::<T>::register_hospital(caller_origin.clone(), old_hospital);

        let new_hospital = HospitalInfo {
            name: "DeBio Hospital 2".as_bytes().to_vec(),
            email: "DeBio Email 2".as_bytes().to_vec(),
            country: "DeBio Country 2".as_bytes().to_vec(),
            region: "DeBio Region 2".as_bytes().to_vec(),
            city: "DeBio City 2".as_bytes().to_vec(),
            address: "DeBio Address 2".as_bytes().to_vec(),
            latitude: Some("DeBio Latitude 2".as_bytes().to_vec()),
            longitude: Some("DeBio Longtitude 2".as_bytes().to_vec()),
            profile_image: Some("DeBio Profile Image owo".as_bytes().to_vec()),
		};
	}: update_hospital(
        RawOrigin::Signed(caller), 
        new_hospital
    )
    
	deregister_hospital {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let hospital = HospitalInfo {
            name: "DeBio Hospital".as_bytes().to_vec(),
            email: "DeBio Email".as_bytes().to_vec(),
            country: "DeBio Country".as_bytes().to_vec(),
            region: "DeBio Region".as_bytes().to_vec(),
            city: "DeBio City".as_bytes().to_vec(),
            address: "DeBio Address".as_bytes().to_vec(),
            latitude: Some("DeBio Latitude".as_bytes().to_vec()),
            longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
            profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};

		let _add_hospitals = Hospitals::<T>::register_hospital(caller_origin.clone(), hospital);
	}: deregister_hospital(
        RawOrigin::Signed(caller)
    )
}

impl_benchmark_test_suite! {Hospitals, crate::mock::ExternalityBuilder::build(), crate::mock::Test}