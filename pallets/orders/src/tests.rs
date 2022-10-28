use crate::{
	mock::*, AccountKeyType, Error, EscrowKey, Order, OrderStatus, PalletAccount, TreasuryKey,
};
use frame_support::{
	assert_noop, assert_ok,
	sp_runtime::traits::{Hash, Keccak256},
};
use genetic_testing::{DnaSampleStatus, DnaTestResultSubmission};
use labs::LabInfo;
use primitives_area_code::{CityCode, CountryCode, RegionCode};
use services::ServiceInfo;
use traits_services::types::ServiceFlow;

use primitives_duration::ExpectedDuration;
use primitives_price_and_currency::{CurrencyType, Price, PriceByCurrency};

#[test]
fn create_order_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let customer = account_key("customer");
		let lab = account_key("lab");

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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(lab), EthereumAddress([b'X'; 20])));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		let prices_by_currency_usdt = PriceByCurrency {
			currency: CurrencyType::USDT,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio.clone(), prices_by_currency_usdt],
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

		let _lab = Labs::lab_by_account_id(lab).unwrap();

		// Order with DBIO
		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None
		));

		let _order_id = Orders::last_order_by_customer_id(customer).unwrap();
		let _dna_sample = GeneticTesting::dna_samples_by_lab_id(lab).unwrap();

		assert_eq!(
			Orders::order_by_id(&_order_id),
			Some(Order {
				id: _order_id,
				service_id: _lab.services[0],
				customer_id: customer,
				customer_box_public_key: Keccak256::hash(
					"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()
				),
				seller_id: lab,
				dna_sample_tracking_id: _dna_sample[0].clone(),
				asset_id: None,
				total_price: 10,
				currency: CurrencyType::DBIO,
				prices: prices_by_currency_dbio.price_components,
				additional_prices: prices_by_currency_dbio.additional_prices,
				status: OrderStatus::default(),
				order_flow: ServiceFlow::StakingRequestService,
				created_at: 0,
				updated_at: 0
			})
		);

		assert_eq!(Orders::orders_by_lab_id(lab), Some(vec![_order_id]));
		assert_eq!(Orders::orders_by_customer_id(customer), Some(vec![_order_id]));
		assert_eq!(
			Orders::pending_genetic_analysis_orders_by_genetic_analyst_id(lab),
			Some(vec![_order_id])
		);
	})
}

#[test]
fn cancel_order_works_when_order_status_unpaid() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let lab = account_key("lab");
		let customer = account_key("customer");

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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(lab), EthereumAddress([b'X'; 20])));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio.clone()],
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

		let _lab = Labs::lab_by_account_id(lab).unwrap();

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let _order_id = Orders::last_order_by_customer_id(customer).unwrap();
		let _dna_sample = GeneticTesting::dna_samples_by_lab_id(lab).unwrap();

		assert_ok!(Orders::cancel_order(Origin::signed(customer), _order_id));

		assert_eq!(
			Orders::order_by_id(&_order_id),
			Some(Order {
				id: _order_id,
				service_id: _lab.services[0],
				customer_id: customer,
				customer_box_public_key: Keccak256::hash(
					"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()
				),
				seller_id: lab,
				dna_sample_tracking_id: _dna_sample[0].clone(),
				asset_id: None,
				total_price: 10,
				currency: CurrencyType::DBIO,
				prices: prices_by_currency_dbio.price_components,
				additional_prices: prices_by_currency_dbio.additional_prices,
				status: OrderStatus::Cancelled,
				order_flow: ServiceFlow::StakingRequestService,
				created_at: 0,
				updated_at: 0
			})
		);

		assert_eq!(GeneticTesting::dna_sample_by_tracking_id(_dna_sample[0].clone()), None);

		assert_eq!(
			Orders::pending_genetic_analysis_orders_by_genetic_analyst_id(lab),
			Some(Vec::new()),
		);
	})
}

