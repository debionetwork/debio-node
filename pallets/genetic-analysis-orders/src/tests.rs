use crate::{mock::*, Error, EscrowKey, GeneticAnalysisOrder, GeneticAnalysisOrderStatus};
use frame_support::{
	assert_noop, assert_ok,
	sp_runtime::traits::{Hash, Keccak256},
};
use frame_system::RawOrigin;
use genetic_analysis::GeneticAnalysisStatus;
use genetic_analyst_services::GeneticAnalystServiceInfo;
use genetic_analysts::GeneticAnalystInfo;

use primitives_duration::ExpectedDuration;
use primitives_price_and_currency::{CurrencyType, PriceByCurrency};

#[test]
fn create_genetic_analysis_order() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(1),
			GeneticAnalystInfo {
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
			}
		));

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			Origin::signed(1),
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
			Origin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec()
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
			Origin::signed(1),
			_genetic_data_ids[0],
			_genetic_analyst.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
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
				currency: CurrencyType::default(),
				prices: PriceByCurrency::default().price_components,
				additional_prices: PriceByCurrency::default().additional_prices,
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
			Origin::signed(1),
			GeneticAnalystInfo {
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
			}
		));

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			Origin::signed(1),
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
			Origin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec()
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
			Origin::signed(1),
			_genetic_data_ids[0],
			_genetic_analyst.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
		));

		let _genetic_analysis_order_id =
			GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();
		let _genetic_analysis = GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::cancel_genetic_analysis_order(
			Origin::signed(1),
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
				currency: CurrencyType::default(),
				prices: PriceByCurrency::default().price_components,
				additional_prices: PriceByCurrency::default().additional_prices,
				status: GeneticAnalysisOrderStatus::Cancelled,
				created_at: 0,
				updated_at: 0
			})
		);
	})
}

#[test]
fn set_genetic_analysis_order_paid_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(1),
			GeneticAnalystInfo {
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
			}
		));

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			Origin::signed(1),
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
			Origin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec()
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
			Origin::signed(1),
			_genetic_data_ids[0],
			_genetic_analyst.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
		));

		let _genetic_analysis_order_id =
			GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();
		let _genetic_analysis = GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

		EscrowKey::<Test>::put(3);

		assert_ok!(GeneticAnalysisOrders::set_genetic_analysis_order_paid(
			Origin::signed(3),
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
				currency: CurrencyType::default(),
				prices: PriceByCurrency::default().price_components,
				additional_prices: PriceByCurrency::default().additional_prices,
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
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(1),
			GeneticAnalystInfo {
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
			}
		));

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			Origin::signed(1),
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
			Origin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec()
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
			Origin::signed(1),
			_genetic_data_ids[0],
			_genetic_analyst.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
		));

		let _genetic_analysis_order_id =
			GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();
		let _genetic_analysis = GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

		assert_ok!(GeneticAnalysis::submit_genetic_analysis(
			Origin::signed(1),
			_genetic_analysis[0].clone(),
			"report_link".as_bytes().to_vec(),
			Some("comment".as_bytes().to_vec()),
		));

		assert_ok!(GeneticAnalysis::process_genetic_analysis(
			Origin::signed(1),
			_genetic_analysis[0].clone(),
			GeneticAnalysisStatus::ResultReady,
		));

		assert_ok!(GeneticAnalysisOrders::fulfill_genetic_analysis_order(
			Origin::signed(1),
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
				currency: CurrencyType::default(),
				prices: PriceByCurrency::default().price_components,
				additional_prices: PriceByCurrency::default().additional_prices,
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

		EscrowKey::<Test>::put(3);

		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(1),
			GeneticAnalystInfo {
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
			}
		));

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			Origin::signed(1),
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
			Origin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec()
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
			Origin::signed(1),
			_genetic_data_ids[0],
			_genetic_analyst.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
		));

		let _genetic_analysis_order_id =
			GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();
		let _genetic_analysis = GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

		assert_ok!(GeneticAnalysis::submit_genetic_analysis(
			Origin::signed(1),
			_genetic_analysis[0].clone(),
			"report_link".as_bytes().to_vec(),
			Some("comment".as_bytes().to_vec()),
		));

		assert_ok!(GeneticAnalysis::process_genetic_analysis(
			Origin::signed(1),
			_genetic_analysis[0].clone(),
			GeneticAnalysisStatus::Rejected,
		));

		assert_ok!(GeneticAnalysisOrders::set_genetic_analysis_order_refunded(
			Origin::signed(3),
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
				currency: CurrencyType::default(),
				prices: PriceByCurrency::default().price_components,
				additional_prices: PriceByCurrency::default().additional_prices,
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
			Origin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec()
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_noop!(
			GeneticAnalysisOrders::create_genetic_analysis_order(
				Origin::signed(1),
				_genetic_data_ids[0],
				Keccak256::hash("genetic_analyst_serviceId".as_bytes()),
				0,
				Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
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
			Origin::signed(1),
			GeneticAnalystInfo {
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
			}
		));

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			Origin::signed(1),
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
			Origin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec()
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_noop!(
			GeneticAnalysisOrders::create_genetic_analysis_order(
				Origin::signed(1),
				_genetic_data_ids[0],
				_genetic_analyst.services[0],
				10,
				Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
			),
			Error::<Test>::PriceIndexNotFound
		);
	})
}

