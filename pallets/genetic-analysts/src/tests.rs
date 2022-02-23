use crate::{
	mock::*, Error, GeneticAnalyst, GeneticAnalystInfo, GeneticAnalystVerifierKey, PalletAccount,
	StakeStatus,
};
use frame_support::{
	assert_noop, assert_ok,
	sp_runtime::{
		traits::{Hash, Keccak256},
		SaturatedConversion,
	},
};
use frame_system::RawOrigin;
use primitives_verification_status::VerificationStatus;

#[test]
fn register_genetic_analyst_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(1),
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

		assert_eq!(
			GeneticAnalysts::genetic_analyst_by_account_id(1),
			Some(GeneticAnalyst {
				account_id: 1,
				services: Vec::new(),
				qualifications: Vec::new(),
				stake_amount: 0u128.saturated_into(),
				stake_status: StakeStatus::default(),
				verification_status: VerificationStatus::default(),
				info: GeneticAnalystInfo {
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
			})
		);

		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(2),
			GeneticAnalystInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: "First Name 2".as_bytes().to_vec(),
				last_name: "Last Name 2".as_bytes().to_vec(),
				gender: "Gender 2".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email 2".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst 2".as_bytes().to_vec(),
				profile_link: "DeBio Genetic Analyst profile_link 2".as_bytes().to_vec(),
				profile_image: Some("DeBio Genetic Analyst profile_image 2".as_bytes().to_vec()),
			}
		));

		assert_eq!(GeneticAnalysts::genetic_analyst_count(), Some(2),);
	})
}

#[test]
fn update_genetic_analyst_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(1),
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

		assert_eq!(
			GeneticAnalysts::genetic_analyst_by_account_id(1),
			Some(GeneticAnalyst {
				account_id: 1,
				services: Vec::new(),
				qualifications: Vec::new(),
				stake_amount: 0u128.saturated_into(),
				stake_status: StakeStatus::default(),
				verification_status: VerificationStatus::default(),
				info: GeneticAnalystInfo {
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
			})
		);

		assert_ok!(GeneticAnalysts::update_genetic_analyst(
			Origin::signed(1),
			GeneticAnalystInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: "First Name 2".as_bytes().to_vec(),
				last_name: "Last Name 2".as_bytes().to_vec(),
				gender: "Gender 2".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email 2".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst 2".as_bytes().to_vec(),
				profile_link: "DeBio Genetic Analyst profile_link 2".as_bytes().to_vec(),
				profile_image: Some("DeBio Genetic Analyst profile_image 2".as_bytes().to_vec()),
			}
		));

		assert_eq!(
			GeneticAnalysts::genetic_analyst_by_account_id(1),
			Some(GeneticAnalyst {
				account_id: 1,
				services: Vec::new(),
				qualifications: Vec::new(),
				stake_amount: 0u128.saturated_into(),
				stake_status: StakeStatus::default(),
				verification_status: VerificationStatus::default(),
				info: GeneticAnalystInfo {
					box_public_key: Keccak256::hash(
						"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
					),
					first_name: "First Name 2".as_bytes().to_vec(),
					last_name: "Last Name 2".as_bytes().to_vec(),
					gender: "Gender 2".as_bytes().to_vec(),
					date_of_birth: 0,
					email: "Email 2".as_bytes().to_vec(),
					phone_number: "+6893026516".as_bytes().to_vec(),
					specialization: "DeBio Genetic Analyst 2".as_bytes().to_vec(),
					profile_link: "DeBio Genetic Analyst profile_link 2".as_bytes().to_vec(),
					profile_image: Some(
						"DeBio Genetic Analyst profile_image 2".as_bytes().to_vec()
					),
				}
			})
		);
	})
}

#[test]
fn deregister_genetic_analyst_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(1),
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

		assert_ok!(GeneticAnalysts::deregister_genetic_analyst(Origin::signed(1),));

		assert_eq!(GeneticAnalysts::genetic_analyst_by_account_id(1), None);

		assert_eq!(GeneticAnalysts::genetic_analyst_count(), Some(0),);
	})
}

