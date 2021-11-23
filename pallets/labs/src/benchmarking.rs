use super::*;

#[allow(unused)]
use crate::{LabInfo, LabVerificationStatus, LabVerifierKey, Pallet as Labs};
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

use frame_support::sp_runtime::traits::Hash;
use primitives_area_code::{CityCode, CountryCode, RegionCode};

benchmarks! {
	register_lab {
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
		let _add_labs = Labs::<T>::register_lab(caller_origin.clone(), old_lab);

		let new_lab = LabInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			name: "DeBio Lab 2".as_bytes().to_vec(),
			email: "DeBio Email 2".as_bytes().to_vec(),
			country: CountryCode::from_vec("C2".as_bytes().to_vec()),
			region: RegionCode::from_vec("DBI2".as_bytes().to_vec()),
			city: CityCode::from_vec("C2C2".as_bytes().to_vec()),
			address: "DeBio Address 2".as_bytes().to_vec(),
			phone_number: "+6281394653625".as_bytes().to_vec(),
			website: "DeBio Website".as_bytes().to_vec(),
			latitude: Some("DeBio Latitude 2".as_bytes().to_vec()),
			longitude: Some("DeBio Longtitude 2".as_bytes().to_vec()),
			profile_image: Some("DeBio Profile Image owo".as_bytes().to_vec()),
		};
	}: update_lab(
		RawOrigin::Signed(caller),
		new_lab
	)

	update_lab_verification_status {
		let caller: T::AccountId = LabVerifierKey::<T>::get();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let old_lab = LabInfo {
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
		let _add_labs = Labs::<T>::register_lab(caller_origin.clone(), old_lab);
	}: update_lab_verification_status(
		RawOrigin::Signed(caller),
		caller.clone(),
		LabVerificationStatus::default()
	)

	deregister_lab {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

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
	}: deregister_lab(
		RawOrigin::Signed(caller)
	)
}

impl_benchmark_test_suite! {Labs, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