#[test]
fn cant_cancel_genetic_analysis_order_when_not_exist() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_noop!(
			GeneticAnalysisOrders::cancel_genetic_analysis_order(
				Origin::signed(1),
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
			Origin::signed(1),
			GeneticAnalystInfo {
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
			}
		));

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			Origin::signed(1),
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
			Origin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec()
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
			Origin::signed(1),
			_genetic_data_ids[0],
			_genetic_analyst.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
		));

		let _genetic_analysis_order_id =
			GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();
		let _genetic_analysis = GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

		assert_noop!(
			GeneticAnalysisOrders::cancel_genetic_analysis_order(
				Origin::signed(3),
				_genetic_analysis_order_id
			),
			Error::<Test>::UnauthorizedGeneticAnalysisOrderCancellation
		);
	})
}

#[test]
fn cant_set_genetic_analysis_order_paid_when_unauthorized() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(1),
			GeneticAnalystInfo {
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
			}
		));

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			Origin::signed(1),
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
			Origin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec()
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
			Origin::signed(1),
			_genetic_data_ids[0],
			_genetic_analyst.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
		));

		let _genetic_analysis_order_id =
			GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();

		assert_noop!(
			GeneticAnalysisOrders::set_genetic_analysis_order_paid(
				Origin::signed(3),
				_genetic_analysis_order_id
			),
			Error::<Test>::Unauthorized
		);
	})
}

#[test]
fn cant_set_genetic_analysis_order_paid_when_not_exist() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));

		EscrowKey::<Test>::put(3);

		assert_noop!(
			GeneticAnalysisOrders::set_genetic_analysis_order_paid(
				Origin::signed(3),
				Keccak256::hash("genetic_analysis_order_id".as_bytes())
			),
			Error::<Test>::GeneticAnalysisOrderNotFound
		);
	})
}

#[test]
fn cant_fulfill_genetic_analysis_order_when_not_exist() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_noop!(
			GeneticAnalysisOrders::fulfill_genetic_analysis_order(
				Origin::signed(1),
				Keccak256::hash("genetic_analysis_order_id".as_bytes())
			),
			Error::<Test>::GeneticAnalysisOrderNotFound
		);
	})
}

#[test]
fn cant_fulfill_genetic_analysis_order_when_unauthorized() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(1),
			GeneticAnalystInfo {
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
			}
		));

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			Origin::signed(1),
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
			Origin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec()
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
			Origin::signed(1),
			_genetic_data_ids[0],
			_genetic_analyst.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
		));

		let _genetic_analysis_order_id =
			GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();

		assert_noop!(
			GeneticAnalysisOrders::fulfill_genetic_analysis_order(
				Origin::signed(4),
				_genetic_analysis_order_id
			),
			Error::<Test>::UnauthorizedGeneticAnalysisOrderFulfillment
		);
	})
}

#[test]
fn cant_fulfill_genetic_analysis_order_when_genetic_analysis_not_process() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(1),
			GeneticAnalystInfo {
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
			}
		));

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			Origin::signed(1),
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
			Origin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec()
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
			Origin::signed(1),
			_genetic_data_ids[0],
			_genetic_analyst.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
		));

		let _genetic_analysis_order_id =
			GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();

		assert_noop!(
			GeneticAnalysisOrders::fulfill_genetic_analysis_order(
				Origin::signed(1),
				_genetic_analysis_order_id
			),
			Error::<Test>::GeneticAnalysisNotSuccessfullyProcessed
		);
	})
}

