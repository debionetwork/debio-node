use crate::{mock::*, AdminKey, Error, Request, RequestStatus, ServiceInvoice, ServiceOffer};
use frame_support::{
	assert_noop, assert_ok,
	sp_runtime::traits::{Hash, Keccak256},
};
use frame_system::RawOrigin;
use labs::{LabInfo, LabVerifierKey};
use primitives_area_code::{CityCode, CountryCode, RegionCode};
use primitives_verification_status::VerificationStatus;

#[test]
fn create_request_works() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(1),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let hash = ServiceRequest::request_by_account_id(1)[0];

		assert_eq!(
			ServiceRequest::request_by_id(hash),
			Some(Request {
				hash,
				requester_address: 1,
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
			1
		);

		assert_eq!(Balances::free_balance(1), 90);
	})
}

#[test]
fn claim_request_works_when_lab_is_verified() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(1),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(1)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(2),
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

		LabVerifierKey::<Test>::put(3);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(3),
			2,
			VerificationStatus::Verified
		));

		assert_ok!(ServiceRequest::claim_request(
			Origin::signed(2),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			1,
			1
		));

		assert_eq!(
			ServiceRequest::request_by_id(request_id),
			Some(Request {
				hash: request_id,
				requester_address: 1,
				lab_address: Some(2),
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
				lab_address: 2,
				service_id: Keccak256::hash("service_id".as_bytes()),
				testing_price: 1,
				qc_price: 1
			})
		);
	})
}

#[test]
fn claim_request_works_when_lab_is_unverified() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(1),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			1
		));

		let request_id = ServiceRequest::request_by_account_id(1)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(2),
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
			Origin::signed(2),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			1,
			1
		));

		assert_eq!(
			ServiceRequest::request_by_id(request_id),
			Some(Request {
				hash: request_id,
				requester_address: 1,
				lab_address: None,
				country: String::from("Indonesia").into_bytes(),
				region: String::from("West Java").into_bytes(),
				city: String::from("Bogor").into_bytes(),
				service_category: String::from("Vaksin").into_bytes(),
				staking_amount: 1,
				status: RequestStatus::Open,
				created_at: 0,
				updated_at: None,
				unstaked_at: None,
			})
		);

		assert_eq!(ServiceRequest::service_offer_by_id(request_id), None);

		assert_eq!(ServiceRequest::requests_by_lab_id(2), vec![request_id])
	})
}

#[test]
fn process_request_works_when_stacking_amount_greater_than_total_payment() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(1),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(1)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(2),
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

		LabVerifierKey::<Test>::put(3);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(3),
			2,
			VerificationStatus::Verified
		));

		assert_ok!(ServiceRequest::claim_request(
			Origin::signed(2),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			1,
			1
		));

		assert_ok!(ServiceRequest::process_request(
			Origin::signed(1),
			2,
			request_id,
			Keccak256::hash("order_id".as_bytes()),
			String::from("dnasample").into_bytes(),
			0
		));

		assert_eq!(
			ServiceRequest::request_by_id(request_id),
			Some(Request {
				hash: request_id,
				requester_address: 1,
				lab_address: Some(2),
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
				customer_address: 1,
				seller_address: 2,
				dna_sample_tracking_id: String::from("dnasample").into_bytes(),
				testing_price: 1,
				qc_price: 1,
				pay_amount: 2
			})
		);

		assert_eq!(
			ServiceRequest::service_invoice_by_order_id(Keccak256::hash("order_id".as_bytes())),
			Some(ServiceInvoice {
				request_hash: request_id,
				order_id: Keccak256::hash("order_id".as_bytes()),
				service_id: Keccak256::hash("service_id".as_bytes()),
				customer_address: 1,
				seller_address: 2,
				dna_sample_tracking_id: String::from("dnasample").into_bytes(),
				testing_price: 1,
				qc_price: 1,
				pay_amount: 2
			})
		);

		assert_eq!(Balances::free_balance(1), 98);
	})
}

