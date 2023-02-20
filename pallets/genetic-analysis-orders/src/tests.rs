use crate::{
	mock::*, AccountKeyType, Error, EscrowKey, GeneticAnalysisOrder, GeneticAnalysisOrderStatus,
	PalletAccount, TreasuryKey,
};
use frame_support::{
	assert_noop, assert_ok,
	sp_runtime::{
		traits::{Hash, Keccak256},
		SaturatedConversion,
	},
};
use frame_system::RawOrigin;
use genetic_analysis::GeneticAnalysisStatus;
use genetic_analyst_services::GeneticAnalystServiceInfo;
use genetic_analysts::GeneticAnalystInfo;

use primitives_availability_status::AvailabilityStatus;
use primitives_duration::ExpectedDuration;
use primitives_price_and_currency::{CurrencyType, Price, PriceByCurrency};

#[test]
fn create_genetic_analysis_order() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			RuntimeOrigin::signed(1),
			GeneticAnalystInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
				profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticAnalysts::update_genetic_analyst_availability_status(
			RuntimeOrigin::signed(1),
			AvailabilityStatus::Available
		));

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			RuntimeOrigin::signed(1),
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
		));

		let _genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

		let _add_genetic_data = GeneticData::add_genetic_data(
			RuntimeOrigin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec(),
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
			RuntimeOrigin::signed(1),
			_genetic_data_ids[0],
			_genetic_analyst.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			"DeBio Genetic Genetic Link".as_bytes().to_vec(),
			None,
		));

		let _genetic_analysis_order_id =
			GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();
		let _genetic_analysis = GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

		assert_eq!(
			GeneticAnalysisOrders::genetic_analysis_order_by_id(&_genetic_analysis_order_id),
			Some(GeneticAnalysisOrder {
				id: _genetic_analysis_order_id,
				genetic_data_id: _genetic_data_ids[0],
				service_id: _genetic_analyst.services[0],
				customer_id: 1,
				customer_box_public_key: Keccak256::hash(
					"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()
				),
				seller_id: 1,
				genetic_analysis_tracking_id: _genetic_analysis[0].clone(),
				genetic_link: "DeBio Genetic Genetic Link".as_bytes().to_vec(),
				asset_id: None,
				currency: CurrencyType::default(),
				prices: PriceByCurrency::default().price_components,
				additional_prices: PriceByCurrency::default().additional_prices,
				total_price: PriceByCurrency::default().total_price,
				status: GeneticAnalysisOrderStatus::default(),
				created_at: 0,
				updated_at: 0
			})
		);
	})
}

#[test]
fn cancel_genetic_analysis_order_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			RuntimeOrigin::signed(1),
			GeneticAnalystInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
				profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticAnalysts::update_genetic_analyst_availability_status(
			RuntimeOrigin::signed(1),
			AvailabilityStatus::Available
		));

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			RuntimeOrigin::signed(1),
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
		));

		let _genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

		let _add_genetic_data = GeneticData::add_genetic_data(
			RuntimeOrigin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec(),
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
			RuntimeOrigin::signed(1),
			_genetic_data_ids[0],
			_genetic_analyst.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			"DeBio Genetic Genetic Link".as_bytes().to_vec(),
			None,
		));

		let _genetic_analysis_order_id =
			GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();
		let _genetic_analysis = GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::cancel_genetic_analysis_order(
			RuntimeOrigin::signed(1),
			_genetic_analysis_order_id
		));

		assert_eq!(
			GeneticAnalysisOrders::genetic_analysis_order_by_id(&_genetic_analysis_order_id),
			Some(GeneticAnalysisOrder {
				id: _genetic_analysis_order_id,
				genetic_data_id: _genetic_data_ids[0],
				service_id: _genetic_analyst.services[0],
				customer_id: 1,
				customer_box_public_key: Keccak256::hash(
					"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()
				),
				seller_id: 1,
				genetic_analysis_tracking_id: _genetic_analysis[0].clone(),
				genetic_link: "DeBio Genetic Genetic Link".as_bytes().to_vec(),
				asset_id: None,
				currency: CurrencyType::default(),
				prices: PriceByCurrency::default().price_components,
				additional_prices: PriceByCurrency::default().additional_prices,
				total_price: PriceByCurrency::default().total_price,
				status: GeneticAnalysisOrderStatus::Cancelled,
				created_at: 0,
				updated_at: 0
			})
		);
	})
}