#[test]
fn cant_register_genetic_analyst_when_already_exist() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(1),
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

		assert_noop!(
			GeneticAnalysts::register_genetic_analyst(
				Origin::signed(1),
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
			),
			Error::<Test>::GeneticAnalystAlreadyRegistered
		);
	})
}

#[test]
fn cant_update_and_deregister_genetic_analyst_when_not_exist() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_noop!(
			GeneticAnalysts::update_genetic_analyst(
				Origin::signed(1),
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
			),
			Error::<Test>::GeneticAnalystDoesNotExist
		);

		assert_noop!(
			GeneticAnalysts::deregister_genetic_analyst(Origin::signed(1)),
			Error::<Test>::GeneticAnalystDoesNotExist
		);
	})
}

#[test]
fn call_event_should_work() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(1),
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

		System::assert_last_event(Event::GeneticAnalysts(crate::Event::GeneticAnalystRegistered(
			GeneticAnalyst {
				account_id: 1,
				services: Vec::new(),
				qualifications: Vec::new(),
				stake_amount: 0u128.saturated_into(),
				stake_status: StakeStatus::default(),
				verification_status: VerificationStatus::default(),
				info: GeneticAnalystInfo {
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
				},
			},
			1,
		)));

		assert_ok!(GeneticAnalysts::update_genetic_analyst(
			Origin::signed(1),
			GeneticAnalystInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
				),
				first_name: "First Name 2".as_bytes().to_vec(),
				last_name: "Last Name 2".as_bytes().to_vec(),
				gender: "Gender 2".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email 2".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst 2".as_bytes().to_vec(),
				profile_link: "DeBio Genetic Analyst profile_link 2".as_bytes().to_vec(),
				profile_image: Some("DeBio Genetic Analyst profile_image 2".as_bytes().to_vec()),
			}
		));

		System::assert_last_event(Event::GeneticAnalysts(crate::Event::GeneticAnalystUpdated(
			GeneticAnalyst {
				account_id: 1,
				services: Vec::new(),
				qualifications: Vec::new(),
				stake_amount: 0u128.saturated_into(),
				stake_status: StakeStatus::default(),
				verification_status: VerificationStatus::default(),
				info: GeneticAnalystInfo {
					box_public_key: Keccak256::hash(
						"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
					),
					first_name: "First Name 2".as_bytes().to_vec(),
					last_name: "Last Name 2".as_bytes().to_vec(),
					gender: "Gender 2".as_bytes().to_vec(),
					date_of_birth: 0,
					email: "Email 2".as_bytes().to_vec(),
					phone_number: "+6893026516".as_bytes().to_vec(),
					specialization: "DeBio Genetic Analyst 2".as_bytes().to_vec(),
					profile_link: "DeBio Genetic Analyst profile_link 2".as_bytes().to_vec(),
					profile_image: Some(
						"DeBio Genetic Analyst profile_image 2".as_bytes().to_vec(),
					),
				},
			},
			1,
		)));

		assert_ok!(GeneticAnalysts::deregister_genetic_analyst(Origin::signed(1)));

		System::assert_last_event(Event::GeneticAnalysts(crate::Event::GeneticAnalystDeleted(
			GeneticAnalyst {
				account_id: 1,
				services: Vec::new(),
				qualifications: Vec::new(),
				stake_amount: 0u128.saturated_into(),
				stake_status: StakeStatus::default(),
				verification_status: VerificationStatus::default(),
				info: GeneticAnalystInfo {
					box_public_key: Keccak256::hash(
						"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
					),
					first_name: "First Name 2".as_bytes().to_vec(),
					last_name: "Last Name 2".as_bytes().to_vec(),
					gender: "Gender 2".as_bytes().to_vec(),
					date_of_birth: 0,
					email: "Email 2".as_bytes().to_vec(),
					phone_number: "+6893026516".as_bytes().to_vec(),
					specialization: "DeBio Genetic Analyst 2".as_bytes().to_vec(),
					profile_link: "DeBio Genetic Analyst profile_link 2".as_bytes().to_vec(),
					profile_image: Some(
						"DeBio Genetic Analyst profile_image 2".as_bytes().to_vec(),
					),
				},
			},
			1,
		)));
	})
}

