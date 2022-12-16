mod mock;

#[cfg(test)]

mod tests {
	use crate::mock::*;

	use frame_support::{
		assert_noop, assert_ok,
		sp_runtime::traits::{Hash, Keccak256},
	};
	use health_professional::{
		types::HealthProfessionalInfo, Error, Event as HealthProfessionalEvent,
		HealthProfessional as HealthProfessionalStruct, HealthProfessionalVerifierKey,
		MinimumStakeAmount, UnstakeTime,
	};
	use pallet_timestamp::Now;
	use primitives_availability_status::AvailabilityStatus;
	use primitives_stake_status::StakeStatus;
	use primitives_verification_status::VerificationStatus;

	#[test]
	fn register_health_professional_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let doctor = account_key("doctor");
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
				profile_link: b"DeBio Genetic Analyst profile_link".to_vec(),
				profile_image: Some(b"DeBio Genetic Analyst profile_image".to_vec()),
				anonymous: false,
			};

			let health_professional =
				HealthProfessionalStruct::new(&doctor, &health_professional_info);

			assert_ok!(HealthProfessional::register(
				Origin::signed(doctor),
				health_professional_info
			));

			assert_eq!(
				HealthProfessional::health_professional_by_account_id(doctor),
				Some(health_professional),
			);
		});
	}

	#[test]
	fn update_health_professional_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let doctor = account_key("doctor");
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
				profile_link: b"DeBio Genetic Analyst profile_link".to_vec(),
				profile_image: Some(b"DeBio Genetic Analyst profile_image".to_vec()),
				anonymous: false,
			};

			assert_ok!(HealthProfessional::register(
				Origin::signed(doctor),
				health_professional_info
			));

			let updated_info = HealthProfessionalInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: b"First Name".to_vec(),
				last_name: b"Last Name".to_vec(),
				myriad_username: b"myriaduser".to_vec(),
				gender: b"Gender".to_vec(),
				date_of_birth: 0,
				email: b"Email".to_vec(),
				phone_number: b"+6893026516".to_vec(),
				role: b"doctor".to_vec(),
				category: b"Mental Health".to_vec(),
				profile_link: b"DeBio Genetic Analyst profile_link".to_vec(),
				profile_image: Some(b"DeBio Genetic Analyst profile_image".to_vec()),
				anonymous: false,
			};

			let health_professional = HealthProfessionalStruct::new(&doctor, &updated_info);

			assert_ok!(HealthProfessional::update_info(Origin::signed(doctor), updated_info));

			assert_eq!(
				HealthProfessional::health_professional_by_account_id(doctor),
				Some(health_professional),
			);
		});
	}

	#[test]
	fn update_health_professional_availability_status_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let admin = account_key("admin");
			let doctor = account_key("doctor");

			HealthProfessionalVerifierKey::<Test>::put(admin);

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
				profile_link: b"DeBio Genetic Analyst profile_link".to_vec(),
				profile_image: Some(b"DeBio Genetic Analyst profile_image".to_vec()),
				anonymous: false,
			};

			let mut health_professional =
				HealthProfessionalStruct::new(&doctor, &health_professional_info);

			assert_ok!(HealthProfessional::register(
				Origin::signed(doctor),
				health_professional_info
			));

			assert_ok!(HealthProfessional::update_availability_status(
				Origin::signed(admin),
				doctor,
				AvailabilityStatus::Unavailable,
			));

			health_professional.update_availability_status(&AvailabilityStatus::Unavailable);

			assert_eq!(
				HealthProfessional::health_professional_by_account_id(doctor),
				Some(health_professional),
			);
		});
	}

	#[test]
	fn update_health_professional_verification_status_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let admin = account_key("admin");
			let doctor = account_key("doctor");

			HealthProfessionalVerifierKey::<Test>::put(admin);

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
				profile_link: b"DeBio Genetic Analyst profile_link".to_vec(),
				profile_image: Some(b"DeBio Genetic Analyst profile_image".to_vec()),
				anonymous: false,
			};

			let mut health_professional =
				HealthProfessionalStruct::new(&doctor, &health_professional_info);

			assert_ok!(HealthProfessional::register(
				Origin::signed(doctor),
				health_professional_info
			));

			assert_ok!(HealthProfessional::update_verification_status(
				Origin::signed(admin),
				doctor,
				VerificationStatus::Verified,
			));

			assert_ok!(HealthProfessional::update_verification_status(
				Origin::signed(admin),
				doctor,
				VerificationStatus::Verified,
			));

			health_professional.update_verification_status(&VerificationStatus::Verified);

			assert_eq!(
				HealthProfessional::health_professional_by_account_id(doctor),
				Some(health_professional),
			);
		});
	}

	#[test]
	fn deregister_health_professional_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let doctor = account_key("doctor");

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
				profile_link: b"DeBio Genetic Analyst profile_link".to_vec(),
				profile_image: Some(b"DeBio Genetic Analyst profile_image".to_vec()),
				anonymous: false,
			};

			assert_ok!(HealthProfessional::register(
				Origin::signed(doctor),
				health_professional_info
			));

			assert_ok!(HealthProfessional::deregister(Origin::signed(doctor)));

			assert_eq!(HealthProfessional::health_professional_by_account_id(doctor), None,);
		});
	}

	#[test]
	fn stake_health_professional_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let doctor = account_key("doctor");
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
				profile_link: b"DeBio Genetic Analyst profile_link".to_vec(),
				profile_image: Some(b"DeBio Genetic Analyst profile_image".to_vec()),
				anonymous: false,
			};

			let mut health_professional =
				HealthProfessionalStruct::new(&doctor, &health_professional_info);

			MinimumStakeAmount::<Test>::put(10);

			assert_ok!(HealthProfessional::register(
				Origin::signed(doctor),
				health_professional_info
			));

			assert_ok!(HealthProfessional::stake(Origin::signed(doctor),));

			health_professional.update_stake_status(StakeStatus::Staked, 10);

			let staking_account_id = HealthProfessional::staking_account_id(&doctor);

			assert_eq!(
				HealthProfessional::health_professional_by_account_id(doctor),
				Some(health_professional),
			);
			assert_eq!(HealthProfessional::total_staked_amount(), 10);
			assert_eq!(Balances::free_balance(doctor), 290);
			assert_eq!(Balances::free_balance(staking_account_id), 10);
		});
	}

	#[test]
	fn unstake_health_professional_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let doctor = account_key("doctor");
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
				profile_link: b"DeBio Genetic Analyst profile_link".to_vec(),
				profile_image: Some(b"DeBio Genetic Analyst profile_image".to_vec()),
				anonymous: false,
			};

			let mut health_professional =
				HealthProfessionalStruct::new(&doctor, &health_professional_info);

			MinimumStakeAmount::<Test>::put(10);

			assert_ok!(HealthProfessional::register(
				Origin::signed(doctor),
				health_professional_info
			));

			health_professional.update_stake_status(StakeStatus::Staked, 10);

			assert_ok!(HealthProfessional::stake(Origin::signed(doctor),));

			assert_ok!(HealthProfessional::unstake(Origin::signed(doctor)));

			health_professional.update_stake_status(StakeStatus::WaitingForUnstaked, 0);
			health_professional.update_unstaked_at(Some(0));

			assert_eq!(
				HealthProfessional::health_professional_by_account_id(doctor),
				Some(health_professional),
			);
		});
	}

	#[test]
	fn retrieve_unstaked_amount_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let doctor = account_key("doctor");
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
				profile_link: b"DeBio Genetic Analyst profile_link".to_vec(),
				profile_image: Some(b"DeBio Genetic Analyst profile_image".to_vec()),
				anonymous: false,
			};

			let mut health_professional =
				HealthProfessionalStruct::new(&doctor, &health_professional_info);

			MinimumStakeAmount::<Test>::put(10);
			UnstakeTime::<Test>::put(10);

			assert_ok!(HealthProfessional::register(
				Origin::signed(doctor),
				health_professional_info
			));

			assert_ok!(HealthProfessional::stake(Origin::signed(doctor),));

			assert_ok!(HealthProfessional::unstake(Origin::signed(doctor)));

			Now::<Test>::put(10);

			assert_ok!(HealthProfessional::retrieve_unstaked_amount(Origin::signed(doctor)));

			health_professional.update_stake_status(StakeStatus::Unstaked, 0);

			let staking_account_id = HealthProfessional::staking_account_id(&doctor);

			assert_eq!(
				HealthProfessional::health_professional_by_account_id(doctor),
				Some(health_professional),
			);
			assert_eq!(Balances::free_balance(doctor), 300);
			assert_eq!(Balances::free_balance(staking_account_id), 0);
			assert_eq!(HealthProfessional::total_staked_amount(), 0);
		});
	}

	#[test]
	fn update_stake_amount_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let admin = account_key("admin");

			HealthProfessionalVerifierKey::<Test>::put(admin);

			assert_ok!(HealthProfessional::update_stake_amount(
				Origin::signed(admin),
				100_000_000_000,
			));

			assert_eq!(HealthProfessional::minimum_stake_amount(), Some(100_000_000_000),);
		});
	}

	#[test]
	fn update_unstake_time_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let admin = account_key("admin");

			HealthProfessionalVerifierKey::<Test>::put(admin);

			assert_ok!(HealthProfessional::update_unstake_time(
				Origin::signed(admin),
				100_000_000_000,
			));

			assert_eq!(HealthProfessional::unstake_time(), Some(100_000_000_000),);
		});
	}

	#[test]
	fn update_verifier_key_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let admin = account_key("admin");
			let new_admin = account_key("new_admin");

			HealthProfessionalVerifierKey::<Test>::put(admin);

			assert_ok!(HealthProfessional::update_verifier_key(Origin::signed(admin), new_admin,));

			assert_eq!(HealthProfessional::verifier_key(), Some(new_admin),);
		});
	}

	#[test]
	fn sudo_update_verifier_key_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let admin = account_key("admin");

			assert_ok!(HealthProfessional::sudo_update_verifier_key(Origin::root(), admin,));

			assert_eq!(HealthProfessional::verifier_key(), Some(admin),);
		});
	}

	#[test]
	fn cant_register_health_professional_when_already_registered() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let doctor = account_key("doctor");
			let info = HealthProfessionalInfo {
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
				profile_link: b"DeBio Genetic Analyst profile_link".to_vec(),
				profile_image: Some(b"DeBio Genetic Analyst profile_image".to_vec()),
				anonymous: false,
			};

			assert_ok!(HealthProfessional::register(Origin::signed(doctor), info.clone()));

			assert_noop!(
				HealthProfessional::register(Origin::signed(doctor), info.clone()),
				Error::<Test>::AlreadyRegistered,
			);
		});
	}

	#[test]
	fn cant_update_health_professional_info_when_not_registered() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let doctor = account_key("doctor");
			let updated_info = HealthProfessionalInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: b"First Name".to_vec(),
				last_name: b"Last Name".to_vec(),
				myriad_username: b"myriaduser".to_vec(),
				gender: b"Gender".to_vec(),
				date_of_birth: 0,
				email: b"Email".to_vec(),
				phone_number: b"+6893026516".to_vec(),
				role: b"doctor".to_vec(),
				category: b"Mental Health".to_vec(),
				profile_link: b"DeBio Genetic Analyst profile_link".to_vec(),
				profile_image: Some(b"DeBio Genetic Analyst profile_image".to_vec()),
				anonymous: false,
			};

			assert_noop!(
				HealthProfessional::update_info(Origin::signed(doctor), updated_info),
				Error::<Test>::NotFound,
			);
		});
	}

	#[test]
	fn cant_update_health_professional_availability_status_when_not_admin() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let admin = account_key("admin");
			let doctor = account_key("doctor");
			let other_admin = account_key("other_admin");

			HealthProfessionalVerifierKey::<Test>::put(admin);

			assert_noop!(
				HealthProfessional::update_availability_status(
					Origin::signed(other_admin),
					doctor,
					AvailabilityStatus::Unavailable,
				),
				Error::<Test>::Unauthorized,
			);
		});
	}

	#[test]
	fn cant_update_health_professional_availability_status_when_not_registered() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let admin = account_key("admin");
			let doctor = account_key("doctor");

			HealthProfessionalVerifierKey::<Test>::put(admin);

			assert_noop!(
				HealthProfessional::update_availability_status(
					Origin::signed(admin),
					doctor,
					AvailabilityStatus::Unavailable,
				),
				Error::<Test>::NotFound,
			);
		});
	}

	#[test]
	fn cant_update_health_professional_verification_status_when_not_admin() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let admin = account_key("admin");
			let doctor = account_key("doctor");

			assert_noop!(
				HealthProfessional::update_availability_status(
					Origin::signed(admin),
					doctor,
					AvailabilityStatus::Unavailable,
				),
				Error::<Test>::Unauthorized,
			);
		});
	}

	#[test]
	fn cant_update_health_professional_verification_status_when_not_registered() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let admin = account_key("admin");
			let doctor = account_key("doctor");

			HealthProfessionalVerifierKey::<Test>::put(admin);

			assert_noop!(
				HealthProfessional::update_verification_status(
					Origin::signed(admin),
					doctor,
					VerificationStatus::Unverified,
				),
				Error::<Test>::NotFound,
			);
		});
	}

	#[test]
	fn cant_deregister_health_professional_when_not_registered() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let doctor = account_key("doctor");

			assert_noop!(
				HealthProfessional::deregister(Origin::signed(doctor)),
				Error::<Test>::NotFound,
			);
		});
	}

	#[test]
	fn cant_stake_health_professional_when_insufficient_balance() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let doctor = account_key("doctor");

			assert_noop!(
				HealthProfessional::stake(Origin::signed(doctor)),
				Error::<Test>::InsufficientBalance,
			);

			MinimumStakeAmount::<Test>::put(500);

			assert_noop!(
				HealthProfessional::stake(Origin::signed(doctor)),
				Error::<Test>::InsufficientBalance,
			);
		});
	}

	#[test]
	fn cant_stake_health_professional_when_already_staked() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let doctor = account_key("doctor");
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
				profile_link: b"DeBio Genetic Analyst profile_link".to_vec(),
				profile_image: Some(b"DeBio Genetic Analyst profile_image".to_vec()),
				anonymous: false,
			};

			MinimumStakeAmount::<Test>::put(10);

			assert_ok!(HealthProfessional::register(
				Origin::signed(doctor),
				health_professional_info
			));

			assert_ok!(HealthProfessional::stake(Origin::signed(doctor)));

			assert_noop!(
				HealthProfessional::stake(Origin::signed(doctor)),
				Error::<Test>::AlreadyStaked,
			);

			assert_ok!(HealthProfessional::unstake(Origin::signed(doctor)));

			assert_noop!(
				HealthProfessional::stake(Origin::signed(doctor)),
				Error::<Test>::CannotStaked,
			);
		});
	}

	#[test]
	fn cant_stake_health_professional_when_not_registered() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let doctor = account_key("doctor");

			MinimumStakeAmount::<Test>::put(10);

			assert_noop!(
				HealthProfessional::stake(Origin::signed(doctor)),
				Error::<Test>::NotFound,
			);
		});
	}

	#[test]
	fn cant_unstake_health_professional_when_already_unstaked() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let doctor = account_key("doctor");
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
				profile_link: b"DeBio Genetic Analyst profile_link".to_vec(),
				profile_image: Some(b"DeBio Genetic Analyst profile_image".to_vec()),
				anonymous: false,
			};

			MinimumStakeAmount::<Test>::put(10);

			assert_ok!(HealthProfessional::register(
				Origin::signed(doctor),
				health_professional_info
			));

			assert_noop!(
				HealthProfessional::unstake(Origin::signed(doctor)),
				Error::<Test>::CannotUnstaked
			);

			assert_ok!(HealthProfessional::stake(Origin::signed(doctor)));
			assert_ok!(HealthProfessional::unstake(Origin::signed(doctor)));

			assert_noop!(
				HealthProfessional::unstake(Origin::signed(doctor)),
				Error::<Test>::CannotUnstaked
			);
		});
	}

	#[test]
	fn cant_unstake_health_professional_when_not_registered() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let doctor = account_key("doctor");

			MinimumStakeAmount::<Test>::put(10);

			assert_noop!(
				HealthProfessional::unstake(Origin::signed(doctor)),
				Error::<Test>::NotFound,
			);
		});
	}

	#[test]
	fn cant_retrieve_unstaked_amount_when_not_ready_to_unstaked() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let doctor = account_key("doctor");

			assert_noop!(
				HealthProfessional::retrieve_unstaked_amount(Origin::signed(doctor)),
				Error::<Test>::CannotRetrieveUnstakedAmount,
			);

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
				profile_link: b"DeBio Genetic Analyst profile_link".to_vec(),
				profile_image: Some(b"DeBio Genetic Analyst profile_image".to_vec()),
				anonymous: false,
			};

			MinimumStakeAmount::<Test>::put(10);
			UnstakeTime::<Test>::put(10);

			assert_ok!(HealthProfessional::register(
				Origin::signed(doctor),
				health_professional_info
			));

			assert_noop!(
				HealthProfessional::retrieve_unstaked_amount(Origin::signed(doctor)),
				Error::<Test>::CannotRetrieveUnstakedAmount,
			);

			assert_ok!(HealthProfessional::stake(Origin::signed(doctor)));

			assert_noop!(
				HealthProfessional::retrieve_unstaked_amount(Origin::signed(doctor)),
				Error::<Test>::CannotRetrieveUnstakedAmount,
			);

			assert_ok!(HealthProfessional::unstake(Origin::signed(doctor)));

			Now::<Test>::put(5);

			assert_noop!(
				HealthProfessional::retrieve_unstaked_amount(Origin::signed(doctor)),
				Error::<Test>::NotReadyToUnstaked,
			);
		});
	}

	#[test]
	fn cant_retrieve_unstaked_amount_when_health_professional_not_registered() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let doctor = account_key("doctor");

			MinimumStakeAmount::<Test>::put(10);
			UnstakeTime::<Test>::put(10);

			assert_noop!(
				HealthProfessional::retrieve_unstaked_amount(Origin::signed(doctor)),
				Error::<Test>::NotFound,
			);
		});
	}

	#[test]
	fn cant_update_stake_amount_when_not_admin() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let admin = account_key("admin");

			assert_noop!(
				HealthProfessional::update_stake_amount(Origin::signed(admin), 100_000_000_000,),
				Error::<Test>::Unauthorized,
			);
		});
	}

	#[test]
	fn cant_update_unstake_time_when_not_admin() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let admin = account_key("admin");

			assert_noop!(
				HealthProfessional::update_unstake_time(Origin::signed(admin), 100_000_000_000,),
				Error::<Test>::Unauthorized,
			);
		});
	}

	#[test]
	fn cant_update_verifier_key_when_not_admin() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			let admin = account_key("admin");
			let new_admin = account_key("new_admin");

			assert_noop!(
				HealthProfessional::update_verifier_key(Origin::signed(admin), new_admin),
				Error::<Test>::Unauthorized,
			);
		});
	}

	#[test]
	fn call_event_should_work() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			System::set_block_number(1);

			let doctor = account_key("doctor");
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
				profile_link: b"DeBio Genetic Analyst profile_link".to_vec(),
				profile_image: Some(b"DeBio Genetic Analyst profile_image".to_vec()),
				anonymous: false,
			};

			let health_professional =
				HealthProfessionalStruct::new(&doctor, &health_professional_info);

			assert_ok!(HealthProfessional::register(
				Origin::signed(doctor),
				health_professional_info
			));

			System::assert_last_event(Event::HealthProfessional(
				HealthProfessionalEvent::HealthProfessionalRegistered(doctor, health_professional),
			));

			let updated_info = HealthProfessionalInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: b"First Name".to_vec(),
				last_name: b"Last Name".to_vec(),
				myriad_username: b"myriaduser".to_vec(),
				gender: b"Gender".to_vec(),
				date_of_birth: 0,
				email: b"Email".to_vec(),
				phone_number: b"+6893026516".to_vec(),
				role: b"doctor".to_vec(),
				category: b"Mental Health".to_vec(),
				profile_link: b"DeBio Genetic Analyst profile_link".to_vec(),
				profile_image: Some(b"DeBio Genetic Analyst profile_image".to_vec()),
				anonymous: false,
			};

			assert_ok!(HealthProfessional::update_info(
				Origin::signed(doctor),
				updated_info.clone()
			));

			System::assert_last_event(Event::HealthProfessional(
				HealthProfessionalEvent::HealthProfessionalInfoUpdated(doctor, updated_info),
			));

			let admin = account_key("admin");
			let _ = HealthProfessionalVerifierKey::<Test>::put(admin);

			assert_ok!(HealthProfessional::update_availability_status(
				Origin::signed(admin),
				doctor,
				AvailabilityStatus::Available
			));

			System::assert_last_event(Event::HealthProfessional(
				HealthProfessionalEvent::HealthProfessionalAvailabilityStatusUpdated(
					doctor,
					AvailabilityStatus::Available,
				),
			));

			assert_ok!(HealthProfessional::update_verification_status(
				Origin::signed(admin),
				doctor,
				VerificationStatus::Verified
			));

			System::assert_last_event(Event::HealthProfessional(
				HealthProfessionalEvent::HealthProfessionalVerificationStatusUpdated(
					doctor,
					VerificationStatus::Verified,
				),
			));

			MinimumStakeAmount::<Test>::put(10);

			assert_ok!(HealthProfessional::stake(Origin::signed(doctor)));

			System::assert_last_event(Event::HealthProfessional(
				HealthProfessionalEvent::HealthProfessionalStaked(doctor, 10),
			));

			assert_ok!(HealthProfessional::unstake(Origin::signed(doctor)));

			System::assert_last_event(Event::HealthProfessional(
				HealthProfessionalEvent::HealthProfessionalUnstaked(doctor),
			));

			UnstakeTime::<Test>::put(10);
			Now::<Test>::put(10);

			assert_ok!(HealthProfessional::retrieve_unstaked_amount(Origin::signed(doctor)));

			System::assert_last_event(Event::HealthProfessional(
				HealthProfessionalEvent::HealthProfessionalUnstakedAmount(doctor, 10),
			));

			assert_ok!(HealthProfessional::deregister(Origin::signed(doctor)));

			System::assert_last_event(Event::HealthProfessional(
				HealthProfessionalEvent::HealthProfessionalUnregistered(doctor),
			));

			assert_ok!(HealthProfessional::update_stake_amount(Origin::signed(admin), 10));

			System::assert_last_event(Event::HealthProfessional(
				HealthProfessionalEvent::MinimumStakeAmountUpdated(10),
			));

			assert_ok!(HealthProfessional::update_unstake_time(Origin::signed(admin), 10));

			System::assert_last_event(Event::HealthProfessional(
				HealthProfessionalEvent::UnstakeTimeUpdated(10),
			));

			assert_ok!(HealthProfessional::update_verifier_key(Origin::signed(admin), admin));

			System::assert_last_event(Event::HealthProfessional(
				HealthProfessionalEvent::VerifierKeyUpdated(admin),
			));

			assert_ok!(HealthProfessional::sudo_update_verifier_key(Origin::root(), admin));

			System::assert_last_event(Event::HealthProfessional(
				HealthProfessionalEvent::VerifierKeyUpdated(admin),
			));
		});
	}
}
