mod mock;

#[cfg(test)]
mod tests {
	use crate::mock::*;

	use frame_support::{
		assert_noop, assert_ok,
		sp_runtime::traits::{Hash, Keccak256},
	};

	use genetic_analysis::{Error, GeneticAnalysisStatus};
	use genetic_analysis_orders::PalletAccount;
	use genetic_analyst_services::GeneticAnalystServiceInfo;
	use genetic_analysts::GeneticAnalystInfo;

	use traits_genetic_analysis::GeneticAnalysisTracking;

	use primitives_availability_status::AvailabilityStatus;
	use primitives_duration::ExpectedDuration;
	use primitives_price_and_currency::PriceByCurrency;
	use primitives_tracking_id::TrackingId;

	#[test]
	fn reject_genetic_analysis_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
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

			assert_ok!(UserProfile::set_eth_address(
				RuntimeOrigin::signed(1),
				EthereumAddress([b'X'; 20])
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
				}
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

			let _genetic_analysis =
				GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

			assert_ok!(GeneticAnalysis::reject_genetic_analysis(
				RuntimeOrigin::signed(1),
				_genetic_analysis[0].clone(),
				"Reject DNA Title".as_bytes().to_vec(),
				"Reject DNA Description".as_bytes().to_vec()
			));

			let _genetic_analysis_info =
				GeneticAnalysis::genetic_analysis_by_genetic_analysis_tracking_id(
					_genetic_analysis[0].clone(),
				)
				.unwrap();

			assert_eq!(
				_genetic_analysis_info.get_genetic_analysis_tracking_id(),
				&_genetic_analysis[0]
			);
			assert!(_genetic_analysis_info.is_rejected());
		})
	}

	#[test]
	fn cannot_reject_genetic_analysis_not_found() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			assert_noop!(
				GeneticAnalysis::reject_genetic_analysis(
					RuntimeOrigin::signed(1),
					TrackingId::from_vec("xxxxxxxxxxxxxxxxxxxxx".as_bytes().to_vec()),
					"Reject DNA Title".as_bytes().to_vec(),
					"Reject DNA Description".as_bytes().to_vec()
				),
				Error::<Test>::GeneticAnalysisNotFound
			);
		})
	}

	#[test]
	fn cannot_reject_genetic_analysis_unauthorized() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
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

			assert_ok!(UserProfile::set_eth_address(
				RuntimeOrigin::signed(1),
				EthereumAddress([b'X'; 20])
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
				}
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

			let _genetic_analysis =
				GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

			assert_noop!(
				GeneticAnalysis::reject_genetic_analysis(
					RuntimeOrigin::signed(2),
					_genetic_analysis[0].clone(),
					"Reject DNA Title".as_bytes().to_vec(),
					"Reject DNA Description".as_bytes().to_vec()
				),
				Error::<Test>::Unauthorized
			);
		})
	}

	#[test]
	fn process_genetic_analysis_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
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

			assert_ok!(UserProfile::set_eth_address(
				RuntimeOrigin::signed(1),
				EthereumAddress([b'X'; 20])
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
				}
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

			let _genetic_analysis =
				GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

			assert_ok!(GeneticAnalysis::submit_genetic_analysis(
				RuntimeOrigin::signed(1),
				_genetic_analysis[0].clone(),
				"Genetic Analysis report_link".as_bytes().to_vec(),
				Some("Genetic Analysis comments".as_bytes().to_vec())
			));

			let _genetic_analysis_ =
				GeneticAnalysis::genetic_analysis_by_genetic_analysis_tracking_id(
					_genetic_analysis[0].clone(),
				)
				.unwrap();

			assert_eq!(
				_genetic_analysis_.genetic_analysis_tracking_id,
				_genetic_analysis[0].clone()
			);
			assert_eq!(_genetic_analysis_.genetic_analyst_id, 1);
			assert_eq!(_genetic_analysis_.owner_id, 1);
			assert_eq!(
				_genetic_analysis_.comment,
				Some("Genetic Analysis comments".as_bytes().to_vec())
			);
			assert_eq!(
				_genetic_analysis_.report_link,
				"Genetic Analysis report_link".as_bytes().to_vec()
			);

			assert_ok!(GeneticAnalysis::process_genetic_analysis(
				RuntimeOrigin::signed(1),
				_genetic_analysis[0].clone(),
				GeneticAnalysisStatus::ResultReady
			));

			let _genetic_analysis_info =
				GeneticAnalysis::genetic_analysis_by_genetic_analysis_tracking_id(
					_genetic_analysis[0].clone(),
				)
				.unwrap();

			assert_eq!(
				_genetic_analysis_info.get_genetic_analysis_tracking_id(),
				&_genetic_analysis[0]
			);
			assert!(_genetic_analysis_info.process_success());
		})
	}

	#[test]
	fn accept_genetic_analysis_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
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

			assert_ok!(UserProfile::set_eth_address(
				RuntimeOrigin::signed(1),
				EthereumAddress([b'X'; 20])
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
				}
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

			let _genetic_analysis =
				GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

			assert_ok!(GeneticAnalysis::process_genetic_analysis(
				RuntimeOrigin::signed(1),
				_genetic_analysis[0].clone(),
				GeneticAnalysisStatus::InProgress
			));

			let _genetic_analysis_info =
				GeneticAnalysis::genetic_analysis_by_genetic_analysis_tracking_id(
					_genetic_analysis[0].clone(),
				)
				.unwrap();

			assert_eq!(
				_genetic_analysis_info.get_genetic_analysis_tracking_id(),
				&_genetic_analysis[0]
			);
			assert_eq!(_genetic_analysis_info.status, GeneticAnalysisStatus::InProgress);
		})
	}

	#[test]
	fn cannot_process_genetic_analysis_not_found() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			assert_noop!(
				GeneticAnalysis::process_genetic_analysis(
					RuntimeOrigin::signed(1),
					TrackingId::from_vec("xxxxxxxxxxxxxxxxxxxxx".as_bytes().to_vec()),
					GeneticAnalysisStatus::ResultReady
				),
				Error::<Test>::GeneticAnalysisNotFound
			);
		})
	}

	#[test]
	fn cannot_process_genetic_analysis_unauthorized() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
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

			assert_ok!(UserProfile::set_eth_address(
				RuntimeOrigin::signed(1),
				EthereumAddress([b'X'; 20])
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
				}
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

			let _genetic_analysis =
				GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

			assert_ok!(GeneticAnalysis::submit_genetic_analysis(
				RuntimeOrigin::signed(1),
				_genetic_analysis[0].clone(),
				"Genetic Analysis report_link".as_bytes().to_vec(),
				Some("Genetic Analysis comments".as_bytes().to_vec())
			));

			let _genetic_analysis_ =
				GeneticAnalysis::genetic_analysis_by_genetic_analysis_tracking_id(
					_genetic_analysis[0].clone(),
				)
				.unwrap();

			assert_eq!(
				_genetic_analysis_.genetic_analysis_tracking_id,
				_genetic_analysis[0].clone()
			);
			assert_eq!(_genetic_analysis_.genetic_analyst_id, 1);
			assert_eq!(_genetic_analysis_.owner_id, 1);
			assert_eq!(
				_genetic_analysis_.comment,
				Some("Genetic Analysis comments".as_bytes().to_vec())
			);
			assert_eq!(
				_genetic_analysis_.report_link,
				"Genetic Analysis report_link".as_bytes().to_vec()
			);

			assert_noop!(
				GeneticAnalysis::process_genetic_analysis(
					RuntimeOrigin::signed(2),
					_genetic_analysis[0].clone(),
					GeneticAnalysisStatus::ResultReady
				),
				Error::<Test>::Unauthorized
			);
		})
	}

	#[test]
	fn cannot_process_genetic_analysis_not_submitted() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
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

			assert_ok!(UserProfile::set_eth_address(
				RuntimeOrigin::signed(1),
				EthereumAddress([b'X'; 20])
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
				}
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

			let _genetic_analysis =
				GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

			assert_noop!(
				GeneticAnalysis::process_genetic_analysis(
					RuntimeOrigin::signed(1),
					_genetic_analysis[0].clone(),
					GeneticAnalysisStatus::ResultReady
				),
				Error::<Test>::GeneticAnalysisNotYetSubmitted
			);
		})
	}

	#[test]
	fn submit_genetic_analysis_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
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

			assert_ok!(UserProfile::set_eth_address(
				RuntimeOrigin::signed(1),
				EthereumAddress([b'X'; 20])
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
				}
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

			let _genetic_analysis =
				GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

			assert_ok!(GeneticAnalysis::submit_genetic_analysis(
				RuntimeOrigin::signed(1),
				_genetic_analysis[0].clone(),
				"Genetic Analysis report_link".as_bytes().to_vec(),
				Some("Genetic Analysis comments".as_bytes().to_vec())
			));

			let _genetic_analysis_ =
				GeneticAnalysis::genetic_analysis_by_genetic_analysis_tracking_id(
					_genetic_analysis[0].clone(),
				)
				.unwrap();

			assert_eq!(
				_genetic_analysis_.genetic_analysis_tracking_id,
				_genetic_analysis[0].clone()
			);
			assert_eq!(_genetic_analysis_.genetic_analyst_id, 1);
			assert_eq!(_genetic_analysis_.owner_id, 1);
			assert_eq!(
				_genetic_analysis_.comment,
				Some("Genetic Analysis comments".as_bytes().to_vec())
			);
			assert_eq!(
				_genetic_analysis_.report_link,
				"Genetic Analysis report_link".as_bytes().to_vec()
			);
		})
	}

	#[test]
	fn cannot_submit_genetic_analysis_not_found() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
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

			assert_ok!(UserProfile::set_eth_address(
				RuntimeOrigin::signed(1),
				EthereumAddress([b'X'; 20])
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
				}
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

			assert_noop!(
				GeneticAnalysis::submit_genetic_analysis(
					RuntimeOrigin::signed(1),
					TrackingId::from_vec("xxxxxxxxxxxxxxxxxxxxx".as_bytes().to_vec()),
					"Genetic Analysis report_link".as_bytes().to_vec(),
					Some("Genetic Analysis comments".as_bytes().to_vec())
				),
				Error::<Test>::GeneticAnalysisNotFound
			);
		})
	}

	#[test]
	fn cannot_submit_genetic_analysis_unauthorized() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
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

			assert_ok!(UserProfile::set_eth_address(
				RuntimeOrigin::signed(1),
				EthereumAddress([b'X'; 20])
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
				}
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

			let _genetic_analysis =
				GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

			assert_noop!(
				GeneticAnalysis::submit_genetic_analysis(
					RuntimeOrigin::signed(2),
					_genetic_analysis[0].clone(),
					"Genetic Analysis report_link".as_bytes().to_vec(),
					Some("Genetic Analysis comments".as_bytes().to_vec())
				),
				Error::<Test>::Unauthorized
			);
		})
	}
}