#[test]
fn cancel_genetic_analysis_order_with_refund_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 0, 1, 0));
		assert_ok!(Balances::set_balance(
			RawOrigin::Root.into(),
			1,
			20000000000000000000u128.saturated_into(),
			0
		));

		PalletAccount::<Test>::put(0);

		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			RuntimeOrigin::signed(1),
			GeneticAnalystInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
				profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticAnalysts::update_genetic_analyst_availability_status(
			RuntimeOrigin::signed(1),
			AvailabilityStatus::Available
		));

		let _price = Price {
			component: "Price Component".as_bytes().to_vec(),
			value: 5000000000000000000u128.saturated_into(),
		};

		let _price_by_currency = PriceByCurrency {
			currency: CurrencyType::default(),
			total_price: 10000000000000000000u128.saturated_into(),
			price_components: vec![_price.clone()],
			additional_prices: vec![_price],
		};

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			RuntimeOrigin::signed(1),
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![_price_by_currency],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
		));

		let _genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

		let _add_genetic_data = GeneticData::add_genetic_data(
			RuntimeOrigin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec(),
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
			RuntimeOrigin::signed(1),
			_genetic_data_ids[0],
			_genetic_analyst.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			"DeBio Genetic Genetic Link".as_bytes().to_vec(),
			None,
		));

		let _genetic_analysis_order_id =
			GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();
		let _genetic_analysis = GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

		EscrowKey::<Test>::put(3);

		assert_ok!(GeneticAnalysisOrders::set_genetic_analysis_order_paid(
			RuntimeOrigin::signed(1),
			_genetic_analysis_order_id
		));

		let _price = Price {
			component: "Price Component".as_bytes().to_vec(),
			value: 5000000000000000000u128.saturated_into(),
		};

		let _price_by_currency = PriceByCurrency {
			currency: CurrencyType::default(),
			total_price: 10000000000000000000u128.saturated_into(),
			price_components: vec![_price.clone()],
			additional_prices: vec![_price],
		};

		assert_eq!(
			GeneticAnalysisOrders::genetic_analysis_order_by_id(&_genetic_analysis_order_id),
			Some(GeneticAnalysisOrder {
				id: _genetic_analysis_order_id,
				genetic_data_id: _genetic_data_ids[0],
				service_id: _genetic_analyst.services[0],
				customer_id: 1,
				customer_box_public_key: Keccak256::hash(
					"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()
				),
				seller_id: 1,
				genetic_analysis_tracking_id: _genetic_analysis[0].clone(),
				genetic_link: "DeBio Genetic Genetic Link".as_bytes().to_vec(),
				asset_id: None,
				currency: _price_by_currency.currency,
				prices: _price_by_currency.price_components,
				additional_prices: _price_by_currency.additional_prices,
				total_price: _price_by_currency.total_price,
				status: GeneticAnalysisOrderStatus::Paid,
				created_at: 0,
				updated_at: 0
			})
		);

		assert_ok!(GeneticAnalysisOrders::cancel_genetic_analysis_order(
			RuntimeOrigin::signed(1),
			_genetic_analysis_order_id
		));

		let _price = Price {
			component: "Price Component".as_bytes().to_vec(),
			value: 5000000000000000000u128.saturated_into(),
		};

		let _price_by_currency = PriceByCurrency {
			currency: CurrencyType::default(),
			total_price: 10000000000000000000u128.saturated_into(),
			price_components: vec![_price.clone()],
			additional_prices: vec![_price],
		};

		assert_eq!(
			GeneticAnalysisOrders::genetic_analysis_order_by_id(&_genetic_analysis_order_id),
			Some(GeneticAnalysisOrder {
				id: _genetic_analysis_order_id,
				genetic_data_id: _genetic_data_ids[0],
				service_id: _genetic_analyst.services[0],
				customer_id: 1,
				customer_box_public_key: Keccak256::hash(
					"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()
				),
				seller_id: 1,
				genetic_analysis_tracking_id: _genetic_analysis[0].clone(),
				genetic_link: "DeBio Genetic Genetic Link".as_bytes().to_vec(),
				asset_id: None,
				currency: _price_by_currency.currency,
				prices: _price_by_currency.price_components,
				additional_prices: _price_by_currency.additional_prices,
				total_price: _price_by_currency.total_price,
				status: GeneticAnalysisOrderStatus::Refunded,
				created_at: 0,
				updated_at: 0
			})
		);

		assert_eq!(
			GeneticAnalysisOrders::pending_genetic_analysis_orders_by_genetic_analyst_id(1),
			Some(Vec::new())
		);
	})
}

