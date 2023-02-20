mod mock;

#[cfg(test)]
mod tests {
	use crate::mock::*;

	use labs::LabInfo;
	use services::{Error, Service, ServiceInfo};
	use traits_services::types::ServiceFlow;

	use primitives_duration::ExpectedDuration;
	use primitives_price_and_currency::PriceByCurrency;

	use frame_support::{
		assert_noop, assert_ok,
		sp_runtime::traits::{Hash, Keccak256},
	};
	use primitives_area_code::{CityCode, CountryCode, RegionCode};

	#[test]
	fn create_service_works() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(Labs::register_lab(
				RuntimeOrigin::signed(1),
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

			assert_ok!(UserProfile::set_eth_address(
				RuntimeOrigin::signed(1),
				EthereumAddress([b'X'; 20])
			));

			assert_ok!(Services::create_service(
				RuntimeOrigin::signed(1),
				ServiceInfo {
					name: "DeBio service name".as_bytes().to_vec(),
					prices_by_currency: vec![PriceByCurrency::default()],
					expected_duration: ExpectedDuration::default(),
					category: "DeBio service category".as_bytes().to_vec(),
					description: "DeBio service description".as_bytes().to_vec(),
					dna_collection_process: "DeBio service dna_collection_process"
						.as_bytes()
						.to_vec(),
					test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
					long_description: Some("DeBio service long_description".as_bytes().to_vec()),
					image: Some("DeBio service image".as_bytes().to_vec()),
				},
				ServiceFlow::default()
			));

			let lab = Labs::lab_by_account_id(1).unwrap();

			assert_eq!(
				Services::service_by_id(lab.services[0]),
				Some(Service {
					id: lab.services[0],
					owner_id: 1,
					info: ServiceInfo {
						name: "DeBio service name".as_bytes().to_vec(),
						prices_by_currency: vec![PriceByCurrency::default()],
						expected_duration: ExpectedDuration::default(),
						category: "DeBio service category".as_bytes().to_vec(),
						description: "DeBio service description".as_bytes().to_vec(),
						dna_collection_process: "DeBio service dna_collection_process"
							.as_bytes()
							.to_vec(),
						test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
						long_description: Some(
							"DeBio service long_description".as_bytes().to_vec()
						),
						image: Some("DeBio service image".as_bytes().to_vec()),
					},
					service_flow: ServiceFlow::default()
				})
			);

			assert_eq!(Services::services_count_by_owner(1), Some(1));
		})
	}

	#[test]
	fn update_service_works() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(Labs::register_lab(
				RuntimeOrigin::signed(1),
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

			assert_ok!(UserProfile::set_eth_address(
				RuntimeOrigin::signed(1),
				EthereumAddress([b'X'; 20])
			));

			assert_ok!(Services::create_service(
				RuntimeOrigin::signed(1),
				ServiceInfo {
					name: "DeBio service name".as_bytes().to_vec(),
					prices_by_currency: vec![PriceByCurrency::default()],
					expected_duration: ExpectedDuration::default(),
					category: "DeBio service category".as_bytes().to_vec(),
					description: "DeBio service description".as_bytes().to_vec(),
					dna_collection_process: "DeBio service dna_collection_process"
						.as_bytes()
						.to_vec(),
					test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
					long_description: Some("DeBio service long_description".as_bytes().to_vec()),
					image: Some("DeBio service image".as_bytes().to_vec()),
				},
				ServiceFlow::default()
			));

			let lab = Labs::lab_by_account_id(1).unwrap();

			assert_ok!(Services::update_service(
				RuntimeOrigin::signed(1),
				lab.services[0],
				ServiceInfo {
					name: "DeBio service name 2".as_bytes().to_vec(),
					prices_by_currency: vec![PriceByCurrency::default()],
					expected_duration: ExpectedDuration::default(),
					category: "DeBio service category 2".as_bytes().to_vec(),
					description: "DeBio service description 2".as_bytes().to_vec(),
					dna_collection_process: "DeBio service dna_collection_process 2"
						.as_bytes()
						.to_vec(),
					test_result_sample: "DeBio service test_result_sample 2".as_bytes().to_vec(),
					long_description: Some("DeBio service long_description 2".as_bytes().to_vec()),
					image: Some("DeBio service image 2".as_bytes().to_vec()),
				}
			));

			assert_eq!(
				Services::service_by_id(lab.services[0]),
				Some(Service {
					id: lab.services[0],
					owner_id: 1,
					info: ServiceInfo {
						name: "DeBio service name 2".as_bytes().to_vec(),
						prices_by_currency: vec![PriceByCurrency::default()],
						expected_duration: ExpectedDuration::default(),
						category: "DeBio service category 2".as_bytes().to_vec(),
						description: "DeBio service description 2".as_bytes().to_vec(),
						dna_collection_process: "DeBio service dna_collection_process 2"
							.as_bytes()
							.to_vec(),
						test_result_sample: "DeBio service test_result_sample 2"
							.as_bytes()
							.to_vec(),
						long_description: Some(
							"DeBio service long_description 2".as_bytes().to_vec()
						),
						image: Some("DeBio service image 2".as_bytes().to_vec()),
					},
					service_flow: ServiceFlow::default()
				})
			);

			assert_eq!(Services::services_count_by_owner(1), Some(1));
		})
	}

	#[test]
	fn delete_service_works() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(Labs::register_lab(
				RuntimeOrigin::signed(1),
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

			assert_ok!(UserProfile::set_eth_address(
				RuntimeOrigin::signed(1),
				EthereumAddress([b'X'; 20])
			));

			assert_ok!(Services::create_service(
				RuntimeOrigin::signed(1),
				ServiceInfo {
					name: "DeBio service name".as_bytes().to_vec(),
					prices_by_currency: vec![PriceByCurrency::default()],
					expected_duration: ExpectedDuration::default(),
					category: "DeBio service category".as_bytes().to_vec(),
					description: "DeBio service description".as_bytes().to_vec(),
					dna_collection_process: "DeBio service dna_collection_process"
						.as_bytes()
						.to_vec(),
					test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
					long_description: Some("DeBio service long_description".as_bytes().to_vec()),
					image: Some("DeBio service image".as_bytes().to_vec()),
				},
				ServiceFlow::default()
			));

			let lab = Labs::lab_by_account_id(1).unwrap();

			assert_ok!(Services::delete_service(RuntimeOrigin::signed(1), lab.services[0]));

			assert_eq!(Services::services_count_by_owner(1), Some(0));
		})
	}

	#[test]
	fn not_allowed_to_create_service() {
		ExternalityBuilder::build().execute_with(|| {
			assert_noop!(
				Services::create_service(
					RuntimeOrigin::signed(1),
					ServiceInfo {
						name: "DeBio service name".as_bytes().to_vec(),
						prices_by_currency: vec![PriceByCurrency::default()],
						expected_duration: ExpectedDuration::default(),
						category: "DeBio service category".as_bytes().to_vec(),
						description: "DeBio service description".as_bytes().to_vec(),
						dna_collection_process: "DeBio service dna_collection_process"
							.as_bytes()
							.to_vec(),
						test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
						long_description: Some(
							"DeBio service long_description".as_bytes().to_vec()
						),
						image: Some("DeBio service image".as_bytes().to_vec()),
					},
					ServiceFlow::default()
				),
				Error::<Test>::NotAllowedToCreate
			);
		})
	}

	#[test]
	fn update_service_does_not_exist() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(Labs::register_lab(
				RuntimeOrigin::signed(1),
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
				Services::update_service(
					RuntimeOrigin::signed(1),
					Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
					ServiceInfo {
						name: "DeBio service name 2".as_bytes().to_vec(),
						prices_by_currency: vec![PriceByCurrency::default()],
						expected_duration: ExpectedDuration::default(),
						category: "DeBio service category 2".as_bytes().to_vec(),
						description: "DeBio service description 2".as_bytes().to_vec(),
						dna_collection_process: "DeBio service dna_collection_process 2"
							.as_bytes()
							.to_vec(),
						test_result_sample: "DeBio service test_result_sample 2"
							.as_bytes()
							.to_vec(),
						long_description: Some(
							"DeBio service long_description 2".as_bytes().to_vec()
						),
						image: Some("DeBio service image 2".as_bytes().to_vec()),
					}
				),
				Error::<Test>::ServiceDoesNotExist
			);
		})
	}

	#[test]
	fn update_service_not_owner() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(Labs::register_lab(
				RuntimeOrigin::signed(1),
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

			assert_ok!(UserProfile::set_eth_address(
				RuntimeOrigin::signed(1),
				EthereumAddress([b'X'; 20])
			));

			assert_ok!(Services::create_service(
				RuntimeOrigin::signed(1),
				ServiceInfo {
					name: "DeBio service name".as_bytes().to_vec(),
					prices_by_currency: vec![PriceByCurrency::default()],
					expected_duration: ExpectedDuration::default(),
					category: "DeBio service category".as_bytes().to_vec(),
					description: "DeBio service description".as_bytes().to_vec(),
					dna_collection_process: "DeBio service dna_collection_process"
						.as_bytes()
						.to_vec(),
					test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
					long_description: Some("DeBio service long_description".as_bytes().to_vec()),
					image: Some("DeBio service image".as_bytes().to_vec()),
				},
				ServiceFlow::default()
			));

			let lab = Labs::lab_by_account_id(1).unwrap();

			assert_noop!(
				Services::update_service(
					RuntimeOrigin::signed(2),
					lab.services[0],
					ServiceInfo {
						name: "DeBio service name 2".as_bytes().to_vec(),
						prices_by_currency: vec![PriceByCurrency::default()],
						expected_duration: ExpectedDuration::default(),
						category: "DeBio service category 2".as_bytes().to_vec(),
						description: "DeBio service description 2".as_bytes().to_vec(),
						dna_collection_process: "DeBio service dna_collection_process 2"
							.as_bytes()
							.to_vec(),
						test_result_sample: "DeBio service test_result_sample 2"
							.as_bytes()
							.to_vec(),
						long_description: Some(
							"DeBio service long_description 2".as_bytes().to_vec()
						),
						image: Some("DeBio service image 2".as_bytes().to_vec()),
					}
				),
				Error::<Test>::NotServiceOwner
			);
		})
	}

	#[test]
	fn delete_service_does_not_exist() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(Labs::register_lab(
				RuntimeOrigin::signed(1),
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
				Services::delete_service(
					RuntimeOrigin::signed(1),
					Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes())
				),
				Error::<Test>::ServiceDoesNotExist
			);
		})
	}

	#[test]
	fn delete_service_not_owner() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(Labs::register_lab(
				RuntimeOrigin::signed(1),
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

			assert_ok!(UserProfile::set_eth_address(
				RuntimeOrigin::signed(1),
				EthereumAddress([b'X'; 20])
			));

			assert_ok!(Services::create_service(
				RuntimeOrigin::signed(1),
				ServiceInfo {
					name: "DeBio service name".as_bytes().to_vec(),
					prices_by_currency: vec![PriceByCurrency::default()],
					expected_duration: ExpectedDuration::default(),
					category: "DeBio service category".as_bytes().to_vec(),
					description: "DeBio service description".as_bytes().to_vec(),
					dna_collection_process: "DeBio service dna_collection_process"
						.as_bytes()
						.to_vec(),
					test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
					long_description: Some("DeBio service long_description".as_bytes().to_vec()),
					image: Some("DeBio service image".as_bytes().to_vec()),
				},
				ServiceFlow::default()
			));

			let lab = Labs::lab_by_account_id(1).unwrap();

			assert_noop!(
				Services::delete_service(RuntimeOrigin::signed(2), lab.services[0]),
				Error::<Test>::NotServiceOwner
			);
		})
	}
}
