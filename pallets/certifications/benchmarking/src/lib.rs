#![cfg_attr(not(feature = "std"), no_std)]
mod mock;

#[allow(unused)]
use certifications::Pallet as Certifications;
use certifications::{
	Config as CertificationsConfig,
	CertificationInfo
};

#[allow(unused)]
use labs::Pallet as Labs;
use labs::{
	Config as LabsConfig,
	LabInfo
};

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

pub struct Pallet<T: Config>(Certifications<T>);

pub trait Config:
	CertificationsConfig
	+ LabsConfig 
{}

use certifications::Call;
use frame_support::sp_runtime::traits::Hash;
use traits_area_code::{CountryCode, RegionCode, CityCode};

benchmarks! {
	create_certification {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));

		let lab = LabInfo {
            box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
            name: "DeBio Lab".as_bytes().to_vec(),
            email: "DeBio Email".as_bytes().to_vec(),
            country: CountryCode::from_vec("DC".as_bytes().to_vec()),
            region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
            city: CityCode::from_vec("City".as_bytes().to_vec()),
            address: "DeBio Address".as_bytes().to_vec(),
            phone_number: "+6281394653625".as_bytes().to_vec(),
            website: "DeBio Website".as_bytes().to_vec(),
            latitude: Some("DeBio Latitude".as_bytes().to_vec()),
            longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
            profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};
		let _add_labs = Labs::<T>::register_lab(caller_origin.clone(), lab);

		let certification = CertificationInfo {
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

		let lab = LabInfo {
            box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
            name: "DeBio Lab".as_bytes().to_vec(),
            email: "DeBio Email".as_bytes().to_vec(),
            country: CountryCode::from_vec("DC".as_bytes().to_vec()),
            region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
            city: CityCode::from_vec("City".as_bytes().to_vec()),
            address: "DeBio Address".as_bytes().to_vec(),
            phone_number: "+6281394653625".as_bytes().to_vec(),
            website: "DeBio Website".as_bytes().to_vec(),
            latitude: Some("DeBio Latitude".as_bytes().to_vec()),
            longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
            profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};
		let _add_labs = Labs::<T>::register_lab(caller_origin.clone(), lab);

		let old_certification = CertificationInfo {
			title: "DeBio certificate".as_bytes().to_vec(),
			issuer: "DeBio".as_bytes().to_vec(),
			month: "August".as_bytes().to_vec(),
			year: "2021".as_bytes().to_vec(),
			description: "This is my description".as_bytes().to_vec(),
			supporting_document: Some("This is my document".as_bytes().to_vec()),
		};
		let _create_certification = Certifications::<T>::create_certification(caller_origin.clone(), old_certification);
        let _lab = Labs::<T>::lab_by_account_id(caller.clone())
            .unwrap();

		let new_certification = CertificationInfo {
			title: "DeBio certificate 2".as_bytes().to_vec(),
			issuer: "DeBio 2".as_bytes().to_vec(),
			month: "September".as_bytes().to_vec(),
			year: "2022".as_bytes().to_vec(),
			description: "This is my description 2".as_bytes().to_vec(),
			supporting_document: Some("This is my document 2".as_bytes().to_vec()),
		};
	}: update_certification(RawOrigin::Signed(caller), _lab.certifications[0], new_certification)

	delete_certification {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));

		let lab = LabInfo {
            box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
            name: "DeBio Lab".as_bytes().to_vec(),
            email: "DeBio Email".as_bytes().to_vec(),
            country: CountryCode::from_vec("DC".as_bytes().to_vec()),
            region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
            city: CityCode::from_vec("City".as_bytes().to_vec()),
            address: "DeBio Address".as_bytes().to_vec(),
            phone_number: "+6281394653625".as_bytes().to_vec(),
            website: "DeBio Website".as_bytes().to_vec(),
            latitude: Some("DeBio Latitude".as_bytes().to_vec()),
            longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
            profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};
		let _add_labs = Labs::<T>::register_lab(caller_origin.clone(), lab);

		let old_certification = CertificationInfo {
			title: "DeBio certificate".as_bytes().to_vec(),
			issuer: "DeBio".as_bytes().to_vec(),
			month: "August".as_bytes().to_vec(),
			year: "2021".as_bytes().to_vec(),
			description: "This is my description".as_bytes().to_vec(),
			supporting_document: Some("This is my document".as_bytes().to_vec()),
		};
		let _create_certification = Certifications::<T>::create_certification(caller_origin.clone(), old_certification);
        let _lab = Labs::<T>::lab_by_account_id(caller.clone())
            .unwrap();
	}: delete_certification(RawOrigin::Signed(caller), _lab.certifications[0])
}

impl_benchmark_test_suite! {Pallet, crate::mock::ExternalityBuilder::build(), crate::mock::Test}