#[test]
fn process_request_works_when_stacking_amount_less_than_total_payment() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(1),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			1
		));

		let request_id = ServiceRequest::request_by_account_id(1)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(2),
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

		LabVerifierKey::<Test>::put(3);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(3),
			2,
			VerificationStatus::Verified
		));

		assert_ok!(ServiceRequest::claim_request(
			Origin::signed(2),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			1,
			1
		));

		assert_ok!(ServiceRequest::process_request(
			Origin::signed(1),
			2,
			request_id,
			Keccak256::hash("order_id".as_bytes()),
			String::from("dnasample").into_bytes(),
			1
		));

		assert_eq!(
			ServiceRequest::request_by_id(request_id),
			Some(Request {
				hash: request_id,
				requester_address: 1,
				lab_address: Some(2),
				country: String::from("Indonesia").into_bytes(),
				region: String::from("West Java").into_bytes(),
				city: String::from("Bogor").into_bytes(),
				service_category: String::from("Vaksin").into_bytes(),
				staking_amount: 1,
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
				customer_address: 1,
				seller_address: 2,
				dna_sample_tracking_id: String::from("dnasample").into_bytes(),
				testing_price: 1,
				qc_price: 1,
				pay_amount: 2
			})
		);

		assert_eq!(
			ServiceRequest::service_invoice_by_order_id(Keccak256::hash("order_id".as_bytes())),
			Some(ServiceInvoice {
				request_hash: request_id,
				order_id: Keccak256::hash("order_id".as_bytes()),
				service_id: Keccak256::hash("service_id".as_bytes()),
				customer_address: 1,
				seller_address: 2,
				dna_sample_tracking_id: String::from("dnasample").into_bytes(),
				testing_price: 1,
				qc_price: 1,
				pay_amount: 2
			})
		);

		assert_eq!(Balances::free_balance(1), 98);
	})
}

#[test]
fn finalize_request_works_when_test_result_success() {
	<ExternalityBuilder>::default().existential_deposit(0).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 2, 100, 0));

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(1),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(1)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(2),
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

		LabVerifierKey::<Test>::put(3);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(3),
			2,
			VerificationStatus::Verified
		));

		assert_ok!(ServiceRequest::claim_request(
			Origin::signed(2),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			1,
			1
		));

		assert_ok!(ServiceRequest::process_request(
			Origin::signed(1),
			2,
			request_id,
			Keccak256::hash("order_id".as_bytes()),
			String::from("dnasample").into_bytes(),
			0
		));

		AdminKey::<Test>::put(3);

		assert_ok!(ServiceRequest::finalize_request(Origin::signed(3), request_id, true,));

		assert_eq!(
			ServiceRequest::request_by_id(request_id),
			Some(Request {
				hash: request_id,
				requester_address: 1,
				lab_address: Some(2),
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

		assert_eq!(Balances::free_balance(1), 98);

		assert_eq!(Balances::free_balance(2), 102);
	})
}

#[test]
fn finalize_request_works_when_test_result_not_success() {
	<ExternalityBuilder>::default().existential_deposit(0).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 2, 100, 0));

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(1),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(1)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(2),
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

		LabVerifierKey::<Test>::put(3);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(3),
			2,
			VerificationStatus::Verified
		));

		assert_ok!(ServiceRequest::claim_request(
			Origin::signed(2),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			1,
			1
		));

		assert_ok!(ServiceRequest::process_request(
			Origin::signed(1),
			2,
			request_id,
			Keccak256::hash("order_id".as_bytes()),
			String::from("dnasample").into_bytes(),
			0
		));

		AdminKey::<Test>::put(3);

		assert_ok!(ServiceRequest::finalize_request(Origin::signed(3), request_id, false,));

		assert_eq!(
			ServiceRequest::request_by_id(request_id),
			Some(Request {
				hash: request_id,
				requester_address: 1,
				lab_address: Some(2),
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

		assert_eq!(Balances::free_balance(1), 99);

		assert_eq!(Balances::free_balance(2), 101);
	})
}