#[test]
fn set_order_paid_with_dbio_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let lab = account_key("lab");
		let admin = account_key("admin");
		let customer = account_key("customer");
		let pallet_id = account_key("pallet_id");

		PalletAccount::<Test>::put(pallet_id);

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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(lab), EthereumAddress([b'X'; 20])));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio.clone()],
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

		let _lab = Labs::lab_by_account_id(lab).unwrap();

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let _order_id = Orders::last_order_by_customer_id(customer).unwrap();
		let _dna_sample = GeneticTesting::dna_samples_by_lab_id(lab).unwrap();

		EscrowKey::<Test>::put(admin);

		assert_ok!(Orders::set_order_paid(Origin::signed(customer), _order_id));

		assert_eq!(
			Orders::order_by_id(&_order_id),
			Some(Order {
				id: _order_id,
				service_id: _lab.services[0],
				customer_id: customer,
				customer_box_public_key: Keccak256::hash(
					"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()
				),
				seller_id: lab,
				dna_sample_tracking_id: _dna_sample[0].clone(),
				asset_id: None,
				total_price: 10,
				currency: CurrencyType::default(),
				prices: prices_by_currency_dbio.price_components,
				additional_prices: prices_by_currency_dbio.additional_prices,
				status: OrderStatus::Paid,
				order_flow: ServiceFlow::StakingRequestService,
				created_at: 0,
				updated_at: 0
			})
		);

		assert_eq!(Balances::free_balance(customer), 190);
		assert_eq!(Balances::free_balance(pallet_id), 11);
	})
}

#[test]
fn set_order_paid_with_app_chain_token_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let lab = account_key("lab");
		let admin = account_key("admin");
		let customer = account_key("customer");
		let pallet_id = account_key("pallet_id");

		PalletAccount::<Test>::put(pallet_id);

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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(lab), EthereumAddress([b'X'; 20])));

		let prices_by_currency_usdt = PriceByCurrency {
			currency: CurrencyType::USDT,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_usdt.clone()],
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

		let _lab = Labs::lab_by_account_id(lab).unwrap();
		let asset_id = 1;

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			Some(asset_id),
		));

		let _order_id = Orders::last_order_by_customer_id(customer).unwrap();
		let _dna_sample = GeneticTesting::dna_samples_by_lab_id(lab).unwrap();

		EscrowKey::<Test>::put(admin);

		assert_ok!(Orders::set_order_paid(Origin::signed(customer), _order_id));

		assert_eq!(
			Orders::order_by_id(&_order_id),
			Some(Order {
				id: _order_id,
				service_id: _lab.services[0],
				customer_id: customer,
				customer_box_public_key: Keccak256::hash(
					"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()
				),
				seller_id: lab,
				dna_sample_tracking_id: _dna_sample[0].clone(),
				asset_id: Some(asset_id),
				total_price: 10,
				currency: CurrencyType::USDT,
				prices: prices_by_currency_usdt.price_components,
				additional_prices: prices_by_currency_usdt.additional_prices,
				status: OrderStatus::Paid,
				order_flow: ServiceFlow::StakingRequestService,
				created_at: 0,
				updated_at: 0
			})
		);

		assert_eq!(Assets::balance(asset_id, customer), 190);
		assert_eq!(Assets::balance(asset_id, pallet_id), 11);
	})
}

