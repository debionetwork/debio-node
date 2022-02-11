use crate::{mock::*, Error, EscrowKey, Order, OrderStatus};
use frame_support::{
	assert_noop, assert_ok,
	sp_runtime::traits::{Hash, Keccak256},
};
use frame_system::RawOrigin;
use genetic_testing::{DnaSampleStatus, DnaTestResultSubmission};
use labs::LabInfo;
use primitives_area_code::{CityCode, CountryCode, RegionCode};
use services::ServiceInfo;
use traits_services::types::{CurrencyType, ExpectedDuration, PriceByCurrency, ServiceFlow};

#[test]
fn create_order() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(Labs::register_lab(
			Origin::signed(1),
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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(1), EthereumAddress([b'X'; 20])));

		assert_ok!(Services::create_service(
			Origin::signed(1),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio category".as_bytes().to_vec(),
				description: "This is my description".as_bytes().to_vec(),
				test_result_sample: "Test result sample".as_bytes().to_vec(),
				dna_collection_process: "Dna Collection Process".as_bytes().to_vec(),
				long_description: Some("This is my long description".as_bytes().to_vec()),
				image: Some("This is my image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(1).unwrap();

		assert_ok!(Orders::create_order(
			Origin::signed(2),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService
		));

		let _order_id = Orders::last_order_by_customer_id(2).unwrap();
		let _dna_sample = GeneticTesting::dna_samples_by_lab_id(1).unwrap();

		assert_eq!(
			Orders::order_by_id(&_order_id),
			Some(Order {
				id: _order_id,
				service_id: _lab.services[0],
				customer_id: 2,
				customer_box_public_key: Keccak256::hash(
					"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()
				),
				seller_id: 1,
				dna_sample_tracking_id: _dna_sample[0].clone(),
				currency: CurrencyType::default(),
				prices: PriceByCurrency::default().price_components,
				additional_prices: PriceByCurrency::default().additional_prices,
				status: OrderStatus::default(),
				order_flow: ServiceFlow::StakingRequestService,
				created_at: 0,
				updated_at: 0
			})
		);
	})
}

#[test]
fn cancel_order_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(Labs::register_lab(
			Origin::signed(1),
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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(1), EthereumAddress([b'X'; 20])));

		assert_ok!(Services::create_service(
			Origin::signed(1),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio category".as_bytes().to_vec(),
				description: "This is my description".as_bytes().to_vec(),
				test_result_sample: "Test result sample".as_bytes().to_vec(),
				dna_collection_process: "Dna Collection Process".as_bytes().to_vec(),
				long_description: Some("This is my long description".as_bytes().to_vec()),
				image: Some("This is my image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(1).unwrap();

		assert_ok!(Orders::create_order(
			Origin::signed(2),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService
		));

		let _order_id = Orders::last_order_by_customer_id(2).unwrap();
		let _dna_sample = GeneticTesting::dna_samples_by_lab_id(1).unwrap();

		assert_ok!(Orders::cancel_order(Origin::signed(2), _order_id));

		assert_eq!(
			Orders::order_by_id(&_order_id),
			Some(Order {
				id: _order_id,
				service_id: _lab.services[0],
				customer_id: 2,
				customer_box_public_key: Keccak256::hash(
					"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()
				),
				seller_id: 1,
				dna_sample_tracking_id: _dna_sample[0].clone(),
				currency: CurrencyType::default(),
				prices: PriceByCurrency::default().price_components,
				additional_prices: PriceByCurrency::default().additional_prices,
				status: OrderStatus::Cancelled,
				order_flow: ServiceFlow::StakingRequestService,
				created_at: 0,
				updated_at: 0
			})
		);
	})
}

