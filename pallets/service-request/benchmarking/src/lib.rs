#![cfg_attr(not(feature = "std"), no_std)]
mod mock;

use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::{
	sp_runtime::{traits::Hash, SaturatedConversion},
	traits::Currency,
};
use frame_system::RawOrigin;
use genetic_testing::{
	Config as GeneticTestingConfig, DnaSampleStatus, DnaTestResultSubmission,
	Pallet as GeneticTesting,
};
use labs::{Config as LabsConfig, LabInfo, LabVerifierKey, Pallet as Labs};
use orders::{Config as OrdersConfig, Pallet as Orders};
use services::{Config as ServiceConfig, Pallet as Services, ServiceInfo};

#[allow(unused)]
use service_request::{
	AdminKey, Call, Config as ServiceRequestConfig, Pallet as ServiceRequest, RequestByAccountId,
};

use primitives_area_code::{CityCode, CountryCode, RegionCode};
use primitives_duration::ExpectedDuration;
use primitives_price_and_currency::{CurrencyType, Price, PriceByCurrency};
use primitives_verification_status::VerificationStatus;
use sp_std::vec;
use traits_services::types::ServiceFlow;

pub trait Config:
	ServiceRequestConfig + LabsConfig + OrdersConfig + ServiceConfig + GeneticTestingConfig
{
}

pub struct Pallet<T: Config>(ServiceRequest<T>);

const SEED: u32 = 0;