#[test]
fn cancel_order_works_when_order_status_paid() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let lab = account_key("lab");
		let customer = account_key("customer");
		let pallet_id = account_key("pallet_id");

		PalletAccount::<Test>::put(pallet_id);

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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(lab), EthereumAddress([b'X'; 20])));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio.clone()],
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

		let _lab = Labs::lab_by_account_id(lab).unwrap();

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let _order_id = Orders::last_order_by_customer_id(customer).unwrap();
		let _dna_sample = GeneticTesting::dna_samples_by_lab_id(lab).unwrap();

		assert_ok!(Orders::set_order_paid(Origin::signed(customer), _order_id));

		assert_eq!(Balances::free_balance(customer), 190);
		assert_eq!(Balances::free_balance(pallet_id), 11);

		assert_ok!(Orders::cancel_order(Origin::signed(customer), _order_id));

		assert_eq!(
			Orders::order_by_id(&_order_id),
			Some(Order {
				id: _order_id,
				service_id: _lab.services[0],
				customer_id: customer,
				customer_box_public_key: Keccak256::hash(
					"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()
				),
				seller_id: lab,
				dna_sample_tracking_id: _dna_sample[0].clone(),
				asset_id: None,
				total_price: 10,
				currency: CurrencyType::DBIO,
				prices: prices_by_currency_dbio.price_components,
				additional_prices: prices_by_currency_dbio.additional_prices,
				status: OrderStatus::Refunded,
				order_flow: ServiceFlow::StakingRequestService,
				created_at: 0,
				updated_at: 0
			})
		);

		assert_eq!(GeneticTesting::dna_sample_by_tracking_id(_dna_sample[0].clone()), None);

		assert_eq!(
			Orders::pending_genetic_analysis_orders_by_genetic_analyst_id(lab),
			Some(Vec::new()),
		);

		assert_eq!(Balances::free_balance(customer), 200);
		assert_eq!(Balances::free_balance(pallet_id), 1);
	})
}

#[test]
fn fulfill_order_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let lab = account_key("lab");
		let admin = account_key("admin");
		let customer = account_key("customer");
		let pallet_id = account_key("pallet_id");
		let treasury_key = account_key("treasury_key");

		PalletAccount::<Test>::put(pallet_id);
		TreasuryKey::<Test>::put(treasury_key);

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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(lab), EthereumAddress([b'X'; 20])));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 30,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 20 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 10 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio.clone()],
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

		let _lab = Labs::lab_by_account_id(lab).unwrap();

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let _order_id = Orders::last_order_by_customer_id(customer).unwrap();
		let _dna_sample = GeneticTesting::dna_samples_by_lab_id(lab).unwrap();

		assert_ok!(Orders::set_order_paid(Origin::signed(customer), _order_id));

		assert_ok!(GeneticTesting::submit_test_result(
			Origin::signed(lab),
			_dna_sample[0].clone(),
			DnaTestResultSubmission {
				comments: Some("comment".as_bytes().to_vec()),
				result_link: Some("result_link".as_bytes().to_vec()),
				report_link: Some("report_link".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticTesting::process_dna_sample(
			Origin::signed(lab),
			_dna_sample[0].clone(),
			DnaSampleStatus::ResultReady,
		));

		EscrowKey::<Test>::put(admin);

		assert_ok!(Orders::fulfill_order(Origin::signed(lab), _order_id));

		assert_eq!(
			Orders::order_by_id(&_order_id),
			Some(Order {
				id: _order_id,
				service_id: _lab.services[0],
				customer_id: customer,
				customer_box_public_key: Keccak256::hash(
					"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()
				),
				seller_id: lab,
				dna_sample_tracking_id: _dna_sample[0].clone(),
				asset_id: None,
				total_price: 30,
				currency: CurrencyType::default(),
				prices: prices_by_currency_dbio.price_components,
				additional_prices: prices_by_currency_dbio.additional_prices,
				status: OrderStatus::Fulfilled,
				order_flow: ServiceFlow::StakingRequestService,
				created_at: 0,
				updated_at: 0
			})
		);

		assert_eq!(Balances::free_balance(customer), 170);
		assert_eq!(Balances::free_balance(lab), 329);
		assert_eq!(Balances::free_balance(pallet_id), 1);
		assert_eq!(Balances::free_balance(treasury_key), 401);
	})
}

