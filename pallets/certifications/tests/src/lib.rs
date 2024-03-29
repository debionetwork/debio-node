mod mock;

#[cfg(test)]
mod tests {
	use crate::mock::*;

	use certifications::{Certification, CertificationInfo, Error};
	use labs::LabInfo;

	use frame_support::{
		assert_noop, assert_ok,
		sp_runtime::traits::{Hash, Keccak256},
	};
	use primitives_area_code::{CityCode, CountryCode, RegionCode};

	#[test]
	fn create_certification_works() {
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

			assert_ok!(Certifications::create_certification(
				RuntimeOrigin::signed(1),
				CertificationInfo {
					title: "DeBio title".as_bytes().to_vec(),
					issuer: "DeBio issuer".as_bytes().to_vec(),
					month: "DeBio month".as_bytes().to_vec(),
					year: "DeBio year".as_bytes().to_vec(),
					description: "DeBio description".as_bytes().to_vec(),
					supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				}
			));

			let lab = Labs::lab_by_account_id(1).unwrap();

			assert_eq!(
				Certifications::certification_by_id(lab.certifications[0]),
				Some(Certification {
					id: lab.certifications[0],
					owner_id: 1,
					info: CertificationInfo {
						title: "DeBio title".as_bytes().to_vec(),
						issuer: "DeBio issuer".as_bytes().to_vec(),
						month: "DeBio month".as_bytes().to_vec(),
						year: "DeBio year".as_bytes().to_vec(),
						description: "DeBio description".as_bytes().to_vec(),
						supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
					}
				})
			);

			assert_eq!(Certifications::certification_count_by_owner(1), Some(1));
		})
	}

	#[test]
	fn update_certification_works() {
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

			assert_ok!(Certifications::create_certification(
				RuntimeOrigin::signed(1),
				CertificationInfo {
					title: "DeBio title".as_bytes().to_vec(),
					issuer: "DeBio issuer".as_bytes().to_vec(),
					month: "DeBio month".as_bytes().to_vec(),
					year: "DeBio year".as_bytes().to_vec(),
					description: "DeBio description".as_bytes().to_vec(),
					supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				}
			));

			let lab = Labs::lab_by_account_id(1).unwrap();

			assert_ok!(Certifications::update_certification(
				RuntimeOrigin::signed(1),
				lab.certifications[0],
				CertificationInfo {
					title: "DeBio title 2".as_bytes().to_vec(),
					issuer: "DeBio issuer 2".as_bytes().to_vec(),
					month: "DeBio month 2".as_bytes().to_vec(),
					year: "DeBio year 2".as_bytes().to_vec(),
					description: "DeBio description 2".as_bytes().to_vec(),
					supporting_document: Some("DeBio Profile Image uwu 2".as_bytes().to_vec()),
				}
			));

			assert_eq!(
				Certifications::certification_by_id(lab.certifications[0]),
				Some(Certification {
					id: lab.certifications[0],
					owner_id: 1,
					info: CertificationInfo {
						title: "DeBio title 2".as_bytes().to_vec(),
						issuer: "DeBio issuer 2".as_bytes().to_vec(),
						month: "DeBio month 2".as_bytes().to_vec(),
						year: "DeBio year 2".as_bytes().to_vec(),
						description: "DeBio description 2".as_bytes().to_vec(),
						supporting_document: Some("DeBio Profile Image uwu 2".as_bytes().to_vec()),
					}
				})
			);

			assert_eq!(Certifications::certification_count_by_owner(1), Some(1));
		})
	}

	#[test]
	fn delete_certification_works() {
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

			assert_ok!(Certifications::create_certification(
				RuntimeOrigin::signed(1),
				CertificationInfo {
					title: "DeBio title".as_bytes().to_vec(),
					issuer: "DeBio issuer".as_bytes().to_vec(),
					month: "DeBio month".as_bytes().to_vec(),
					year: "DeBio year".as_bytes().to_vec(),
					description: "DeBio description".as_bytes().to_vec(),
					supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				}
			));

			let lab = Labs::lab_by_account_id(1).unwrap();

			assert_ok!(Certifications::delete_certification(
				RuntimeOrigin::signed(1),
				lab.certifications[0]
			));

			assert_eq!(Certifications::certification_count_by_owner(1), Some(0));
		})
	}

	#[test]
	fn not_allowed_to_create_certification() {
		ExternalityBuilder::build().execute_with(|| {
			assert_noop!(
				Certifications::create_certification(
					RuntimeOrigin::signed(1),
					CertificationInfo {
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
	fn update_certification_does_not_exist() {
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
				Certifications::update_certification(
					RuntimeOrigin::signed(1),
					Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
					CertificationInfo {
						title: "DeBio title 2".as_bytes().to_vec(),
						issuer: "DeBio issuer 2".as_bytes().to_vec(),
						month: "DeBio month 2".as_bytes().to_vec(),
						year: "DeBio year 2".as_bytes().to_vec(),
						description: "DeBio description 2".as_bytes().to_vec(),
						supporting_document: Some("DeBio Profile Image uwu 2".as_bytes().to_vec()),
					}
				),
				Error::<Test>::CertificationDoesNotExist
			);
		})
	}

	#[test]
	fn update_certification_not_owner() {
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

			assert_ok!(Certifications::create_certification(
				RuntimeOrigin::signed(1),
				CertificationInfo {
					title: "DeBio title".as_bytes().to_vec(),
					issuer: "DeBio issuer".as_bytes().to_vec(),
					month: "DeBio month".as_bytes().to_vec(),
					year: "DeBio year".as_bytes().to_vec(),
					description: "DeBio description".as_bytes().to_vec(),
					supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				}
			));

			let lab = Labs::lab_by_account_id(1).unwrap();

			assert_noop!(
				Certifications::update_certification(
					RuntimeOrigin::signed(2),
					lab.certifications[0],
					CertificationInfo {
						title: "DeBio title 2".as_bytes().to_vec(),
						issuer: "DeBio issuer 2".as_bytes().to_vec(),
						month: "DeBio month 2".as_bytes().to_vec(),
						year: "DeBio year 2".as_bytes().to_vec(),
						description: "DeBio description 2".as_bytes().to_vec(),
						supporting_document: Some("DeBio Profile Image uwu 2".as_bytes().to_vec()),
					}
				),
				Error::<Test>::NotCertificationOwner
			);
		})
	}

	#[test]
	fn delete_certification_does_not_exist() {
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
				Certifications::delete_certification(
					RuntimeOrigin::signed(1),
					Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes())
				),
				Error::<Test>::CertificationDoesNotExist
			);
		})
	}

	#[test]
	fn delete_certification_not_owner() {
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

			assert_ok!(Certifications::create_certification(
				RuntimeOrigin::signed(1),
				CertificationInfo {
					title: "DeBio title".as_bytes().to_vec(),
					issuer: "DeBio issuer".as_bytes().to_vec(),
					month: "DeBio month".as_bytes().to_vec(),
					year: "DeBio year".as_bytes().to_vec(),
					description: "DeBio description".as_bytes().to_vec(),
					supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				}
			));

			let lab = Labs::lab_by_account_id(1).unwrap();

			assert_noop!(
				Certifications::delete_certification(
					RuntimeOrigin::signed(2),
					lab.certifications[0]
				),
				Error::<Test>::NotCertificationOwner
			);
		})
	}
}