#[test]
fn cant_set_genetic_analysis_order_refunded_when_unauthorized() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));

		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(1),
			GeneticAnalystInfo {
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
			}
		));

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			Origin::signed(1),
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
			Origin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec()
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
			Origin::signed(1),
			_genetic_data_ids[0],
			_genetic_analyst.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
		));

		let _genetic_analysis_order_id =
			GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();
		let _genetic_analysis = GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

		EscrowKey::<Test>::put(3);

		assert_noop!(
			GeneticAnalysisOrders::set_genetic_analysis_order_refunded(
				Origin::signed(3),
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
		assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));
		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(1),
			GeneticAnalystInfo {
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
			}
		));

		assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
			Origin::signed(1),
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
			Origin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec()
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
			Origin::signed(1),
			_genetic_data_ids[0],
			_genetic_analyst.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
		));

		let _genetic_analysis_order_id =
			GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();
		let _genetic_analysis = GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

		System::assert_last_event(Event::GeneticAnalysisOrders(
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
				currency: CurrencyType::default(),
				prices: PriceByCurrency::default().price_components,
				additional_prices: PriceByCurrency::default().additional_prices,
				status: GeneticAnalysisOrderStatus::default(),
				created_at: 0,
				updated_at: 0,
			}),
		));

		assert_ok!(GeneticAnalysisOrders::cancel_genetic_analysis_order(
			Origin::signed(1),
			_genetic_analysis_order_id
		));

		System::assert_last_event(Event::GeneticAnalysisOrders(
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
				currency: CurrencyType::default(),
				prices: PriceByCurrency::default().price_components,
				additional_prices: PriceByCurrency::default().additional_prices,
				status: GeneticAnalysisOrderStatus::Cancelled,
				created_at: 0,
				updated_at: 0,
			}),
		));

		let _add_genetic_data = GeneticData::add_genetic_data(
			Origin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec()
		);

		let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
			Origin::signed(1),
			_genetic_data_ids[0],
			_genetic_analyst.services[0],
			0,
			Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
		));

		let _genetic_analysis_order_id =
			GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();
		let _genetic_analysis = GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

		EscrowKey::<Test>::put(3);

		assert_ok!(GeneticAnalysisOrders::set_genetic_analysis_order_paid(
			Origin::signed(3),
			_genetic_analysis_order_id
		));

		System::assert_last_event(Event::GeneticAnalysisOrders(
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
				currency: CurrencyType::default(),
				prices: PriceByCurrency::default().price_components,
				additional_prices: PriceByCurrency::default().additional_prices,
				status: GeneticAnalysisOrderStatus::Paid,
				created_at: 0,
				updated_at: 0,
			}),
		));

		assert_ok!(GeneticAnalysis::submit_genetic_analysis(
			Origin::signed(1),
			_genetic_analysis[0].clone(),
			"report_link".as_bytes().to_vec(),
			Some("comment".as_bytes().to_vec()),
		));

		assert_ok!(GeneticAnalysis::process_genetic_analysis(
			Origin::signed(1),
			_genetic_analysis[0].clone(),
			GeneticAnalysisStatus::ResultReady,
		));

		assert_ok!(GeneticAnalysisOrders::fulfill_genetic_analysis_order(
			Origin::signed(1),
			_genetic_analysis_order_id
		));

		System::assert_last_event(Event::GeneticAnalysisOrders(
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
				currency: CurrencyType::default(),
				prices: PriceByCurrency::default().price_components,
				additional_prices: PriceByCurrency::default().additional_prices,
				status: GeneticAnalysisOrderStatus::Fulfilled,
				created_at: 0,
				updated_at: 0,
			}),
		));

		assert_ok!(GeneticAnalysis::process_genetic_analysis(
			Origin::signed(1),
			_genetic_analysis[0].clone(),
			GeneticAnalysisStatus::Rejected,
		));

		assert_ok!(GeneticAnalysisOrders::set_genetic_analysis_order_refunded(
			Origin::signed(3),
			_genetic_analysis_order_id
		));

		System::assert_last_event(Event::GeneticAnalysisOrders(
			crate::Event::GeneticAnalysisOrderRefunded(GeneticAnalysisOrder {
				id: _genetic_analysis_order_id,
				genetic_data_id: _genetic_data_ids[0],
				service_id: _genetic_analyst.services[0],
				customer_id: 1,
				customer_box_public_key: Keccak256::hash(
					"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes(),
				),
				seller_id: 1,
				genetic_analysis_tracking_id: _genetic_analysis[0].clone(),
				currency: CurrencyType::default(),
				prices: PriceByCurrency::default().price_components,
				additional_prices: PriceByCurrency::default().additional_prices,
				status: GeneticAnalysisOrderStatus::Refunded,
				created_at: 0,
				updated_at: 0,
			}),
		));
	});
}