#[test]
fn set_genetic_analysis_order_paid_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(
			RawOrigin::Root.into(),
			1,
			20000000000000000000u128.saturated_into(),
			0
		));

		PalletAccount::<Test>::put(0);

		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			RuntimeOrigin::signed(1),
			GeneticAnalystInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
				profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticAnalysts::update_genetic_analyst_availability_status(
			RuntimeOrigin::signed(1),
			AvailabilityStatus::Available
		));

		let _price = Price {
			component: "Price Component".as_bytes().to_vec(),
			value: 5000000000000000000u128.saturated_into(),
		};

		let _price_by_currency = PriceByCurrency {
			currency: CurrencyType::default(),
			total_price: 10000000000000000000u128.saturated_into(),
			price_components: vec![_price.clone()],
			additional_prices: vec![_price],
		};

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			RuntimeOrigin::signed(1),
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![_price_by_currency],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
		));

		let _genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

		let _add_genetic_data = GeneticData::add_genetic_data(
			RuntimeOrigin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec(),
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
			RuntimeOrigin::signed(1),
			_genetic_data_ids[0],
			_genetic_analyst.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			"DeBio Genetic Genetic Link".as_bytes().to_vec(),
			None,
		));

		let _genetic_analysis_order_id =
			GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();
		let _genetic_analysis = GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

		EscrowKey::<Test>::put(3);

		assert_ok!(GeneticAnalysisOrders::set_genetic_analysis_order_paid(
			RuntimeOrigin::signed(1),
			_genetic_analysis_order_id
		));

		let _price = Price {
			component: "Price Component".as_bytes().to_vec(),
			value: 5000000000000000000u128.saturated_into(),
		};

		let _price_by_currency = PriceByCurrency {
			currency: CurrencyType::default(),
			total_price: 10000000000000000000u128.saturated_into(),
			price_components: vec![_price.clone()],
			additional_prices: vec![_price],
		};

		assert_eq!(
			GeneticAnalysisOrders::genetic_analysis_order_by_id(&_genetic_analysis_order_id),
			Some(GeneticAnalysisOrder {
				id: _genetic_analysis_order_id,
				genetic_data_id: _genetic_data_ids[0],
				service_id: _genetic_analyst.services[0],
				customer_id: 1,
				customer_box_public_key: Keccak256::hash(
					"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()
				),
				seller_id: 1,
				genetic_analysis_tracking_id: _genetic_analysis[0].clone(),
				genetic_link: "DeBio Genetic Genetic Link".as_bytes().to_vec(),
				asset_id: None,
				currency: _price_by_currency.currency,
				prices: _price_by_currency.price_components,
				additional_prices: _price_by_currency.additional_prices,
				total_price: _price_by_currency.total_price,
				status: GeneticAnalysisOrderStatus::Paid,
				created_at: 0,
				updated_at: 0
			})
		);
	})
}

#[test]
fn fulfill_genetic_analysis_order_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 10000, 0));
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 0, 1, 0));

		PalletAccount::<Test>::put(0);
		EscrowKey::<Test>::put(1);
		TreasuryKey::<Test>::put(2);

		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			RuntimeOrigin::signed(1),
			GeneticAnalystInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
				profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticAnalysts::update_genetic_analyst_availability_status(
			RuntimeOrigin::signed(1),
			AvailabilityStatus::Available
		));

		let _price = Price {
			component: "Price Component".as_bytes().to_vec(),
			value: 500u128.saturated_into(),
		};

		let _price_by_currency = PriceByCurrency {
			currency: CurrencyType::default(),
			total_price: 1000u128.saturated_into(),
			price_components: vec![_price.clone()],
			additional_prices: vec![_price.clone()],
		};

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			RuntimeOrigin::signed(1),
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![_price_by_currency],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
		));

		let _genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

		let _add_genetic_data = GeneticData::add_genetic_data(
			RuntimeOrigin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec(),
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
			RuntimeOrigin::signed(1),
			_genetic_data_ids[0],
			_genetic_analyst.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			"DeBio Genetic Genetic Link".as_bytes().to_vec(),
			None,
		));

		let _genetic_analysis_order_id =
			GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::set_genetic_analysis_order_paid(
			RuntimeOrigin::signed(1),
			_genetic_analysis_order_id
		));

		let _genetic_analysis = GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

		assert_ok!(GeneticAnalysis::submit_genetic_analysis(
			RuntimeOrigin::signed(1),
			_genetic_analysis[0].clone(),
			"report_link".as_bytes().to_vec(),
			Some("comment".as_bytes().to_vec()),
		));

		assert_ok!(GeneticAnalysis::process_genetic_analysis(
			RuntimeOrigin::signed(1),
			_genetic_analysis[0].clone(),
			GeneticAnalysisStatus::ResultReady,
		));

		assert_eq!(Balances::free_balance(1), 9000);

		assert_ok!(GeneticAnalysisOrders::fulfill_genetic_analysis_order(
			RuntimeOrigin::signed(1),
			_genetic_analysis_order_id
		));

		assert_eq!(Balances::free_balance(1), 9950);
		assert_eq!(Balances::free_balance(2), 50);

		assert_eq!(
			GeneticAnalysisOrders::genetic_analysis_order_by_id(&_genetic_analysis_order_id),
			Some(GeneticAnalysisOrder {
				id: _genetic_analysis_order_id,
				genetic_data_id: _genetic_data_ids[0],
				service_id: _genetic_analyst.services[0],
				customer_id: 1,
				customer_box_public_key: Keccak256::hash(
					"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()
				),
				seller_id: 1,
				genetic_analysis_tracking_id: _genetic_analysis[0].clone(),
				genetic_link: "DeBio Genetic Genetic Link".as_bytes().to_vec(),
				asset_id: None,
				currency: CurrencyType::default(),
				prices: vec![_price.clone()],
				additional_prices: vec![_price.clone()],
				total_price: 1000u128.saturated_into(),
				status: GeneticAnalysisOrderStatus::Fulfilled,
				created_at: 0,
				updated_at: 0
			})
		);

		assert_eq!(
			GeneticAnalysisOrders::genetic_analysis_order_by_id(&_genetic_analysis_order_id),
			Some(GeneticAnalysisOrder {
				id: _genetic_analysis_order_id,
				genetic_data_id: _genetic_data_ids[0],
				service_id: _genetic_analyst.services[0],
				customer_id: 1,
				customer_box_public_key: Keccak256::hash(
					"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()
				),
				seller_id: 1,
				genetic_analysis_tracking_id: _genetic_analysis[0].clone(),
				genetic_link: "DeBio Genetic Genetic Link".as_bytes().to_vec(),
				asset_id: None,
				currency: CurrencyType::default(),
				prices: vec![_price.clone()],
				additional_prices: vec![_price],
				total_price: 1000u128.saturated_into(),
				status: GeneticAnalysisOrderStatus::Fulfilled,
				created_at: 0,
				updated_at: 0
			})
		);
	})
}

