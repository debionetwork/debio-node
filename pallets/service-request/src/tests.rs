use crate::{mock::*, AdminKey, Error, Request, RequestStatus};
use frame_support::{
	assert_noop, assert_ok,
	sp_runtime::traits::{Hash, Keccak256},
};
use genetic_testing::{DnaSampleStatus, DnaTestResultSubmission};
use labs::{LabInfo, LabVerifierKey};
use orders::{EscrowKey, PalletAccount};
use pallet_timestamp::Now;
use primitives_area_code::{CityCode, CountryCode, RegionCode};
use primitives_duration::ExpectedDuration;
use primitives_price_and_currency::{CurrencyType, Price, PriceByCurrency};
use primitives_verification_status::VerificationStatus;
use services::ServiceInfo;
use traits_services::types::ServiceFlow;

#[test]
fn create_request_works() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let customer = account_key("customer");

		assert_ok!(ServiceRequest::create_request(
			Origin::signed(customer),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(customer)[0];

		assert_eq!(
			ServiceRequest::request_by_id(request_id),
			Some(Request {
				hash: request_id,
				requester_address: customer,
				lab_address: None,
				service_id: None,
				order_id: None,
				country: String::from("Indonesia").into_bytes(),
				region: String::from("West Java").into_bytes(),
				city: String::from("Bogor").into_bytes(),
				service_category: String::from("Vaksin").into_bytes(),
				staking_amount: 10,
				status: RequestStatus::Open,
				created_at: 0,
				updated_at: None,
				unstaked_at: None,
			})
		);

		assert_eq!(
			ServiceRequest::service_count_request((
				String::from("Indonesia").into_bytes(),
				String::from("West Java").into_bytes(),
				String::from("Bogor").into_bytes(),
				String::from("Vaksin").into_bytes(),
			)),
			1,
		);

		let staking_account_id = ServiceRequest::staking_account_id(request_id);

		assert_eq!(
			ServiceRequest::staking_account_id_by_request_id(request_id),
			Some(staking_account_id)
		);

		assert_eq!(Balances::free_balance(customer), 190);
		assert_eq!(Balances::free_balance(staking_account_id), 10);
	})
}

#[test]
fn unstake_works() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let customer = account_key("customer");

		assert_ok!(ServiceRequest::create_request(
			Origin::signed(customer),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(customer)[0];

		assert_ok!(ServiceRequest::unstake(Origin::signed(customer), request_id));

		assert_eq!(
			ServiceRequest::request_by_id(request_id),
			Some(Request {
				hash: request_id,
				requester_address: customer,
				lab_address: None,
				service_id: None,
				order_id: None,
				country: String::from("Indonesia").into_bytes(),
				region: String::from("West Java").into_bytes(),
				city: String::from("Bogor").into_bytes(),
				service_category: String::from("Vaksin").into_bytes(),
				staking_amount: 10,
				status: RequestStatus::WaitingForUnstaked,
				created_at: 0,
				updated_at: None,
				unstaked_at: Some(0),
			})
		);
	})
}

#[test]
fn retrieve_unstake_works() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let customer = account_key("customer");

		assert_ok!(ServiceRequest::create_request(
			Origin::signed(customer),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(customer)[0];

		assert_ok!(ServiceRequest::unstake(Origin::signed(customer), request_id));

		Now::<Test>::put(10);

		assert_ok!(ServiceRequest::retrieve_unstaked_amount(Origin::signed(customer), request_id));

		assert_eq!(
			ServiceRequest::request_by_id(request_id),
			Some(Request {
				hash: request_id,
				requester_address: customer,
				lab_address: None,
				service_id: None,
				order_id: None,
				country: String::from("Indonesia").into_bytes(),
				region: String::from("West Java").into_bytes(),
				city: String::from("Bogor").into_bytes(),
				service_category: String::from("Vaksin").into_bytes(),
				staking_amount: 10,
				status: RequestStatus::Unstaked,
				created_at: 0,
				updated_at: None,
				unstaked_at: Some(0),
			})
		);

		let staking_account_id = ServiceRequest::staking_account_id(request_id);

		assert_eq!(Balances::free_balance(customer), 200);
		assert_eq!(Balances::free_balance(staking_account_id), 0);
	})
}

#[test]
fn claim_request_works() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let admin = account_key("admin");
		let customer = account_key("customer");
		let lab = account_key("lab");

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(customer),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(customer)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(lab),
			LabInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()
				),
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
			}
		));

		LabVerifierKey::<Test>::put(admin);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(admin),
			lab,
			VerificationStatus::Verified
		));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio service name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio service category".as_bytes().to_vec(),
				description: "DeBio service description".as_bytes().to_vec(),
				dna_collection_process: "DeBio service dna_collection_process".as_bytes().to_vec(),
				test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
				long_description: Some("DeBio service long_description".as_bytes().to_vec()),
				image: Some("DeBio service image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(lab).unwrap();
		let service_id = _lab.services[0];

		assert_ok!(ServiceRequest::claim_request(Origin::signed(lab), request_id, service_id,));

		assert_eq!(
			ServiceRequest::request_by_id(request_id),
			Some(Request {
				hash: request_id,
				requester_address: customer,
				lab_address: Some(lab),
				service_id: Some(service_id),
				order_id: None,
				country: String::from("Indonesia").into_bytes(),
				region: String::from("West Java").into_bytes(),
				city: String::from("Bogor").into_bytes(),
				service_category: String::from("Vaksin").into_bytes(),
				staking_amount: 10,
				status: RequestStatus::Claimed,
				created_at: 0,
				updated_at: Some(0),
				unstaked_at: None,
			})
		);
	})
}

