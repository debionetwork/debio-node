mod mock;

#[cfg(test)]
mod tests {
	use crate::mock::*;
	
	use doctors::DoctorInfo;
	use doctor_certifications::{
		Error, 
		DoctorCertification, DoctorCertificationInfo
	};
	
	use frame_support::{
		assert_noop, assert_ok,
		sp_runtime::traits::{Hash, Keccak256},
	};
	use primitives_area_code::{CityCode, CountryCode, RegionCode};
	
	#[test]
	fn create_doctor_certification_works() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(Doctors::register_doctor(
				Origin::signed(1),
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
			
			assert_ok!(DoctorCertifications::create_certification(
				Origin::signed(1),
				DoctorCertificationInfo {
					title: "DeBio title".as_bytes().to_vec(),
					issuer: "DeBio issuer".as_bytes().to_vec(),
					month: "DeBio month".as_bytes().to_vec(),
					year: "DeBio year".as_bytes().to_vec(),
					description: "DeBio description".as_bytes().to_vec(),
					supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				}
			));
	
			let doctor = Doctors::doctor_by_account_id(1).unwrap();
	
			assert_eq!(
				DoctorCertifications::certification_by_id(doctor.certifications[0]),
				Some(
					DoctorCertification {
						id: doctor.certifications[0],
						owner_id: 1,
						info: DoctorCertificationInfo {
							title: "DeBio title".as_bytes().to_vec(),
							issuer: "DeBio issuer".as_bytes().to_vec(),
							month: "DeBio month".as_bytes().to_vec(),
							year: "DeBio year".as_bytes().to_vec(),
							description: "DeBio description".as_bytes().to_vec(),
							supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
						}
					}
				)
			);
	
			assert_eq!(
				DoctorCertifications::certification_count_by_owner(1),
				Some(1)
			);
		})
	}
	
	#[test]
	fn update_doctor_certification_works() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(Doctors::register_doctor(
				Origin::signed(1),
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
			
			assert_ok!(DoctorCertifications::create_certification(
				Origin::signed(1),
				DoctorCertificationInfo {
					title: "DeBio title".as_bytes().to_vec(),
					issuer: "DeBio issuer".as_bytes().to_vec(),
					month: "DeBio month".as_bytes().to_vec(),
					year: "DeBio year".as_bytes().to_vec(),
					description: "DeBio description".as_bytes().to_vec(),
					supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				}
			));
	
			let doctor = Doctors::doctor_by_account_id(1).unwrap();
			
			assert_ok!(DoctorCertifications::update_certification(
				Origin::signed(1),
				doctor.certifications[0],
				DoctorCertificationInfo {
					title: "DeBio title 2".as_bytes().to_vec(),
					issuer: "DeBio issuer 2".as_bytes().to_vec(),
					month: "DeBio month 2".as_bytes().to_vec(),
					year: "DeBio year 2".as_bytes().to_vec(),
					description: "DeBio description 2".as_bytes().to_vec(),
					supporting_document: Some("DeBio Profile Image uwu 2".as_bytes().to_vec()),
				}
			));
	
			assert_eq!(
				DoctorCertifications::certification_by_id(doctor.certifications[0]),
				Some(
					DoctorCertification {
						id: doctor.certifications[0],
						owner_id: 1,
						info: DoctorCertificationInfo {
							title: "DeBio title 2".as_bytes().to_vec(),
							issuer: "DeBio issuer 2".as_bytes().to_vec(),
							month: "DeBio month 2".as_bytes().to_vec(),
							year: "DeBio year 2".as_bytes().to_vec(),
							description: "DeBio description 2".as_bytes().to_vec(),
							supporting_document: Some("DeBio Profile Image uwu 2".as_bytes().to_vec()),
						}
					}
				)
			);
	
			assert_eq!(
				DoctorCertifications::certification_count_by_owner(1),
				Some(1)
			);
		})
	}
	
	#[test]
	fn delete_doctor_certification_works() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(Doctors::register_doctor(
				Origin::signed(1),
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
			
			assert_ok!(DoctorCertifications::create_certification(
				Origin::signed(1),
				DoctorCertificationInfo {
					title: "DeBio title".as_bytes().to_vec(),
					issuer: "DeBio issuer".as_bytes().to_vec(),
					month: "DeBio month".as_bytes().to_vec(),
					year: "DeBio year".as_bytes().to_vec(),
					description: "DeBio description".as_bytes().to_vec(),
					supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				}
			));
	
			let doctor = Doctors::doctor_by_account_id(1).unwrap();
			
			assert_ok!(DoctorCertifications::delete_certification(
				Origin::signed(1),
				doctor.certifications[0]
			));
	
			assert_eq!(
				DoctorCertifications::certification_count_by_owner(1),
				Some(0)
			);
		})
	}
	
	#[test]
	fn not_allowed_to_create_doctor_certification() {
		ExternalityBuilder::build().execute_with(|| {		
			assert_noop!(
				DoctorCertifications::create_certification(
					Origin::signed(1),
					DoctorCertificationInfo {
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
	fn update_doctor_certification_does_not_exist() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(Doctors::register_doctor(
				Origin::signed(1),
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
				DoctorCertifications::update_certification(
					Origin::signed(1),
					Keccak256::hash(
						"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()
					),
					DoctorCertificationInfo {
						title: "DeBio title 2".as_bytes().to_vec(),
						issuer: "DeBio issuer 2".as_bytes().to_vec(),
						month: "DeBio month 2".as_bytes().to_vec(),
						year: "DeBio year 2".as_bytes().to_vec(),
						description: "DeBio description 2".as_bytes().to_vec(),
						supporting_document: Some("DeBio Profile Image uwu 2".as_bytes().to_vec()),
					}
				),
				Error::<Test>::DoctorCertificationDoesNotExist
			);
		})
	}
	
	#[test]
	fn update_doctor_certification_not_owner() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(Doctors::register_doctor(
				Origin::signed(1),
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
			
			assert_ok!(DoctorCertifications::create_certification(
				Origin::signed(1),
				DoctorCertificationInfo {
					title: "DeBio title".as_bytes().to_vec(),
					issuer: "DeBio issuer".as_bytes().to_vec(),
					month: "DeBio month".as_bytes().to_vec(),
					year: "DeBio year".as_bytes().to_vec(),
					description: "DeBio description".as_bytes().to_vec(),
					supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				}
			));
	
			let doctor = Doctors::doctor_by_account_id(1).unwrap();
			
			assert_noop!(
				DoctorCertifications::update_certification(
					Origin::signed(2),
					doctor.certifications[0],
					DoctorCertificationInfo {
						title: "DeBio title 2".as_bytes().to_vec(),
						issuer: "DeBio issuer 2".as_bytes().to_vec(),
						month: "DeBio month 2".as_bytes().to_vec(),
						year: "DeBio year 2".as_bytes().to_vec(),
						description: "DeBio description 2".as_bytes().to_vec(),
						supporting_document: Some("DeBio Profile Image uwu 2".as_bytes().to_vec()),
					}
				),
				Error::<Test>::NotDoctorCertificationOwner
			);
		})
	}
	
	#[test]
	fn delete_doctor_certification_does_not_exist() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(Doctors::register_doctor(
				Origin::signed(1),
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
				DoctorCertifications::delete_certification(
					Origin::signed(1),
					Keccak256::hash(
						"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()
					)
				),
				Error::<Test>::DoctorCertificationDoesNotExist
			);
		})
	}
	
	#[test]
	fn delete_doctor_certification_not_owner() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(Doctors::register_doctor(
				Origin::signed(1),
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
			
			assert_ok!(DoctorCertifications::create_certification(
				Origin::signed(1),
				DoctorCertificationInfo {
					title: "DeBio title".as_bytes().to_vec(),
					issuer: "DeBio issuer".as_bytes().to_vec(),
					month: "DeBio month".as_bytes().to_vec(),
					year: "DeBio year".as_bytes().to_vec(),
					description: "DeBio description".as_bytes().to_vec(),
					supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				}
			));
	
			let doctor = Doctors::doctor_by_account_id(1).unwrap();
			
			assert_noop!(
				DoctorCertifications::delete_certification(
					Origin::signed(2),
					doctor.certifications[0]
				),
				Error::<Test>::NotDoctorCertificationOwner
			);
		})
	}
}