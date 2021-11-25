use crate::{mock::*, Error, Hospital, HospitalInfo};
use frame_support::{assert_noop, assert_ok};
use primitives_area_code::{CityCode, CountryCode, CountryRegionCode, RegionCode};

#[test]
fn register_hospital_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Hospitals::register_hospital(
			Origin::signed(1),
			HospitalInfo {
				name: "DeBio Hospital".as_bytes().to_vec(),
				email: "DeBio Email".as_bytes().to_vec(),
				country: CountryCode::from_vec("DC".as_bytes().to_vec()),
				region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
				city: CityCode::from_vec("City".as_bytes().to_vec()),
				address: "DeBio Address".as_bytes().to_vec(),
				latitude: Some("DeBio Latitude".as_bytes().to_vec()),
				longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
				profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
			}
		));

		assert_eq!(
			Hospitals::hospital_by_account_id(1),
			Some(Hospital {
				account_id: 1,
				certifications: Vec::new(),
				info: HospitalInfo {
					name: "DeBio Hospital".as_bytes().to_vec(),
					email: "DeBio Email".as_bytes().to_vec(),
					country: CountryCode::from_vec("DC".as_bytes().to_vec()),
					region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
					city: CityCode::from_vec("City".as_bytes().to_vec()),
					address: "DeBio Address".as_bytes().to_vec(),
					latitude: Some("DeBio Latitude".as_bytes().to_vec()),
					longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
					profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				}
			})
		);

		let country_region_code = CountryRegionCode::from_vec("DC-DBIO".as_bytes().to_vec());
		let city_code = CityCode::from_vec("City".as_bytes().to_vec());

		assert_eq!(
			Hospitals::hospitals_by_country_region_city(&country_region_code, &city_code),
			Some(vec![1])
		);

		assert_eq!(Hospitals::hospital_count(), Some(1));

		assert_eq!(
			Hospitals::hospital_count_by_country_region_city(&country_region_code, &city_code),
			Some(1)
		);
	})
}

#[test]
fn update_hospital_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Hospitals::register_hospital(
			Origin::signed(1),
			HospitalInfo {
				name: "DeBio Hospital".as_bytes().to_vec(),
				email: "DeBio Email".as_bytes().to_vec(),
				country: CountryCode::from_vec("DC".as_bytes().to_vec()),
				region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
				city: CityCode::from_vec("City".as_bytes().to_vec()),
				address: "DeBio Address".as_bytes().to_vec(),
				latitude: Some("DeBio Latitude".as_bytes().to_vec()),
				longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
				profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
			}
		));

		assert_ok!(Hospitals::update_hospital(
			Origin::signed(1),
			HospitalInfo {
				name: "My Hospital".as_bytes().to_vec(),
				email: "DeBio Email".as_bytes().to_vec(),
				country: CountryCode::from_vec("DC".as_bytes().to_vec()),
				region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
				city: CityCode::from_vec("City".as_bytes().to_vec()),
				address: "DeBio Address".as_bytes().to_vec(),
				latitude: Some("DeBio Latitude".as_bytes().to_vec()),
				longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
				profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
			}
		));

		assert_eq!(
			Hospitals::hospital_by_account_id(1),
			Some(Hospital {
				account_id: 1,
				certifications: Vec::new(),
				info: HospitalInfo {
					name: "My Hospital".as_bytes().to_vec(),
					email: "DeBio Email".as_bytes().to_vec(),
					country: CountryCode::from_vec("DC".as_bytes().to_vec()),
					region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
					city: CityCode::from_vec("City".as_bytes().to_vec()),
					address: "DeBio Address".as_bytes().to_vec(),
					latitude: Some("DeBio Latitude".as_bytes().to_vec()),
					longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
					profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				}
			})
		);

		assert_ok!(Hospitals::update_hospital(
			Origin::signed(1),
			HospitalInfo {
				name: "My Hospital".as_bytes().to_vec(),
				email: "DeBio Email".as_bytes().to_vec(),
				country: CountryCode::from_vec("ID".as_bytes().to_vec()),
				region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
				city: CityCode::from_vec("City".as_bytes().to_vec()),
				address: "DeBio Address".as_bytes().to_vec(),
				latitude: Some("DeBio Latitude".as_bytes().to_vec()),
				longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
				profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
			}
		));

		assert_eq!(
			Hospitals::hospital_by_account_id(1),
			Some(Hospital {
				account_id: 1,
				certifications: Vec::new(),
				info: HospitalInfo {
					name: "My Hospital".as_bytes().to_vec(),
					email: "DeBio Email".as_bytes().to_vec(),
					country: CountryCode::from_vec("ID".as_bytes().to_vec()),
					region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
					city: CityCode::from_vec("City".as_bytes().to_vec()),
					address: "DeBio Address".as_bytes().to_vec(),
					latitude: Some("DeBio Latitude".as_bytes().to_vec()),
					longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
					profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				}
			})
		);

		let old_country_region_code = CountryRegionCode::from_vec("DB-DBIO".as_bytes().to_vec());
		let old_city_code = CityCode::from_vec("City".as_bytes().to_vec());

		let new_country_region_code = CountryRegionCode::from_vec("ID-DBIO".as_bytes().to_vec());
		let new_city_code = CityCode::from_vec("City".as_bytes().to_vec());

		assert_eq!(
			Hospitals::hospitals_by_country_region_city(&old_country_region_code, &old_city_code),
			None
		);

		assert_eq!(
			Hospitals::hospitals_by_country_region_city(&new_country_region_code, &new_city_code),
			Some(vec![1])
		);

		assert_eq!(
			Hospitals::hospital_count_by_country_region_city(
				&old_country_region_code,
				&old_city_code
			),
			None
		);

		assert_eq!(
			Hospitals::hospital_count_by_country_region_city(
				&new_country_region_code,
				&new_city_code
			),
			Some(1)
		);
	})
}