#[test]
fn cant_create_request_when_stacking_amount_zero() {
	<ExternalityBuilder>::default().existential_deposit(0).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_noop!(
			ServiceRequest::create_request(
				Origin::signed(1),
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
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 1, 0));
		assert_noop!(
			ServiceRequest::create_request(
				Origin::signed(1),
				String::from("Indonesia").into_bytes(),
				String::from("West Java").into_bytes(),
				String::from("Bogor").into_bytes(),
				String::from("Vaksin").into_bytes(),
				10
			),
			Error::<Test>::BadSignature
		);
	})
}

#[test]
fn cant_claim_and_process_request_when_not_exists() {
	<ExternalityBuilder>::default().existential_deposit(0).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 1, 0));
		assert_noop!(
			ServiceRequest::claim_request(
				Origin::signed(1),
				Keccak256::hash("request_id".as_bytes()),
				Keccak256::hash("service_id".as_bytes()),
				1,
				1
			),
			Error::<Test>::RequestNotFound
		);

		assert_noop!(
			ServiceRequest::process_request(
				Origin::signed(1),
				2,
				Keccak256::hash("request_id".as_bytes()),
				Keccak256::hash("order_id".as_bytes()),
				String::from("dna_sample").into_bytes(),
				0
			),
			Error::<Test>::RequestNotFound
		);
	})
}

#[test]
fn cant_claim_request_when_already_claimed() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(1),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(1)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(2),
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

		LabVerifierKey::<Test>::put(3);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(3),
			2,
			VerificationStatus::Verified
		));

		assert_ok!(ServiceRequest::claim_request(
			Origin::signed(2),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			1,
			1
		));

		assert_noop!(
			ServiceRequest::claim_request(
				Origin::signed(3),
				request_id,
				Keccak256::hash("service_id".as_bytes()),
				1,
				1
			),
			Error::<Test>::RequestAlreadyClaimed
		);
	})
}

#[test]
fn cant_claim_process_request_when_lab_not_exists() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(1),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(1)[0];

		assert_noop!(
			ServiceRequest::claim_request(
				Origin::signed(3),
				request_id,
				Keccak256::hash("service_id".as_bytes()),
				1,
				1
			),
			Error::<Test>::LabNotFound
		);

		assert_noop!(
			ServiceRequest::process_request(
				Origin::signed(1),
				4,
				request_id,
				Keccak256::hash("order_id".as_bytes()),
				String::from("dna_sample").into_bytes(),
				0
			),
			Error::<Test>::LabNotFound
		);

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(3),
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
		assert_ok!(Labs::register_lab(
			Origin::signed(4),
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

		LabVerifierKey::<Test>::put(2);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(2),
			3,
			VerificationStatus::Verified
		));

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(2),
			4,
			VerificationStatus::Verified
		));

		assert_ok!(ServiceRequest::claim_request(
			Origin::signed(3),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			1,
			1
		));

		assert_noop!(
			ServiceRequest::process_request(
				Origin::signed(1),
				4,
				request_id,
				Keccak256::hash("order_id".as_bytes()),
				String::from("dna_sample").into_bytes(),
				0
			),
			Error::<Test>::LabNotFound
		);
	})
}

#[test]
fn cant_put_in_claim_list_when_already_exists() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(1),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			1
		));

		let request_id = ServiceRequest::request_by_account_id(1)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(2),
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
			Origin::signed(2),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			1,
			1
		));

		assert_noop!(
			ServiceRequest::claim_request(
				Origin::signed(2),
				request_id,
				Keccak256::hash("service_id".as_bytes()),
				1,
				1
			),
			Error::<Test>::RequestAlreadyInList
		);
	})
}

