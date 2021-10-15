use super::*;

#[allow(unused)]
use crate::Pallet as Doctors;
use crate::DoctorInfo;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, vec, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::vec::Vec;

use frame_support::sp_runtime::traits::Hash;
use frame_support::sp_std::convert::TryInto;

benchmarks! {
	register_doctor {
		let doctor = DoctorInfo {
            name: "DeBio Doctor".as_bytes().to_vec(),
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
	}: register_doctor(
        RawOrigin::Signed(caller),
        doctor
    )
    
	update_doctor {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let old_doctor = DoctorInfo {
            name: "DeBio Doctor".as_bytes().to_vec(),
            email: "DeBio Email".as_bytes().to_vec(),
            country: "DeBio Country".as_bytes().to_vec(),
            region: "DeBio Region".as_bytes().to_vec(),
            city: "DeBio City".as_bytes().to_vec(),
            address: "DeBio Address".as_bytes().to_vec(),
            latitude: Some("DeBio Latitude".as_bytes().to_vec()),
            longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
            profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};
		let _add_doctors = Doctors::<T>::register_doctor(caller_origin.clone(), old_doctor);

        let new_doctor = DoctorInfo {
            name: "DeBio Doctor 2".as_bytes().to_vec(),
            email: "DeBio Email 2".as_bytes().to_vec(),
            country: "DeBio Country 2".as_bytes().to_vec(),
            region: "DeBio Region 2".as_bytes().to_vec(),
            city: "DeBio City 2".as_bytes().to_vec(),
            address: "DeBio Address 2".as_bytes().to_vec(),
            latitude: Some("DeBio Latitude 2".as_bytes().to_vec()),
            longitude: Some("DeBio Longtitude 2".as_bytes().to_vec()),
            profile_image: Some("DeBio Profile Image owo".as_bytes().to_vec()),
		};
	}: update_doctor(
        RawOrigin::Signed(caller), 
        new_doctor
    )
    
	deregister_doctor {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let doctor = DoctorInfo {
            name: "DeBio Doctor".as_bytes().to_vec(),
            email: "DeBio Email".as_bytes().to_vec(),
            country: "DeBio Country".as_bytes().to_vec(),
            region: "DeBio Region".as_bytes().to_vec(),
            city: "DeBio City".as_bytes().to_vec(),
            address: "DeBio Address".as_bytes().to_vec(),
            latitude: Some("DeBio Latitude".as_bytes().to_vec()),
            longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
            profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};

		let _add_doctors = Doctors::<T>::register_doctor(caller_origin.clone(), doctor);
	}: deregister_doctor(
        RawOrigin::Signed(caller)
    )
}

impl_benchmark_test_suite! {Doctors, crate::mock::ExternalityBuilder::build(), crate::mock::Test}