#[test]
fn deregister_hospital_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Hospitals::register_hospital(
			Origin::signed(1),
			HospitalInfo {
				name: "DeBio Hospital".as_bytes().to_vec(),
				email: "DeBio Email".as_bytes().to_vec(),
				country: CountryCode::from_vec("DC".as_bytes().to_vec()),
				region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
				city: CityCode::from_vec("City".as_bytes().to_vec()),
				address: "DeBio Address".as_bytes().to_vec(),
				latitude: Some("DeBio Latitude".as_bytes().to_vec()),
				longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
				profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
			}
		));

		let country_region_code = CountryRegionCode::from_vec("DC-DB".as_bytes().to_vec());
		let city_code = CityCode::from_vec("CITY".as_bytes().to_vec());

		assert_ok!(Hospitals::deregister_hospital(Origin::signed(1)));

		assert_eq!(Hospitals::hospital_by_account_id(1), None);

		assert_eq!(
			Hospitals::hospitals_by_country_region_city(&country_region_code, &city_code),
			None
		);

		assert_eq!(
			Hospitals::hospital_count_by_country_region_city(&country_region_code, &city_code),
			None
		)
	})
}

#[test]
fn cant_update_and_unregister_hospital_when_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			Hospitals::update_hospital(
				Origin::signed(1),
				HospitalInfo {
					name: "DeBio Hospital".as_bytes().to_vec(),
					email: "DeBio Email".as_bytes().to_vec(),
					country: CountryCode::from_vec("DC".as_bytes().to_vec()),
					region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
					city: CityCode::from_vec("City".as_bytes().to_vec()),
					address: "DeBio Address".as_bytes().to_vec(),
					latitude: Some("DeBio Latitude".as_bytes().to_vec()),
					longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
					profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				}
			),
			Error::<Test>::HospitalDoesNotExist
		);

		assert_noop!(
			Hospitals::deregister_hospital(Origin::signed(1)),
			Error::<Test>::HospitalDoesNotExist
		);
	})
}