#[test]
fn update_genetic_analyst_verification_status_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(
			RawOrigin::Root.into(),
			1,
			60000000000000000000000u128.saturated_into(),
			0
		));

		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(1),
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

		GeneticAnalystVerifierKey::<Test>::put(2);

		assert_ok!(GeneticAnalysts::stake_genetic_analyst(Origin::signed(1),));

		assert_ok!(GeneticAnalysts::update_genetic_analyst_verification_status(
			Origin::signed(2),
			1,
			VerificationStatus::Verified,
		));

		assert_eq!(
			GeneticAnalysts::genetic_analyst_by_account_id(1),
			Some(GeneticAnalyst {
				account_id: 1,
				services: Vec::new(),
				qualifications: Vec::new(),
				stake_amount: 50000000000000000000000u128.saturated_into(),
				stake_status: StakeStatus::Staked,
				verification_status: VerificationStatus::Verified,
				info: GeneticAnalystInfo {
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
			})
		);
	})
}

#[test]
fn update_genetic_analyst_verification_status_reject_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(
			RawOrigin::Root.into(),
			1,
			60000000000000000000000u128.saturated_into(),
			0
		));

		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(1),
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

		GeneticAnalystVerifierKey::<Test>::put(2);

		assert_ok!(GeneticAnalysts::update_minimum_stake_amount(
			Origin::signed(2),
			0u128.saturated_into(),
		));

		assert_ok!(GeneticAnalysts::stake_genetic_analyst(Origin::signed(1),));

		assert_ok!(GeneticAnalysts::update_genetic_analyst_verification_status(
			Origin::signed(2),
			1,
			VerificationStatus::Rejected,
		));

		assert_eq!(
			GeneticAnalysts::genetic_analyst_by_account_id(1),
			Some(GeneticAnalyst {
				account_id: 1,
				services: Vec::new(),
				qualifications: Vec::new(),
				stake_amount: 0u128.saturated_into(),
				stake_status: StakeStatus::Unstaked,
				verification_status: VerificationStatus::Rejected,
				info: GeneticAnalystInfo {
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
			})
		);
	})
}

#[test]
fn cant_update_genetic_analyst_verification_status_when_is_not_staked() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(1),
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

		GeneticAnalystVerifierKey::<Test>::put(2);

		assert_noop!(
			GeneticAnalysts::update_genetic_analyst_verification_status(
				Origin::signed(2),
				1,
				VerificationStatus::Verified,
			),
			Error::<Test>::GeneticAnalystIsNotStaked
		);

		assert_eq!(
			GeneticAnalysts::genetic_analyst_by_account_id(1),
			Some(GeneticAnalyst {
				account_id: 1,
				services: Vec::new(),
				qualifications: Vec::new(),
				stake_amount: 0u128.saturated_into(),
				stake_status: StakeStatus::default(),
				verification_status: VerificationStatus::default(),
				info: GeneticAnalystInfo {
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
			})
		);
	})
}

#[test]
fn cant_update_genetic_analyst_verification_status_when_not_admin() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(1),
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

		assert_noop!(
			GeneticAnalysts::update_genetic_analyst_verification_status(
				Origin::signed(2),
				1,
				VerificationStatus::Verified,
			),
			Error::<Test>::Unauthorized
		);
	})
}

#[test]
fn cant_update_genetic_analyst_verification_status_when_not_exist() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		GeneticAnalystVerifierKey::<Test>::put(2);

		assert_noop!(
			GeneticAnalysts::update_genetic_analyst_verification_status(
				Origin::signed(2),
				1,
				VerificationStatus::Verified,
			),
			Error::<Test>::GeneticAnalystDoesNotExist
		);
	})
}

#[test]
fn cant_update_genetic_analyst_verification_status_when_bad_signature() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(
			RawOrigin::Root.into(),
			1,
			60000000000000000000000u128.saturated_into(),
			0
		));

		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(1),
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

		GeneticAnalystVerifierKey::<Test>::put(2);

		assert_ok!(GeneticAnalysts::stake_genetic_analyst(Origin::signed(1),));

		assert_ok!(Balances::set_balance(
			RawOrigin::Root.into(),
			PalletAccount::<Test>::get(),
			0u128.saturated_into(),
			0
		));

		assert_noop!(
			GeneticAnalysts::update_genetic_analyst_verification_status(
				Origin::signed(2),
				1,
				VerificationStatus::Rejected,
			),
			Error::<Test>::BadSignature
		);
	})
}