#[test]
fn set_order_refunded_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let lab = account_key("lab");
		let admin = account_key("admin");
		let customer = account_key("customer");
		let pallet_id = account_key("pallet_id");

		PalletAccount::<Test>::put(pallet_id);
		EscrowKey::<Test>::put(admin);

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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(lab), EthereumAddress([b'X'; 20])));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio.clone()],
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

		let _lab = Labs::lab_by_account_id(lab).unwrap();

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let _order_id = Orders::last_order_by_customer_id(customer).unwrap();
		let _dna_sample = GeneticTesting::dna_samples_by_lab_id(lab).unwrap();

		assert_ok!(Orders::set_order_paid(Origin::signed(customer), _order_id));

		assert_ok!(GeneticTesting::submit_test_result(
			Origin::signed(lab),
			_dna_sample[0].clone(),
			DnaTestResultSubmission {
				comments: Some("comment".as_bytes().to_vec()),
				result_link: Some("result_link".as_bytes().to_vec()),
				report_link: Some("report_link".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticTesting::process_dna_sample(
			Origin::signed(lab),
			_dna_sample[0].clone(),
			DnaSampleStatus::Rejected,
		));

		assert_ok!(Orders::set_order_refunded(Origin::signed(admin), _order_id));

		assert_eq!(
			Orders::order_by_id(&_order_id),
			Some(Order {
				id: _order_id,
				service_id: _lab.services[0],
				customer_id: customer,
				customer_box_public_key: Keccak256::hash(
					"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()
				),
				seller_id: lab,
				dna_sample_tracking_id: _dna_sample[0].clone(),
				asset_id: None,
				total_price: 10,
				currency: CurrencyType::default(),
				prices: prices_by_currency_dbio.price_components,
				additional_prices: prices_by_currency_dbio.additional_prices,
				status: OrderStatus::Refunded,
				order_flow: ServiceFlow::StakingRequestService,
				created_at: 0,
				updated_at: 0
			})
		);

		assert_eq!(Balances::free_balance(customer), 195);
		assert_eq!(Balances::free_balance(lab), 305);
		assert_eq!(Balances::free_balance(pallet_id), 1);
	})
}

#[test]
fn cant_create_order_when_service_not_exists() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let customer = account_key("customer");

		assert_noop!(
			Orders::create_order(
				Origin::signed(customer),
				Keccak256::hash("serviceId".as_bytes()),
				0,
				Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
				ServiceFlow::StakingRequestService,
				None,
			),
			Error::<Test>::ServiceDoesNotExist
		);
	})
}

#[test]
fn cant_create_order_when_price_index_not_found() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let lab = account_key("lab");
		let customer = account_key("customer");

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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(lab), EthereumAddress([b'X'; 20])));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio],
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

		let _lab = Labs::lab_by_account_id(lab).unwrap();

		assert_noop!(
			Orders::create_order(
				Origin::signed(customer),
				_lab.services[0],
				10,
				Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
				ServiceFlow::StakingRequestService,
				None,
			),
			Error::<Test>::PriceIndexNotFound
		);
	})
}

#[test]
fn cant_create_order_when_asset_id_not_found() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let lab = account_key("lab");
		let customer = account_key("customer");

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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(lab), EthereumAddress([b'X'; 20])));

		let prices_by_currency_usdt = PriceByCurrency {
			currency: CurrencyType::USDT,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_usdt],
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

		let _lab = Labs::lab_by_account_id(lab).unwrap();

		assert_noop!(
			Orders::create_order(
				Origin::signed(customer),
				_lab.services[0],
				0,
				Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
				ServiceFlow::StakingRequestService,
				None,
			),
			Error::<Test>::AssetIdNotFound
		);

		assert_noop!(
			Orders::create_order(
				Origin::signed(customer),
				_lab.services[0],
				0,
				Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
				ServiceFlow::StakingRequestService,
				Some(2),
			),
			Error::<Test>::AssetIdNotFound
		);
	})
}

#[test]
fn cant_cancel_order_when_order_ongoing() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let lab = account_key("lab");
		let customer = account_key("customer");
		let pallet_id = account_key("pallet_id");

		PalletAccount::<Test>::put(pallet_id);

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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(lab), EthereumAddress([b'X'; 20])));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio],
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

		let _lab = Labs::lab_by_account_id(lab).unwrap();

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let _order_id = Orders::last_order_by_customer_id(customer).unwrap();
		let _dna_sample = GeneticTesting::dna_samples_by_lab_id(lab).unwrap();

		assert_ok!(Orders::set_order_paid(Origin::signed(customer), _order_id));
		assert_ok!(GeneticTesting::process_dna_sample(
			Origin::signed(lab),
			_dna_sample[0].clone(),
			DnaSampleStatus::Arrived,
		));

		assert_noop!(
			Orders::cancel_order(Origin::signed(customer), _order_id),
			Error::<Test>::OngoingOrderCannotBeCancelled
		);
	})
}

