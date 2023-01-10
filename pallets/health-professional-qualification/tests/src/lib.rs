mod mock;

#[cfg(test)]

mod tests {
	use crate::mock::*;

	use frame_support::{
		assert_noop, assert_ok,
		sp_runtime::traits::{Hash, Keccak256},
	};
	use health_professional::types::HealthProfessionalInfo;
	use health_professional_qualification::{
		types::{Certification, Experience, Qualification},
		Error, Event as HealthProfessionalQualificationEvent,
	};

	#[test]
	fn create_health_professional_qualification_works() {
		ExternalityBuilder::build().execute_with(|| {
			let health_professional_info = HealthProfessionalInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: b"First Name".to_vec(),
				last_name: b"Last Name".to_vec(),
				myriad_username: b"debiouser".to_vec(),
				gender: b"Gender".to_vec(),
				date_of_birth: 0,
				email: b"Email".to_vec(),
				phone_number: b"+6893026516".to_vec(),
				role: b"doctor".to_vec(),
				category: b"Mental Health".to_vec(),
				profile_link: Some(b"DeBio Genetic Analyst profile_link".to_vec()),
				profile_image: Some(b"DeBio Genetic Analyst profile_image".to_vec()),
				anonymous: false,
			};

			assert_ok!(HealthProfessional::register(
				Origin::signed(1),
				health_professional_info.clone()
			));
			assert_ok!(HealthProfessional::register(Origin::signed(2), health_professional_info));

			let experience = Experience { title: b"DeBio title".to_vec() };
			let certification = Certification {
				title: b"DeBio title".to_vec(),
				issuer: b"DeBio issuer".to_vec(),
				month: b"DeBio month".to_vec(),
				year: b"DeBio year".to_vec(),
				description: b"DeBio description".to_vec(),
				supporting_document: Some(b"DeBio Profile Image uwu".to_vec()),
			};

			assert_ok!(HealthProfessionalQualification::create(
				Origin::signed(1),
				vec![experience.clone()],
				vec![certification.clone()],
			));

			assert_ok!(HealthProfessionalQualification::create(
				Origin::signed(2),
				vec![experience.clone()],
				vec![certification.clone()],
			));

			let result = HealthProfessional::health_professional_by_account_id(1);
			let health_professional = result.unwrap();
			let id = health_professional.qualifications()[0];

			assert_eq!(
				HealthProfessionalQualification::health_professional_qualification_by_id(id),
				Some(Qualification::new(id, &1, &[experience], &[certification])),
			);

			assert_eq!(
				HealthProfessionalQualification::health_professional_qualification_count(),
				2,
			);

			assert_eq!(
				HealthProfessionalQualification::health_professional_qualification_count_by_owner(
					1
				),
				1,
			);

			assert_eq!(
				HealthProfessionalQualification::health_professional_qualification_count_by_owner(
					2
				),
				1,
			);
		});
	}

	#[test]
	fn update_health_professional_qualification_works() {
		ExternalityBuilder::build().execute_with(|| {
			let health_professional_info = HealthProfessionalInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: b"First Name".to_vec(),
				last_name: b"Last Name".to_vec(),
				myriad_username: b"debiouser".to_vec(),
				gender: b"Gender".to_vec(),
				date_of_birth: 0,
				email: b"Email".to_vec(),
				phone_number: b"+6893026516".to_vec(),
				role: b"doctor".to_vec(),
				category: b"Mental Health".to_vec(),
				profile_link: Some(b"DeBio Genetic Analyst profile_link".to_vec()),
				profile_image: Some(b"DeBio Genetic Analyst profile_image".to_vec()),
				anonymous: false,
			};

			assert_ok!(HealthProfessional::register(Origin::signed(1), health_professional_info));

			let experience = Experience { title: b"DeBio title".to_vec() };
			let certification = Certification {
				title: b"DeBio title".to_vec(),
				issuer: b"DeBio issuer".to_vec(),
				month: b"DeBio month".to_vec(),
				year: b"DeBio year".to_vec(),
				description: b"DeBio description".to_vec(),
				supporting_document: Some(b"DeBio Profile Image uwu".to_vec()),
			};

			assert_ok!(HealthProfessionalQualification::create(
				Origin::signed(1),
				vec![experience],
				vec![certification],
			));

			let result = HealthProfessional::health_professional_by_account_id(1);
			let health_professional = result.unwrap();
			let id = health_professional.qualifications()[0];

			let experience = Experience { title: b"Myriad title".to_vec() };
			let certification = Certification {
				title: b"Myriad title".to_vec(),
				issuer: b"DeBio issuer".to_vec(),
				month: b"DeBio month".to_vec(),
				year: b"DeBio year".to_vec(),
				description: b"DeBio description".to_vec(),
				supporting_document: Some(b"DeBio Profile Image uwu".to_vec()),
			};

			assert_ok!(HealthProfessionalQualification::update(
				Origin::signed(1),
				id,
				Some(vec![experience.clone()]),
				Some(vec![certification.clone()]),
			));

			assert_eq!(
				HealthProfessionalQualification::health_professional_qualification_by_id(id),
				Some(Qualification::new(id, &1, &[experience], &[certification])),
			);

			assert!(HealthProfessional::health_professional_by_account_id(1)
				.filter(|health_professional| {
					let mut exist = false;
					for hash_id in health_professional.qualifications().iter() {
						if hash_id == &id {
							exist = true;
							break
						}
					}

					exist
				})
				.is_some());
		});
	}

	#[test]
	fn delete_health_professional_qualification_works() {
		ExternalityBuilder::build().execute_with(|| {
			let health_professional_info = HealthProfessionalInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: b"First Name".to_vec(),
				last_name: b"Last Name".to_vec(),
				myriad_username: b"debiouser".to_vec(),
				gender: b"Gender".to_vec(),
				date_of_birth: 0,
				email: b"Email".to_vec(),
				phone_number: b"+6893026516".to_vec(),
				role: b"doctor".to_vec(),
				category: b"Mental Health".to_vec(),
				profile_link: Some(b"DeBio Genetic Analyst profile_link".to_vec()),
				profile_image: Some(b"DeBio Genetic Analyst profile_image".to_vec()),
				anonymous: false,
			};

			assert_ok!(HealthProfessional::register(
				Origin::signed(1),
				health_professional_info.clone()
			));
			assert_ok!(HealthProfessional::register(Origin::signed(2), health_professional_info));

			let experience = Experience { title: b"DeBio title".to_vec() };
			let certification = Certification {
				title: b"DeBio title".to_vec(),
				issuer: b"DeBio issuer".to_vec(),
				month: b"DeBio month".to_vec(),
				year: b"DeBio year".to_vec(),
				description: b"DeBio description".to_vec(),
				supporting_document: Some(b"DeBio Profile Image uwu".to_vec()),
			};

			assert_ok!(HealthProfessionalQualification::create(
				Origin::signed(1),
				vec![experience.clone()],
				vec![certification.clone()],
			));

			assert_ok!(HealthProfessionalQualification::create(
				Origin::signed(2),
				vec![experience.clone()],
				vec![certification.clone()],
			));

			assert_ok!(HealthProfessionalQualification::create(
				Origin::signed(1),
				vec![experience],
				vec![certification],
			));

			let result = HealthProfessional::health_professional_by_account_id(1);
			let health_professional = result.unwrap();
			let id = health_professional.qualifications()[0];

			assert_ok!(HealthProfessionalQualification::delete(Origin::signed(1), id,));

			assert_eq!(
				HealthProfessionalQualification::health_professional_qualification_by_id(id),
				None,
			);

			assert_eq!(
				HealthProfessionalQualification::health_professional_qualification_count(),
				2,
			);

			assert_eq!(
				HealthProfessionalQualification::health_professional_qualification_count_by_owner(
					1
				),
				1,
			);

			assert!(HealthProfessional::health_professional_by_account_id(1)
				.filter(|health_professional| {
					let mut exist = false;
					for hash_id in health_professional.qualifications().iter() {
						if hash_id == &id {
							exist = true;
							break
						}
					}

					exist
				})
				.is_none());
		});
	}

	#[test]
	fn cant_create_health_professional_qualification_when_owner_not_registered() {
		ExternalityBuilder::build().execute_with(|| {
			let experience = Experience { title: b"DeBio title".to_vec() };
			let certification = Certification {
				title: b"DeBio title".to_vec(),
				issuer: b"DeBio issuer".to_vec(),
				month: b"DeBio month".to_vec(),
				year: b"DeBio year".to_vec(),
				description: b"DeBio description".to_vec(),
				supporting_document: Some(b"DeBio Profile Image uwu".to_vec()),
			};

			assert_noop!(
				HealthProfessionalQualification::create(
					Origin::signed(1),
					vec![experience],
					vec![certification],
				),
				Error::<Test>::NotRegistered,
			);
		});
	}

	#[test]
	fn cant_update_health_professional_when_not_found() {
		ExternalityBuilder::build().execute_with(|| {
			assert_noop!(
				HealthProfessionalQualification::update(
					Origin::signed(1),
					Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),),
					None,
					None,
				),
				Error::<Test>::NotFound,
			);
		});
	}

	#[test]
	fn cant_delete_health_professional_when_not_found() {
		ExternalityBuilder::build().execute_with(|| {
			assert_noop!(
				HealthProfessionalQualification::delete(
					Origin::signed(1),
					Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),),
				),
				Error::<Test>::NotFound,
			);
		});
	}

	#[test]
	fn cant_update_health_professional_qualification_when_unauthorized() {
		ExternalityBuilder::build().execute_with(|| {
			let health_professional_info = HealthProfessionalInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: b"First Name".to_vec(),
				last_name: b"Last Name".to_vec(),
				myriad_username: b"debiouser".to_vec(),
				gender: b"Gender".to_vec(),
				date_of_birth: 0,
				email: b"Email".to_vec(),
				phone_number: b"+6893026516".to_vec(),
				role: b"doctor".to_vec(),
				category: b"Mental Health".to_vec(),
				profile_link: Some(b"DeBio Genetic Analyst profile_link".to_vec()),
				profile_image: Some(b"DeBio Genetic Analyst profile_image".to_vec()),
				anonymous: false,
			};

			assert_ok!(HealthProfessional::register(Origin::signed(1), health_professional_info));

			let experience = Experience { title: b"DeBio title".to_vec() };
			let certification = Certification {
				title: b"DeBio title".to_vec(),
				issuer: b"DeBio issuer".to_vec(),
				month: b"DeBio month".to_vec(),
				year: b"DeBio year".to_vec(),
				description: b"DeBio description".to_vec(),
				supporting_document: Some(b"DeBio Profile Image uwu".to_vec()),
			};

			assert_ok!(HealthProfessionalQualification::create(
				Origin::signed(1),
				vec![experience],
				vec![certification],
			));

			let result = HealthProfessional::health_professional_by_account_id(1);
			let health_professional = result.unwrap();
			let id = health_professional.qualifications()[0];

			let experience = Experience { title: b"Myriad title".to_vec() };
			let certification = Certification {
				title: b"Myriad title".to_vec(),
				issuer: b"DeBio issuer".to_vec(),
				month: b"DeBio month".to_vec(),
				year: b"DeBio year".to_vec(),
				description: b"DeBio description".to_vec(),
				supporting_document: Some(b"DeBio Profile Image uwu".to_vec()),
			};

			assert_noop!(
				HealthProfessionalQualification::update(
					Origin::signed(2),
					id,
					Some(vec![experience]),
					Some(vec![certification]),
				),
				Error::<Test>::Unauthorized,
			);
		});
	}

	#[test]
	fn cant_delete_health_professional_qualification_when_unauthorized() {
		ExternalityBuilder::build().execute_with(|| {
			let health_professional_info = HealthProfessionalInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: b"First Name".to_vec(),
				last_name: b"Last Name".to_vec(),
				myriad_username: b"debiouser".to_vec(),
				gender: b"Gender".to_vec(),
				date_of_birth: 0,
				email: b"Email".to_vec(),
				phone_number: b"+6893026516".to_vec(),
				role: b"doctor".to_vec(),
				category: b"Mental Health".to_vec(),
				profile_link: Some(b"DeBio Genetic Analyst profile_link".to_vec()),
				profile_image: Some(b"DeBio Genetic Analyst profile_image".to_vec()),
				anonymous: false,
			};

			assert_ok!(HealthProfessional::register(Origin::signed(1), health_professional_info));

			let experience = Experience { title: b"DeBio title".to_vec() };
			let certification = Certification {
				title: b"DeBio title".to_vec(),
				issuer: b"DeBio issuer".to_vec(),
				month: b"DeBio month".to_vec(),
				year: b"DeBio year".to_vec(),
				description: b"DeBio description".to_vec(),
				supporting_document: Some(b"DeBio Profile Image uwu".to_vec()),
			};

			assert_ok!(HealthProfessionalQualification::create(
				Origin::signed(1),
				vec![experience],
				vec![certification],
			));

			let result = HealthProfessional::health_professional_by_account_id(1);
			let health_professional = result.unwrap();
			let id = health_professional.qualifications()[0];

			assert_noop!(
				HealthProfessionalQualification::delete(Origin::signed(2), id,),
				Error::<Test>::Unauthorized,
			);
		});
	}

	#[test]
	fn call_event_should_work() {
		ExternalityBuilder::build().execute_with(|| {
			System::set_block_number(1);

			let health_professional_info = HealthProfessionalInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: b"First Name".to_vec(),
				last_name: b"Last Name".to_vec(),
				myriad_username: b"debiouser".to_vec(),
				gender: b"Gender".to_vec(),
				date_of_birth: 0,
				email: b"Email".to_vec(),
				phone_number: b"+6893026516".to_vec(),
				role: b"doctor".to_vec(),
				category: b"Mental Health".to_vec(),
				profile_link: Some(b"DeBio Genetic Analyst profile_link".to_vec()),
				profile_image: Some(b"DeBio Genetic Analyst profile_image".to_vec()),
				anonymous: false,
			};

			assert_ok!(HealthProfessional::register(Origin::signed(1), health_professional_info));

			let experience = Experience { title: b"DeBio title".to_vec() };
			let certification = Certification {
				title: b"DeBio title".to_vec(),
				issuer: b"DeBio issuer".to_vec(),
				month: b"DeBio month".to_vec(),
				year: b"DeBio year".to_vec(),
				description: b"DeBio description".to_vec(),
				supporting_document: Some(b"DeBio Profile Image uwu".to_vec()),
			};

			assert_ok!(HealthProfessionalQualification::create(
				Origin::signed(1),
				vec![experience.clone()],
				vec![certification.clone()],
			));

			let result = HealthProfessional::health_professional_by_account_id(1);
			let health_professional = result.unwrap();
			let id = health_professional.qualifications()[0];

			let qualification = Qualification::new(id, &1, &[experience], &[certification]);

			System::assert_last_event(Event::HealthProfessionalQualification(
				HealthProfessionalQualificationEvent::HealthProfessionalQualificationCreated(
					1,
					qualification,
				),
			));

			let experience = Experience { title: b"Myriad title".to_vec() };
			let certification = Certification {
				title: b"Myriad title".to_vec(),
				issuer: b"DeBio issuer".to_vec(),
				month: b"DeBio month".to_vec(),
				year: b"DeBio year".to_vec(),
				description: b"DeBio description".to_vec(),
				supporting_document: Some(b"DeBio Profile Image uwu".to_vec()),
			};

			let qualification =
				Qualification::new(id, &1, &[experience.clone()], &[certification.clone()]);

			assert_ok!(HealthProfessionalQualification::update(
				Origin::signed(1),
				id,
				Some(vec![experience]),
				Some(vec![certification]),
			));

			System::assert_last_event(Event::HealthProfessionalQualification(
				HealthProfessionalQualificationEvent::HealthProfessionalQualificationUpdated(
					1,
					qualification,
				),
			));

			assert_ok!(HealthProfessionalQualification::delete(Origin::signed(1), id,));

			System::assert_last_event(Event::HealthProfessionalQualification(
				HealthProfessionalQualificationEvent::HealthProfessionalQualificationDeleted(1, id),
			));
		});
	}
}