#[test]
fn process_request_works() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let admin = account_key("admin");
		let customer = account_key("customer");
		let lab = account_key("lab");

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(customer),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(customer)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(lab),
			LabInfo {
				box_public_key: Keccak256::hash("box_public_key".as_bytes()),
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
			}
		));

		LabVerifierKey::<Test>::put(admin);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(admin),
			lab,
			VerificationStatus::Verified
		));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio service name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio service category".as_bytes().to_vec(),
				description: "DeBio service description".as_bytes().to_vec(),
				dna_collection_process: "DeBio service dna_collection_process".as_bytes().to_vec(),
				test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
				long_description: Some("DeBio service long_description".as_bytes().to_vec()),
				image: Some("DeBio service image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(lab).unwrap();
		let service_id = _lab.services[0];

		assert_ok!(ServiceRequest::claim_request(Origin::signed(lab), request_id, service_id,));

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			service_id,
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let order_id = Orders::last_order_by_customer_id(customer).unwrap();

		assert_ok!(
			ServiceRequest::process_request(Origin::signed(customer), request_id, order_id,)
		);

		assert_eq!(
			ServiceRequest::request_by_id(request_id),
			Some(Request {
				hash: request_id,
				requester_address: customer,
				lab_address: Some(lab),
				service_id: Some(service_id),
				order_id: Some(order_id),
				country: String::from("Indonesia").into_bytes(),
				region: String::from("West Java").into_bytes(),
				city: String::from("Bogor").into_bytes(),
				service_category: String::from("Vaksin").into_bytes(),
				staking_amount: 10,
				status: RequestStatus::Processed,
				created_at: 0,
				updated_at: Some(0),
				unstaked_at: None,
			})
		);
	})
}

#[test]
fn finalize_request_works() {
	<ExternalityBuilder>::default().existential_deposit(0).build().execute_with(|| {
		let admin = account_key("admin");
		let customer = account_key("customer");
		let lab = account_key("lab");
		let pallet_id = account_key("pallet_id");

		PalletAccount::<Test>::put(pallet_id);

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(customer),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(customer)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(lab),
			LabInfo {
				box_public_key: Keccak256::hash("box_public_key".as_bytes()),
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
			}
		));

		LabVerifierKey::<Test>::put(admin);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(admin),
			lab,
			VerificationStatus::Verified
		));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio service name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio service category".as_bytes().to_vec(),
				description: "DeBio service description".as_bytes().to_vec(),
				dna_collection_process: "DeBio service dna_collection_process".as_bytes().to_vec(),
				test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
				long_description: Some("DeBio service long_description".as_bytes().to_vec()),
				image: Some("DeBio service image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(lab).unwrap();
		let service_id = _lab.services[0];

		assert_ok!(ServiceRequest::claim_request(Origin::signed(lab), request_id, service_id,));

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			service_id,
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let order_id = Orders::last_order_by_customer_id(customer).unwrap();

		assert_ok!(
			ServiceRequest::process_request(Origin::signed(customer), request_id, order_id,)
		);

		let dna_sample = GeneticTesting::dna_samples_by_lab_id(lab).unwrap();

		assert_ok!(Orders::set_order_paid(Origin::signed(customer), order_id));

		assert_eq!(Balances::free_balance(customer), 180);

		assert_ok!(GeneticTesting::submit_test_result(
			Origin::signed(lab),
			dna_sample[0].clone(),
			DnaTestResultSubmission {
				comments: Some("comment".as_bytes().to_vec()),
				result_link: Some("result_link".as_bytes().to_vec()),
				report_link: Some("report_link".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticTesting::process_dna_sample(
			Origin::signed(lab),
			dna_sample[0].clone(),
			DnaSampleStatus::ResultReady,
		));

		EscrowKey::<Test>::put(admin);

		assert_ok!(Orders::fulfill_order(Origin::signed(lab), order_id));

		assert_ok!(ServiceRequest::finalize_request(Origin::signed(lab), request_id));

		assert_eq!(
			ServiceRequest::request_by_id(request_id),
			Some(Request {
				hash: request_id,
				requester_address: customer,
				lab_address: Some(lab),
				service_id: Some(service_id),
				order_id: Some(order_id),
				country: String::from("Indonesia").into_bytes(),
				region: String::from("West Java").into_bytes(),
				city: String::from("Bogor").into_bytes(),
				service_category: String::from("Vaksin").into_bytes(),
				staking_amount: 10,
				status: RequestStatus::Finalized,
				created_at: 0,
				updated_at: Some(0),
				unstaked_at: None,
			})
		);

		assert_eq!(
			ServiceRequest::service_count_request((
				String::from("Indonesia").into_bytes(),
				String::from("West Java").into_bytes(),
				String::from("Bogor").into_bytes(),
				String::from("Vaksin").into_bytes(),
			)),
			0
		);

		assert_eq!(ServiceRequest::request_by_account_id(customer), Vec::new(),);

		let staking_account_id = ServiceRequest::staking_account_id(request_id);

		assert_eq!(Balances::free_balance(customer), 190);
		assert_eq!(Balances::free_balance(staking_account_id), 0);
	})
}

#[test]
fn cant_create_request_when_stacking_amount_zero() {
	<ExternalityBuilder>::default().existential_deposit(0).build().execute_with(|| {
		let customer = account_key("customer");

		assert_noop!(
			ServiceRequest::create_request(
				Origin::signed(customer),
				String::from("Indonesia").into_bytes(),
				String::from("West Java").into_bytes(),
				String::from("Bogor").into_bytes(),
				String::from("Vaksin").into_bytes(),
				0
			),
			Error::<Test>::NotValidAmount
		);
	})
}

#[test]
fn cant_create_request_when_balance_not_enough() {
	<ExternalityBuilder>::default().existential_deposit(0).build().execute_with(|| {
		let other = account_key("other");

		assert_noop!(
			ServiceRequest::create_request(
				Origin::signed(other),
				String::from("Indonesia").into_bytes(),
				String::from("West Java").into_bytes(),
				String::from("Bogor").into_bytes(),
				String::from("Vaksin").into_bytes(),
				500
			),
			Error::<Test>::Arithmetic
		);
	})
}

#[test]
fn cant_retrive_unstake_amount_unstake_claim_process_and_finalize_request_when_not_exists() {
	<ExternalityBuilder>::default().existential_deposit(0).build().execute_with(|| {
		let admin = account_key("admin");
		let customer = account_key("customer");
		let lab = account_key("lab");

		assert_noop!(
			ServiceRequest::unstake(
				Origin::signed(customer),
				Keccak256::hash("request_id".as_bytes())
			),
			Error::<Test>::RequestNotFound
		);

		assert_noop!(
			ServiceRequest::claim_request(
				Origin::signed(lab),
				Keccak256::hash("request_id".as_bytes()),
				Keccak256::hash("service_id".as_bytes()),
			),
			Error::<Test>::RequestNotFound
		);

		assert_noop!(
			ServiceRequest::process_request(
				Origin::signed(customer),
				Keccak256::hash("request_id".as_bytes()),
				Keccak256::hash("order_id".as_bytes()),
			),
			Error::<Test>::RequestNotFound
		);

		AdminKey::<Test>::put(admin);

		assert_noop!(
			ServiceRequest::retrieve_unstaked_amount(
				Origin::signed(admin),
				Keccak256::hash("request_id".as_bytes())
			),
			Error::<Test>::RequestNotFound,
		);

		assert_noop!(
			ServiceRequest::finalize_request(
				Origin::signed(admin),
				Keccak256::hash("request_id".as_bytes()),
			),
			Error::<Test>::RequestNotFound,
		);
	})
}

#[test]
fn cant_claim_and_process_request_when_already_unstaked() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let customer = account_key("customer");
		let lab = account_key("lab");

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(customer),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(customer)[0];

		assert_ok!(ServiceRequest::unstake(Origin::signed(customer), request_id));

		assert_noop!(
			ServiceRequest::claim_request(
				Origin::signed(lab),
				request_id,
				Keccak256::hash("service_id".as_bytes()),
			),
			Error::<Test>::RequestAlreadyUnstaked
		);

		assert_noop!(
			ServiceRequest::process_request(
				Origin::signed(customer),
				request_id,
				Keccak256::hash("order_id".as_bytes()),
			),
			Error::<Test>::RequestAlreadyUnstaked
		);
	})
}