#[test]
fn set_order_paid_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(Labs::register_lab(
			Origin::signed(1),
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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(1), EthereumAddress([b'X'; 20])));

		assert_ok!(Services::create_service(
			Origin::signed(1),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio category".as_bytes().to_vec(),
				description: "This is my description".as_bytes().to_vec(),
				test_result_sample: "Test result sample".as_bytes().to_vec(),
				dna_collection_process: "Dna Collection Process".as_bytes().to_vec(),
				long_description: Some("This is my long description".as_bytes().to_vec()),
				image: Some("This is my image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(1).unwrap();

		assert_ok!(Orders::create_order(
			Origin::signed(2),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService
		));

		let _order_id = Orders::last_order_by_customer_id(2).unwrap();
		let _dna_sample = GeneticTesting::dna_samples_by_lab_id(1).unwrap();

		EscrowKey::<Test>::put(3);

		assert_ok!(Orders::set_order_paid(Origin::signed(3), _order_id));

		assert_eq!(
			Orders::order_by_id(&_order_id),
			Some(Order {
				id: _order_id,
				service_id: _lab.services[0],
				customer_id: 2,
				customer_box_public_key: Keccak256::hash(
					"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()
				),
				seller_id: 1,
				dna_sample_tracking_id: _dna_sample[0].clone(),
				currency: CurrencyType::default(),
				prices: PriceByCurrency::default().price_components,
				additional_prices: PriceByCurrency::default().additional_prices,
				status: OrderStatus::Paid,
				order_flow: ServiceFlow::StakingRequestService,
				created_at: 0,
				updated_at: 0
			})
		);
	})
}

#[test]
fn fulfill_order_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(Labs::register_lab(
			Origin::signed(1),
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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(1), EthereumAddress([b'X'; 20])));

		assert_ok!(Services::create_service(
			Origin::signed(1),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio category".as_bytes().to_vec(),
				description: "This is my description".as_bytes().to_vec(),
				test_result_sample: "Test result sample".as_bytes().to_vec(),
				dna_collection_process: "Dna Collection Process".as_bytes().to_vec(),
				long_description: Some("This is my long description".as_bytes().to_vec()),
				image: Some("This is my image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(1).unwrap();

		assert_ok!(Orders::create_order(
			Origin::signed(2),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService
		));

		let _order_id = Orders::last_order_by_customer_id(2).unwrap();
		let _dna_sample = GeneticTesting::dna_samples_by_lab_id(1).unwrap();

		assert_ok!(GeneticTesting::submit_test_result(
			Origin::signed(1),
			_dna_sample[0].clone(),
			DnaTestResultSubmission {
				comments: Some("comment".as_bytes().to_vec()),
				result_link: Some("result_link".as_bytes().to_vec()),
				report_link: Some("report_link".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticTesting::process_dna_sample(
			Origin::signed(1),
			_dna_sample[0].clone(),
			DnaSampleStatus::ResultReady,
		));

		assert_ok!(Orders::fulfill_order(Origin::signed(1), _order_id));

		assert_eq!(
			Orders::order_by_id(&_order_id),
			Some(Order {
				id: _order_id,
				service_id: _lab.services[0],
				customer_id: 2,
				customer_box_public_key: Keccak256::hash(
					"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()
				),
				seller_id: 1,
				dna_sample_tracking_id: _dna_sample[0].clone(),
				currency: CurrencyType::default(),
				prices: PriceByCurrency::default().price_components,
				additional_prices: PriceByCurrency::default().additional_prices,
				status: OrderStatus::Fulfilled,
				order_flow: ServiceFlow::StakingRequestService,
				created_at: 0,
				updated_at: 0
			})
		);
	})
}

#[test]
fn set_order_refunded_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));

		EscrowKey::<Test>::put(3);

		assert_ok!(Labs::register_lab(
			Origin::signed(1),
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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(1), EthereumAddress([b'X'; 20])));

		assert_ok!(Services::create_service(
			Origin::signed(1),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio category".as_bytes().to_vec(),
				description: "This is my description".as_bytes().to_vec(),
				test_result_sample: "Test result sample".as_bytes().to_vec(),
				dna_collection_process: "Dna Collection Process".as_bytes().to_vec(),
				long_description: Some("This is my long description".as_bytes().to_vec()),
				image: Some("This is my image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(1).unwrap();

		assert_ok!(Orders::create_order(
			Origin::signed(2),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService
		));

		let _order_id = Orders::last_order_by_customer_id(2).unwrap();
		let _dna_sample = GeneticTesting::dna_samples_by_lab_id(1).unwrap();

		assert_ok!(GeneticTesting::submit_test_result(
			Origin::signed(1),
			_dna_sample[0].clone(),
			DnaTestResultSubmission {
				comments: Some("comment".as_bytes().to_vec()),
				result_link: Some("result_link".as_bytes().to_vec()),
				report_link: Some("report_link".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticTesting::process_dna_sample(
			Origin::signed(1),
			_dna_sample[0].clone(),
			DnaSampleStatus::Rejected,
		));

		assert_ok!(Orders::set_order_refunded(Origin::signed(3), _order_id));

		assert_eq!(
			Orders::order_by_id(&_order_id),
			Some(Order {
				id: _order_id,
				service_id: _lab.services[0],
				customer_id: 2,
				customer_box_public_key: Keccak256::hash(
					"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()
				),
				seller_id: 1,
				dna_sample_tracking_id: _dna_sample[0].clone(),
				currency: CurrencyType::default(),
				prices: PriceByCurrency::default().price_components,
				additional_prices: PriceByCurrency::default().additional_prices,
				status: OrderStatus::Refunded,
				order_flow: ServiceFlow::StakingRequestService,
				created_at: 0,
				updated_at: 0
			})
		);
	})
}

#[test]
fn cant_create_order_when_service_not_exists() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_noop!(
			Orders::create_order(
				Origin::signed(1),
				Keccak256::hash("serviceId".as_bytes()),
				0,
				Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
				ServiceFlow::StakingRequestService
			),
			Error::<Test>::ServiceDoesNotExist
		);
	})
}

#[test]
fn cant_create_order_when_price_index_not_found() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(Labs::register_lab(
			Origin::signed(1),
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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(1), EthereumAddress([b'X'; 20])));

		assert_ok!(Services::create_service(
			Origin::signed(1),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio category".as_bytes().to_vec(),
				description: "This is my description".as_bytes().to_vec(),
				test_result_sample: "Test result sample".as_bytes().to_vec(),
				dna_collection_process: "Dna Collection Process".as_bytes().to_vec(),
				long_description: Some("This is my long description".as_bytes().to_vec()),
				image: Some("This is my image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(1).unwrap();

		assert_noop!(
			Orders::create_order(
				Origin::signed(1),
				_lab.services[0],
				10,
				Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
				ServiceFlow::StakingRequestService
			),
			Error::<Test>::PriceIndexNotFound
		);
	})
}

#[test]
fn cant_cancel_order_when_not_exist() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_noop!(
			Orders::cancel_order(Origin::signed(1), Keccak256::hash("order_id".as_bytes())),
			Error::<Test>::OrderNotFound
		);
	})
}