#[test]
fn cant_cancel_order_when_order_not_exist() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let customer = account_key("customer");

		assert_noop!(
			Orders::cancel_order(Origin::signed(customer), Keccak256::hash("order_id".as_bytes())),
			Error::<Test>::OrderNotFound
		);
	})
}

#[test]
fn cant_cancel_order_when_unathorized_user() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let lab = account_key("lab");
		let customer = account_key("customer");
		let other_customer = account_key("other_customer");

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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(lab), EthereumAddress([b'X'; 20])));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio],
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

		let _lab = Labs::lab_by_account_id(lab).unwrap();

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let _order_id = Orders::last_order_by_customer_id(customer).unwrap();
		let _dna_sample = GeneticTesting::dna_samples_by_lab_id(lab).unwrap();

		assert_noop!(
			Orders::cancel_order(Origin::signed(other_customer), _order_id),
			Error::<Test>::UnauthorizedOrderCancellation
		);
	})
}

#[test]
fn cant_cancel_order_when_order_already_finished() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let lab = account_key("lab");
		let customer = account_key("customer");

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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(lab), EthereumAddress([b'X'; 20])));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio],
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

		let _lab = Labs::lab_by_account_id(lab).unwrap();

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let _order_id = Orders::last_order_by_customer_id(customer).unwrap();
		let _dna_sample = GeneticTesting::dna_samples_by_lab_id(lab).unwrap();

		assert_ok!(Orders::cancel_order(Origin::signed(customer), _order_id));

		assert_noop!(
			Orders::cancel_order(Origin::signed(customer), _order_id),
			Error::<Test>::OrderCannotBeCancelled,
		);
	})
}

#[test]
fn cant_set_order_paid_when_unauthorized() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let lab = account_key("lab");
		let customer = account_key("customer");
		let other_customer = account_key("other_customer");

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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(lab), EthereumAddress([b'X'; 20])));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio],
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

		let _lab = Labs::lab_by_account_id(lab).unwrap();

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let _order_id = Orders::last_order_by_customer_id(customer).unwrap();

		assert_noop!(
			Orders::set_order_paid(Origin::signed(other_customer), _order_id),
			Error::<Test>::Unauthorized
		);
	})
}

#[test]
fn cant_set_order_paid_when_not_exist() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_noop!(
			Orders::set_order_paid(Origin::signed(3), Keccak256::hash("order_id".as_bytes())),
			Error::<Test>::OrderNotFound
		);
	})
}

#[test]
fn cant_set_order_paid_when_already_finished() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let lab = account_key("lab");
		let customer = account_key("customer");
		let pallet_id = account_key("pallet_id");

		PalletAccount::<Test>::put(pallet_id);

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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(lab), EthereumAddress([b'X'; 20])));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio],
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

		let _lab = Labs::lab_by_account_id(lab).unwrap();

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let _order_id = Orders::last_order_by_customer_id(customer).unwrap();

		assert_ok!(Orders::set_order_paid(Origin::signed(customer), _order_id));

		assert_noop!(
			Orders::set_order_paid(Origin::signed(customer), _order_id),
			Error::<Test>::OrderCannotBePaid
		);
	})
}

#[test]
fn cant_fulfill_order_when_not_exist() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let admin = account_key("admin");

		EscrowKey::<Test>::put(admin);

		assert_noop!(
			Orders::fulfill_order(Origin::signed(admin), Keccak256::hash("order_id".as_bytes())),
			Error::<Test>::OrderNotFound
		);
	})
}