#[test]
fn cant_claim_request_when_already_claimed() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let admin = account_key("admin");
		let customer = account_key("customer");
		let lab = account_key("lab");

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(customer),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(customer)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(lab),
			LabInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()
				),
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
			}
		));

		LabVerifierKey::<Test>::put(admin);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(admin),
			lab,
			VerificationStatus::Verified
		));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio service name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio service category".as_bytes().to_vec(),
				description: "DeBio service description".as_bytes().to_vec(),
				dna_collection_process: "DeBio service dna_collection_process".as_bytes().to_vec(),
				test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
				long_description: Some("DeBio service long_description".as_bytes().to_vec()),
				image: Some("DeBio service image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(lab).unwrap();
		let service_id = _lab.services[0];

		assert_ok!(ServiceRequest::claim_request(Origin::signed(lab), request_id, service_id,));

		assert_noop!(
			ServiceRequest::claim_request(Origin::signed(lab), request_id, service_id,),
			Error::<Test>::RequestAlreadyClaimed
		);
	})
}

#[test]
fn cant_claim_request_when_already_processed_or_finalized() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let admin = account_key("admin");
		let customer = account_key("customer");
		let lab = account_key("lab");
		let pallet_id = account_key("pallet_id");

		PalletAccount::<Test>::put(pallet_id);

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(customer),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(customer)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(lab),
			LabInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()
				),
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
			}
		));

		LabVerifierKey::<Test>::put(admin);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(admin),
			lab,
			VerificationStatus::Verified
		));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio service name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio service category".as_bytes().to_vec(),
				description: "DeBio service description".as_bytes().to_vec(),
				dna_collection_process: "DeBio service dna_collection_process".as_bytes().to_vec(),
				test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
				long_description: Some("DeBio service long_description".as_bytes().to_vec()),
				image: Some("DeBio service image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(lab).unwrap();
		let service_id = _lab.services[0];

		assert_ok!(ServiceRequest::claim_request(Origin::signed(lab), request_id, service_id,));

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			service_id,
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let order_id = Orders::last_order_by_customer_id(customer).unwrap();

		assert_ok!(
			ServiceRequest::process_request(Origin::signed(customer), request_id, order_id,)
		);

		assert_noop!(
			ServiceRequest::claim_request(Origin::signed(lab), request_id, service_id,),
			Error::<Test>::RequestUnableToClaimed
		);

		let dna_sample = GeneticTesting::dna_samples_by_lab_id(lab).unwrap();

		assert_ok!(Orders::set_order_paid(Origin::signed(customer), order_id));

		assert_ok!(GeneticTesting::submit_test_result(
			Origin::signed(lab),
			dna_sample[0].clone(),
			DnaTestResultSubmission {
				comments: Some("comment".as_bytes().to_vec()),
				result_link: Some("result_link".as_bytes().to_vec()),
				report_link: Some("report_link".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticTesting::process_dna_sample(
			Origin::signed(lab),
			dna_sample[0].clone(),
			DnaSampleStatus::ResultReady,
		));

		EscrowKey::<Test>::put(admin);

		assert_ok!(Orders::fulfill_order(Origin::signed(lab), order_id));

		assert_ok!(ServiceRequest::finalize_request(Origin::signed(lab), request_id));

		assert_noop!(
			ServiceRequest::claim_request(Origin::signed(lab), request_id, service_id),
			Error::<Test>::RequestUnableToClaimed
		);
	})
}

#[test]
fn cant_claim_and_finalize_request_when_lab_not_exists() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let admin = account_key("admin");
		let customer = account_key("customer");
		let lab = account_key("lab");
		let other_lab = account_key("other_lab");

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(customer),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(customer)[0];

		assert_noop!(
			ServiceRequest::claim_request(
				Origin::signed(lab),
				request_id,
				Keccak256::hash("service_id".as_bytes()),
			),
			Error::<Test>::LabNotFound
		);

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(lab),
			LabInfo {
				box_public_key: Keccak256::hash("box_public_key".as_bytes()),
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
			}
		));

		LabVerifierKey::<Test>::put(admin);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(admin),
			lab,
			VerificationStatus::Verified
		));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio service name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio service category".as_bytes().to_vec(),
				description: "DeBio service description".as_bytes().to_vec(),
				dna_collection_process: "DeBio service dna_collection_process".as_bytes().to_vec(),
				test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
				long_description: Some("DeBio service long_description".as_bytes().to_vec()),
				image: Some("DeBio service image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(lab).unwrap();
		let service_id = _lab.services[0];

		assert_ok!(ServiceRequest::claim_request(Origin::signed(lab), request_id, service_id,));

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			service_id,
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let order_id = Orders::last_order_by_customer_id(customer).unwrap();

		assert_ok!(
			ServiceRequest::process_request(Origin::signed(customer), request_id, order_id,)
		);

		assert_noop!(
			ServiceRequest::finalize_request(Origin::signed(other_lab), request_id),
			Error::<Test>::LabNotFound,
		);
	})
}