#[test]
fn cant_cancel_order_when_unathorized_user() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(Labs::register_lab(
			Origin::signed(1),
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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(1), EthereumAddress([b'X'; 20])));

		assert_ok!(Services::create_service(
			Origin::signed(1),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio category".as_bytes().to_vec(),
				description: "This is my description".as_bytes().to_vec(),
				test_result_sample: "Test result sample".as_bytes().to_vec(),
				dna_collection_process: "Dna Collection Process".as_bytes().to_vec(),
				long_description: Some("This is my long description".as_bytes().to_vec()),
				image: Some("This is my image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(1).unwrap();

		assert_ok!(Orders::create_order(
			Origin::signed(2),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService
		));

		let _order_id = Orders::last_order_by_customer_id(2).unwrap();
		let _dna_sample = GeneticTesting::dna_samples_by_lab_id(1).unwrap();

		assert_noop!(
			Orders::cancel_order(Origin::signed(3), _order_id),
			Error::<Test>::UnauthorizedOrderCancellation
		);
	})
}

#[test]
fn cant_set_order_paid_when_unauthorized() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(Labs::register_lab(
			Origin::signed(1),
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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(1), EthereumAddress([b'X'; 20])));

		assert_ok!(Services::create_service(
			Origin::signed(1),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio category".as_bytes().to_vec(),
				description: "This is my description".as_bytes().to_vec(),
				test_result_sample: "Test result sample".as_bytes().to_vec(),
				dna_collection_process: "Dna Collection Process".as_bytes().to_vec(),
				long_description: Some("This is my long description".as_bytes().to_vec()),
				image: Some("This is my image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(1).unwrap();

		assert_ok!(Orders::create_order(
			Origin::signed(2),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService
		));

		let _order_id = Orders::last_order_by_customer_id(2).unwrap();

		assert_noop!(
			Orders::set_order_paid(Origin::signed(3), _order_id),
			Error::<Test>::Unauthorized
		);
	})
}

#[test]
fn cant_set_order_paid_when_not_exist() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));

		EscrowKey::<Test>::put(3);

		assert_noop!(
			Orders::set_order_paid(Origin::signed(3), Keccak256::hash("order_id".as_bytes())),
			Error::<Test>::OrderNotFound
		);
	})
}

#[test]
fn cant_fulfill_order_when_not_exist() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_noop!(
			Orders::fulfill_order(Origin::signed(1), Keccak256::hash("order_id".as_bytes())),
			Error::<Test>::OrderNotFound
		);
	})
}

