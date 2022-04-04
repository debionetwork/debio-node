mod mock;

#[cfg(test)]
mod tests {
	use crate::mock::*;

	use labs::{
		Error, Event as EventC, Lab, LabInfo, LabVerifierKey,
		PalletAccount,
	};

	use frame_support::{
		assert_noop, assert_ok,
		sp_runtime::{
			traits::{Hash, Keccak256},
			SaturatedConversion,
		},
	};
	use frame_system::RawOrigin;

	use genetic_testing::{DnaSampleTracking, DnaSampleStatus, DnaTestResultSubmission};
	use orders::{Order, OrderStatus};
	use services::ServiceInfo;

	use primitives_area_code::{CityCode, CountryCode, CountryRegionCode, RegionCode};
	use primitives_stake_status::StakeStatus;
	use primitives_verification_status::VerificationStatus;
	
	use traits_services::types::{ServiceFlow, CurrencyType, PriceByCurrency, ExpectedDuration};
	
	#[test]
	fn register_lab_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
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
	
			assert_eq!(
				Labs::lab_by_account_id(1),
				Some(Lab {
					account_id: 1,
					services: Vec::new(),
					certifications: Vec::new(),
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					stake_amount: 0u128.saturated_into(),
					stake_status: StakeStatus::default(),
					verification_status: VerificationStatus::default(),
					info: LabInfo {
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
				})
			);
	
			let country_region_code = CountryRegionCode::from_vec("DC-DB".as_bytes().to_vec());
			let city_code = CityCode::from_vec("CITY".as_bytes().to_vec());
	
			assert_eq!(
				Labs::labs_by_country_region_city(&country_region_code, &city_code),
				Some(vec![1])
			);
			assert_eq!(Labs::lab_count(), Some(1));
			assert_eq!(
				Labs::lab_count_by_country_region_city(&country_region_code, &city_code),
				Some(1)
			);
		})
	}
	
	#[test]
	fn update_lab_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
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
	
			assert_ok!(Labs::update_lab(
				Origin::signed(1),
				LabInfo {
					box_public_key: Keccak256::hash(
						"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()
					),
					name: "DeBio Labs".as_bytes().to_vec(),
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
	
			assert_eq!(
				Labs::lab_by_account_id(1),
				Some(Lab {
					account_id: 1,
					services: Vec::new(),
					certifications: Vec::new(),
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					stake_amount: 0u128.saturated_into(),
					stake_status: StakeStatus::default(),
					verification_status: VerificationStatus::default(),
					info: LabInfo {
						box_public_key: Keccak256::hash(
							"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()
						),
						name: "DeBio Labs".as_bytes().to_vec(),
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
				})
			);
	
			let old_country_region_code = CountryRegionCode::from_vec("DC-DB".as_bytes().to_vec());
			let old_city_code = CityCode::from_vec("CITY".as_bytes().to_vec());
	
			assert_ok!(Labs::update_lab(
				Origin::signed(1),
				LabInfo {
					box_public_key: Keccak256::hash(
						"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()
					),
					name: "DeBio Labs".as_bytes().to_vec(),
					email: "DeBio Email".as_bytes().to_vec(),
					country: CountryCode::from_vec("ID".as_bytes().to_vec()),
					region: RegionCode::from_vec("WJ".as_bytes().to_vec()),
					city: CityCode::from_vec("CITY".as_bytes().to_vec()),
					address: "DeBio Address".as_bytes().to_vec(),
					phone_number: "+6281394653625".as_bytes().to_vec(),
					website: "DeBio Website".as_bytes().to_vec(),
					latitude: Some("DeBio Latitude".as_bytes().to_vec()),
					longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
					profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				}
			));
	
			assert_eq!(
				Labs::lab_by_account_id(1),
				Some(Lab {
					account_id: 1,
					services: Vec::new(),
					certifications: Vec::new(),
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					stake_amount: 0u128.saturated_into(),
					stake_status: StakeStatus::default(),
					verification_status: VerificationStatus::default(),
					info: LabInfo {
						box_public_key: Keccak256::hash(
							"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()
						),
						name: "DeBio Labs".as_bytes().to_vec(),
						email: "DeBio Email".as_bytes().to_vec(),
						country: CountryCode::from_vec("ID".as_bytes().to_vec()),
						region: RegionCode::from_vec("WJ".as_bytes().to_vec()),
						city: CityCode::from_vec("CITY".as_bytes().to_vec()),
						address: "DeBio Address".as_bytes().to_vec(),
						phone_number: "+6281394653625".as_bytes().to_vec(),
						website: "DeBio Website".as_bytes().to_vec(),
						latitude: Some("DeBio Latitude".as_bytes().to_vec()),
						longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
						profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
					}
				})
			);
	
			assert_eq!(
				Labs::labs_by_country_region_city(&old_country_region_code, &old_city_code),
				Some(Vec::new())
			);
			assert_eq!(
				Labs::lab_count_by_country_region_city(&old_country_region_code, &old_city_code),
				Some(0)
			);
	
			let new_country_region_code = CountryRegionCode::from_vec("ID-WJ".as_bytes().to_vec());
			let new_city_code = CityCode::from_vec("CITY".as_bytes().to_vec());
	
			assert_eq!(
				Labs::labs_by_country_region_city(&new_country_region_code, &new_city_code),
				Some(vec![1])
			);
			assert_eq!(
				Labs::lab_count_by_country_region_city(new_country_region_code, new_city_code),
				Some(1)
			);
		})
	}
	
	#[test]
	fn update_lab_verification_status_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
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
	
			LabVerifierKey::<Test>::put(2);
	
			assert_ok!(Labs::update_lab_verification_status(
				Origin::signed(2),
				1,
				VerificationStatus::Verified,
			));
	
			assert_eq!(
				Labs::lab_by_account_id(1),
				Some(Lab {
					account_id: 1,
					services: Vec::new(),
					certifications: Vec::new(),
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					stake_amount: 0u128.saturated_into(),
					stake_status: StakeStatus::default(),
					verification_status: VerificationStatus::Verified,
					info: LabInfo {
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
				})
			);
		})
	}
	
	#[test]
	fn deregister_lab_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
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
	
			let country_region_code = CountryRegionCode::from_vec("DC-DB".as_bytes().to_vec());
			let city_code = CityCode::from_vec("CITY".as_bytes().to_vec());
	
			assert_ok!(Labs::deregister_lab(Origin::signed(1)));
	
			assert_eq!(Labs::lab_by_account_id(1), None);
	
			assert_eq!(
				Labs::labs_by_country_region_city(&country_region_code, &city_code),
				Some(Vec::new())
			);
	
			assert_eq!(Labs::lab_count(), Some(0));
	
			assert_eq!(
				Labs::lab_count_by_country_region_city(&country_region_code, &city_code),
				Some(0)
			);
		})
	}
	
	#[test]
	fn cant_register_lab_when_already_registered() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
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
	
			assert_noop!(
				Labs::register_lab(
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
				),
				Error::<Test>::LabAlreadyRegistered
			);
		})
	}
	
	#[test]
	fn cant_update_lab_verification_status_when_not_admin() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
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
	
			assert_noop!(
				Labs::update_lab_verification_status(
					Origin::signed(2),
					1,
					VerificationStatus::Verified,
				),
				Error::<Test>::Unauthorized
			);
		})
	}
	
	#[test]
	fn cant_update_lab_verification_status_when_not_exist() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			LabVerifierKey::<Test>::put(2);
	
			assert_noop!(
				Labs::update_lab_verification_status(
					Origin::signed(2),
					1,
					VerificationStatus::Verified,
				),
				Error::<Test>::LabDoesNotExist
			);
		})
	}
	
	#[test]
	fn cant_update_and_delete_lab_when_not_exist() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			assert_noop!(
				Labs::update_lab(
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
				),
				Error::<Test>::LabDoesNotExist
			);
	
			LabVerifierKey::<Test>::put(2);
	
			assert_noop!(
				Labs::update_lab_verification_status(
					Origin::signed(2),
					1,
					VerificationStatus::Verified
				),
				Error::<Test>::LabDoesNotExist
			);
	
			assert_noop!(Labs::deregister_lab(Origin::signed(1)), Error::<Test>::LabDoesNotExist);
		})
	}
	
	#[test]
	fn call_event_should_work() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			System::set_block_number(1);
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
	
			System::assert_last_event(Event::Labs(EventC::LabRegistered(
				Lab {
					account_id: 1,
					services: Vec::new(),
					certifications: Vec::new(),
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					stake_amount: 0u128.saturated_into(),
					stake_status: StakeStatus::default(),
					verification_status: VerificationStatus::default(),
					info: LabInfo {
						box_public_key: Keccak256::hash(
							"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
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
					},
				},
				1,
			)));
	
			assert_ok!(Labs::update_lab(
				Origin::signed(1),
				LabInfo {
					box_public_key: Keccak256::hash(
						"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()
					),
					name: "DeBio Labs".as_bytes().to_vec(),
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
	
			System::assert_last_event(Event::Labs(EventC::LabUpdated(
				Lab {
					account_id: 1,
					services: Vec::new(),
					certifications: Vec::new(),
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					stake_amount: 0u128.saturated_into(),
					stake_status: StakeStatus::default(),
					verification_status: VerificationStatus::default(),
					info: LabInfo {
						box_public_key: Keccak256::hash(
							"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
						),
						name: "DeBio Labs".as_bytes().to_vec(),
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
					},
				},
				1,
			)));
	
			LabVerifierKey::<Test>::put(2);
	
			assert_ok!(Labs::update_lab_verification_status(
				Origin::signed(2),
				1,
				VerificationStatus::Verified
			));
	
			System::assert_last_event(Event::Labs(EventC::LabUpdateVerificationStatus(
				Lab {
					account_id: 1,
					services: Vec::new(),
					certifications: Vec::new(),
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					stake_amount: 0u128.saturated_into(),
					stake_status: StakeStatus::default(),
					verification_status: VerificationStatus::Verified,
					info: LabInfo {
						box_public_key: Keccak256::hash(
							"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
						),
						name: "DeBio Labs".as_bytes().to_vec(),
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
					},
				},
				2,
			)));
	
			assert_ok!(Labs::deregister_lab(Origin::signed(1)));
			System::assert_last_event(Event::Labs(EventC::LabDeregistered(
				Lab {
					account_id: 1,
					services: Vec::new(),
					certifications: Vec::new(),
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					stake_amount: 0u128.saturated_into(),
					stake_status: StakeStatus::default(),
					verification_status: VerificationStatus::Verified,
					info: LabInfo {
						box_public_key: Keccak256::hash(
							"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
						),
						name: "DeBio Labs".as_bytes().to_vec(),
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
					},
				},
				1,
			)))
		})
	}
	
	#[test]
	fn stake_lab_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			assert_ok!(Balances::set_balance(
				RawOrigin::Root.into(),
				1,
				60000000000000000000000u128.saturated_into(),
				0
			));
	
			LabVerifierKey::<Test>::put(2);
	
			assert_ok!(Labs::update_minimum_stake_amount(
				Origin::signed(2),
				60000000000000000000000u128.saturated_into(),
			));
	
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
	
			assert_ok!(Labs::stake_lab(Origin::signed(1),));
	
			assert_eq!(
				Labs::lab_by_account_id(1),
				Some(Lab {
					account_id: 1,
					services: Vec::new(),
					certifications: Vec::new(),
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					stake_amount: 60000000000000000000000u128.saturated_into(),
					stake_status: StakeStatus::Staked,
					verification_status: VerificationStatus::default(),
					info: LabInfo {
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
				})
			);
		})
	}
	
	#[test]
	fn cant_stake_lab_when_not_exist() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			assert_noop!(Labs::stake_lab(Origin::signed(1),), Error::<Test>::LabDoesNotExist);
		})
	}
	
	#[test]
	fn cant_stake_lab_when_already_staked() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			assert_ok!(Balances::set_balance(
				RawOrigin::Root.into(),
				1,
				60000000000000000000000u128.saturated_into(),
				0
			));
	
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
	
			assert_ok!(Labs::stake_lab(Origin::signed(1),));
	
			assert_eq!(
				Labs::lab_by_account_id(1),
				Some(Lab {
					account_id: 1,
					services: Vec::new(),
					certifications: Vec::new(),
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					stake_amount: 50000000000000000000000u128.saturated_into(),
					stake_status: StakeStatus::Staked,
					verification_status: VerificationStatus::default(),
					info: LabInfo {
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
				})
			);
	
			assert_noop!(Labs::stake_lab(Origin::signed(1),), Error::<Test>::LabAlreadyStaked);
		})
	}
	
	#[test]
	fn stake_lab_when_waiting_for_unstake_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			assert_ok!(Balances::set_balance(
				RawOrigin::Root.into(),
				1,
				60000000000000000000000u128.saturated_into(),
				0
			));
	
			PalletAccount::<Test>::put(4);
			LabVerifierKey::<Test>::put(2);
	
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
	
			assert_ok!(Labs::stake_lab(Origin::signed(1),));
	
			assert_ok!(Labs::unstake_lab(Origin::signed(1),));
	
			assert_ok!(Labs::stake_lab(Origin::signed(1),));
	
			assert_eq!(
				Labs::lab_by_account_id(1),
				Some(Lab {
					account_id: 1,
					services: Vec::new(),
					certifications: Vec::new(),
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					stake_amount: 50000000000000000000000u128.saturated_into(),
					stake_status: StakeStatus::Staked,
					verification_status: VerificationStatus::default(),
					info: LabInfo {
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
				})
			);
		})
	}
	
	#[test]
	fn cant_stake_lab_when_insufficient_funds() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
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
	
			assert_noop!(Labs::stake_lab(Origin::signed(1),), Error::<Test>::InsufficientFunds);
	
			assert_eq!(
				Labs::lab_by_account_id(1),
				Some(Lab {
					account_id: 1,
					services: Vec::new(),
					certifications: Vec::new(),
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					stake_amount: 0u128.saturated_into(),
					stake_status: StakeStatus::default(),
					verification_status: VerificationStatus::default(),
					info: LabInfo {
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
				})
			);
		})
	}
	
	#[test]
	fn update_minimum_stake_amount_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			LabVerifierKey::<Test>::put(2);
	
			assert_ok!(Labs::update_minimum_stake_amount(
				Origin::signed(2),
				60000000000000000000000u128.saturated_into(),
			));
	
			assert_eq!(
				Labs::minimum_stake_amount(),
				Some(60000000000000000000000u128.saturated_into())
			);
		})
	}
	
	#[test]
	fn cant_update_minimum_stake_amount_when_unauthorized() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			LabVerifierKey::<Test>::put(3);
	
			assert_noop!(
				Labs::update_minimum_stake_amount(
					Origin::signed(2),
					60000000000000000000000u128.saturated_into(),
				),
				Error::<Test>::Unauthorized
			);
		})
	}
	
	#[test]
	fn update_unstake_time_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			LabVerifierKey::<Test>::put(2);
	
			assert_ok!(Labs::update_unstake_time(Origin::signed(2), 1000u64.saturated_into(),));
	
			assert_eq!(Labs::unstake_time(), Some(1000u64.saturated_into()));
		})
	}
	
	#[test]
	fn cant_update_unstake_time_when_unauthorized() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			LabVerifierKey::<Test>::put(3);
	
			assert_noop!(
				Labs::update_unstake_time(Origin::signed(2), 1000u64.saturated_into(),),
				Error::<Test>::Unauthorized
			);
		})
	}
	
	#[test]
	fn update_admin_key_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			LabVerifierKey::<Test>::put(2);
	
			assert_eq!(Labs::admin_key(), 2);
	
			assert_ok!(Labs::update_admin_key(Origin::signed(2), 1,));
	
			assert_eq!(Labs::admin_key(), 1);
		})
	}
	
	#[test]
	fn cant_update_admin_key_when_unauthorized() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			LabVerifierKey::<Test>::put(3);
	
			assert_noop!(Labs::update_admin_key(Origin::signed(2), 1,), Error::<Test>::Unauthorized);
		})
	}
	
	#[test]
	fn sudo_update_admin_key_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			assert_ok!(Labs::sudo_update_admin_key(Origin::root(), 1));
	
			assert_eq!(Labs::admin_key(), 1);
		})
	}
	
	#[test]
	fn cant_sudo_update_admin_key_when_not_sudo() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			assert_noop!(Labs::update_admin_key(Origin::signed(2), 1,), Error::<Test>::Unauthorized);
		})
	}
	
	#[test]
	fn unstake_lab_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			assert_ok!(Balances::set_balance(
				RawOrigin::Root.into(),
				1,
				60000000000000000000000u128.saturated_into(),
				0
			));
	
			PalletAccount::<Test>::put(4);
			LabVerifierKey::<Test>::put(2);
	
			assert_ok!(Labs::update_minimum_stake_amount(Origin::signed(2), 0u128.saturated_into(),));
	
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
	
			assert_ok!(Labs::stake_lab(Origin::signed(1),));
	
			assert_ok!(Labs::unstake_lab(Origin::signed(1),));
	
			assert_eq!(
				Labs::lab_by_account_id(1),
				Some(Lab {
					account_id: 1,
					services: Vec::new(),
					certifications: Vec::new(),
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					stake_amount: 0u128.saturated_into(),
					stake_status: StakeStatus::WaitingForUnstaked,
					verification_status: VerificationStatus::default(),
					info: LabInfo {
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
				})
			);
		})
	}
	
	#[test]
	fn cant_unstake_lab_when_not_exist() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			assert_noop!(Labs::stake_lab(Origin::signed(1),), Error::<Test>::LabDoesNotExist);
		})
	}
	
	#[test]
	fn cant_unstake_lab_when_not_staked() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
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
	
			assert_noop!(Labs::unstake_lab(Origin::signed(1),), Error::<Test>::LabIsNotStaked);
		})
	}
	
	#[test]
	fn cant_unstake_lab_when_pending_order_exists() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			assert_ok!(Balances::set_balance(
				RawOrigin::Root.into(),
				1,
				60000000000000000000000u128.saturated_into(),
				0
			));
	
			PalletAccount::<Test>::put(4);
			LabVerifierKey::<Test>::put(2);
	
			assert_ok!(Labs::update_minimum_stake_amount(Origin::signed(2), 0u128.saturated_into(),));
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
	
			assert_ok!(Labs::stake_lab(Origin::signed(1),));

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
	
			assert_noop!(Labs::unstake_lab(Origin::signed(1),), Error::<Test>::LabHasPendingOrders);
		})
	}
	
	#[test]
	fn unstake_lab_after_pending_order_is_rejected() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			assert_ok!(Balances::set_balance(
				RawOrigin::Root.into(),
				1,
				60000000000000000000000u128.saturated_into(),
				0
			));
	
			PalletAccount::<Test>::put(4);
			LabVerifierKey::<Test>::put(2);
	
			assert_ok!(Labs::update_minimum_stake_amount(Origin::signed(2), 0u128.saturated_into(),));
	
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
	
			assert_ok!(Labs::stake_lab(Origin::signed(1),));

			assert_ok!(Services::create_service(
				Origin::signed(1),
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

			let _lab = Labs::lab_by_account_id(1).unwrap();

			assert_ok!(Orders::create_order(
				Origin::signed(2),
				_lab.services[0],
				0,
				Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
				ServiceFlow::StakingRequestService
			));

			let _dna_sample = GeneticTesting::dna_samples_by_lab_id(1).unwrap();

			assert_ok!(GeneticTesting::reject_dna_sample(
				Origin::signed(1),
				_dna_sample[0].clone(),
				"Reject DNA Title".as_bytes().to_vec(),
				"Reject DNA Description".as_bytes().to_vec()
			));

			let _dna_sample_info =
				GeneticTesting::dna_sample_by_tracking_id(_dna_sample[0].clone()).unwrap();

			assert_eq!(_dna_sample_info.get_tracking_id(), &_dna_sample[0]);
			assert_eq!(_dna_sample_info.is_rejected(), true);
	
			assert_ok!(Labs::unstake_lab(Origin::signed(1),));
		})
	}
	
	#[test]
	fn unstake_lab_after_pending_order_is_result_ready() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			assert_ok!(Balances::set_balance(
				RawOrigin::Root.into(),
				1,
				60000000000000000000000u128.saturated_into(),
				0
			));
	
			PalletAccount::<Test>::put(4);
			LabVerifierKey::<Test>::put(2);
	
			assert_ok!(Labs::update_minimum_stake_amount(Origin::signed(2), 0u128.saturated_into(),));
	
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
	
			assert_ok!(Labs::stake_lab(Origin::signed(1),));
	
			assert_ok!(UserProfile::set_eth_address(Origin::signed(1), EthereumAddress([b'X'; 20])));

			assert_ok!(Services::create_service(
				Origin::signed(1),
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

			let _lab = Labs::lab_by_account_id(1).unwrap();

			assert_ok!(Orders::create_order(
				Origin::signed(2),
				_lab.services[0],
				0,
				Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
				ServiceFlow::StakingRequestService
			));

			let _dna_sample = GeneticTesting::dna_samples_by_lab_id(1).unwrap();

			assert_ok!(GeneticTesting::submit_test_result(
				Origin::signed(1),
				_dna_sample[0].clone(),
				DnaTestResultSubmission {
					comments: Some("DNA Test Result comments".as_bytes().to_vec()),
					result_link: Some("DNA Test Result result_link".as_bytes().to_vec()),
					report_link: Some("DNA Test Result report_link".as_bytes().to_vec())
				}
			));

			let _dna_test_result =
				GeneticTesting::dna_test_result_by_tracking_id(_dna_sample[0].clone()).unwrap();

			assert_eq!(_dna_test_result.tracking_id, _dna_sample[0].clone());
			assert_eq!(_dna_test_result.lab_id, Some(1));
			assert_eq!(_dna_test_result.owner_id, 2);
			assert_eq!(
				_dna_test_result.comments,
				Some("DNA Test Result comments".as_bytes().to_vec())
			);
			assert_eq!(
				_dna_test_result.result_link,
				Some("DNA Test Result result_link".as_bytes().to_vec())
			);
			assert_eq!(
				_dna_test_result.report_link,
				Some("DNA Test Result report_link".as_bytes().to_vec())
			);

			assert_ok!(GeneticTesting::process_dna_sample(
				Origin::signed(1),
				_dna_sample[0].clone(),
				DnaSampleStatus::ResultReady
			));

			let _dna_sample_info =
				GeneticTesting::dna_sample_by_tracking_id(_dna_sample[0].clone()).unwrap();

			assert_eq!(_dna_sample_info.get_tracking_id(), &_dna_sample[0]);
			assert_eq!(_dna_sample_info.process_success(), true);
	
			assert_ok!(Labs::unstake_lab(Origin::signed(1),));
		})
	}
	
	#[test]
	fn retrieve_unstake_amount_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			assert_ok!(Balances::set_balance(
				RawOrigin::Root.into(),
				1,
				60000000000000000000000u128.saturated_into(),
				0
			));
	
			PalletAccount::<Test>::put(4);
			LabVerifierKey::<Test>::put(2);
	
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
	
			assert_ok!(Labs::stake_lab(Origin::signed(1),));
	
			assert_ok!(Labs::unstake_lab(Origin::signed(1),));
	
			assert_eq!(
				Labs::lab_by_account_id(1),
				Some(Lab {
					account_id: 1,
					services: Vec::new(),
					certifications: Vec::new(),
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					stake_amount: 50000000000000000000000u128.saturated_into(),
					stake_status: StakeStatus::WaitingForUnstaked,
					verification_status: VerificationStatus::default(),
					info: LabInfo {
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
				})
			);
	
			assert_ok!(Labs::retrieve_unstake_amount(Origin::signed(2), 1,));
	
			assert_eq!(
				Labs::lab_by_account_id(1),
				Some(Lab {
					account_id: 1,
					services: Vec::new(),
					certifications: Vec::new(),
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					stake_amount: 0u128.saturated_into(),
					stake_status: StakeStatus::default(),
					verification_status: VerificationStatus::default(),
					info: LabInfo {
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
				})
			);
		})
	}
	
	#[test]
	fn cant_retrieve_unstake_amount_when_not_exist() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			LabVerifierKey::<Test>::put(2);
	
			assert_noop!(
				Labs::retrieve_unstake_amount(Origin::signed(2), 1,),
				Error::<Test>::LabDoesNotExist
			);
		})
	}
	
	#[test]
	fn cant_retrieve_unstake_amount_when_not_waiting_for_unstake() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			LabVerifierKey::<Test>::put(2);
	
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
	
			assert_noop!(
				Labs::retrieve_unstake_amount(Origin::signed(2), 1,),
				Error::<Test>::LabIsNotWaitingForUnstake
			);
		})
	}
	
	#[test]
	fn cant_retrieve_unstake_amount_before_unstake_time() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			assert_ok!(Balances::set_balance(
				RawOrigin::Root.into(),
				1,
				60000000000000000000000u128.saturated_into(),
				0
			));
	
			PalletAccount::<Test>::put(4);
			LabVerifierKey::<Test>::put(2);
	
			assert_ok!(Labs::update_unstake_time(Origin::signed(2), 100000u64.saturated_into(),));
	
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
	
			assert_ok!(Labs::stake_lab(Origin::signed(1),));
	
			assert_ok!(Labs::unstake_lab(Origin::signed(1),));
	
			assert_eq!(
				Labs::lab_by_account_id(1),
				Some(Lab {
					account_id: 1,
					services: Vec::new(),
					certifications: Vec::new(),
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 100000u64.saturated_into(),
					stake_amount: 50000000000000000000000u128.saturated_into(),
					stake_status: StakeStatus::WaitingForUnstaked,
					verification_status: VerificationStatus::default(),
					info: LabInfo {
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
				})
			);
	
			assert_noop!(
				Labs::retrieve_unstake_amount(Origin::signed(2), 1,),
				Error::<Test>::LabCannotUnstakeBeforeUnstakeTime
			);
		})
	}
	
	#[test]
	fn cant_unstake_lab_when_insufficient_pallet_funds() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			LabVerifierKey::<Test>::put(2);
	
			assert_ok!(Balances::set_balance(
				RawOrigin::Root.into(),
				1,
				60000000000000000000000u128.saturated_into(),
				0
			));
	
			assert_ok!(Labs::update_minimum_stake_amount(
				Origin::signed(2),
				60000000000000000000000u128.saturated_into(),
			));
	
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
	
			assert_ok!(Labs::stake_lab(Origin::signed(1),));
	
			assert_ok!(Labs::unstake_lab(Origin::signed(1),));
	
			assert_ok!(Balances::set_balance(
				RawOrigin::Root.into(),
				PalletAccount::<Test>::get(),
				0u128.saturated_into(),
				0
			));
	
			assert_noop!(
				Labs::retrieve_unstake_amount(Origin::signed(2), 1,),
				Error::<Test>::InsufficientPalletFunds
			);
		})
	}
}
