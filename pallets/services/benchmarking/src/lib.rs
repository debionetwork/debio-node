#![cfg_attr(not(feature = "std"), no_std)]
mod mock;

#[allow(unused)]
use services::Pallet as Services;
use services::{
	Config as ServicesConfig,
	ServiceInfo
};

#[allow(unused)]
use labs::Pallet as Labs;
use labs::{
	Config as LabsConfig,
	LabInfo
};

#[allow(unused)]
use user_profile::Pallet as UserProfile;
use user_profile::Config as UserProfileConfig;

use traits_services::types::{PriceByCurrency, ExpectedDuration, ServiceFlow};

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::vec;

pub struct Pallet<T: Config>(Services<T>);

pub trait Config:
	ServicesConfig
	+ LabsConfig
	+ UserProfileConfig
{}

use services::Call;
use frame_support::sp_runtime::traits::Hash;
use primitives_area_code::{CountryCode, RegionCode, CityCode};

benchmarks! {
	create_service {
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

        let eth_address = <T as UserProfileConfig>::EthereumAddress::default();
		let _set_eth_address = UserProfile::<T>::set_eth_address(caller_origin, eth_address);

		let service_info = ServiceInfo {
			name: "DeBio name".as_bytes().to_vec(),
			prices_by_currency: vec![
				PriceByCurrency::default()
			],
			expected_duration: ExpectedDuration::default(),
			category: "DeBio category".as_bytes().to_vec(),
			description: "This is my description".as_bytes().to_vec(),
			test_result_sample: "Test result sample".as_bytes().to_vec(),
			dna_collection_process : "Dna Collection Process".as_bytes().to_vec(),
			long_description: Some("This is my long description".as_bytes().to_vec()),
			image: Some("This is my image".as_bytes().to_vec()),
		};
	}: create_service(RawOrigin::Signed(caller), service_info, ServiceFlow::default())

	update_service {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let lab_info = LabInfo {
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
		let _add_labs = Labs::<T>::register_lab(caller_origin.clone(), lab_info);

        let eth_address = <T as UserProfileConfig>::EthereumAddress::default();
		let _set_eth_address = UserProfile::<T>::set_eth_address(caller_origin.clone(), eth_address);

		let old_service_info = ServiceInfo {
			name: "DeBio name".as_bytes().to_vec(),
			prices_by_currency: vec![
				PriceByCurrency::default()
			],
			expected_duration: ExpectedDuration::default(),
			category: "DeBio category".as_bytes().to_vec(),
			description: "This is my description".as_bytes().to_vec(),
			test_result_sample: "Test result sample".as_bytes().to_vec(),
			dna_collection_process : "Dna Collection Process".as_bytes().to_vec(),
			long_description: Some("This is my long description".as_bytes().to_vec()),
			image: Some("This is my image".as_bytes().to_vec()),
		};
		let _create_service = Services::<T>::create_service(caller_origin, old_service_info, ServiceFlow::default());

		let _lab = Labs::<T>::lab_by_account_id(caller.clone())
			.unwrap();

		let new_service_info = ServiceInfo {
			name: "DeBio name 2".as_bytes().to_vec(),
			prices_by_currency: vec![
				PriceByCurrency::default()
			],
			expected_duration: ExpectedDuration::default(),
			category: "DeBio category 2".as_bytes().to_vec(),
			description: "This is my description 2".as_bytes().to_vec(),
			test_result_sample: "Test result sample 2".as_bytes().to_vec(),
			dna_collection_process : "Dna Collection Process 2".as_bytes().to_vec(),
			long_description: Some("This is my long description 2".as_bytes().to_vec()),
			image: Some("This is my image 2".as_bytes().to_vec()),
		};
	}: update_service(RawOrigin::Signed(caller), _lab.services[0], new_service_info)

	delete_service {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let lab_info = LabInfo {
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
		let _add_labs = Labs::<T>::register_lab(caller_origin.clone(), lab_info);

        let eth_address = <T as UserProfileConfig>::EthereumAddress::default();
		let _set_eth_address = UserProfile::<T>::set_eth_address(caller_origin.clone(), eth_address);

		let old_service_info = ServiceInfo {
			name: "DeBio name".as_bytes().to_vec(),
			prices_by_currency: vec![
				PriceByCurrency::default()
			],
			expected_duration: ExpectedDuration::default(),
			category: "DeBio category".as_bytes().to_vec(),
			description: "This is my description".as_bytes().to_vec(),
			test_result_sample: "Test result sample".as_bytes().to_vec(),
			dna_collection_process : "Dna Collection Process".as_bytes().to_vec(),
			long_description: Some("This is my long description".as_bytes().to_vec()),
			image: Some("This is my image".as_bytes().to_vec()),
		};
		let _create_service = Services::<T>::create_service(caller_origin, old_service_info, ServiceFlow::default());

		let _lab = Labs::<T>::lab_by_account_id(caller.clone())
			.unwrap();
	}: delete_service(RawOrigin::Signed(caller), _lab.services[0])
}

impl_benchmark_test_suite! {Pallet, crate::mock::ExternalityBuilder::build(), crate::mock::Test}