#[test]
fn cant_fulfill_order_when_unauthorized() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(Labs::register_lab(
			Origin::signed(1),
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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(1), EthereumAddress([b'X'; 20])));

		assert_ok!(Services::create_service(
			Origin::signed(1),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio category".as_bytes().to_vec(),
				description: "This is my description".as_bytes().to_vec(),
				test_result_sample: "Test result sample".as_bytes().to_vec(),
				dna_collection_process: "Dna Collection Process".as_bytes().to_vec(),
				long_description: Some("This is my long description".as_bytes().to_vec()),
				image: Some("This is my image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(1).unwrap();

		assert_ok!(Orders::create_order(
			Origin::signed(2),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService
		));

		let _order_id = Orders::last_order_by_customer_id(2).unwrap();

		assert_noop!(
			Orders::fulfill_order(Origin::signed(4), _order_id),
			Error::<Test>::UnauthorizedOrderFulfillment
		);
	})
}

#[test]
fn cant_fulfill_order_when_dna_sample_not_process() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(Labs::register_lab(
			Origin::signed(1),
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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(1), EthereumAddress([b'X'; 20])));

		assert_ok!(Services::create_service(
			Origin::signed(1),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio category".as_bytes().to_vec(),
				description: "This is my description".as_bytes().to_vec(),
				test_result_sample: "Test result sample".as_bytes().to_vec(),
				dna_collection_process: "Dna Collection Process".as_bytes().to_vec(),
				long_description: Some("This is my long description".as_bytes().to_vec()),
				image: Some("This is my image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(1).unwrap();

		assert_ok!(Orders::create_order(
			Origin::signed(2),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService
		));

		let _order_id = Orders::last_order_by_customer_id(2).unwrap();

		assert_noop!(
			Orders::fulfill_order(Origin::signed(1), _order_id),
			Error::<Test>::DnaSampleNotSuccessfullyProcessed
		);
	})
}

#[test]
fn cant_set_order_refunded_when_unauthorized() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));

		assert_ok!(Labs::register_lab(
			Origin::signed(1),
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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(1), EthereumAddress([b'X'; 20])));

		assert_ok!(Services::create_service(
			Origin::signed(1),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio category".as_bytes().to_vec(),
				description: "This is my description".as_bytes().to_vec(),
				test_result_sample: "Test result sample".as_bytes().to_vec(),
				dna_collection_process: "Dna Collection Process".as_bytes().to_vec(),
				long_description: Some("This is my long description".as_bytes().to_vec()),
				image: Some("This is my image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(1).unwrap();

		assert_ok!(Orders::create_order(
			Origin::signed(2),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService
		));

		let _order_id = Orders::last_order_by_customer_id(2).unwrap();
		let _dna_sample = GeneticTesting::dna_samples_by_lab_id(1).unwrap();

		EscrowKey::<Test>::put(3);

		assert_noop!(
			Orders::set_order_refunded(Origin::signed(3), _order_id),
			Error::<Test>::OrderNotYetExpired
		);
	})
}