#[test]
fn cant_process_request_when_unathorized_customer() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(1),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(1)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(2),
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

		LabVerifierKey::<Test>::put(3);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(3),
			2,
			VerificationStatus::Verified
		));

		assert_ok!(ServiceRequest::claim_request(
			Origin::signed(2),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			1,
			1
		));

		assert_noop!(
			ServiceRequest::process_request(
				Origin::signed(4),
				2,
				request_id,
				Keccak256::hash("order_id".as_bytes()),
				String::from("dnasample").into_bytes(),
				0
			),
			Error::<Test>::Unauthorized
		);
	})
}

#[test]
fn can_process_request_when_already_unstaked() {
	<ExternalityBuilder>::default().existential_deposit(2).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(1),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(1)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(2),
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

		assert_ok!(ServiceRequest::unstake(Origin::signed(1), request_id,));

		assert_noop!(
			ServiceRequest::process_request(
				Origin::signed(1),
				2,
				request_id,
				Keccak256::hash("order_id".as_bytes()),
				String::from("dna_sample").into_bytes(),
				0
			),
			Error::<Test>::RequestAlreadyUnstaked
		);
	})
}

#[test]
fn cant_process_request_when_request_is_on_processed_or_finalized() {
	<ExternalityBuilder>::default().existential_deposit(0).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 2, 100, 0));

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(1),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(1)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(2),
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

		LabVerifierKey::<Test>::put(3);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(3),
			2,
			VerificationStatus::Verified
		));

		assert_ok!(ServiceRequest::claim_request(
			Origin::signed(2),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			1,
			1
		));

		assert_ok!(ServiceRequest::process_request(
			Origin::signed(1),
			2,
			request_id,
			Keccak256::hash("order_id".as_bytes()),
			String::from("dnasample").into_bytes(),
			0
		));

		assert_noop!(
			ServiceRequest::process_request(
				Origin::signed(1),
				2,
				request_id,
				Keccak256::hash("order_id".as_bytes()),
				String::from("dnasample").into_bytes(),
				0
			),
			Error::<Test>::RequestUnableToProccess
		);

		AdminKey::<Test>::put(3);

		assert_ok!(ServiceRequest::finalize_request(Origin::signed(3), request_id, true,));

		assert_noop!(
			ServiceRequest::process_request(
				Origin::signed(1),
				2,
				request_id,
				Keccak256::hash("order_id".as_bytes()),
				String::from("dnasample").into_bytes(),
				0
			),
			Error::<Test>::RequestUnableToProccess
		);
	})
}

#[test]
fn cant_finalize_request_when_unauthorized() {
	<ExternalityBuilder>::default().existential_deposit(0).build().execute_with(|| {
		assert_noop!(
			ServiceRequest::finalize_request(
				Origin::signed(1),
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
		AdminKey::<Test>::put(1);
		assert_noop!(
			ServiceRequest::finalize_request(
				Origin::signed(1),
				Keccak256::hash("request_id".as_bytes()),
				true
			),
			Error::<Test>::ServiceInvoiceNotFound
		);
	})
}

#[test]
fn cant_finalize_requst_when_request_is_not_on_processed() {
	<ExternalityBuilder>::default().existential_deposit(0).build().execute_with(|| {
		AdminKey::<Test>::put(3);
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 2, 100, 0));

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(1),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(1)[0];

		// Register lab
		assert_ok!(Labs::register_lab(
			Origin::signed(2),
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

		LabVerifierKey::<Test>::put(3);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(3),
			2,
			VerificationStatus::Verified
		));

		assert_ok!(ServiceRequest::claim_request(
			Origin::signed(2),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			1,
			1
		));

		assert_ok!(ServiceRequest::process_request(
			Origin::signed(1),
			2,
			request_id,
			Keccak256::hash("order_id".as_bytes()),
			String::from("dnasample").into_bytes(),
			0
		));

		AdminKey::<Test>::put(3);

		assert_ok!(ServiceRequest::finalize_request(Origin::signed(3), request_id, true,));

		assert_noop!(
			ServiceRequest::finalize_request(Origin::signed(3), request_id, true,),
			Error::<Test>::RequestUnableToFinalize
		);
	})
}