#[test]
fn cant_fulfill_order_when_unauthorized() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let lab = account_key("lab");
		let admin = account_key("admin");
		let customer = account_key("customer");
		let other_lab = account_key("other_lab");
		let pallet_id = account_key("pallet_id");

		PalletAccount::<Test>::put(pallet_id);

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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(lab), EthereumAddress([b'X'; 20])));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio],
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

		let _lab = Labs::lab_by_account_id(lab).unwrap();

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let _order_id = Orders::last_order_by_customer_id(customer).unwrap();
		let _dna_sample = GeneticTesting::dna_samples_by_lab_id(lab).unwrap();

		assert_ok!(Orders::set_order_paid(Origin::signed(customer), _order_id));

		assert_ok!(GeneticTesting::submit_test_result(
			Origin::signed(lab),
			_dna_sample[0].clone(),
			DnaTestResultSubmission {
				comments: Some("comment".as_bytes().to_vec()),
				result_link: Some("result_link".as_bytes().to_vec()),
				report_link: Some("report_link".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticTesting::process_dna_sample(
			Origin::signed(lab),
			_dna_sample[0].clone(),
			DnaSampleStatus::ResultReady,
		));

		EscrowKey::<Test>::put(admin);

		assert_noop!(
			Orders::fulfill_order(Origin::signed(other_lab), _order_id),
			Error::<Test>::UnauthorizedOrderFulfillment
		);
	})
}

#[test]
fn cant_fulfill_order_when_dna_sample_not_process() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let lab = account_key("lab");
		let admin = account_key("admin");
		let customer = account_key("customer");
		let pallet_id = account_key("pallet_id");

		PalletAccount::<Test>::put(pallet_id);

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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(lab), EthereumAddress([b'X'; 20])));

		assert_ok!(Services::create_service(
			Origin::signed(lab),
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

		let _lab = Labs::lab_by_account_id(lab).unwrap();

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let _order_id = Orders::last_order_by_customer_id(customer).unwrap();

		assert_ok!(Orders::set_order_paid(Origin::signed(customer), _order_id));

		EscrowKey::<Test>::put(admin);

		assert_noop!(
			Orders::fulfill_order(Origin::signed(lab), _order_id),
			Error::<Test>::DnaSampleNotSuccessfullyProcessed
		);
	})
}

#[test]
fn cant_fulfill_order_when_already_fulfilled() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let lab = account_key("lab");
		let admin = account_key("admin");
		let customer = account_key("customer");
		let pallet_id = account_key("pallet_id");
		let treasury_key = account_key("treasury_key");

		PalletAccount::<Test>::put(pallet_id);
		TreasuryKey::<Test>::put(treasury_key);

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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(lab), EthereumAddress([b'X'; 20])));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio],
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

		let _lab = Labs::lab_by_account_id(lab).unwrap();

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let _order_id = Orders::last_order_by_customer_id(customer).unwrap();
		let _dna_sample = GeneticTesting::dna_samples_by_lab_id(lab).unwrap();

		assert_ok!(Orders::set_order_paid(Origin::signed(customer), _order_id));
		assert_ok!(GeneticTesting::submit_test_result(
			Origin::signed(lab),
			_dna_sample[0].clone(),
			DnaTestResultSubmission {
				comments: Some("comment".as_bytes().to_vec()),
				result_link: Some("result_link".as_bytes().to_vec()),
				report_link: Some("report_link".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticTesting::process_dna_sample(
			Origin::signed(lab),
			_dna_sample[0].clone(),
			DnaSampleStatus::ResultReady,
		));

		EscrowKey::<Test>::put(admin);

		assert_ok!(Orders::fulfill_order(Origin::signed(lab), _order_id));

		assert_noop!(
			Orders::fulfill_order(Origin::signed(lab), _order_id),
			Error::<Test>::OrderCannotBeFulfilled
		);
	})
}