#[test]
fn set_genetic_analysis_order_refunded_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));

		PalletAccount::<Test>::put(0);
		EscrowKey::<Test>::put(3);

		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			RuntimeOrigin::signed(1),
			GeneticAnalystInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
				profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticAnalysts::update_genetic_analyst_availability_status(
			RuntimeOrigin::signed(1),
			AvailabilityStatus::Available
		));

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			RuntimeOrigin::signed(1),
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
		));

		let _genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

		let _add_genetic_data = GeneticData::add_genetic_data(
			RuntimeOrigin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec(),
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
			RuntimeOrigin::signed(1),
			_genetic_data_ids[0],
			_genetic_analyst.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			"DeBio Genetic Genetic Link".as_bytes().to_vec(),
			None,
		));

		let _genetic_analysis_order_id =
			GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();
		let _genetic_analysis = GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::set_genetic_analysis_order_paid(
			RuntimeOrigin::signed(1),
			_genetic_analysis_order_id
		));

		assert_ok!(GeneticAnalysis::submit_genetic_analysis(
			RuntimeOrigin::signed(1),
			_genetic_analysis[0].clone(),
			"report_link".as_bytes().to_vec(),
			Some("comment".as_bytes().to_vec()),
		));

		assert_ok!(GeneticAnalysis::process_genetic_analysis(
			RuntimeOrigin::signed(1),
			_genetic_analysis[0].clone(),
			GeneticAnalysisStatus::Rejected,
		));

		assert_ok!(GeneticAnalysisOrders::set_genetic_analysis_order_refunded(
			RuntimeOrigin::signed(3),
			_genetic_analysis_order_id
		));

		assert_eq!(
			GeneticAnalysisOrders::genetic_analysis_order_by_id(&_genetic_analysis_order_id),
			Some(GeneticAnalysisOrder {
				id: _genetic_analysis_order_id,
				genetic_data_id: _genetic_data_ids[0],
				service_id: _genetic_analyst.services[0],
				customer_id: 1,
				customer_box_public_key: Keccak256::hash(
					"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()
				),
				seller_id: 1,
				genetic_analysis_tracking_id: _genetic_analysis[0].clone(),
				genetic_link: "DeBio Genetic Genetic Link".as_bytes().to_vec(),
				asset_id: None,
				currency: CurrencyType::default(),
				prices: PriceByCurrency::default().price_components,
				additional_prices: PriceByCurrency::default().additional_prices,
				total_price: PriceByCurrency::default().total_price,
				status: GeneticAnalysisOrderStatus::Refunded,
				created_at: 0,
				updated_at: 0
			})
		);
	})
}

#[test]
fn cant_create_genetic_analysis_order_when_genetic_analyst_service_not_exists() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));

		let _add_genetic_data = GeneticData::add_genetic_data(
			RuntimeOrigin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec(),
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_noop!(
			GeneticAnalysisOrders::create_genetic_analysis_order(
				RuntimeOrigin::signed(1),
				_genetic_data_ids[0],
				Keccak256::hash("genetic_analyst_serviceId".as_bytes()),
				0,
				Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
				"DeBio Genetic Genetic Link".as_bytes().to_vec(),
				None,
			),
			Error::<Test>::GeneticAnalystServiceDoesNotExist
		);
	})
}

#[test]
fn cant_create_genetic_analysis_order_when_price_index_not_found() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			RuntimeOrigin::signed(1),
			GeneticAnalystInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
				profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticAnalysts::update_genetic_analyst_availability_status(
			RuntimeOrigin::signed(1),
			AvailabilityStatus::Available
		));

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			RuntimeOrigin::signed(1),
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
		));

		let _genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

		let _add_genetic_data = GeneticData::add_genetic_data(
			RuntimeOrigin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec(),
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_noop!(
			GeneticAnalysisOrders::create_genetic_analysis_order(
				RuntimeOrigin::signed(1),
				_genetic_data_ids[0],
				_genetic_analyst.services[0],
				10,
				Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
				"DeBio Genetic Genetic Link".as_bytes().to_vec(),
				None,
			),
			Error::<Test>::PriceIndexNotFound
		);
	})
}

