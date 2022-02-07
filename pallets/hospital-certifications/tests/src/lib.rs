mod mock;

#[cfg(test)]
mod tests {
	use crate::mock::*;

	use hospital_certifications::{Error, HospitalCertification, HospitalCertificationInfo};
	use hospitals::HospitalInfo;

	use frame_support::{
		assert_noop, assert_ok,
		sp_runtime::traits::{Hash, Keccak256},
	};
	use primitives_area_code::{CityCode, CountryCode, RegionCode};

	#[test]
	fn create_hospital_certification_works() {
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

			assert_ok!(HospitalCertifications::create_certification(
				Origin::signed(1),
				HospitalCertificationInfo {
					title: "DeBio title".as_bytes().to_vec(),
					issuer: "DeBio issuer".as_bytes().to_vec(),
					month: "DeBio month".as_bytes().to_vec(),
					year: "DeBio year".as_bytes().to_vec(),
					description: "DeBio description".as_bytes().to_vec(),
					supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				}
			));

			let hospital = Hospitals::hospital_by_account_id(1).unwrap();

			assert_eq!(
				HospitalCertifications::certification_by_id(hospital.certifications[0]),
				Some(HospitalCertification {
					id: hospital.certifications[0],
					owner_id: 1,
					info: HospitalCertificationInfo {
						title: "DeBio title".as_bytes().to_vec(),
						issuer: "DeBio issuer".as_bytes().to_vec(),
						month: "DeBio month".as_bytes().to_vec(),
						year: "DeBio year".as_bytes().to_vec(),
						description: "DeBio description".as_bytes().to_vec(),
						supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
					}
				})
			);

			assert_eq!(HospitalCertifications::certification_count_by_owner(1), Some(1));
		})
	}

	#[test]
	fn update_hospital_certification_works() {
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

			assert_ok!(HospitalCertifications::create_certification(
				Origin::signed(1),
				HospitalCertificationInfo {
					title: "DeBio title".as_bytes().to_vec(),
					issuer: "DeBio issuer".as_bytes().to_vec(),
					month: "DeBio month".as_bytes().to_vec(),
					year: "DeBio year".as_bytes().to_vec(),
					description: "DeBio description".as_bytes().to_vec(),
					supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				}
			));

			let hospital = Hospitals::hospital_by_account_id(1).unwrap();

			assert_ok!(HospitalCertifications::update_certification(
				Origin::signed(1),
				hospital.certifications[0],
				HospitalCertificationInfo {
					title: "DeBio title 2".as_bytes().to_vec(),
					issuer: "DeBio issuer 2".as_bytes().to_vec(),
					month: "DeBio month 2".as_bytes().to_vec(),
					year: "DeBio year 2".as_bytes().to_vec(),
					description: "DeBio description 2".as_bytes().to_vec(),
					supporting_document: Some("DeBio Profile Image uwu 2".as_bytes().to_vec()),
				}
			));

			assert_eq!(
				HospitalCertifications::certification_by_id(hospital.certifications[0]),
				Some(HospitalCertification {
					id: hospital.certifications[0],
					owner_id: 1,
					info: HospitalCertificationInfo {
						title: "DeBio title 2".as_bytes().to_vec(),
						issuer: "DeBio issuer 2".as_bytes().to_vec(),
						month: "DeBio month 2".as_bytes().to_vec(),
						year: "DeBio year 2".as_bytes().to_vec(),
						description: "DeBio description 2".as_bytes().to_vec(),
						supporting_document: Some("DeBio Profile Image uwu 2".as_bytes().to_vec()),
					}
				})
			);

			assert_eq!(HospitalCertifications::certification_count_by_owner(1), Some(1));
		})
	}

	#[test]
	fn delete_hospital_certification_works() {
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

			assert_ok!(HospitalCertifications::create_certification(
				Origin::signed(1),
				HospitalCertificationInfo {
					title: "DeBio title".as_bytes().to_vec(),
					issuer: "DeBio issuer".as_bytes().to_vec(),
					month: "DeBio month".as_bytes().to_vec(),
					year: "DeBio year".as_bytes().to_vec(),
					description: "DeBio description".as_bytes().to_vec(),
					supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				}
			));

			let hospital = Hospitals::hospital_by_account_id(1).unwrap();

			assert_ok!(HospitalCertifications::delete_certification(
				Origin::signed(1),
				hospital.certifications[0]
			));

			assert_eq!(HospitalCertifications::certification_count_by_owner(1), Some(0));
		})
	}

	#[test]
	fn not_allowed_to_create_hospital_certification() {
		ExternalityBuilder::build().execute_with(|| {
			assert_noop!(
				HospitalCertifications::create_certification(
					Origin::signed(1),
					HospitalCertificationInfo {
						title: "DeBio title".as_bytes().to_vec(),
						issuer: "DeBio issuer".as_bytes().to_vec(),
						month: "DeBio month".as_bytes().to_vec(),
						year: "DeBio year".as_bytes().to_vec(),
						description: "DeBio description".as_bytes().to_vec(),
						supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
					}
				),
				Error::<Test>::NotAllowedToCreate
			);
		})
	}

	#[test]
	fn update_hospital_certification_does_not_exist() {
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
				HospitalCertifications::update_certification(
					Origin::signed(1),
					Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
					HospitalCertificationInfo {
						title: "DeBio title 2".as_bytes().to_vec(),
						issuer: "DeBio issuer 2".as_bytes().to_vec(),
						month: "DeBio month 2".as_bytes().to_vec(),
						year: "DeBio year 2".as_bytes().to_vec(),
						description: "DeBio description 2".as_bytes().to_vec(),
						supporting_document: Some("DeBio Profile Image uwu 2".as_bytes().to_vec()),
					}
				),
				Error::<Test>::HospitalCertificationDoesNotExist
			);
		})
	}

	#[test]
	fn update_hospital_certification_not_owner() {
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

			assert_ok!(HospitalCertifications::create_certification(
				Origin::signed(1),
				HospitalCertificationInfo {
					title: "DeBio title".as_bytes().to_vec(),
					issuer: "DeBio issuer".as_bytes().to_vec(),
					month: "DeBio month".as_bytes().to_vec(),
					year: "DeBio year".as_bytes().to_vec(),
					description: "DeBio description".as_bytes().to_vec(),
					supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				}
			));

			let hospital = Hospitals::hospital_by_account_id(1).unwrap();

			assert_noop!(
				HospitalCertifications::update_certification(
					Origin::signed(2),
					hospital.certifications[0],
					HospitalCertificationInfo {
						title: "DeBio title 2".as_bytes().to_vec(),
						issuer: "DeBio issuer 2".as_bytes().to_vec(),
						month: "DeBio month 2".as_bytes().to_vec(),
						year: "DeBio year 2".as_bytes().to_vec(),
						description: "DeBio description 2".as_bytes().to_vec(),
						supporting_document: Some("DeBio Profile Image uwu 2".as_bytes().to_vec()),
					}
				),
				Error::<Test>::NotHospitalCertificationOwner
			);
		})
	}

	#[test]
	fn delete_hospital_certification_does_not_exist() {
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
				HospitalCertifications::delete_certification(
					Origin::signed(1),
					Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes())
				),
				Error::<Test>::HospitalCertificationDoesNotExist
			);
		})
	}

	#[test]
	fn delete_hospital_certification_not_owner() {
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

			assert_ok!(HospitalCertifications::create_certification(
				Origin::signed(1),
				HospitalCertificationInfo {
					title: "DeBio title".as_bytes().to_vec(),
					issuer: "DeBio issuer".as_bytes().to_vec(),
					month: "DeBio month".as_bytes().to_vec(),
					year: "DeBio year".as_bytes().to_vec(),
					description: "DeBio description".as_bytes().to_vec(),
					supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				}
			));

			let hospital = Hospitals::hospital_by_account_id(1).unwrap();

			assert_noop!(
				HospitalCertifications::delete_certification(
					Origin::signed(2),
					hospital.certifications[0]
				),
				Error::<Test>::NotHospitalCertificationOwner
			);
		})
	}
}