#[test]
fn call_event_should_work() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(Labs::register_lab(
			Origin::signed(1),
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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(1), EthereumAddress([b'X'; 20])));

		assert_ok!(Services::create_service(
			Origin::signed(1),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				category: "DeBio category".as_bytes().to_vec(),
				description: "This is my description".as_bytes().to_vec(),
				test_result_sample: "Test result sample".as_bytes().to_vec(),
				dna_collection_process: "Dna Collection Process".as_bytes().to_vec(),
				long_description: Some("This is my long description".as_bytes().to_vec()),
				image: Some("This is my image".as_bytes().to_vec()),
			},
			ServiceFlow::default()
		));

		let _lab = Labs::lab_by_account_id(1).unwrap();

		assert_ok!(Orders::create_order(
			Origin::signed(2),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService
		));

		let _order_id = Orders::last_order_by_customer_id(2).unwrap();
		let _dna_sample = GeneticTesting::dna_samples_by_lab_id(1).unwrap();

		System::assert_last_event(Event::Orders(crate::Event::OrderCreated(Order {
			id: _order_id,
			service_id: _lab.services[0],
			customer_id: 2,
			customer_box_public_key: Keccak256::hash(
				"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes(),
			),
			seller_id: 1,
			dna_sample_tracking_id: _dna_sample[0].clone(),
			currency: CurrencyType::default(),
			prices: PriceByCurrency::default().price_components,
			additional_prices: PriceByCurrency::default().additional_prices,
			status: OrderStatus::default(),
			order_flow: ServiceFlow::StakingRequestService,
			created_at: 0,
			updated_at: 0,
		})));

		assert_ok!(Orders::cancel_order(Origin::signed(2), _order_id));

		System::assert_last_event(Event::Orders(crate::Event::OrderCancelled(Order {
			id: _order_id,
			service_id: _lab.services[0],
			customer_id: 2,
			customer_box_public_key: Keccak256::hash(
				"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes(),
			),
			seller_id: 1,
			dna_sample_tracking_id: _dna_sample[0].clone(),
			currency: CurrencyType::default(),
			prices: PriceByCurrency::default().price_components,
			additional_prices: PriceByCurrency::default().additional_prices,
			status: OrderStatus::Cancelled,
			order_flow: ServiceFlow::StakingRequestService,
			created_at: 0,
			updated_at: 0,
		})));

		assert_ok!(Orders::create_order(
			Origin::signed(2),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService
		));

		let _order_id = Orders::last_order_by_customer_id(2).unwrap();
		let _dna_sample = GeneticTesting::dna_samples_by_lab_id(1).unwrap();

		EscrowKey::<Test>::put(3);

		assert_ok!(Orders::set_order_paid(Origin::signed(3), _order_id));

		System::assert_last_event(Event::Orders(crate::Event::OrderPaid(Order {
			id: _order_id,
			service_id: _lab.services[0],
			customer_id: 2,
			customer_box_public_key: Keccak256::hash(
				"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes(),
			),
			seller_id: 1,
			dna_sample_tracking_id: _dna_sample[0].clone(),
			currency: CurrencyType::default(),
			prices: PriceByCurrency::default().price_components,
			additional_prices: PriceByCurrency::default().additional_prices,
			status: OrderStatus::Paid,
			order_flow: ServiceFlow::StakingRequestService,
			created_at: 0,
			updated_at: 0,
		})));

		assert_ok!(GeneticTesting::submit_test_result(
			Origin::signed(1),
			_dna_sample[0].clone(),
			DnaTestResultSubmission {
				comments: Some("comment".as_bytes().to_vec()),
				result_link: Some("result_link".as_bytes().to_vec()),
				report_link: Some("report_link".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticTesting::process_dna_sample(
			Origin::signed(1),
			_dna_sample[0].clone(),
			DnaSampleStatus::ResultReady,
		));

		assert_ok!(Orders::fulfill_order(Origin::signed(1), _order_id));

		System::assert_last_event(Event::Orders(crate::Event::OrderFulfilled(Order {
			id: _order_id,
			service_id: _lab.services[0],
			customer_id: 2,
			customer_box_public_key: Keccak256::hash(
				"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes(),
			),
			seller_id: 1,
			dna_sample_tracking_id: _dna_sample[0].clone(),
			currency: CurrencyType::default(),
			prices: PriceByCurrency::default().price_components,
			additional_prices: PriceByCurrency::default().additional_prices,
			status: OrderStatus::Fulfilled,
			order_flow: ServiceFlow::StakingRequestService,
			created_at: 0,
			updated_at: 0,
		})));

		assert_ok!(GeneticTesting::process_dna_sample(
			Origin::signed(1),
			_dna_sample[0].clone(),
			DnaSampleStatus::Rejected,
		));

		assert_ok!(Orders::set_order_refunded(Origin::signed(3), _order_id));

		System::assert_last_event(Event::Orders(crate::Event::OrderRefunded(Order {
			id: _order_id,
			service_id: _lab.services[0],
			customer_id: 2,
			customer_box_public_key: Keccak256::hash(
				"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes(),
			),
			seller_id: 1,
			dna_sample_tracking_id: _dna_sample[0].clone(),
			currency: CurrencyType::default(),
			prices: PriceByCurrency::default().price_components,
			additional_prices: PriceByCurrency::default().additional_prices,
			status: OrderStatus::Refunded,
			order_flow: ServiceFlow::StakingRequestService,
			created_at: 0,
			updated_at: 0,
		})));
	});
}

#[test]
fn update_escrow_key_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		EscrowKey::<Test>::put(2);

		assert_eq!(Orders::admin_key(), 2);

		assert_ok!(Orders::update_escrow_key(Origin::signed(2), 1,));

		assert_eq!(Orders::admin_key(), 1);
	})
}