#[test]
fn cant_create_genetic_analysis_order_when_genetic_analyst_unavailable() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			RuntimeOrigin::signed(1),
			GeneticAnalystInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
				profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticAnalysts::update_genetic_analyst_availability_status(
			RuntimeOrigin::signed(1),
			AvailabilityStatus::Unavailable,
		));

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			RuntimeOrigin::signed(1),
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
		));

		let _genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

		let _add_genetic_data = GeneticData::add_genetic_data(
			RuntimeOrigin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec(),
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_noop!(
			GeneticAnalysisOrders::create_genetic_analysis_order(
				RuntimeOrigin::signed(1),
				_genetic_data_ids[0],
				_genetic_analyst.services[0],
				10,
				Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
				"DeBio Genetic Genetic Link".as_bytes().to_vec(),
				None,
			),
			Error::<Test>::GeneticAnalystUnavailable
		);
	})
}

#[test]
fn cant_create_genetic_analysis_order_when_genetic_data_does_not_exist() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			RuntimeOrigin::signed(1),
			GeneticAnalystInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
				profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticAnalysts::update_genetic_analyst_availability_status(
			RuntimeOrigin::signed(1),
			AvailabilityStatus::Available
		));

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			RuntimeOrigin::signed(1),
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
		));

		let _genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

		let _add_genetic_data = GeneticData::add_genetic_data(
			RuntimeOrigin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec(),
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_noop!(
			GeneticAnalysisOrders::create_genetic_analysis_order(
				RuntimeOrigin::signed(1),
				Keccak256::hash("genetic_data_id".as_bytes()),
				_genetic_analyst.services[0],
				10,
				Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
				"DeBio Genetic Genetic Link".as_bytes().to_vec(),
				None,
			),
			Error::<Test>::GeneticDataDoesNotExist
		);
	})
}

#[test]
fn cant_create_genetic_analysis_order_when_not_owner_of_genetic_data() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			RuntimeOrigin::signed(1),
			GeneticAnalystInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
				profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticAnalysts::update_genetic_analyst_availability_status(
			RuntimeOrigin::signed(1),
			AvailabilityStatus::Available
		));

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			RuntimeOrigin::signed(1),
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
		));

		let _genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

		let _add_genetic_data = GeneticData::add_genetic_data(
			RuntimeOrigin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec(),
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_noop!(
			GeneticAnalysisOrders::create_genetic_analysis_order(
				RuntimeOrigin::signed(2),
				_genetic_data_ids[0],
				_genetic_analyst.services[0],
				10,
				Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
				"DeBio Genetic Genetic Link".as_bytes().to_vec(),
				None,
			),
			Error::<Test>::NotOwnerOfGeneticData
		);
	})
}

#[test]
fn cant_cancel_genetic_analysis_order_when_order_ongoing() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		PalletAccount::<Test>::put(0);

		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			RuntimeOrigin::signed(1),
			GeneticAnalystInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
				profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticAnalysts::update_genetic_analyst_availability_status(
			RuntimeOrigin::signed(1),
			AvailabilityStatus::Available
		));

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			RuntimeOrigin::signed(1),
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
		));

		let _genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

		let _add_genetic_data = GeneticData::add_genetic_data(
			RuntimeOrigin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec(),
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
			RuntimeOrigin::signed(1),
			_genetic_data_ids[0],
			_genetic_analyst.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			"DeBio Genetic Genetic Link".as_bytes().to_vec(),
			None,
		));

		let _genetic_analysis_order_id =
			GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();
		let _genetic_analysis = GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::set_genetic_analysis_order_paid(
			RuntimeOrigin::signed(1),
			_genetic_analysis_order_id
		));

		assert_ok!(GeneticAnalysis::process_genetic_analysis(
			RuntimeOrigin::signed(1),
			_genetic_analysis[0].clone(),
			GeneticAnalysisStatus::InProgress,
		));

		assert_noop!(
			GeneticAnalysisOrders::cancel_genetic_analysis_order(
				RuntimeOrigin::signed(1),
				_genetic_analysis_order_id
			),
			Error::<Test>::OngoingGeneticAnalysisOrderCannotBeCancelled
		);
	})
}

#[test]
fn cant_cancel_genetic_analysis_order_when_not_exist() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_noop!(
			GeneticAnalysisOrders::cancel_genetic_analysis_order(
				RuntimeOrigin::signed(1),
				Keccak256::hash("genetic_analysis_order_id".as_bytes())
			),
			Error::<Test>::GeneticAnalysisOrderNotFound
		);
	})
}

#[test]
fn cant_cancel_genetic_analysis_order_when_unathorized_user() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			RuntimeOrigin::signed(1),
			GeneticAnalystInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
				profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticAnalysts::update_genetic_analyst_availability_status(
			RuntimeOrigin::signed(1),
			AvailabilityStatus::Available
		));

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			RuntimeOrigin::signed(1),
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
		));

		let _genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

		let _add_genetic_data = GeneticData::add_genetic_data(
			RuntimeOrigin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec(),
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
			RuntimeOrigin::signed(1),
			_genetic_data_ids[0],
			_genetic_analyst.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			"DeBio Genetic Genetic Link".as_bytes().to_vec(),
			None,
		));

		let _genetic_analysis_order_id =
			GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();
		let _genetic_analysis = GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

		assert_noop!(
			GeneticAnalysisOrders::cancel_genetic_analysis_order(
				RuntimeOrigin::signed(3),
				_genetic_analysis_order_id
			),
			Error::<Test>::UnauthorizedGeneticAnalysisOrderCancellation
		);
	})
}

