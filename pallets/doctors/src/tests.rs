use crate::{mock::*, Doctor, DoctorInfo, Error};
use frame_support::{assert_noop, assert_ok};
use primitives_area_code::{CityCode, CountryCode, CountryRegionCode, RegionCode};

#[test]
fn register_doctor_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Doctors::register_doctor(
			RuntimeOrigin::signed(1),
			DoctorInfo {
				name: "DeBio Doctor".as_bytes().to_vec(),
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
			Doctors::doctor_by_account_id(1),
			Some(Doctor {
				account_id: 1,
				certifications: Vec::new(),
				info: DoctorInfo {
					name: "DeBio Doctor".as_bytes().to_vec(),
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

		assert_ok!(Doctors::register_doctor(
			RuntimeOrigin::signed(2),
			DoctorInfo {
				name: "DeBio Doctor".as_bytes().to_vec(),
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
			Doctors::doctors_by_country_region_city(&country_region_code, &city_code),
			Some(vec![1, 2])
		);

		assert_eq!(Doctors::doctor_count(), Some(2),);

		assert_eq!(
			Doctors::doctor_count_by_country_region_city(&country_region_code, &city_code),
			Some(2),
		)
	})
}

#[test]
fn update_doctor_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Doctors::register_doctor(
			RuntimeOrigin::signed(1),
			DoctorInfo {
				name: "DeBio Doctor".as_bytes().to_vec(),
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

		assert_ok!(Doctors::update_doctor(
			RuntimeOrigin::signed(1),
			DoctorInfo {
				name: "Abdul Hakim".as_bytes().to_vec(),
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
			Doctors::doctor_by_account_id(1),
			Some(Doctor {
				account_id: 1,
				certifications: Vec::new(),
				info: DoctorInfo {
					name: "Abdul Hakim".as_bytes().to_vec(),
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

		let old_country_region_code = CountryRegionCode::from_vec("DC-DBIO".as_bytes().to_vec());
		let old_city_code = CityCode::from_vec("City".as_bytes().to_vec());

		assert_ok!(Doctors::update_doctor(
			RuntimeOrigin::signed(1),
			DoctorInfo {
				name: "Abdul Hakim".as_bytes().to_vec(),
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
			Doctors::doctors_by_country_region_city(&old_country_region_code, &old_city_code),
			Some(Vec::new())
		);

		assert_eq!(
			Doctors::doctor_count_by_country_region_city(&old_country_region_code, &old_city_code),
			Some(0)
		);

		let new_country_region_code = CountryRegionCode::from_vec("ID-DBIO".as_bytes().to_vec());
		let new_city_code = CityCode::from_vec("City".as_bytes().to_vec());

		assert_eq!(
			Doctors::doctors_by_country_region_city(&new_country_region_code, &new_city_code),
			Some(vec![1])
		);

		assert_eq!(
			Doctors::doctor_count_by_country_region_city(&new_country_region_code, &new_city_code),
			Some(1)
		);
	})
}

#[test]
fn deregister_doctor_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Doctors::register_doctor(
			RuntimeOrigin::signed(1),
			DoctorInfo {
				name: "DeBio Doctor".as_bytes().to_vec(),
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

		assert_ok!(Doctors::deregister_doctor(RuntimeOrigin::signed(1),));

		assert_eq!(Doctors::doctor_by_account_id(1), None);

		let country_region_code = CountryRegionCode::from_vec("DC-DBIO".as_bytes().to_vec());
		let city_code = CityCode::from_vec("City".as_bytes().to_vec());

		assert_eq!(
			Doctors::doctors_by_country_region_city(&country_region_code, &city_code),
			Some(Vec::new())
		);

		assert_eq!(Doctors::doctor_count(), Some(0),);

		assert_eq!(
			Doctors::doctor_count_by_country_region_city(&country_region_code, &city_code),
			Some(0),
		)
	})
}

#[test]
fn cant_register_doctor_when_already_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Doctors::register_doctor(
			RuntimeOrigin::signed(1),
			DoctorInfo {
				name: "DeBio Doctor".as_bytes().to_vec(),
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
			Doctors::register_doctor(
				RuntimeOrigin::signed(1),
				DoctorInfo {
					name: "DeBio Doctor".as_bytes().to_vec(),
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
			Error::<Test>::DoctorAlreadyRegistered
		);
	})
}

#[test]
fn cant_update_and_deregister_doctor_when_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			Doctors::update_doctor(
				RuntimeOrigin::signed(1),
				DoctorInfo {
					name: "DeBio Doctor".as_bytes().to_vec(),
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
			Error::<Test>::DoctorDoesNotExist
		);

		assert_noop!(
			Doctors::deregister_doctor(RuntimeOrigin::signed(1)),
			Error::<Test>::DoctorDoesNotExist
		);
	})
}

#[test]
fn call_event_should_work() {
	ExternalityBuilder::build().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(Doctors::register_doctor(
			RuntimeOrigin::signed(1),
			DoctorInfo {
				name: "DeBio Doctor".as_bytes().to_vec(),
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

		System::assert_last_event(RuntimeEvent::Doctors(crate::Event::DoctorRegistered(
			Doctor {
				account_id: 1,
				certifications: Vec::new(),
				info: DoctorInfo {
					name: "DeBio Doctor".as_bytes().to_vec(),
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

		assert_ok!(Doctors::update_doctor(
			RuntimeOrigin::signed(1),
			DoctorInfo {
				name: "Abdul Hakim".as_bytes().to_vec(),
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

		System::assert_last_event(RuntimeEvent::Doctors(crate::Event::DoctorUpdated(
			Doctor {
				account_id: 1,
				certifications: Vec::new(),
				info: DoctorInfo {
					name: "Abdul Hakim".as_bytes().to_vec(),
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

		assert_ok!(Doctors::deregister_doctor(RuntimeOrigin::signed(1)));

		System::assert_last_event(RuntimeEvent::Doctors(crate::Event::DoctorDeleted(
			Doctor {
				account_id: 1,
				certifications: Vec::new(),
				info: DoctorInfo {
					name: "Abdul Hakim".as_bytes().to_vec(),
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
	})
}