#[test]
fn cant_register_hospital_when_already_registered() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Hospitals::register_hospital(
			Origin::signed(1),
			HospitalInfo {
				name: "DeBio Hospital".as_bytes().to_vec(),
				email: "DeBio Email".as_bytes().to_vec(),
				country: CountryCode::from_vec("DC".as_bytes().to_vec()),
				region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
				city: CityCode::from_vec("City".as_bytes().to_vec()),
				address: "DeBio Address".as_bytes().to_vec(),
				latitude: Some("DeBio Latitude".as_bytes().to_vec()),
				longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
				profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
			}
		));

		assert_noop!(
			Hospitals::register_hospital(
				Origin::signed(1),
				HospitalInfo {
					name: "DeBio Hospital".as_bytes().to_vec(),
					email: "DeBio Email".as_bytes().to_vec(),
					country: CountryCode::from_vec("DC".as_bytes().to_vec()),
					region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
					city: CityCode::from_vec("City".as_bytes().to_vec()),
					address: "DeBio Address".as_bytes().to_vec(),
					latitude: Some("DeBio Latitude".as_bytes().to_vec()),
					longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
					profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				}
			),
			Error::<Test>::HospitalAlreadyRegistered
		);
	})
}

#[test]
fn call_event_should_work() {
	ExternalityBuilder::build().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Hospitals::register_hospital(
			Origin::signed(1),
			HospitalInfo {
				name: "DeBio Hospital".as_bytes().to_vec(),
				email: "DeBio Email".as_bytes().to_vec(),
				country: CountryCode::from_vec("DC".as_bytes().to_vec()),
				region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
				city: CityCode::from_vec("City".as_bytes().to_vec()),
				address: "DeBio Address".as_bytes().to_vec(),
				latitude: Some("DeBio Latitude".as_bytes().to_vec()),
				longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
				profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
			}
		));

		System::assert_last_event(Event::Hospitals(crate::Event::HospitalRegistered(
			Hospital {
				account_id: 1,
				certifications: Vec::new(),
				info: HospitalInfo {
					name: "DeBio Hospital".as_bytes().to_vec(),
					email: "DeBio Email".as_bytes().to_vec(),
					country: CountryCode::from_vec("DC".as_bytes().to_vec()),
					region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
					city: CityCode::from_vec("City".as_bytes().to_vec()),
					address: "DeBio Address".as_bytes().to_vec(),
					latitude: Some("DeBio Latitude".as_bytes().to_vec()),
					longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
					profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				},
			},
			1,
		)));

		assert_ok!(Hospitals::update_hospital(
			Origin::signed(1),
			HospitalInfo {
				name: "DeBio Hospital".as_bytes().to_vec(),
				email: "DeBio Email".as_bytes().to_vec(),
				country: CountryCode::from_vec("ID".as_bytes().to_vec()),
				region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
				city: CityCode::from_vec("City".as_bytes().to_vec()),
				address: "DeBio Address".as_bytes().to_vec(),
				latitude: Some("DeBio Latitude".as_bytes().to_vec()),
				longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
				profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
			}
		));

		System::assert_last_event(Event::Hospitals(crate::Event::HospitalUpdated(
			Hospital {
				account_id: 1,
				certifications: Vec::new(),
				info: HospitalInfo {
					name: "DeBio Hospital".as_bytes().to_vec(),
					email: "DeBio Email".as_bytes().to_vec(),
					country: CountryCode::from_vec("ID".as_bytes().to_vec()),
					region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
					city: CityCode::from_vec("City".as_bytes().to_vec()),
					address: "DeBio Address".as_bytes().to_vec(),
					latitude: Some("DeBio Latitude".as_bytes().to_vec()),
					longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
					profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				},
			},
			1,
		)));

		assert_ok!(Hospitals::deregister_hospital(Origin::signed(1)));

		System::assert_last_event(Event::Hospitals(crate::Event::HospitalDeleted(
			Hospital {
				account_id: 1,
				certifications: Vec::new(),
				info: HospitalInfo {
					name: "DeBio Hospital".as_bytes().to_vec(),
					email: "DeBio Email".as_bytes().to_vec(),
					country: CountryCode::from_vec("ID".as_bytes().to_vec()),
					region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
					city: CityCode::from_vec("City".as_bytes().to_vec()),
					address: "DeBio Address".as_bytes().to_vec(),
					latitude: Some("DeBio Latitude".as_bytes().to_vec()),
					longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
					profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				},
			},
			1,
		)));
	});
}
