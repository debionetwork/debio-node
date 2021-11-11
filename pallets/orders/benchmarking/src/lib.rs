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

#[allow(unused)]
use orders::Pallet as Orders;
use orders::{
	Config as OrdersConfig,
	EscrowKey
};

#[allow(unused)]
use genetic_testing::Pallet as GeneticTesting;
use genetic_testing::{
	DnaSampleStatus, DnaTestResultSubmission,
	Config as GeneticTestingConfig
};

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, vec};
use frame_system::RawOrigin;

pub struct Pallet<T: Config>(Orders<T>);

pub trait Config:
	ServicesConfig
	+ LabsConfig
	+ UserProfileConfig
	+ OrdersConfig
	+ GeneticTestingConfig
{}

use orders::Call;
use frame_support::sp_runtime::traits::Hash;
use primitives_area_code::{CountryCode, RegionCode, CityCode};

benchmarks! {
	create_order {
		let caller: T::AccountId = EscrowKey::<T>::get();
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
		let _set_eth_address = UserProfile::<T>::set_eth_address(caller_origin.clone(), eth_address);

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
		let _create_service = Services::<T>::create_service(caller_origin.clone(), service_info, ServiceFlow::default());
		
		let _lab = Labs::<T>::lab_by_account_id(caller.clone())
			.unwrap();
	}: create_order(
		RawOrigin::Signed(caller), 
		_lab.services[0],
		0,
		T::Hashing::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes())
	)
	
	cancel_order {
		let caller: T::AccountId = EscrowKey::<T>::get();
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
		let _set_eth_address = UserProfile::<T>::set_eth_address(caller_origin.clone(), eth_address);

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
		let _create_service = Services::<T>::create_service(caller_origin.clone(), service_info, ServiceFlow::default());
		
		let _lab = Labs::<T>::lab_by_account_id(caller.clone())
			.unwrap();

		let _create_order = Orders::<T>::create_order(
			caller_origin.clone(), 
			_lab.services[0],
			0,
			T::Hashing::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes())
		);

		let _order_id_list = Orders::<T>::orders_by_lab_id(caller.clone())
			.unwrap();
		let _order = Orders::<T>::order_by_id(_order_id_list[0])
			.unwrap();
	}: cancel_order(
		RawOrigin::Signed(caller), 
		_order.id
	)
	
	set_order_paid {
		let caller: T::AccountId = EscrowKey::<T>::get();
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
		let _set_eth_address = UserProfile::<T>::set_eth_address(caller_origin.clone(), eth_address);

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
		let _create_service = Services::<T>::create_service(caller_origin.clone(), service_info, ServiceFlow::default());
		
		let _lab = Labs::<T>::lab_by_account_id(caller.clone())
			.unwrap();

		let _create_order = Orders::<T>::create_order(
			caller_origin.clone(), 
			_lab.services[0],
			0,
			T::Hashing::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes())
		);

		let _order_id_list = Orders::<T>::orders_by_lab_id(caller.clone())
			.unwrap();
		let _order = Orders::<T>::order_by_id(_order_id_list[0])
			.unwrap();
	}: set_order_paid(
		RawOrigin::Signed(caller), 
		_order.id
	)
	
	fulfill_order {
		let caller: T::AccountId = EscrowKey::<T>::get();
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
		let _set_eth_address = UserProfile::<T>::set_eth_address(caller_origin.clone(), eth_address);

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
		let _create_service = Services::<T>::create_service(caller_origin.clone(), service_info, ServiceFlow::default());
		
		let _lab = Labs::<T>::lab_by_account_id(caller.clone())
			.unwrap();

		let _create_order = Orders::<T>::create_order(
			caller_origin.clone(), 
			_lab.services[0],
			0,
			T::Hashing::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes())
		);

		let _order_id_list = Orders::<T>::orders_by_lab_id(caller.clone())
			.unwrap();
		let _order = Orders::<T>::order_by_id(_order_id_list[0])
			.unwrap();

		let _set_order_paid = Orders::<T>::set_order_paid(
			caller_origin.clone(), 
			_order.id.clone()
		);

		let _dna_test_result = DnaTestResultSubmission {
			comments: Some("DNA Test Result comments".as_bytes().to_vec()),
			result_link: Some("DNA Test Result result_link".as_bytes().to_vec()),
			report_link: Some("DNA Test Result report_link".as_bytes().to_vec())
		};

		let _submit_test_result = GeneticTesting::<T>::submit_test_result(
			caller_origin.clone(),
			_order.dna_sample_tracking_id.clone(),
			_dna_test_result
		);

		let _ = GeneticTesting::<T>::process_dna_sample(
			caller_origin.clone(),
			_order.dna_sample_tracking_id,
			DnaSampleStatus::ResultReady
		);
	}: fulfill_order(
		RawOrigin::Signed(caller), 
		_order.id
	)
	
	set_order_refunded {
		let caller: T::AccountId = EscrowKey::<T>::get();
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
		let _set_eth_address = UserProfile::<T>::set_eth_address(caller_origin.clone(), eth_address);

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
		let _create_service = Services::<T>::create_service(caller_origin.clone(), service_info, ServiceFlow::default());
		
		let _lab = Labs::<T>::lab_by_account_id(caller.clone())
			.unwrap();

		let _create_order = Orders::<T>::create_order(
			caller_origin.clone(), 
			_lab.services[0],
			0,
			T::Hashing::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes())
		);

		let _order_id_list = Orders::<T>::orders_by_lab_id(caller.clone())
			.unwrap();
		let _order = Orders::<T>::order_by_id(_order_id_list[0])
			.unwrap();

		let _set_order_paid = Orders::<T>::set_order_paid(
			caller_origin.clone(), 
			_order.id.clone()
		);

		let _ = GeneticTesting::<T>::reject_dna_sample(
			caller_origin.clone(),
			_order.dna_sample_tracking_id,
			"Rejected title".as_bytes().to_vec(),
			"Rejected description".as_bytes().to_vec()
		);
	}: set_order_refunded(
		RawOrigin::Signed(caller), 
		_order.id
	)
}

impl_benchmark_test_suite! {Pallet, crate::mock::ExternalityBuilder::build(), crate::mock::Test}