benchmarks! {
	create_request {
		// Initial account
		let caller: T::AccountId = whitelisted_caller();

		// Default balance
		let init_balance = 1000000000000000000000u128.saturated_into();
		let total_staked = 10000000000000000000u128.saturated_into();

		// Caller initial balance
		let _ = <T as service_request::Config>::Currency::deposit_creating(
			&caller,
			init_balance
		);

		// Create request
		let country = "Indonesia".as_bytes().to_vec();
		let region = "West Java".as_bytes().to_vec();
		let city = "Bogor".as_bytes().to_vec();
		let service_category = "Vaksin".as_bytes().to_vec();
	}: create_request(
		RawOrigin::Signed(caller),
		country,
		region,
		city,
		service_category,
		total_staked
	)

	unstake {
		// Initial account
		let caller: T::AccountId = whitelisted_caller();

		// Default balance
		let init_balance = 1000000000000000000000u128.saturated_into();
		let total_staked = 10000000000000000000u128.saturated_into();

		// Caller init balance
		let _ = <T as service_request::Config>::Currency::deposit_creating(
			&caller,
			init_balance
		);

		// Create request
		let origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));
		let _new_request = ServiceRequest::<T>::create_request(
			origin,
			"Indonesia".as_bytes().to_vec(),
			"West Java".as_bytes().to_vec(),
			"Bogor".as_bytes().to_vec(),
			"Vaksin".as_bytes().to_vec(),
			total_staked
		);

		// Unstake
		let request_ids = RequestByAccountId::<T>::get(caller.clone());
		let request_id = request_ids[0];
	}: unstake(
		RawOrigin::Signed(caller),
		request_ids[0]
	)

	retrieve_unstaked_amount {
		// Initial account
		let caller: T::AccountId = whitelisted_caller();

		// Default balance
		let init_balance = 1000000000000000000000u128.saturated_into();
		let total_staked = 10000000000000000000u128.saturated_into();

		// Caller init balance
		let _ = <T as service_request::Config>::Currency::deposit_creating(
			&caller,
			init_balance
		);

		let origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));
		let _new_request = ServiceRequest::<T>::create_request(
			origin.clone(),
			"Indonesia".as_bytes().to_vec(),
			"West Java".as_bytes().to_vec(),
			"Bogor".as_bytes().to_vec(),
			"Vaksin".as_bytes().to_vec(),
			total_staked
		);

		// Unstake
		let request_ids = RequestByAccountId::<T>::get(caller.clone());
		let request_id = request_ids[0];
		let _request_unstake = ServiceRequest::<T>::unstake(origin, request_id);
	}: retrieve_unstaked_amount(RawOrigin::Signed(caller), request_id)

	claim_request {
		// Initial account
		let caller: T::AccountId = whitelisted_caller();
		let customer: T::AccountId = account("customer", 0, SEED);

		// Default balance
		let init_balance = 1000000000000000000000u128.saturated_into();
		let total_staked = 10000000000000000000u128.saturated_into();
		let total_price = 10000000000000000000u128.saturated_into();
		let testing_price = 10000000000000000000u128.saturated_into();
		let qc_price = 10000000000000000000u128.saturated_into();

		// Caller init balance
		let _ = <T as service_request::Config>::Currency::deposit_creating(
			&customer,
			init_balance
		);

		// Set lab info
		let lab = LabInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			name: "DeBio Lab".as_bytes().to_vec(),
			email: "DeBio Email".as_bytes().to_vec(),
			country: CountryCode::from_vec("DC".as_bytes().to_vec()),
			region: RegionCode::from_vec("DB".as_bytes().to_vec()),
			city: CityCode::from_vec("CITY".as_bytes().to_vec()),
			address: "DeBio Address".as_bytes().to_vec(),
			phone_number: "+6281394653625".as_bytes().to_vec(),
			website: "DeBio Website".as_bytes().to_vec(),
			latitude: Some("DeBio Latitude".as_bytes().to_vec()),
			longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
			profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};

		// register lab
		let origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));
		let _add_labs = Labs::<T>::register_lab(origin.clone(), lab);

		// Verified lab
		let admin_key: T::AccountId = LabVerifierKey::<T>::get().unwrap();
		let admin_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(admin_key));
		let status = VerificationStatus::Verified;
		let _ = Labs::<T>::update_lab_verification_status(admin_origin, caller.clone(), status);

		// Create request
		let cust_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(customer.clone()));
		let _new_request = ServiceRequest::<T>::create_request(
			cust_origin,
			"Indonesia".as_bytes().to_vec(),
			"West Java".as_bytes().to_vec(),
			"Bogor".as_bytes().to_vec(),
			"Vaksin".as_bytes().to_vec(),
			total_staked
		);

		let request_ids = RequestByAccountId::<T>::get(customer);
		let request_id = request_ids[0];

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: testing_price }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: qc_price }],
		};

		let service_info = ServiceInfo {
			name: "DeBio service name".as_bytes().to_vec(),
			prices_by_currency: vec![prices_by_currency_dbio],
			expected_duration: ExpectedDuration::default(),
			category: "DeBio service category".as_bytes().to_vec(),
			description: "DeBio service description".as_bytes().to_vec(),
			dna_collection_process: "DeBio service dna_collection_process".as_bytes().to_vec(),
			test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
			long_description: Some("DeBio service long_description".as_bytes().to_vec()),
			image: Some("DeBio service image".as_bytes().to_vec()),
		};

		let _services = Services::<T>::create_service(
			origin,
			service_info,
			ServiceFlow::default()
		);

		let _lab = Labs::<T>::lab_by_account_id(caller.clone()).unwrap();
		let service_id = _lab.services[0];
	}: claim_request(RawOrigin::Signed(caller),	request_id,	service_id)

	process_request {
		// Init account
		let caller: T::AccountId = whitelisted_caller();
		let lab: T::AccountId = account("lab", 0, SEED);

		// Default balance
		let init_balance = 1000000000000000000000u128.saturated_into();
		let total_staked = 10000000000000000000u128.saturated_into();
		let total_price = 10000000000000000000u128.saturated_into();
		let testing_price = 10000000000000000000u128.saturated_into();
		let qc_price = 10000000000000000000u128.saturated_into();

		// Caller init balance
		let _ = <T as service_request::Config>::Currency::deposit_creating(
			&caller,
			init_balance
		);

		// Set lab info
		let lab_info = LabInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			name: "DeBio Lab".as_bytes().to_vec(),
			email: "DeBio Email".as_bytes().to_vec(),
			country: CountryCode::from_vec("DC".as_bytes().to_vec()),
			region: RegionCode::from_vec("DB".as_bytes().to_vec()),
			city: CityCode::from_vec("CITY".as_bytes().to_vec()),
			address: "DeBio Address".as_bytes().to_vec(),
			phone_number: "+6281394653625".as_bytes().to_vec(),
			website: "DeBio Website".as_bytes().to_vec(),
			latitude: Some("DeBio Latitude".as_bytes().to_vec()),
			longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
			profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};

		// register lab
		let lab_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(lab.clone()));
		let _ = Labs::<T>::register_lab(lab_origin.clone(), lab_info);

		// verified lab
		let admin_key: T::AccountId = LabVerifierKey::<T>::get().unwrap();
		let admin_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(admin_key));
		let status = VerificationStatus::Verified;
		let _verified = Labs::<T>::update_lab_verification_status(
			admin_origin,
			lab.clone(),
			status
		);

		// Create request
		let cust_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));
		let _new_request = ServiceRequest::<T>::create_request(
			cust_origin.clone(),
			"Indonesia".as_bytes().to_vec(),
			"West Java".as_bytes().to_vec(),
			"Bogor".as_bytes().to_vec(),
			"Vaksin".as_bytes().to_vec(),
			total_staked
		);

		let request_ids = RequestByAccountId::<T>::get(caller.clone());
		let request_id = request_ids[0];

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: testing_price }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: qc_price }],
		};

		let service_info = ServiceInfo {
			name: "DeBio service name".as_bytes().to_vec(),
			prices_by_currency: vec![prices_by_currency_dbio],
			expected_duration: ExpectedDuration::default(),
			category: "DeBio service category".as_bytes().to_vec(),
			description: "DeBio service description".as_bytes().to_vec(),
			dna_collection_process: "DeBio service dna_collection_process".as_bytes().to_vec(),
			test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
			long_description: Some("DeBio service long_description".as_bytes().to_vec()),
			image: Some("DeBio service image".as_bytes().to_vec()),
		};

		let _ = Services::<T>::create_service(
			lab_origin.clone(),
			service_info,
			ServiceFlow::default()
		);
		let _lab = Labs::<T>::lab_by_account_id(lab).unwrap();
		let service_id = _lab.services[0];

		let _claim_request = ServiceRequest::<T>::claim_request(
			lab_origin,
			request_id,
			service_id,
		);

		let _order = Orders::<T>::create_order(
			cust_origin,
			service_id,
			0,
			T::Hashing::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		);

		let order_id = Orders::<T>::last_order_by_customer_id(caller.clone()).unwrap();
	}: process_request(RawOrigin::Signed(caller), request_id, order_id)

	// Finalize by lab
	finalize_request{
		// Initial account
		let caller: T::AccountId = whitelisted_caller();
		let customer: T::AccountId = account("customer", 0, SEED);

		// Default balance
		let init_balance = 1000000000000000000000u128.saturated_into();
		let total_staked = 10000000000000000000u128.saturated_into();
		let total_price = 10000000000000000000u128.saturated_into();
		let testing_price = 10000000000000000000u128.saturated_into();
		let qc_price = 10000000000000000000u128.saturated_into();

		// Caller init balance
		let _ = <T as service_request::Config>::Currency::deposit_creating(
			&customer,
			init_balance
		);

		// Seet lab info
		let lab = LabInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			name: "DeBio Lab".as_bytes().to_vec(),
			email: "DeBio Email".as_bytes().to_vec(),
			country: CountryCode::from_vec("DC".as_bytes().to_vec()),
			region: RegionCode::from_vec("DB".as_bytes().to_vec()),
			city: CityCode::from_vec("CITY".as_bytes().to_vec()),
			address: "DeBio Address".as_bytes().to_vec(),
			phone_number: "+6281394653625".as_bytes().to_vec(),
			website: "DeBio Website".as_bytes().to_vec(),
			latitude: Some("DeBio Latitude".as_bytes().to_vec()),
			longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
			profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};

		// Register lab
		let lab_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));
		let _ = Labs::<T>::register_lab(lab_origin.clone(), lab);

		// verified lab
		let admin_key: T::AccountId = LabVerifierKey::<T>::get().unwrap();
		let admin_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(admin_key));
		let status = VerificationStatus::Verified;
		let _verified_labs = Labs::<T>::update_lab_verification_status(
			admin_origin,
			caller.clone(),
			status
		);

		// Create request
		let cust_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(customer.clone()));
		let _new_request = ServiceRequest::<T>::create_request(
			cust_origin.clone(),
			"Indonesia".as_bytes().to_vec(),
			"West Java".as_bytes().to_vec(),
			"Bogor".as_bytes().to_vec(),
			"Vaksin".as_bytes().to_vec(),
			total_staked
		);

		let request_ids = RequestByAccountId::<T>::get(customer.clone());
		let request_id = request_ids[0];

		// Set service info
		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: testing_price }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: qc_price }],
		};

		let service_info = ServiceInfo {
			name: "DeBio service name".as_bytes().to_vec(),
			prices_by_currency: vec![prices_by_currency_dbio],
			expected_duration: ExpectedDuration::default(),
			category: "DeBio service category".as_bytes().to_vec(),
			description: "DeBio service description".as_bytes().to_vec(),
			dna_collection_process: "DeBio service dna_collection_process".as_bytes().to_vec(),
			test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
			long_description: Some("DeBio service long_description".as_bytes().to_vec()),
			image: Some("DeBio service image".as_bytes().to_vec()),
		};

		// claim request
		let _ = Services::<T>::create_service(
			lab_origin.clone(),
			service_info,
			ServiceFlow::default()
		);
		let _lab = Labs::<T>::lab_by_account_id(caller.clone()).unwrap();
		let service_id = _lab.services[0];
		let _ = ServiceRequest::<T>::claim_request(
			lab_origin.clone(),
			request_id,
			service_id,
		);

		// process request
		let _order = Orders::<T>::create_order(
			cust_origin.clone(),
			service_id,
			0,
			T::Hashing::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		);

		let order_id = Orders::<T>::last_order_by_customer_id(customer).unwrap();
		let _ = ServiceRequest::<T>::process_request(
			cust_origin.clone(),
			request_id,
			order_id,
		);

		let _ = Orders::<T>::set_order_paid(cust_origin, order_id);
		let dna_sample = GeneticTesting::<T>::dna_samples_by_lab_id(caller.clone()).unwrap();

		let _ = GeneticTesting::<T>::submit_test_result(
			lab_origin.clone(),
			dna_sample[0].clone(),
			DnaTestResultSubmission {
				comments: Some("comment".as_bytes().to_vec()),
				result_link: Some("result_link".as_bytes().to_vec()),
				report_link: Some("report_link".as_bytes().to_vec()),
			}
		);

		let _ = GeneticTesting::<T>::process_dna_sample(
			lab_origin.clone(),
			dna_sample[0].clone(),
			DnaSampleStatus::ResultReady,
		);

		let _ = Orders::<T>::fulfill_order(
			lab_origin,
			order_id
		);
	}: finalize_request(RawOrigin::Signed(caller), request_id)

	update_admin_key {
		let caller: T::AccountId = AdminKey::<T>::get().unwrap();
		let caller2: T::AccountId = whitelisted_caller();
	}: update_admin_key(RawOrigin::Root, caller2)
}