#[test]
fn cant_set_genetic_analysis_order_paid_when_unauthorized() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(
			RawOrigin::Root.into(),
			1,
			20000000000000000000u128.saturated_into(),
			0
		));

		EscrowKey::<Test>::put(0);

		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			RuntimeOrigin::signed(1),
			GeneticAnalystInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
				profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticAnalysts::update_genetic_analyst_availability_status(
			RuntimeOrigin::signed(1),
			AvailabilityStatus::Available
		));

		let _price = Price {
			component: "Price Component".as_bytes().to_vec(),
			value: 5000000000000000000u128.saturated_into(),
		};

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			RuntimeOrigin::signed(1),
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency {
					currency: CurrencyType::default(),
					total_price: 10000000000000000000u128.saturated_into(),
					price_components: vec![_price.clone()],
					additional_prices: vec![_price],
				}],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
		));

		let _genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

		let _add_genetic_data = GeneticData::add_genetic_data(
			RuntimeOrigin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec(),
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
			RuntimeOrigin::signed(1),
			_genetic_data_ids[0],
			_genetic_analyst.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			"DeBio Genetic Genetic Link".as_bytes().to_vec(),
			None,
		));

		let _genetic_analysis_order_id =
			GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();

		assert_noop!(
			GeneticAnalysisOrders::set_genetic_analysis_order_paid(
				RuntimeOrigin::signed(3),
				_genetic_analysis_order_id
			),
			Error::<Test>::Unauthorized
		);
	})
}

#[test]
fn cant_set_genetic_analysis_order_paid_when_not_exist() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(
			RawOrigin::Root.into(),
			1,
			20000000000000000000u128.saturated_into(),
			0
		));

		EscrowKey::<Test>::put(3);

		assert_noop!(
			GeneticAnalysisOrders::set_genetic_analysis_order_paid(
				RuntimeOrigin::signed(3),
				Keccak256::hash("genetic_analysis_order_id".as_bytes())
			),
			Error::<Test>::GeneticAnalysisOrderNotFound
		);
	})
}

#[test]
fn cant_set_genetic_analysis_order_paid_when_insufficient_funds() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		PalletAccount::<Test>::put(0);
		EscrowKey::<Test>::put(3);

		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			RuntimeOrigin::signed(1),
			GeneticAnalystInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
				profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticAnalysts::update_genetic_analyst_availability_status(
			RuntimeOrigin::signed(1),
			AvailabilityStatus::Available
		));

		let _price = Price {
			component: "Price Component".as_bytes().to_vec(),
			value: 5000000000000000000u128.saturated_into(),
		};

		let _price_by_currency = PriceByCurrency {
			currency: CurrencyType::default(),
			total_price: 10000000000000000000u128.saturated_into(),
			price_components: vec![_price.clone()],
			additional_prices: vec![_price],
		};

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			RuntimeOrigin::signed(1),
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![_price_by_currency],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
		));

		let _genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

		let _add_genetic_data = GeneticData::add_genetic_data(
			RuntimeOrigin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec(),
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
			RuntimeOrigin::signed(1),
			_genetic_data_ids[0],
			_genetic_analyst.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			"DeBio Genetic Genetic Link".as_bytes().to_vec(),
			None,
		));

		let _genetic_analysis_order_id =
			GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();

		assert_noop!(
			GeneticAnalysisOrders::set_genetic_analysis_order_paid(
				RuntimeOrigin::signed(1),
				_genetic_analysis_order_id,
			),
			Error::<Test>::InsufficientFunds
		);
	})
}

#[test]
fn cant_fulfill_genetic_analysis_order_when_not_exist() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		EscrowKey::<Test>::put(1);

		assert_noop!(
			GeneticAnalysisOrders::fulfill_genetic_analysis_order(
				RuntimeOrigin::signed(1),
				Keccak256::hash("genetic_analysis_order_id".as_bytes())
			),
			Error::<Test>::GeneticAnalysisOrderNotFound
		);
	})
}

#[test]
fn cant_fulfill_genetic_analysis_order_when_unauthorized() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		EscrowKey::<Test>::put(1);

		assert_noop!(
			GeneticAnalysisOrders::fulfill_genetic_analysis_order(
				RuntimeOrigin::signed(4),
				Keccak256::hash("genetic_analysis_order_id".as_bytes())
			),
			Error::<Test>::Unauthorized
		);
	})
}