#[test]
fn cant_claim_request_when_service_not_exists() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let admin = account_key("admin");
		let customer = account_key("customer");
		let lab = account_key("lab");

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(customer),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(customer)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(lab),
			LabInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()
				),
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
			}
		));

		LabVerifierKey::<Test>::put(admin);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(admin),
			lab,
			VerificationStatus::Verified
		));

		assert_noop!(
			ServiceRequest::claim_request(
				Origin::signed(lab),
				request_id,
				Keccak256::hash("service_id".as_bytes()),
			),
			Error::<Test>::ServiceNotFound,
		);
	})
}

#[test]
fn cant_claim_request_when_not_service_owner() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let admin = account_key("admin");
		let customer = account_key("customer");
		let lab = account_key("lab");
		let other_lab = account_key("other_lab");

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(customer),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(customer)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(lab),
			LabInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()
				),
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
			}
		));

		// Register other lab
		assert_ok!(Labs::register_lab(
			Origin::signed(other_lab),
			LabInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()
				),
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
			}
		));

		LabVerifierKey::<Test>::put(admin);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(admin),
			lab,
			VerificationStatus::Verified
		));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio service name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio service category".as_bytes().to_vec(),
				description: "DeBio service description".as_bytes().to_vec(),
				dna_collection_process: "DeBio service dna_collection_process".as_bytes().to_vec(),
				test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
				long_description: Some("DeBio service long_description".as_bytes().to_vec()),
				image: Some("DeBio service image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(lab).unwrap();
		let service_id = _lab.services[0];

		assert_noop!(
			ServiceRequest::claim_request(Origin::signed(other_lab), request_id, service_id),
			Error::<Test>::Unauthorized,
		);
	})
}

#[test]
fn cant_process_request_when_unathorized_customer() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let customer = account_key("customer");
		let admin = account_key("admin");
		let lab = account_key("lab");
		let other_customer = account_key("other_customer");

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(customer),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(customer)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(lab),
			LabInfo {
				box_public_key: Keccak256::hash("box_public_key".as_bytes()),
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
			}
		));

		LabVerifierKey::<Test>::put(admin);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(admin),
			lab,
			VerificationStatus::Verified
		));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio service name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio service category".as_bytes().to_vec(),
				description: "DeBio service description".as_bytes().to_vec(),
				dna_collection_process: "DeBio service dna_collection_process".as_bytes().to_vec(),
				test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
				long_description: Some("DeBio service long_description".as_bytes().to_vec()),
				image: Some("DeBio service image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(lab).unwrap();
		let service_id = _lab.services[0];

		assert_ok!(ServiceRequest::claim_request(Origin::signed(lab), request_id, service_id,));

		assert_noop!(
			ServiceRequest::process_request(
				Origin::signed(other_customer),
				request_id,
				Keccak256::hash("order_id".as_bytes()),
			),
			Error::<Test>::Unauthorized
		);
	})
}