#[test]
fn call_event_should_work() {
	<ExternalityBuilder>::default().existential_deposit(0).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 2, 100, 0));
		System::set_block_number(1);

		// Customer create request
		assert_ok!(ServiceRequest::create_request(
			Origin::signed(1),
			String::from("Indonesia").into_bytes(),
			String::from("West Java").into_bytes(),
			String::from("Bogor").into_bytes(),
			String::from("Vaksin").into_bytes(),
			10
		));

		let request_id = ServiceRequest::request_by_account_id(1)[0];

		System::assert_last_event(Event::ServiceRequest(crate::Event::ServiceRequestCreated(
			1,
			Request {
				hash: request_id,
				requester_address: 1,
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
			Origin::signed(2),
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
			Origin::signed(2),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			1,
			1
		));

		System::assert_last_event(Event::ServiceRequest(
			crate::Event::ServiceRequestWaitingForClaimed(
				2,
				ServiceOffer {
					request_hash: request_id,
					lab_address: 2,
					service_id: Keccak256::hash("service_id".as_bytes()),
					testing_price: 1,
					qc_price: 1,
				},
			),
		));

		LabVerifierKey::<Test>::put(3);

		assert_ok!(Labs::update_lab_verification_status(
			Origin::signed(3),
			2,
			VerificationStatus::Verified
		));

		assert_ok!(ServiceRequest::claim_request(
			Origin::signed(2),
			request_id,
			Keccak256::hash("service_id".as_bytes()),
			1,
			1
		));

		System::assert_last_event(Event::ServiceRequest(crate::Event::ServiceRequestClaimed(
			2,
			ServiceOffer {
				request_hash: request_id,
				lab_address: 2,
				service_id: Keccak256::hash("service_id".as_bytes()),
				testing_price: 1,
				qc_price: 1,
			},
		)));

		assert_ok!(ServiceRequest::process_request(
			Origin::signed(1),
			2,
			request_id,
			Keccak256::hash("order_id".as_bytes()),
			String::from("dnasample").into_bytes(),
			0
		));

		System::assert_last_event(Event::ServiceRequest(crate::Event::ServiceRequestProcessed(
			1,
			ServiceInvoice {
				request_hash: request_id,
				order_id: Keccak256::hash("order_id".as_bytes()),
				service_id: Keccak256::hash("service_id".as_bytes()),
				customer_address: 1,
				seller_address: 2,
				dna_sample_tracking_id: String::from("dnasample").into_bytes(),
				testing_price: 1,
				qc_price: 1,
				pay_amount: 2,
			},
		)));

		AdminKey::<Test>::put(3);

		assert_ok!(ServiceRequest::finalize_request(Origin::signed(3), request_id, true,));

		System::assert_last_event(Event::ServiceRequest(crate::Event::ServiceRequestFinalized(
			3,
			ServiceInvoice {
				request_hash: request_id,
				order_id: Keccak256::hash("order_id".as_bytes()),
				service_id: Keccak256::hash("service_id".as_bytes()),
				customer_address: 1,
				seller_address: 2,
				dna_sample_tracking_id: String::from("dnasample").into_bytes(),
				testing_price: 1,
				qc_price: 1,
				pay_amount: 2,
			},
		)));
	})
}

#[test]
fn update_admin_key_works() {
	<ExternalityBuilder>::default().existential_deposit(0).build().execute_with(|| {
		AdminKey::<Test>::put(2);

		assert_eq!(ServiceRequest::admin_key(), 2);

		assert_ok!(ServiceRequest::update_admin_key(Origin::signed(2), 1,));

		assert_eq!(ServiceRequest::admin_key(), 1);
	})
}