#[test]
fn cant_fulfill_genetic_analysis_order_when_genetic_analysis_not_process() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));

		PalletAccount::<Test>::put(0);
		EscrowKey::<Test>::put(1);

		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			RuntimeOrigin::signed(1),
			GeneticAnalystInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
				profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticAnalysts::update_genetic_analyst_availability_status(
			RuntimeOrigin::signed(1),
			AvailabilityStatus::Available
		));

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			RuntimeOrigin::signed(1),
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
		));

		let _genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

		let _add_genetic_data = GeneticData::add_genetic_data(
			RuntimeOrigin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec(),
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
			RuntimeOrigin::signed(1),
			_genetic_data_ids[0],
			_genetic_analyst.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			"DeBio Genetic Genetic Link".as_bytes().to_vec(),
			None,
		));

		let _genetic_analysis_order_id =
			GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::set_genetic_analysis_order_paid(
			RuntimeOrigin::signed(1),
			_genetic_analysis_order_id
		));

		assert_noop!(
			GeneticAnalysisOrders::fulfill_genetic_analysis_order(
				RuntimeOrigin::signed(1),
				_genetic_analysis_order_id
			),
			Error::<Test>::GeneticAnalysisNotSuccessfullyProcessed
		);
	})
}

#[test]
fn cant_set_genetic_analysis_order_refunded_when_unauthorized() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		EscrowKey::<Test>::put(3);

		assert_noop!(
			GeneticAnalysisOrders::set_genetic_analysis_order_refunded(
				RuntimeOrigin::signed(4),
				Keccak256::hash("genetic_analysis_order_id".as_bytes())
			),
			Error::<Test>::Unauthorized
		);
	})
}

#[test]
fn cant_set_genetic_analysis_order_refunded_when_not_exist() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		EscrowKey::<Test>::put(3);

		assert_noop!(
			GeneticAnalysisOrders::set_genetic_analysis_order_refunded(
				RuntimeOrigin::signed(3),
				Keccak256::hash("genetic_analysis_order_id".as_bytes())
			),
			Error::<Test>::GeneticAnalysisOrderNotFound
		);
	})
}

#[test]
fn cant_set_genetic_analysis_order_refunded_when_not_expired() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));

		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			RuntimeOrigin::signed(1),
			GeneticAnalystInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
				profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticAnalysts::update_genetic_analyst_availability_status(
			RuntimeOrigin::signed(1),
			AvailabilityStatus::Available
		));

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			RuntimeOrigin::signed(1),
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
		));

		let _genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

		let _add_genetic_data = GeneticData::add_genetic_data(
			RuntimeOrigin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec(),
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
			RuntimeOrigin::signed(1),
			_genetic_data_ids[0],
			_genetic_analyst.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			"DeBio Genetic Genetic Link".as_bytes().to_vec(),
			None,
		));

		EscrowKey::<Test>::put(3);
		PalletAccount::<Test>::put(0);

		let _genetic_analysis_order_id =
			GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::set_genetic_analysis_order_paid(
			RuntimeOrigin::signed(1),
			_genetic_analysis_order_id
		));

		assert_noop!(
			GeneticAnalysisOrders::set_genetic_analysis_order_refunded(
				RuntimeOrigin::signed(3),
				_genetic_analysis_order_id
			),
			Error::<Test>::GeneticAnalysisOrderNotYetExpired
		);
	})
}