#[test]
fn cant_set_order_refunded_when_not_yet_expired() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let lab = account_key("lab");
		let admin = account_key("admin");
		let customer = account_key("customer");
		let pallet_id = account_key("pallet_id");

		PalletAccount::<Test>::put(pallet_id);

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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(lab), EthereumAddress([b'X'; 20])));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio],
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

		let _lab = Labs::lab_by_account_id(lab).unwrap();

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let _order_id = Orders::last_order_by_customer_id(customer).unwrap();
		let _dna_sample = GeneticTesting::dna_samples_by_lab_id(lab).unwrap();

		EscrowKey::<Test>::put(admin);

		assert_ok!(Orders::set_order_paid(Origin::signed(customer), _order_id));

		assert_noop!(
			Orders::set_order_refunded(Origin::signed(admin), _order_id),
			Error::<Test>::OrderNotYetExpired
		);
	})
}

#[test]
fn cant_set_order_refunded_when_already_refunded() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		let lab = account_key("lab");
		let admin = account_key("admin");
		let customer = account_key("customer");
		let pallet_id = account_key("pallet_id");

		PalletAccount::<Test>::put(pallet_id);

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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(lab), EthereumAddress([b'X'; 20])));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio],
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

		let _lab = Labs::lab_by_account_id(lab).unwrap();

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let _order_id = Orders::last_order_by_customer_id(customer).unwrap();
		let _dna_sample = GeneticTesting::dna_samples_by_lab_id(lab).unwrap();

		assert_ok!(Orders::set_order_paid(Origin::signed(customer), _order_id));

		assert_ok!(GeneticTesting::submit_test_result(
			Origin::signed(lab),
			_dna_sample[0].clone(),
			DnaTestResultSubmission {
				comments: Some("comment".as_bytes().to_vec()),
				result_link: Some("result_link".as_bytes().to_vec()),
				report_link: Some("report_link".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticTesting::process_dna_sample(
			Origin::signed(lab),
			_dna_sample[0].clone(),
			DnaSampleStatus::Rejected,
		));

		EscrowKey::<Test>::put(admin);

		assert_ok!(Orders::set_order_refunded(Origin::signed(admin), _order_id));

		assert_noop!(
			Orders::set_order_refunded(Origin::signed(admin), _order_id),
			Error::<Test>::OrderCannotBeRefunded
		);
	})
}

