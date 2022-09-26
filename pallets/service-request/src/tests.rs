use crate::{
	mock::*, AdminKey, Error, Request, RequestStatus, ServiceInvoice, ServiceOffer, ServicePrice,
};
use frame_support::{
	assert_noop, assert_ok,
	sp_runtime::traits::{Hash, Keccak256},
};
use labs::{LabInfo, LabVerifierKey};
use primitives_area_code::{CityCode, CountryCode, RegionCode};
use primitives_verification_status::VerificationStatus;

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

		let hash = ServiceRequest::request_by_account_id(customer)[0];

		assert_eq!(
			ServiceRequest::request_by_id(hash),
			Some(Request {
				hash,
				requester_address: customer,
				lab_address: None,
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

		assert_eq!(Balances::free_balance(customer), 190);
	})
}

#[test]
fn claim_request_works_when_lab_is_verified() {
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

		assert_ok!(ServiceRequest::claim_request(
			Origin::signed(lab),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			ServicePrice::new(b"1", 10, 10),
		));

		assert_eq!(
			ServiceRequest::request_by_id(request_id),
			Some(Request {
				hash: request_id,
				requester_address: customer,
				lab_address: Some(lab),
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

		assert_eq!(
			ServiceRequest::service_offer_by_id(request_id),
			Some(ServiceOffer {
				request_hash: request_id,
				lab_address: lab,
				service_id: Keccak256::hash("service_id".as_bytes()),
				service_price: ServicePrice::new(b"1", 10, 10),
			})
		);
	})
}

#[test]
fn claim_request_works_when_lab_is_unverified() {
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

		assert_ok!(ServiceRequest::claim_request(
			Origin::signed(lab),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			ServicePrice::new(b"1", 10, 10),
		));

		assert_eq!(
			ServiceRequest::request_by_id(request_id),
			Some(Request {
				hash: request_id,
				requester_address: customer,
				lab_address: None,
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

		assert_eq!(ServiceRequest::service_offer_by_id(request_id), None);

		assert_eq!(ServiceRequest::requests_by_lab_id(lab), vec![request_id])
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

		assert_ok!(ServiceRequest::claim_request(
			Origin::signed(lab),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			ServicePrice::new(b"1", 10, 10),
		));

		assert_ok!(ServiceRequest::process_request(
			Origin::signed(customer),
			lab,
			request_id,
			Keccak256::hash("order_id".as_bytes()),
			String::from("dnasample").into_bytes(),
		));

		assert_eq!(
			ServiceRequest::request_by_id(request_id),
			Some(Request {
				hash: request_id,
				requester_address: customer,
				lab_address: Some(lab),
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

		assert_eq!(
			ServiceRequest::service_invoice_by_id(request_id),
			Some(ServiceInvoice {
				request_hash: request_id,
				order_id: Keccak256::hash("order_id".as_bytes()),
				service_id: Keccak256::hash("service_id".as_bytes()),
				customer_address: customer,
				seller_address: lab,
				dna_sample_tracking_id: String::from("dnasample").into_bytes(),
				service_price: ServicePrice::new(b"1", 10, 10),
			})
		);

		assert_eq!(
			ServiceRequest::service_invoice_by_order_id(Keccak256::hash("order_id".as_bytes())),
			Some(ServiceInvoice {
				request_hash: request_id,
				order_id: Keccak256::hash("order_id".as_bytes()),
				service_id: Keccak256::hash("service_id".as_bytes()),
				customer_address: customer,
				seller_address: lab,
				dna_sample_tracking_id: String::from("dnasample").into_bytes(),
				service_price: ServicePrice::new(b"1", 10, 10),
			})
		);

		assert_eq!(Assets::balance(1, customer), 180);
	})
}

#[test]
fn finalize_request_works_when_test_result_success() {
	<ExternalityBuilder>::default().existential_deposit(0).build().execute_with(|| {
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

		assert_ok!(ServiceRequest::claim_request(
			Origin::signed(lab),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			ServicePrice::new(b"1", 10, 10),
		));

		assert_ok!(ServiceRequest::process_request(
			Origin::signed(customer),
			lab,
			request_id,
			Keccak256::hash("order_id".as_bytes()),
			String::from("dnasample").into_bytes(),
		));

		AdminKey::<Test>::put(admin);

		assert_ok!(ServiceRequest::finalize_request(Origin::signed(admin), request_id, true,));

		assert_eq!(
			ServiceRequest::request_by_id(request_id),
			Some(Request {
				hash: request_id,
				requester_address: customer,
				lab_address: Some(lab),
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

		assert_eq!(Balances::free_balance(customer), 200);
		assert_eq!(Assets::balance(1, customer), 180);
		assert_eq!(Assets::balance(1, lab), 320);
	})
}

#[test]
fn finalize_request_works_when_test_result_not_success() {
	<ExternalityBuilder>::default().existential_deposit(0).build().execute_with(|| {
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

		assert_ok!(ServiceRequest::claim_request(
			Origin::signed(lab),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			ServicePrice::new(b"1", 10, 10),
		));

		assert_ok!(ServiceRequest::process_request(
			Origin::signed(customer),
			lab,
			request_id,
			Keccak256::hash("order_id".as_bytes()),
			String::from("dnasample").into_bytes(),
		));

		AdminKey::<Test>::put(admin);

		assert_ok!(ServiceRequest::finalize_request(Origin::signed(admin), request_id, false));

		assert_eq!(
			ServiceRequest::request_by_id(request_id),
			Some(Request {
				hash: request_id,
				requester_address: customer,
				lab_address: Some(lab),
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

		assert_eq!(Balances::free_balance(customer), 200);
		assert_eq!(Assets::balance(1, customer), 190);
		assert_eq!(Assets::balance(1, lab), 310);
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
				ServicePrice::new(b"1", 10, 10),
			),
			Error::<Test>::RequestNotFound
		);

		assert_noop!(
			ServiceRequest::process_request(
				Origin::signed(customer),
				lab,
				Keccak256::hash("request_id".as_bytes()),
				Keccak256::hash("order_id".as_bytes()),
				String::from("dna_sample").into_bytes(),
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
				false
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
				ServicePrice::new(b"1", 10, 10)
			),
			Error::<Test>::RequestAlreadyUnstaked
		);

		assert_noop!(
			ServiceRequest::process_request(
				Origin::signed(customer),
				lab,
				request_id,
				Keccak256::hash("order_id".as_bytes()),
				String::from("dnasample").into_bytes(),
			),
			Error::<Test>::RequestAlreadyUnstaked
		);
	})
}

#[test]
fn claim_request_works_when_asset_not_exists() {
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
				ServicePrice::new(b"2", 10, 10),
			),
			Error::<Test>::AssetNotExists
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

		assert_ok!(ServiceRequest::claim_request(
			Origin::signed(lab),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			ServicePrice::new(b"1", 10, 10),
		));

		assert_noop!(
			ServiceRequest::claim_request(
				Origin::signed(lab),
				request_id,
				Keccak256::hash("service_id".as_bytes()),
				ServicePrice::new(b"1", 10, 10)
			),
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

		assert_ok!(ServiceRequest::claim_request(
			Origin::signed(lab),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			ServicePrice::new(b"1", 10, 10),
		));

		assert_ok!(ServiceRequest::process_request(
			Origin::signed(customer),
			lab,
			request_id,
			Keccak256::hash("order_id".as_bytes()),
			String::from("dnasample").into_bytes(),
		));

		assert_noop!(
			ServiceRequest::claim_request(
				Origin::signed(lab),
				request_id,
				Keccak256::hash("service_id".as_bytes()),
				ServicePrice::new(b"1", 10, 10)
			),
			Error::<Test>::RequestUnableToClaimed
		);

		AdminKey::<Test>::put(admin);

		assert_ok!(ServiceRequest::finalize_request(Origin::signed(admin), request_id, true,));

		assert_noop!(
			ServiceRequest::claim_request(
				Origin::signed(lab),
				request_id,
				Keccak256::hash("service_id".as_bytes()),
				ServicePrice::new(b"1", 10, 10)
			),
			Error::<Test>::RequestUnableToClaimed
		);
	})
}

#[test]
fn cant_claim_and_process_request_when_lab_not_exists() {
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
				ServicePrice::new(b"1", 10, 10),
			),
			Error::<Test>::LabNotFound
		);

		assert_noop!(
			ServiceRequest::process_request(
				Origin::signed(customer),
				lab,
				request_id,
				Keccak256::hash("order_id".as_bytes()),
				String::from("dna_sample").into_bytes(),
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

		assert_ok!(ServiceRequest::claim_request(
			Origin::signed(lab),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			ServicePrice::new(b"1", 10, 10),
		));

		assert_noop!(
			ServiceRequest::process_request(
				Origin::signed(customer),
				other_lab,
				request_id,
				Keccak256::hash("order_id".as_bytes()),
				String::from("dna_sample").into_bytes(),
			),
			Error::<Test>::LabNotFound
		);
	})
}

#[test]
fn cant_put_in_claim_list_when_already_exists() {
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

		assert_ok!(ServiceRequest::claim_request(
			Origin::signed(lab),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			ServicePrice::new(b"1", 10, 10),
		));

		assert_noop!(
			ServiceRequest::claim_request(
				Origin::signed(lab),
				request_id,
				Keccak256::hash("service_id".as_bytes()),
				ServicePrice::new(b"1", 10, 10),
			),
			Error::<Test>::RequestAlreadyInList
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

		assert_ok!(ServiceRequest::claim_request(
			Origin::signed(lab),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			ServicePrice::new(b"1", 10, 10)
		));

		assert_noop!(
			ServiceRequest::process_request(
				Origin::signed(other_customer),
				lab,
				request_id,
				Keccak256::hash("order_id".as_bytes()),
				String::from("dnasample").into_bytes(),
			),
			Error::<Test>::Unauthorized
		);
	})
}

#[test]
fn cant_process_request_when_request_is_on_processed_or_finalized() {
	<ExternalityBuilder>::default().existential_deposit(0).build().execute_with(|| {
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

		assert_ok!(ServiceRequest::claim_request(
			Origin::signed(lab),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			ServicePrice::new(b"1", 10, 10)
		));

		assert_ok!(ServiceRequest::process_request(
			Origin::signed(customer),
			lab,
			request_id,
			Keccak256::hash("order_id".as_bytes()),
			String::from("dnasample").into_bytes(),
		));

		assert_noop!(
			ServiceRequest::process_request(
				Origin::signed(customer),
				lab,
				request_id,
				Keccak256::hash("order_id".as_bytes()),
				String::from("dnasample").into_bytes(),
			),
			Error::<Test>::RequestUnableToProccess
		);

		AdminKey::<Test>::put(admin);

		assert_ok!(ServiceRequest::finalize_request(Origin::signed(admin), request_id, true,));

		assert_noop!(
			ServiceRequest::process_request(
				Origin::signed(customer),
				lab,
				request_id,
				Keccak256::hash("order_id".as_bytes()),
				String::from("dnasample").into_bytes(),
			),
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

		AdminKey::<Test>::put(admin);

		assert_noop!(
			ServiceRequest::retrieve_unstaked_amount(Origin::signed(admin), request_id,),
			Error::<Test>::RequestUnableToRetrieveUnstake
		);
	})
}

#[test]
fn cant_finalize_request_when_unauthorized() {
	<ExternalityBuilder>::default().existential_deposit(0).build().execute_with(|| {
		let admin = account_key("admin");

		assert_noop!(
			ServiceRequest::finalize_request(
				Origin::signed(admin),
				Keccak256::hash("request_id".as_bytes()),
				true
			),
			Error::<Test>::Unauthorized
		);
	})
}

#[test]
fn cant_finalize_request_when_invoice_not_exist() {
	<ExternalityBuilder>::default().existential_deposit(0).build().execute_with(|| {
		let customer = account_key("customer");
		let admin = account_key("admin");

		AdminKey::<Test>::put(admin);

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
			ServiceRequest::finalize_request(Origin::signed(admin), request_id, true),
			Error::<Test>::ServiceInvoiceNotFound
		);
	})
}

#[test]
fn cant_finalize_requst_when_request_is_not_on_processed() {
	<ExternalityBuilder>::default().existential_deposit(0).build().execute_with(|| {
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

		assert_ok!(ServiceRequest::claim_request(
			Origin::signed(lab),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			ServicePrice::new(b"1", 10, 10)
		));

		assert_ok!(ServiceRequest::process_request(
			Origin::signed(customer),
			lab,
			request_id,
			Keccak256::hash("order_id".as_bytes()),
			String::from("dnasample").into_bytes(),
		));

		AdminKey::<Test>::put(admin);

		assert_ok!(ServiceRequest::finalize_request(Origin::signed(admin), request_id, true,));

		assert_noop!(
			ServiceRequest::finalize_request(Origin::signed(admin), request_id, true,),
			Error::<Test>::RequestUnableToFinalize
		);
	})
}

#[test]
fn call_event_should_work() {
	<ExternalityBuilder>::default().existential_deposit(0).build().execute_with(|| {
		let customer = account_key("customer");
		let admin = account_key("admin");
		let lab = account_key("lab");

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

		assert_ok!(ServiceRequest::claim_request(
			Origin::signed(lab),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			ServicePrice::new(b"1", 10, 10),
		));

		System::assert_last_event(Event::ServiceRequest(
			crate::Event::ServiceRequestWaitingForClaimed(
				lab,
				ServiceOffer {
					request_hash: request_id,
					lab_address: lab,
					service_id: Keccak256::hash("service_id".as_bytes()),
					service_price: ServicePrice::new(b"1", 10, 10),
				},
			),
		));

		LabVerifierKey::<Test>::put(admin);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(admin),
			lab,
			VerificationStatus::Verified
		));

		assert_ok!(ServiceRequest::claim_request(
			Origin::signed(lab),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			ServicePrice::new(b"1", 10, 10)
		));

		System::assert_last_event(Event::ServiceRequest(crate::Event::ServiceRequestClaimed(
			lab,
			ServiceOffer {
				request_hash: request_id,
				lab_address: lab,
				service_id: Keccak256::hash("service_id".as_bytes()),
				service_price: ServicePrice::new(b"1", 10, 10),
			},
		)));

		assert_ok!(ServiceRequest::process_request(
			Origin::signed(customer),
			lab,
			request_id,
			Keccak256::hash("order_id".as_bytes()),
			String::from("dnasample").into_bytes(),
		));

		System::assert_last_event(Event::ServiceRequest(crate::Event::ServiceRequestProcessed(
			customer,
			ServiceInvoice {
				request_hash: request_id,
				order_id: Keccak256::hash("order_id".as_bytes()),
				service_id: Keccak256::hash("service_id".as_bytes()),
				customer_address: customer,
				seller_address: lab,
				dna_sample_tracking_id: String::from("dnasample").into_bytes(),
				service_price: ServicePrice::new(b"1", 10, 10),
			},
		)));

		AdminKey::<Test>::put(admin);

		assert_ok!(ServiceRequest::finalize_request(Origin::signed(admin), request_id, true,));

		System::assert_last_event(Event::ServiceRequest(crate::Event::ServiceRequestFinalized(
			admin,
			ServiceInvoice {
				request_hash: request_id,
				order_id: Keccak256::hash("order_id".as_bytes()),
				service_id: Keccak256::hash("service_id".as_bytes()),
				customer_address: customer,
				seller_address: lab,
				dna_sample_tracking_id: String::from("dnasample").into_bytes(),
				service_price: ServicePrice::new(b"1", 10, 10),
			},
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