#[test]
fn cant_process_request_when_order_not_found() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let admin = account_key("admin");
		let customer = account_key("customer");
		let lab = account_key("lab");

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(customer),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(customer)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(lab),
			LabInfo {
				box_public_key: Keccak256::hash("box_public_key".as_bytes()),
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
			}
		));

		LabVerifierKey::<Test>::put(admin);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(admin),
			lab,
			VerificationStatus::Verified
		));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio service name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio service category".as_bytes().to_vec(),
				description: "DeBio service description".as_bytes().to_vec(),
				dna_collection_process: "DeBio service dna_collection_process".as_bytes().to_vec(),
				test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
				long_description: Some("DeBio service long_description".as_bytes().to_vec()),
				image: Some("DeBio service image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(lab).unwrap();
		let service_id = _lab.services[0];

		assert_ok!(ServiceRequest::claim_request(Origin::signed(lab), request_id, service_id,));

		assert_noop!(
			ServiceRequest::process_request(
				Origin::signed(customer),
				request_id,
				Keccak256::hash("order_id".as_bytes())
			),
			Error::<Test>::OrderNotFound,
		);
	})
}

#[test]
fn cant_process_request_when_order_fullfilled() {
	<ExternalityBuilder>::default().existential_deposit(0).build().execute_with(|| {
		let admin = account_key("admin");
		let customer = account_key("customer");
		let lab = account_key("lab");
		let pallet_id = account_key("pallet_id");

		PalletAccount::<Test>::put(pallet_id);

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(customer),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(customer)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(lab),
			LabInfo {
				box_public_key: Keccak256::hash("box_public_key".as_bytes()),
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
			}
		));

		LabVerifierKey::<Test>::put(admin);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(admin),
			lab,
			VerificationStatus::Verified
		));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio service name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio service category".as_bytes().to_vec(),
				description: "DeBio service description".as_bytes().to_vec(),
				dna_collection_process: "DeBio service dna_collection_process".as_bytes().to_vec(),
				test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
				long_description: Some("DeBio service long_description".as_bytes().to_vec()),
				image: Some("DeBio service image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(lab).unwrap();
		let service_id = _lab.services[0];

		assert_ok!(ServiceRequest::claim_request(Origin::signed(lab), request_id, service_id,));

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			service_id,
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let order_id = Orders::last_order_by_customer_id(customer).unwrap();

		assert_ok!(
			ServiceRequest::process_request(Origin::signed(customer), request_id, order_id,)
		);

		let dna_sample = GeneticTesting::dna_samples_by_lab_id(lab).unwrap();

		assert_ok!(Orders::set_order_paid(Origin::signed(customer), order_id));

		assert_eq!(Balances::free_balance(customer), 180);

		assert_ok!(GeneticTesting::submit_test_result(
			Origin::signed(lab),
			dna_sample[0].clone(),
			DnaTestResultSubmission {
				comments: Some("comment".as_bytes().to_vec()),
				result_link: Some("result_link".as_bytes().to_vec()),
				report_link: Some("report_link".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticTesting::process_dna_sample(
			Origin::signed(lab),
			dna_sample[0].clone(),
			DnaSampleStatus::ResultReady,
		));

		EscrowKey::<Test>::put(admin);

		assert_ok!(Orders::fulfill_order(Origin::signed(lab), order_id));

		assert_ok!(ServiceRequest::finalize_request(Origin::signed(lab), request_id));

		assert_noop!(
			ServiceRequest::process_request(Origin::signed(customer), request_id, order_id),
			Error::<Test>::RequestUnableToProccess,
		);
	})
}

#[test]
fn cant_process_request_when_order_from_other_lab() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let admin = account_key("admin");
		let customer = account_key("customer");
		let lab = account_key("lab");
		let other_lab = account_key("other_lab");

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(customer),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(customer)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(lab),
			LabInfo {
				box_public_key: Keccak256::hash("box_public_key".as_bytes()),
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
			}
		));

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(other_lab),
			LabInfo {
				box_public_key: Keccak256::hash("box_public_key".as_bytes()),
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
			}
		));

		LabVerifierKey::<Test>::put(admin);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(admin),
			lab,
			VerificationStatus::Verified
		));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio service name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio.clone()],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio service category".as_bytes().to_vec(),
				description: "DeBio service description".as_bytes().to_vec(),
				dna_collection_process: "DeBio service dna_collection_process".as_bytes().to_vec(),
				test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
				long_description: Some("DeBio service long_description".as_bytes().to_vec()),
				image: Some("DeBio service image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		assert_ok!(Services::create_service(
			Origin::signed(other_lab),
			ServiceInfo {
				name: "DeBio service name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio service category".as_bytes().to_vec(),
				description: "DeBio service description".as_bytes().to_vec(),
				dna_collection_process: "DeBio service dna_collection_process".as_bytes().to_vec(),
				test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
				long_description: Some("DeBio service long_description".as_bytes().to_vec()),
				image: Some("DeBio service image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(lab).unwrap();
		let service_id = _lab.services[0];

		let _other_lab = Labs::lab_by_account_id(other_lab).unwrap();
		let other_service_id = _other_lab.services[0];

		assert_ok!(ServiceRequest::claim_request(Origin::signed(lab), request_id, service_id,));

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			other_service_id,
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let order_id = Orders::last_order_by_customer_id(customer).unwrap();

		assert_noop!(
			ServiceRequest::process_request(Origin::signed(customer), request_id, order_id),
			Error::<Test>::RequestUnableToProccess
		);
	})
}

