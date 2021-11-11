use super::*;

#[allow(unused)]
use crate::Pallet as Hospitals;
use crate::HospitalInfo;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use primitives_area_code::{CountryCode, RegionCode, CityCode};

benchmarks! {
	register_hospital {
		let hospital = HospitalInfo {
            name: "DeBio Hospital".as_bytes().to_vec(),
            email: "DeBio Email".as_bytes().to_vec(),
            country: CountryCode::from_vec("DC".as_bytes().to_vec()),
            region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
            city: CityCode::from_vec("City".as_bytes().to_vec()),
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
            country: CountryCode::from_vec("DC".as_bytes().to_vec()),
            region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
            city: CityCode::from_vec("City".as_bytes().to_vec()),
            address: "DeBio Address".as_bytes().to_vec(),
            latitude: Some("DeBio Latitude".as_bytes().to_vec()),
            longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
            profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};
		let _add_hospitals = Hospitals::<T>::register_hospital(caller_origin.clone(), old_hospital);

        let new_hospital = HospitalInfo {
            name: "DeBio Hospital 2".as_bytes().to_vec(),
            email: "DeBio Email 2".as_bytes().to_vec(),
            country: CountryCode::from_vec("C2".as_bytes().to_vec()),
            region: RegionCode::from_vec("DBI2".as_bytes().to_vec()),
            city: CityCode::from_vec("C2C2".as_bytes().to_vec()),
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
            country: CountryCode::from_vec("DC".as_bytes().to_vec()),
            region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
            city: CityCode::from_vec("City".as_bytes().to_vec()),
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