#[test]
fn call_event_should_work() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		System::set_block_number(1);
		PalletAccount::<Test>::put(0);
		TreasuryKey::<Test>::put(0);

		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			RuntimeOrigin::signed(1),
			GeneticAnalystInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
				profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
			}
		));

		assert_ok!(GeneticAnalysts::update_genetic_analyst_availability_status(
			RuntimeOrigin::signed(1),
			AvailabilityStatus::Available
		));

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			RuntimeOrigin::signed(1),
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
		));

		let _genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

		let _add_genetic_data = GeneticData::add_genetic_data(
			RuntimeOrigin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec(),
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
			RuntimeOrigin::signed(1),
			_genetic_data_ids[0],
			_genetic_analyst.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			"DeBio Genetic Genetic Link".as_bytes().to_vec(),
			None,
		));

		let _genetic_analysis_order_id =
			GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();
		let _genetic_analysis = GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

		System::assert_last_event(RuntimeEvent::GeneticAnalysisOrders(
			crate::Event::GeneticAnalysisOrderCreated(GeneticAnalysisOrder {
				id: _genetic_analysis_order_id,
				genetic_data_id: _genetic_data_ids[0],
				service_id: _genetic_analyst.services[0],
				customer_id: 1,
				customer_box_public_key: Keccak256::hash(
					"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes(),
				),
				seller_id: 1,
				genetic_analysis_tracking_id: _genetic_analysis[0].clone(),
				genetic_link: "DeBio Genetic Genetic Link".as_bytes().to_vec(),
				asset_id: None,
				currency: CurrencyType::default(),
				prices: PriceByCurrency::default().price_components,
				additional_prices: PriceByCurrency::default().additional_prices,
				total_price: PriceByCurrency::default().total_price,
				status: GeneticAnalysisOrderStatus::default(),
				created_at: 0,
				updated_at: 0,
			}),
		));

		assert_ok!(GeneticAnalysisOrders::cancel_genetic_analysis_order(
			RuntimeOrigin::signed(1),
			_genetic_analysis_order_id
		));

		System::assert_last_event(RuntimeEvent::GeneticAnalysisOrders(
			crate::Event::GeneticAnalysisOrderCancelled(GeneticAnalysisOrder {
				id: _genetic_analysis_order_id,
				genetic_data_id: _genetic_data_ids[0],
				service_id: _genetic_analyst.services[0],
				customer_id: 1,
				customer_box_public_key: Keccak256::hash(
					"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes(),
				),
				seller_id: 1,
				genetic_analysis_tracking_id: _genetic_analysis[0].clone(),
				genetic_link: "DeBio Genetic Genetic Link".as_bytes().to_vec(),
				asset_id: None,
				currency: CurrencyType::default(),
				prices: PriceByCurrency::default().price_components,
				additional_prices: PriceByCurrency::default().additional_prices,
				total_price: PriceByCurrency::default().total_price,
				status: GeneticAnalysisOrderStatus::Cancelled,
				created_at: 0,
				updated_at: 0,
			}),
		));

		let _add_genetic_data = GeneticData::add_genetic_data(
			RuntimeOrigin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec(),
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
			RuntimeOrigin::signed(1),
			_genetic_data_ids[0],
			_genetic_analyst.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			"DeBio Genetic Genetic Link".as_bytes().to_vec(),
			None,
		));

		let _genetic_analysis_order_id =
			GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();
		let _genetic_analysis = GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

		EscrowKey::<Test>::put(3);

		assert_ok!(GeneticAnalysisOrders::set_genetic_analysis_order_paid(
			RuntimeOrigin::signed(1),
			_genetic_analysis_order_id
		));

		System::assert_last_event(RuntimeEvent::GeneticAnalysisOrders(
			crate::Event::GeneticAnalysisOrderPaid(GeneticAnalysisOrder {
				id: _genetic_analysis_order_id,
				genetic_data_id: _genetic_data_ids[0],
				service_id: _genetic_analyst.services[0],
				customer_id: 1,
				customer_box_public_key: Keccak256::hash(
					"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes(),
				),
				seller_id: 1,
				genetic_analysis_tracking_id: _genetic_analysis[0].clone(),
				genetic_link: "DeBio Genetic Genetic Link".as_bytes().to_vec(),
				asset_id: None,
				currency: CurrencyType::default(),
				prices: PriceByCurrency::default().price_components,
				additional_prices: PriceByCurrency::default().additional_prices,
				total_price: PriceByCurrency::default().total_price,
				status: GeneticAnalysisOrderStatus::Paid,
				created_at: 0,
				updated_at: 0,
			}),
		));

		assert_ok!(GeneticAnalysis::submit_genetic_analysis(
			RuntimeOrigin::signed(1),
			_genetic_analysis[0].clone(),
			"report_link".as_bytes().to_vec(),
			Some("comment".as_bytes().to_vec()),
		));

		assert_ok!(GeneticAnalysis::process_genetic_analysis(
			RuntimeOrigin::signed(1),
			_genetic_analysis[0].clone(),
			GeneticAnalysisStatus::ResultReady,
		));

		assert_ok!(GeneticAnalysisOrders::fulfill_genetic_analysis_order(
			RuntimeOrigin::signed(3),
			_genetic_analysis_order_id
		));

		System::assert_last_event(RuntimeEvent::GeneticAnalysisOrders(
			crate::Event::GeneticAnalysisOrderFulfilled(GeneticAnalysisOrder {
				id: _genetic_analysis_order_id,
				genetic_data_id: _genetic_data_ids[0],
				service_id: _genetic_analyst.services[0],
				customer_id: 1,
				customer_box_public_key: Keccak256::hash(
					"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes(),
				),
				seller_id: 1,
				genetic_analysis_tracking_id: _genetic_analysis[0].clone(),
				genetic_link: "DeBio Genetic Genetic Link".as_bytes().to_vec(),
				asset_id: None,
				currency: CurrencyType::default(),
				prices: PriceByCurrency::default().price_components,
				additional_prices: PriceByCurrency::default().additional_prices,
				total_price: PriceByCurrency::default().total_price,
				status: GeneticAnalysisOrderStatus::Fulfilled,
				created_at: 0,
				updated_at: 0,
			}),
		));
	});
}

#[test]
fn update_escrow_key_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		EscrowKey::<Test>::put(2);

		assert_eq!(GeneticAnalysisOrders::admin_key(), Some(2));

		assert_ok!(GeneticAnalysisOrders::update_key(
			RuntimeOrigin::signed(2),
			AccountKeyType::EscrowKey(1)
		));

		assert_eq!(GeneticAnalysisOrders::admin_key(), Some(1));
	})
}

#[test]
fn sudo_update_treasury_key_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(GeneticAnalysisOrders::sudo_update_key(
			RuntimeOrigin::root(),
			AccountKeyType::TreasuryKey(1)
		));

		assert_eq!(GeneticAnalysisOrders::treasury_key(), Some(1));
	})
}