#[test]
fn call_event_should_work() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		System::set_block_number(1);

		let lab = account_key("lab");
		let admin = account_key("admin");
		let customer = account_key("customer");
		let pallet_id = account_key("pallet_id");
		let treasury_key = account_key("treasury_key");

		PalletAccount::<Test>::put(pallet_id);
		TreasuryKey::<Test>::put(treasury_key);

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

		assert_ok!(UserProfile::set_eth_address(Origin::signed(lab), EthereumAddress([b'X'; 20])));

		let prices_by_currency_dbio = PriceByCurrency {
			currency: CurrencyType::DBIO,
			total_price: 10,
			price_components: vec![Price { component: b"testing_price".to_vec(), value: 5 }],
			additional_prices: vec![Price { component: b"qc_price".to_vec(), value: 5 }],
		};

		assert_ok!(Services::create_service(
			Origin::signed(lab),
			ServiceInfo {
				name: "DeBio name".as_bytes().to_vec(),
				prices_by_currency: vec![prices_by_currency_dbio.clone()],
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

		let _lab = Labs::lab_by_account_id(lab).unwrap();

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let _order_id = Orders::last_order_by_customer_id(customer).unwrap();
		let _dna_sample = GeneticTesting::dna_samples_by_lab_id(lab).unwrap();

		System::assert_last_event(Event::Orders(crate::Event::OrderCreated(Order {
			id: _order_id,
			service_id: _lab.services[0],
			customer_id: customer,
			customer_box_public_key: Keccak256::hash(
				"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes(),
			),
			seller_id: lab,
			dna_sample_tracking_id: _dna_sample[0].clone(),
			asset_id: None,
			total_price: 10,
			currency: CurrencyType::default(),
			prices: prices_by_currency_dbio.price_components.clone(),
			additional_prices: prices_by_currency_dbio.additional_prices.clone(),
			status: OrderStatus::default(),
			order_flow: ServiceFlow::StakingRequestService,
			created_at: 0,
			updated_at: 0,
		})));

		assert_ok!(Orders::cancel_order(Origin::signed(customer), _order_id));

		System::assert_last_event(Event::Orders(crate::Event::OrderCancelled(Order {
			id: _order_id,
			service_id: _lab.services[0],
			customer_id: customer,
			customer_box_public_key: Keccak256::hash(
				"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes(),
			),
			seller_id: lab,
			dna_sample_tracking_id: _dna_sample[0].clone(),
			asset_id: None,
			total_price: 10,
			currency: CurrencyType::default(),
			prices: prices_by_currency_dbio.price_components.clone(),
			additional_prices: prices_by_currency_dbio.additional_prices.clone(),
			status: OrderStatus::Cancelled,
			order_flow: ServiceFlow::StakingRequestService,
			created_at: 0,
			updated_at: 0,
		})));

		assert_ok!(Orders::create_order(
			Origin::signed(customer),
			_lab.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			ServiceFlow::StakingRequestService,
			None,
		));

		let _order_id = Orders::last_order_by_customer_id(customer).unwrap();
		let _dna_sample = GeneticTesting::dna_samples_by_lab_id(lab).unwrap();

		EscrowKey::<Test>::put(admin);

		assert_ok!(Orders::set_order_paid(Origin::signed(customer), _order_id));

		System::assert_last_event(Event::Orders(crate::Event::OrderPaid(Order {
			id: _order_id,
			service_id: _lab.services[0],
			customer_id: customer,
			customer_box_public_key: Keccak256::hash(
				"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes(),
			),
			seller_id: lab,
			dna_sample_tracking_id: _dna_sample[0].clone(),
			asset_id: None,
			total_price: 10,
			currency: CurrencyType::default(),
			prices: prices_by_currency_dbio.price_components.clone(),
			additional_prices: prices_by_currency_dbio.additional_prices.clone(),
			status: OrderStatus::Paid,
			order_flow: ServiceFlow::StakingRequestService,
			created_at: 0,
			updated_at: 0,
		})));

		assert_ok!(GeneticTesting::submit_test_result(
			Origin::signed(lab),
			_dna_sample[0].clone(),
			DnaTestResultSubmission {
				comments: Some("comment".as_bytes().to_vec()),
				result_link: Some("result_link".as_bytes().to_vec()),
				report_link: Some("report_link".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticTesting::process_dna_sample(
			Origin::signed(lab),
			_dna_sample[0].clone(),
			DnaSampleStatus::ResultReady,
		));

		assert_ok!(Orders::fulfill_order(Origin::signed(lab), _order_id));

		System::assert_last_event(Event::Orders(crate::Event::OrderFulfilled(Order {
			id: _order_id,
			service_id: _lab.services[0],
			customer_id: customer,
			customer_box_public_key: Keccak256::hash(
				"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes(),
			),
			seller_id: lab,
			dna_sample_tracking_id: _dna_sample[0].clone(),
			asset_id: None,
			total_price: 10,
			currency: CurrencyType::default(),
			prices: prices_by_currency_dbio.price_components.clone(),
			additional_prices: prices_by_currency_dbio.additional_prices,
			status: OrderStatus::Fulfilled,
			order_flow: ServiceFlow::StakingRequestService,
			created_at: 0,
			updated_at: 0,
		})));
	});
}

#[test]
fn update_key_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		EscrowKey::<Test>::put(2);

		assert_eq!(Orders::admin_key(), Some(2));

		assert_ok!(Orders::update_key(Origin::signed(2), AccountKeyType::EscrowKey(1)));

		assert_eq!(Orders::admin_key(), Some(1));
	})
}

#[test]
fn sudo_update_key_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Orders::sudo_update_key(Origin::root(), AccountKeyType::TreasuryKey(1)));

		assert_eq!(Orders::treasury_key(), Some(1));
	})
}