#[test]
fn cant_process_request_when_order_not_customer() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let admin = account_key("admin");
		let customer = account_key("customer");
		let lab = account_key("lab");
		let other_customer = account_key("other_customer");

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(customer),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(customer)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(lab),
			LabInfo {
				box_public_key: Keccak256::hash("box_public_key".as_bytes()),
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
			}
		));

		LabVerifierKey::<Test>::put(admin);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(admin),
			lab,
			VerificationStatus::Verified
		));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio service name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio service category".as_bytes().to_vec(),
				description: "DeBio service description".as_bytes().to_vec(),
				dna_collection_process: "DeBio service dna_collection_process".as_bytes().to_vec(),
				test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
				long_description: Some("DeBio service long_description".as_bytes().to_vec()),
				image: Some("DeBio service image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(lab).unwrap();
		let service_id = _lab.services[0];

		assert_ok!(ServiceRequest::claim_request(Origin::signed(lab), request_id, service_id,));

		assert_ok!(Orders::create_order(
			Origin::signed(other_customer),
			service_id,
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let order_id = Orders::last_order_by_customer_id(other_customer).unwrap();

		assert_noop!(
			ServiceRequest::process_request(Origin::signed(customer), request_id, order_id),
			Error::<Test>::RequestUnableToProccess
		);
	})
}

#[test]
fn cant_process_request_when_order_from_wrong_service() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let admin = account_key("admin");
		let customer = account_key("customer");
		let lab = account_key("lab");

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(customer),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(customer)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(lab),
			LabInfo {
				box_public_key: Keccak256::hash("box_public_key".as_bytes()),
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
			}
		));

		LabVerifierKey::<Test>::put(admin);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(admin),
			lab,
			VerificationStatus::Verified
		));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio service name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio.clone()],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio service category".as_bytes().to_vec(),
				description: "DeBio service description".as_bytes().to_vec(),
				dna_collection_process: "DeBio service dna_collection_process".as_bytes().to_vec(),
				test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
				long_description: Some("DeBio service long_description".as_bytes().to_vec()),
				image: Some("DeBio service image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio service name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio service category".as_bytes().to_vec(),
				description: "DeBio service description".as_bytes().to_vec(),
				dna_collection_process: "DeBio service dna_collection_process".as_bytes().to_vec(),
				test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
				long_description: Some("DeBio service long_description".as_bytes().to_vec()),
				image: Some("DeBio service image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(lab).unwrap();
		let service_id = _lab.services[0];
		let other_service_id = _lab.services[1];

		assert_ok!(ServiceRequest::claim_request(Origin::signed(lab), request_id, service_id,));

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			other_service_id,
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let order_id = Orders::last_order_by_customer_id(customer).unwrap();

		assert_noop!(
			ServiceRequest::process_request(Origin::signed(customer), request_id, order_id),
			Error::<Test>::RequestUnableToProccess
		);
	})
}

#[test]
fn cant_process_request_when_request_is_on_processed_or_finalized() {
	<ExternalityBuilder>::default().existential_deposit(0).build().execute_with(|| {
		let customer = account_key("customer");
		let admin = account_key("admin");
		let lab = account_key("lab");
		let pallet_id = account_key("pallet_id");

		PalletAccount::<Test>::put(pallet_id);

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(customer),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(customer)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(lab),
			LabInfo {
				box_public_key: Keccak256::hash("box_public_key".as_bytes()),
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
			}
		));

		LabVerifierKey::<Test>::put(admin);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(admin),
			lab,
			VerificationStatus::Verified
		));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio service name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio service category".as_bytes().to_vec(),
				description: "DeBio service description".as_bytes().to_vec(),
				dna_collection_process: "DeBio service dna_collection_process".as_bytes().to_vec(),
				test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
				long_description: Some("DeBio service long_description".as_bytes().to_vec()),
				image: Some("DeBio service image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(lab).unwrap();
		let service_id = _lab.services[0];

		assert_ok!(ServiceRequest::claim_request(Origin::signed(lab), request_id, service_id,));

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			service_id,
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let order_id = Orders::last_order_by_customer_id(customer).unwrap();

		assert_ok!(
			ServiceRequest::process_request(Origin::signed(customer), request_id, order_id,)
		);

		assert_noop!(
			ServiceRequest::process_request(Origin::signed(customer), request_id, order_id,),
			Error::<Test>::RequestUnableToProccess
		);

		let dna_sample = GeneticTesting::dna_samples_by_lab_id(lab).unwrap();

		assert_ok!(Orders::set_order_paid(Origin::signed(customer), order_id));

		assert_eq!(Balances::free_balance(customer), 180);

		assert_ok!(GeneticTesting::submit_test_result(
			Origin::signed(lab),
			dna_sample[0].clone(),
			DnaTestResultSubmission {
				comments: Some("comment".as_bytes().to_vec()),
				result_link: Some("result_link".as_bytes().to_vec()),
				report_link: Some("report_link".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticTesting::process_dna_sample(
			Origin::signed(lab),
			dna_sample[0].clone(),
			DnaSampleStatus::ResultReady,
		));

		EscrowKey::<Test>::put(admin);

		assert_ok!(Orders::fulfill_order(Origin::signed(lab), order_id));

		assert_ok!(ServiceRequest::finalize_request(Origin::signed(lab), request_id));

		assert_noop!(
			ServiceRequest::process_request(Origin::signed(customer), request_id, order_id,),
			Error::<Test>::RequestUnableToProccess
		);
	})
}

#[test]
fn cant_retrieve_unstake_when_not_unauthorized() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let customer = account_key("customer");
		let admin = account_key("admin");
		let lab = account_key("lab");

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(customer),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(customer)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(lab),
			LabInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()
				),
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
			}
		));

		assert_noop!(
			ServiceRequest::retrieve_unstaked_amount(Origin::signed(admin), request_id),
			Error::<Test>::Unauthorized
		);
	})
}