#[test]
fn stake_genetic_analyst_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(
			RawOrigin::Root.into(),
			1,
			60000000000000000000000u128.saturated_into(),
			0
		));

		GeneticAnalystVerifierKey::<Test>::put(2);

		assert_ok!(GeneticAnalysts::update_minimum_stake_amount(
			Origin::signed(2),
			60000000000000000000000u128.saturated_into(),
		));

		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(1),
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

		assert_ok!(GeneticAnalysts::stake_genetic_analyst(Origin::signed(1),));

		assert_eq!(
			GeneticAnalysts::genetic_analyst_by_account_id(1),
			Some(GeneticAnalyst {
				account_id: 1,
				services: Vec::new(),
				qualifications: Vec::new(),
				stake_amount: 60000000000000000000000u128.saturated_into(),
				stake_status: StakeStatus::Staked,
				verification_status: VerificationStatus::default(),
				info: GeneticAnalystInfo {
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
			})
		);
	})
}

#[test]
fn cant_stake_genetic_analyst_when_not_exist() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_noop!(
			GeneticAnalysts::stake_genetic_analyst(Origin::signed(1),),
			Error::<Test>::GeneticAnalystDoesNotExist
		);
	})
}

#[test]
fn cant_stake_genetic_analyst_when_insufficient_funds() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(1),
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

		assert_noop!(
			GeneticAnalysts::stake_genetic_analyst(Origin::signed(1),),
			Error::<Test>::InsufficientFunds
		);

		assert_eq!(
			GeneticAnalysts::genetic_analyst_by_account_id(1),
			Some(GeneticAnalyst {
				account_id: 1,
				services: Vec::new(),
				qualifications: Vec::new(),
				stake_amount: 0u128.saturated_into(),
				stake_status: StakeStatus::default(),
				verification_status: VerificationStatus::default(),
				info: GeneticAnalystInfo {
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
			})
		);
	})
}

#[test]
fn cant_stake_genetic_analyst_when_already_staked() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(Balances::set_balance(
			RawOrigin::Root.into(),
			1,
			60000000000000000000000u128.saturated_into(),
			0
		));

		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(1),
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

		assert_ok!(GeneticAnalysts::stake_genetic_analyst(Origin::signed(1),));

		assert_eq!(
			GeneticAnalysts::genetic_analyst_by_account_id(1),
			Some(GeneticAnalyst {
				account_id: 1,
				services: Vec::new(),
				qualifications: Vec::new(),
				stake_amount: 50000000000000000000000u128.saturated_into(),
				stake_status: StakeStatus::Staked,
				verification_status: VerificationStatus::default(),
				info: GeneticAnalystInfo {
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
			})
		);

		assert_noop!(
			GeneticAnalysts::stake_genetic_analyst(Origin::signed(1),),
			Error::<Test>::GeneticAnalystAlreadyStaked
		);
	})
}

#[test]
fn update_minimum_stake_amount_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		GeneticAnalystVerifierKey::<Test>::put(2);

		assert_ok!(GeneticAnalysts::update_minimum_stake_amount(
			Origin::signed(2),
			60000000000000000000000u128.saturated_into(),
		));

		assert_eq!(
			GeneticAnalysts::minimum_stake_amount(),
			Some(60000000000000000000000u128.saturated_into())
		);
	})
}

#[test]
fn update_admin_key_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		GeneticAnalystVerifierKey::<Test>::put(2);

		assert_eq!(GeneticAnalysts::admin_key(), 2);

		assert_ok!(GeneticAnalysts::update_admin_key(Origin::signed(2), 1,));

		assert_eq!(GeneticAnalysts::admin_key(), 1);
	})
}

#[test]
fn sudo_update_admin_key_works() {
	<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
		assert_ok!(GeneticAnalysts::sudo_update_admin_key(Origin::root(), 1));

		assert_eq!(GeneticAnalysts::admin_key(), 1);
	})
}
