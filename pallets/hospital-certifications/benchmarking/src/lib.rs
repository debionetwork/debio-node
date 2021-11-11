#![cfg_attr(not(feature = "std"), no_std)]
mod mock;

#[allow(unused)]
use hospital_certifications::Pallet as HospitalCertifications;
use hospital_certifications::{
	Config as HospitalCertificationsConfig,
	HospitalCertificationInfo
};

#[allow(unused)]
use hospitals::Pallet as Hospitals;
use hospitals::{
	Config as HospitalsConfig,
	HospitalInfo
};

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

pub struct Pallet<T: Config>(HospitalCertifications<T>);

pub trait Config:
	HospitalCertificationsConfig
	+ HospitalsConfig 
{}

use hospital_certifications::Call;
use primitives_area_code::{CountryCode, RegionCode, CityCode};

benchmarks! {
	create_certification {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));

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

		let certification = HospitalCertificationInfo {
			title: "DeBio certificate".as_bytes().to_vec(),
			issuer: "DeBio".as_bytes().to_vec(),
			month: "August".as_bytes().to_vec(),
			year: "2021".as_bytes().to_vec(),
			description: "This is my description".as_bytes().to_vec(),
			supporting_document: Some("This is my document".as_bytes().to_vec()),
		};
	}: create_certification(RawOrigin::Signed(caller), certification)

	update_certification {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));

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

		let old_certification = HospitalCertificationInfo {
			title: "DeBio certificate".as_bytes().to_vec(),
			issuer: "DeBio".as_bytes().to_vec(),
			month: "August".as_bytes().to_vec(),
			year: "2021".as_bytes().to_vec(),
			description: "This is my description".as_bytes().to_vec(),
			supporting_document: Some("This is my document".as_bytes().to_vec()),
		};
		let _create_certification = HospitalCertifications::<T>::create_certification(caller_origin.clone(), old_certification);
        let _hospital = Hospitals::<T>::hospital_by_account_id(caller.clone())
            .unwrap();

		let new_certification = HospitalCertificationInfo {
			title: "DeBio certificate 2".as_bytes().to_vec(),
			issuer: "DeBio 2".as_bytes().to_vec(),
			month: "September".as_bytes().to_vec(),
			year: "2022".as_bytes().to_vec(),
			description: "This is my description 2".as_bytes().to_vec(),
			supporting_document: Some("This is my document 2".as_bytes().to_vec()),
		};
	}: update_certification(RawOrigin::Signed(caller), _hospital.certifications[0], new_certification)

	delete_certification {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));

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

		let old_certification = HospitalCertificationInfo {
			title: "DeBio certificate".as_bytes().to_vec(),
			issuer: "DeBio".as_bytes().to_vec(),
			month: "August".as_bytes().to_vec(),
			year: "2021".as_bytes().to_vec(),
			description: "This is my description".as_bytes().to_vec(),
			supporting_document: Some("This is my document".as_bytes().to_vec()),
		};
		let _create_certification = HospitalCertifications::<T>::create_certification(caller_origin.clone(), old_certification);
        let _hospital = Hospitals::<T>::hospital_by_account_id(caller.clone())
            .unwrap();
	}: delete_certification(RawOrigin::Signed(caller), _hospital.certifications[0])
}

impl_benchmark_test_suite! {Pallet, crate::mock::ExternalityBuilder::build(), crate::mock::Test}