#[test]
fn cant_retrieve_unstake_when_not_unstaked() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let customer = account_key("customer");
		let lab = account_key("lab");

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(customer),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(customer)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(lab),
			LabInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()
				),
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
			}
		));

		assert_noop!(
			ServiceRequest::retrieve_unstaked_amount(Origin::signed(customer), request_id,),
			Error::<Test>::RequestUnableToRetrieveUnstake
		);
	})
}

#[test]
fn cant_retrieve_unstake_when_unstaked_periode_not_fulfilled() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		let customer = account_key("customer");

		assert_ok!(ServiceRequest::create_request(
			Origin::signed(customer),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(customer)[0];

		assert_ok!(ServiceRequest::unstake(Origin::signed(customer), request_id));

		assert_noop!(
			ServiceRequest::retrieve_unstaked_amount(Origin::signed(customer), request_id),
			Error::<Test>::RequestWaitingForUnstaked,
		);
	})
}

#[test]
fn cant_finalize_requst_when_request_is_not_on_processed() {
	<ExternalityBuilder>::default().existential_deposit(0).build().execute_with(|| {
		let customer = account_key("customer");
		let admin = account_key("admin");
		let lab = account_key("lab");
		let pallet_id = account_key("pallet_id");

		PalletAccount::<Test>::put(pallet_id);

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(customer),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(customer)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(lab),
			LabInfo {
				box_public_key: Keccak256::hash("box_public_key".as_bytes()),
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
			}
		));

		LabVerifierKey::<Test>::put(admin);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(admin),
			lab,
			VerificationStatus::Verified
		));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio service name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio service category".as_bytes().to_vec(),
				description: "DeBio service description".as_bytes().to_vec(),
				dna_collection_process: "DeBio service dna_collection_process".as_bytes().to_vec(),
				test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
				long_description: Some("DeBio service long_description".as_bytes().to_vec()),
				image: Some("DeBio service image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(lab).unwrap();
		let service_id = _lab.services[0];

		assert_ok!(ServiceRequest::claim_request(Origin::signed(lab), request_id, service_id,));

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			service_id,
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let order_id = Orders::last_order_by_customer_id(customer).unwrap();

		assert_ok!(
			ServiceRequest::process_request(Origin::signed(customer), request_id, order_id,)
		);

		let dna_sample = GeneticTesting::dna_samples_by_lab_id(lab).unwrap();

		assert_ok!(Orders::set_order_paid(Origin::signed(customer), order_id));

		assert_eq!(Balances::free_balance(customer), 180);

		assert_ok!(GeneticTesting::submit_test_result(
			Origin::signed(lab),
			dna_sample[0].clone(),
			DnaTestResultSubmission {
				comments: Some("comment".as_bytes().to_vec()),
				result_link: Some("result_link".as_bytes().to_vec()),
				report_link: Some("report_link".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticTesting::process_dna_sample(
			Origin::signed(lab),
			dna_sample[0].clone(),
			DnaSampleStatus::ResultReady,
		));

		EscrowKey::<Test>::put(admin);

		assert_ok!(Orders::fulfill_order(Origin::signed(lab), order_id));

		assert_ok!(ServiceRequest::finalize_request(Origin::signed(lab), request_id));

		assert_noop!(
			ServiceRequest::finalize_request(Origin::signed(lab), request_id),
			Error::<Test>::RequestUnableToFinalize
		);
	})
}

#[test]
fn cant_finalize_request_when_order_not_fullfilled() {
	<ExternalityBuilder>::default().existential_deposit(0).build().execute_with(|| {
		let admin = account_key("admin");
		let customer = account_key("customer");
		let lab = account_key("lab");
		let pallet_id = account_key("pallet_id");

		PalletAccount::<Test>::put(pallet_id);

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(customer),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(customer)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(lab),
			LabInfo {
				box_public_key: Keccak256::hash("box_public_key".as_bytes()),
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
			}
		));

		LabVerifierKey::<Test>::put(admin);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(admin),
			lab,
			VerificationStatus::Verified
		));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio service name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio service category".as_bytes().to_vec(),
				description: "DeBio service description".as_bytes().to_vec(),
				dna_collection_process: "DeBio service dna_collection_process".as_bytes().to_vec(),
				test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
				long_description: Some("DeBio service long_description".as_bytes().to_vec()),
				image: Some("DeBio service image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(lab).unwrap();
		let service_id = _lab.services[0];

		assert_ok!(ServiceRequest::claim_request(Origin::signed(lab), request_id, service_id,));

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			service_id,
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let order_id = Orders::last_order_by_customer_id(customer).unwrap();

		assert_ok!(
			ServiceRequest::process_request(Origin::signed(customer), request_id, order_id,)
		);

		let dna_sample = GeneticTesting::dna_samples_by_lab_id(lab).unwrap();

		assert_ok!(Orders::set_order_paid(Origin::signed(customer), order_id));

		assert_eq!(Balances::free_balance(customer), 180);

		assert_ok!(GeneticTesting::submit_test_result(
			Origin::signed(lab),
			dna_sample[0].clone(),
			DnaTestResultSubmission {
				comments: Some("comment".as_bytes().to_vec()),
				result_link: Some("result_link".as_bytes().to_vec()),
				report_link: Some("report_link".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticTesting::process_dna_sample(
			Origin::signed(lab),
			dna_sample[0].clone(),
			DnaSampleStatus::ResultReady,
		));

		EscrowKey::<Test>::put(admin);

		assert_noop!(
			ServiceRequest::finalize_request(Origin::signed(lab), request_id),
			Error::<Test>::RequestUnableToFinalize,
		);
	})
}

#[test]
fn call_event_should_work() {
	<ExternalityBuilder>::default().existential_deposit(0).build().execute_with(|| {
		let customer = account_key("customer");
		let admin = account_key("admin");
		let lab = account_key("lab");
		let pallet_id = account_key("pallet_id");

		PalletAccount::<Test>::put(pallet_id);

		System::set_block_number(1);

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(customer),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(customer)[0];

		System::assert_last_event(Event::ServiceRequest(crate::Event::ServiceRequestCreated(
			customer,
			Request {
				hash: request_id,
				requester_address: customer,
				lab_address: None,
				service_id: None,
				order_id: None,
				country: String::from("Indonesia").into_bytes(),
				region: String::from("West Java").into_bytes(),
				city: String::from("Bogor").into_bytes(),
				service_category: String::from("Vaksin").into_bytes(),
				staking_amount: 10,
				status: RequestStatus::Open,
				created_at: 0,
				updated_at: None,
				unstaked_at: None,
			},
		)));

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(lab),
			LabInfo {
				box_public_key: Keccak256::hash("box_public_key".as_bytes()),
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
			}
		));

		LabVerifierKey::<Test>::put(admin);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(admin),
			lab,
			VerificationStatus::Verified
		));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio service name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio service category".as_bytes().to_vec(),
				description: "DeBio service description".as_bytes().to_vec(),
				dna_collection_process: "DeBio service dna_collection_process".as_bytes().to_vec(),
				test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
				long_description: Some("DeBio service long_description".as_bytes().to_vec()),
				image: Some("DeBio service image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(lab).unwrap();
		let service_id = _lab.services[0];

		assert_ok!(ServiceRequest::claim_request(Origin::signed(lab), request_id, service_id,));

		System::assert_last_event(Event::ServiceRequest(crate::Event::ServiceRequestUpdated(
			lab,
			request_id,
			RequestStatus::Claimed,
		)));

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			service_id,
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let order_id = Orders::last_order_by_customer_id(customer).unwrap();

		assert_ok!(
			ServiceRequest::process_request(Origin::signed(customer), request_id, order_id,)
		);

		System::assert_last_event(Event::ServiceRequest(crate::Event::ServiceRequestUpdated(
			customer,
			request_id,
			RequestStatus::Processed,
		)));

		let dna_sample = GeneticTesting::dna_samples_by_lab_id(lab).unwrap();

		assert_ok!(Orders::set_order_paid(Origin::signed(customer), order_id));

		assert_eq!(Balances::free_balance(customer), 180);

		assert_ok!(GeneticTesting::submit_test_result(
			Origin::signed(lab),
			dna_sample[0].clone(),
			DnaTestResultSubmission {
				comments: Some("comment".as_bytes().to_vec()),
				result_link: Some("result_link".as_bytes().to_vec()),
				report_link: Some("report_link".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticTesting::process_dna_sample(
			Origin::signed(lab),
			dna_sample[0].clone(),
			DnaSampleStatus::ResultReady,
		));

		EscrowKey::<Test>::put(admin);

		assert_ok!(Orders::fulfill_order(Origin::signed(lab), order_id));

		assert_ok!(ServiceRequest::finalize_request(Origin::signed(lab), request_id));

		System::assert_last_event(Event::ServiceRequest(crate::Event::ServiceRequestUpdated(
			lab,
			request_id,
			RequestStatus::Finalized,
		)));
	})
}

#[test]
fn update_admin_key_works() {
	<ExternalityBuilder>::default().existential_deposit(0).build().execute_with(|| {
		let admin = account_key("admin");
		let other_admin = account_key("other_admin");

		AdminKey::<Test>::put(admin);

		assert_eq!(ServiceRequest::admin_key(), Some(admin));

		assert_ok!(ServiceRequest::update_admin_key(Origin::root(), other_admin));

		assert_eq!(ServiceRequest::admin_key(), Some(other_admin));
	})
}
