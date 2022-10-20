mod mock;

#[cfg(test)]
mod tests {
	use crate::mock::*;

	use genetic_analysts::{
		Error, Event as EventC, GeneticAnalyst, GeneticAnalystInfo, GeneticAnalystVerifierKey,
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

	use genetic_analysis::{GeneticAnalysisStatus, GeneticAnalysisTracking};
	use genetic_analysis_orders::{
		GeneticAnalysisOrder, GeneticAnalysisOrderStatus, PalletAccount as OrderPalletAccount,
	};
	use genetic_analyst_services::GeneticAnalystServiceInfo;

	use primitives_availability_status::AvailabilityStatus;
	use primitives_duration::ExpectedDuration;
	use primitives_price_and_currency::{CurrencyType, PriceByCurrency};
	use primitives_stake_status::StakeStatus;
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
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					account_id: 1,
					services: Vec::new(),
					qualifications: Vec::new(),
					stake_amount: 0u128.saturated_into(),
					stake_status: StakeStatus::default(),
					verification_status: VerificationStatus::default(),
					availability_status: AvailabilityStatus::default(),
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
						profile_image: Some(
							"DeBio Genetic Analyst profile_image".as_bytes().to_vec()
						),
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
					profile_image: Some(
						"DeBio Genetic Analyst profile_image 2".as_bytes().to_vec()
					),
				}
			));

			assert_eq!(GeneticAnalysts::genetic_analyst_count(), Some(2),);
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
						profile_image: Some(
							"DeBio Genetic Analyst profile_image".as_bytes().to_vec()
						),
					}
				),
				Error::<Test>::GeneticAnalystAlreadyRegistered
			);
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
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					account_id: 1,
					services: Vec::new(),
					qualifications: Vec::new(),
					stake_amount: 0u128.saturated_into(),
					stake_status: StakeStatus::default(),
					verification_status: VerificationStatus::default(),
					availability_status: AvailabilityStatus::default(),
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
						profile_image: Some(
							"DeBio Genetic Analyst profile_image".as_bytes().to_vec()
						),
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
					profile_image: Some(
						"DeBio Genetic Analyst profile_image 2".as_bytes().to_vec()
					),
				}
			));

			assert_eq!(
				GeneticAnalysts::genetic_analyst_by_account_id(1),
				Some(GeneticAnalyst {
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					account_id: 1,
					services: Vec::new(),
					qualifications: Vec::new(),
					stake_amount: 0u128.saturated_into(),
					stake_status: StakeStatus::default(),
					verification_status: VerificationStatus::default(),
					availability_status: AvailabilityStatus::default(),
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
						profile_image: Some(
							"DeBio Genetic Analyst profile_image".as_bytes().to_vec()
						),
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

			System::assert_last_event(Event::GeneticAnalysts(EventC::GeneticAnalystRegistered(
				GeneticAnalyst {
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					account_id: 1,
					services: Vec::new(),
					qualifications: Vec::new(),
					stake_amount: 0u128.saturated_into(),
					stake_status: StakeStatus::default(),
					verification_status: VerificationStatus::default(),
					availability_status: AvailabilityStatus::default(),
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
						profile_image: Some(
							"DeBio Genetic Analyst profile_image".as_bytes().to_vec(),
						),
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
					profile_image: Some(
						"DeBio Genetic Analyst profile_image 2".as_bytes().to_vec()
					),
				}
			));

			System::assert_last_event(Event::GeneticAnalysts(EventC::GeneticAnalystUpdated(
				GeneticAnalyst {
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					account_id: 1,
					services: Vec::new(),
					qualifications: Vec::new(),
					stake_amount: 0u128.saturated_into(),
					stake_status: StakeStatus::default(),
					verification_status: VerificationStatus::default(),
					availability_status: AvailabilityStatus::default(),
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

			System::assert_last_event(Event::GeneticAnalysts(EventC::GeneticAnalystDeleted(
				GeneticAnalyst {
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					account_id: 1,
					services: Vec::new(),
					qualifications: Vec::new(),
					stake_amount: 0u128.saturated_into(),
					stake_status: StakeStatus::default(),
					verification_status: VerificationStatus::default(),
					availability_status: AvailabilityStatus::default(),
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

			PalletAccount::<Test>::put(4);
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
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					account_id: 1,
					services: Vec::new(),
					qualifications: Vec::new(),
					stake_amount: 50000000000000000000000u128.saturated_into(),
					stake_status: StakeStatus::Staked,
					verification_status: VerificationStatus::Verified,
					availability_status: AvailabilityStatus::default(),
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
						profile_image: Some(
							"DeBio Genetic Analyst profile_image".as_bytes().to_vec()
						),
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

			PalletAccount::<Test>::put(4);
			GeneticAnalystVerifierKey::<Test>::put(2);

			assert_ok!(GeneticAnalysts::stake_genetic_analyst(Origin::signed(1),));

			assert_ok!(GeneticAnalysts::update_genetic_analyst_verification_status(
				Origin::signed(2),
				1,
				VerificationStatus::Rejected,
			));

			assert_eq!(
				GeneticAnalysts::genetic_analyst_by_account_id(1),
				Some(GeneticAnalyst {
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					account_id: 1,
					services: Vec::new(),
					qualifications: Vec::new(),
					stake_amount: 0u128.saturated_into(),
					stake_status: StakeStatus::Unstaked,
					verification_status: VerificationStatus::Rejected,
					availability_status: AvailabilityStatus::Unavailable,
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
						profile_image: Some(
							"DeBio Genetic Analyst profile_image".as_bytes().to_vec()
						),
					}
				})
			);
		})
	}

	#[test]
	fn cant_update_genetic_analyst_verification_status_when_unauthorized() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			GeneticAnalystVerifierKey::<Test>::put(4);

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
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					account_id: 1,
					services: Vec::new(),
					qualifications: Vec::new(),
					stake_amount: 0u128.saturated_into(),
					stake_status: StakeStatus::default(),
					verification_status: VerificationStatus::default(),
					availability_status: AvailabilityStatus::default(),
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
						profile_image: Some(
							"DeBio Genetic Analyst profile_image".as_bytes().to_vec()
						),
					}
				})
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
	fn cant_update_genetic_analyst_verification_status_when_insufficient_pallet_funds() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			assert_ok!(Balances::set_balance(
				RawOrigin::Root.into(),
				1,
				60000000000000000000000u128.saturated_into(),
				0
			));

			PalletAccount::<Test>::put(0);

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
				PalletAccount::<Test>::get().unwrap(),
				0u128.saturated_into(),
				0
			));

			assert_noop!(
				GeneticAnalysts::update_genetic_analyst_verification_status(
					Origin::signed(2),
					1,
					VerificationStatus::Rejected,
				),
				Error::<Test>::InsufficientPalletFunds
			);
		})
	}

	#[test]
	fn stake_genetic_analyst_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			assert_ok!(Balances::set_balance(
				RawOrigin::Root.into(),
				1,
				70000000000000000000000u128.saturated_into(),
				0
			));

			PalletAccount::<Test>::put(4);
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
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					account_id: 1,
					services: Vec::new(),
					qualifications: Vec::new(),
					stake_amount: 60000000000000000000000u128.saturated_into(),
					stake_status: StakeStatus::Staked,
					verification_status: VerificationStatus::default(),
					availability_status: AvailabilityStatus::default(),
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
						profile_image: Some(
							"DeBio Genetic Analyst profile_image".as_bytes().to_vec()
						),
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
	fn cant_stake_genetic_analyst_when_already_staked() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			PalletAccount::<Test>::put(4);
			GeneticAnalystVerifierKey::<Test>::put(1);

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
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					account_id: 1,
					services: Vec::new(),
					qualifications: Vec::new(),
					stake_amount: 50000000000000000000000u128.saturated_into(),
					stake_status: StakeStatus::Staked,
					verification_status: VerificationStatus::default(),
					availability_status: AvailabilityStatus::default(),
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
						profile_image: Some(
							"DeBio Genetic Analyst profile_image".as_bytes().to_vec()
						),
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
	fn stake_genetic_analyst_when_waiting_for_unstake_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			assert_ok!(Balances::set_balance(
				RawOrigin::Root.into(),
				1,
				60000000000000000000000u128.saturated_into(),
				0
			));

			PalletAccount::<Test>::put(4);
			GeneticAnalystVerifierKey::<Test>::put(2);

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

			assert_ok!(GeneticAnalysts::unstake_genetic_analyst(Origin::signed(1),));

			assert_ok!(GeneticAnalysts::stake_genetic_analyst(Origin::signed(1),));

			assert_eq!(
				GeneticAnalysts::genetic_analyst_by_account_id(1),
				Some(GeneticAnalyst {
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					account_id: 1,
					services: Vec::new(),
					qualifications: Vec::new(),
					stake_amount: 50000000000000000000000u128.saturated_into(),
					stake_status: StakeStatus::Staked,
					verification_status: VerificationStatus::default(),
					availability_status: AvailabilityStatus::Unavailable,
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
						profile_image: Some(
							"DeBio Genetic Analyst profile_image".as_bytes().to_vec()
						),
					}
				})
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
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					account_id: 1,
					services: Vec::new(),
					qualifications: Vec::new(),
					stake_amount: 0u128.saturated_into(),
					stake_status: StakeStatus::default(),
					verification_status: VerificationStatus::default(),
					availability_status: AvailabilityStatus::default(),
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
						profile_image: Some(
							"DeBio Genetic Analyst profile_image".as_bytes().to_vec()
						),
					}
				})
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
	fn cant_update_minimum_stake_amount_when_unauthorized() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			GeneticAnalystVerifierKey::<Test>::put(3);

			assert_noop!(
				GeneticAnalysts::update_minimum_stake_amount(
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
			GeneticAnalystVerifierKey::<Test>::put(2);

			assert_ok!(GeneticAnalysts::update_unstake_time(
				Origin::signed(2),
				1000u64.saturated_into(),
			));

			assert_eq!(GeneticAnalysts::unstake_time(), Some(1000u64.saturated_into()));
		})
	}

	#[test]
	fn cant_update_unstake_time_when_unauthorized() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			GeneticAnalystVerifierKey::<Test>::put(3);

			assert_noop!(
				GeneticAnalysts::update_unstake_time(Origin::signed(2), 1000u64.saturated_into(),),
				Error::<Test>::Unauthorized
			);
		})
	}

	#[test]
	fn update_admin_key_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			GeneticAnalystVerifierKey::<Test>::put(2);

			assert_eq!(GeneticAnalysts::admin_key(), Some(2));

			assert_ok!(GeneticAnalysts::update_admin_key(Origin::signed(2), 1,));

			assert_eq!(GeneticAnalysts::admin_key(), Some(1));
		})
	}

	#[test]
	fn cant_update_admin_key_when_unauthorized() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			GeneticAnalystVerifierKey::<Test>::put(3);

			assert_noop!(
				GeneticAnalysts::update_admin_key(Origin::signed(2), 1,),
				Error::<Test>::Unauthorized
			);
		})
	}

	#[test]
	fn sudo_update_admin_key_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			assert_ok!(GeneticAnalysts::sudo_update_admin_key(Origin::root(), 1));

			assert_eq!(GeneticAnalysts::admin_key(), Some(1));
		})
	}

	#[test]
	fn cant_sudo_update_admin_key_when_not_sudo() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			GeneticAnalystVerifierKey::<Test>::put(4);
			assert_noop!(
				GeneticAnalysts::update_admin_key(Origin::signed(2), 1,),
				Error::<Test>::Unauthorized
			);
		})
	}

	#[test]
	fn update_genetic_analyst_availability_status_works() {
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

			PalletAccount::<Test>::put(4);
			GeneticAnalystVerifierKey::<Test>::put(2);

			assert_ok!(GeneticAnalysts::stake_genetic_analyst(Origin::signed(1),));

			assert_ok!(GeneticAnalysts::update_genetic_analyst_verification_status(
				Origin::signed(2),
				1,
				VerificationStatus::Verified,
			));

			assert_ok!(GeneticAnalysts::update_genetic_analyst_availability_status(
				Origin::signed(1),
				AvailabilityStatus::Available,
			));

			assert_eq!(
				GeneticAnalysts::genetic_analyst_by_account_id(1),
				Some(GeneticAnalyst {
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					account_id: 1,
					services: Vec::new(),
					qualifications: Vec::new(),
					stake_amount: 50000000000000000000000u128.saturated_into(),
					stake_status: StakeStatus::Staked,
					verification_status: VerificationStatus::Verified,
					availability_status: AvailabilityStatus::Available,
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
						profile_image: Some(
							"DeBio Genetic Analyst profile_image".as_bytes().to_vec()
						),
					}
				})
			);
		})
	}

	#[test]
	fn update_genetic_analyst_availability_status_unavailable_works() {
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

			PalletAccount::<Test>::put(4);
			GeneticAnalystVerifierKey::<Test>::put(2);

			assert_ok!(GeneticAnalysts::update_minimum_stake_amount(
				Origin::signed(2),
				0u128.saturated_into(),
			));

			assert_ok!(GeneticAnalysts::stake_genetic_analyst(Origin::signed(1),));

			assert_ok!(GeneticAnalysts::update_genetic_analyst_availability_status(
				Origin::signed(1),
				AvailabilityStatus::Available,
			));

			assert_ok!(GeneticAnalysts::update_genetic_analyst_availability_status(
				Origin::signed(1),
				AvailabilityStatus::Unavailable,
			));

			assert_eq!(
				GeneticAnalysts::genetic_analyst_by_account_id(1),
				Some(GeneticAnalyst {
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					account_id: 1,
					services: Vec::new(),
					qualifications: Vec::new(),
					stake_amount: 0u128.saturated_into(),
					stake_status: StakeStatus::Staked,
					verification_status: VerificationStatus::Unverified,
					availability_status: AvailabilityStatus::Unavailable,
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
						profile_image: Some(
							"DeBio Genetic Analyst profile_image".as_bytes().to_vec()
						),
					}
				})
			);
		})
	}

	#[test]
	fn cant_update_genetic_analyst_availability_status_when_not_exist() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			GeneticAnalystVerifierKey::<Test>::put(2);

			assert_noop!(
				GeneticAnalysts::update_genetic_analyst_availability_status(
					Origin::signed(1),
					AvailabilityStatus::Available,
				),
				Error::<Test>::GeneticAnalystDoesNotExist
			);
		})
	}

	#[test]
	fn unstake_genetic_analyst_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			assert_ok!(Balances::set_balance(
				RawOrigin::Root.into(),
				1,
				60000000000000000000000u128.saturated_into(),
				0
			));

			PalletAccount::<Test>::put(4);
			GeneticAnalystVerifierKey::<Test>::put(2);

			assert_ok!(GeneticAnalysts::update_minimum_stake_amount(
				Origin::signed(2),
				0u128.saturated_into(),
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

			assert_ok!(GeneticAnalysts::unstake_genetic_analyst(Origin::signed(1),));

			assert_eq!(
				GeneticAnalysts::genetic_analyst_by_account_id(1),
				Some(GeneticAnalyst {
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					account_id: 1,
					services: Vec::new(),
					qualifications: Vec::new(),
					stake_amount: 0u128.saturated_into(),
					stake_status: StakeStatus::WaitingForUnstaked,
					verification_status: VerificationStatus::default(),
					availability_status: AvailabilityStatus::Unavailable,
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
						profile_image: Some(
							"DeBio Genetic Analyst profile_image".as_bytes().to_vec()
						),
					}
				})
			);
		})
	}

	#[test]
	fn cant_unstake_genetic_analyst_when_not_exist() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			assert_noop!(
				GeneticAnalysts::stake_genetic_analyst(Origin::signed(1),),
				Error::<Test>::GeneticAnalystDoesNotExist
			);
		})
	}

	#[test]
	fn cant_unstake_genetic_analyst_when_not_staked() {
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
				GeneticAnalysts::unstake_genetic_analyst(Origin::signed(1),),
				Error::<Test>::GeneticAnalystIsNotStaked
			);
		})
	}

	#[test]
	fn cant_unstake_genetic_analyst_when_pending_order_exists() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			assert_ok!(Balances::set_balance(
				RawOrigin::Root.into(),
				1,
				60000000000000000000000u128.saturated_into(),
				0
			));

			PalletAccount::<Test>::put(4);
			GeneticAnalystVerifierKey::<Test>::put(2);

			assert_ok!(GeneticAnalysts::update_minimum_stake_amount(
				Origin::signed(2),
				0u128.saturated_into(),
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

			assert_ok!(GeneticAnalysts::update_genetic_analyst_availability_status(
				Origin::signed(1),
				AvailabilityStatus::Available
			));

			assert_ok!(GeneticAnalysts::stake_genetic_analyst(Origin::signed(1),));

			assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
				Origin::signed(1),
				GeneticAnalystServiceInfo {
					name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
					prices_by_currency: vec![PriceByCurrency::default()],
					expected_duration: ExpectedDuration::default(),
					description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
					test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
						.as_bytes()
						.to_vec(),
				},
			));

			let _genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

			let _add_genetic_data = GeneticData::add_genetic_data(
				Origin::signed(1),
				"DeBio Genetic Data".as_bytes().to_vec(),
				"DeBio Genetic Data Document Description".as_bytes().to_vec(),
				"DeBio Genetic Data Link".as_bytes().to_vec(),
			);

			let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

			assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
				Origin::signed(1),
				_genetic_data_ids[0],
				_genetic_analyst.services[0],
				0,
				Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
				"DeBio Genetic Genetic Link".as_bytes().to_vec(),
				None,
			));

			let _genetic_analysis_order_id =
				GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();
			let _genetic_analysis =
				GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

			assert_eq!(
				GeneticAnalysisOrders::genetic_analysis_order_by_id(&_genetic_analysis_order_id),
				Some(GeneticAnalysisOrder {
					id: _genetic_analysis_order_id,
					genetic_data_id: _genetic_data_ids[0],
					service_id: _genetic_analyst.services[0],
					customer_id: 1,
					customer_box_public_key: Keccak256::hash(
						"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()
					),
					seller_id: 1,
					genetic_analysis_tracking_id: _genetic_analysis[0].clone(),
					genetic_link: "DeBio Genetic Genetic Link".as_bytes().to_vec(),
					asset_id: None,
					currency: CurrencyType::default(),
					prices: PriceByCurrency::default().price_components,
					additional_prices: PriceByCurrency::default().additional_prices,
					total_price: PriceByCurrency::default().total_price,
					status: GeneticAnalysisOrderStatus::default(),
					created_at: 0,
					updated_at: 0
				})
			);

			assert_noop!(
				GeneticAnalysts::unstake_genetic_analyst(Origin::signed(1),),
				Error::<Test>::GeneticAnalystHasPendingOrders
			);
		})
	}

	#[test]
	fn cant_update_genetic_analyst_availability_status_unavailable_when_pending_order_exists() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			assert_ok!(Balances::set_balance(
				RawOrigin::Root.into(),
				1,
				60000000000000000000000u128.saturated_into(),
				0
			));

			PalletAccount::<Test>::put(4);
			GeneticAnalystVerifierKey::<Test>::put(2);

			assert_ok!(GeneticAnalysts::update_minimum_stake_amount(
				Origin::signed(2),
				0u128.saturated_into(),
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

			assert_ok!(GeneticAnalysts::update_genetic_analyst_availability_status(
				Origin::signed(1),
				AvailabilityStatus::Available
			));

			assert_ok!(GeneticAnalysts::stake_genetic_analyst(Origin::signed(1),));

			assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
				Origin::signed(1),
				GeneticAnalystServiceInfo {
					name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
					prices_by_currency: vec![PriceByCurrency::default()],
					expected_duration: ExpectedDuration::default(),
					description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
					test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
						.as_bytes()
						.to_vec(),
				},
			));

			let _genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

			let _add_genetic_data = GeneticData::add_genetic_data(
				Origin::signed(1),
				"DeBio Genetic Data".as_bytes().to_vec(),
				"DeBio Genetic Data Document Description".as_bytes().to_vec(),
				"DeBio Genetic Data Link".as_bytes().to_vec(),
			);

			let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

			assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
				Origin::signed(1),
				_genetic_data_ids[0],
				_genetic_analyst.services[0],
				0,
				Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
				"DeBio Genetic Genetic Link".as_bytes().to_vec(),
				None,
			));

			let _genetic_analysis_order_id =
				GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();
			let _genetic_analysis =
				GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

			assert_eq!(
				GeneticAnalysisOrders::genetic_analysis_order_by_id(&_genetic_analysis_order_id),
				Some(GeneticAnalysisOrder {
					id: _genetic_analysis_order_id,
					genetic_data_id: _genetic_data_ids[0],
					service_id: _genetic_analyst.services[0],
					customer_id: 1,
					customer_box_public_key: Keccak256::hash(
						"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()
					),
					seller_id: 1,
					genetic_analysis_tracking_id: _genetic_analysis[0].clone(),
					genetic_link: "DeBio Genetic Genetic Link".as_bytes().to_vec(),
					asset_id: None,
					currency: CurrencyType::default(),
					prices: PriceByCurrency::default().price_components,
					additional_prices: PriceByCurrency::default().additional_prices,
					total_price: PriceByCurrency::default().total_price,
					status: GeneticAnalysisOrderStatus::default(),
					created_at: 0,
					updated_at: 0
				})
			);

			assert_noop!(
				GeneticAnalysts::update_genetic_analyst_availability_status(
					Origin::signed(1),
					AvailabilityStatus::Unavailable,
				),
				Error::<Test>::GeneticAnalystHasPendingOrders
			);
		})
	}

	#[test]
	fn cant_update_genetic_analyst_verification_status_reject_when_pending_order_exists() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			assert_ok!(Balances::set_balance(
				RawOrigin::Root.into(),
				1,
				60000000000000000000000u128.saturated_into(),
				0
			));

			PalletAccount::<Test>::put(4);
			GeneticAnalystVerifierKey::<Test>::put(2);

			assert_ok!(GeneticAnalysts::update_minimum_stake_amount(
				Origin::signed(2),
				0u128.saturated_into(),
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

			assert_ok!(GeneticAnalysts::update_genetic_analyst_availability_status(
				Origin::signed(1),
				AvailabilityStatus::Available
			));

			assert_ok!(GeneticAnalysts::stake_genetic_analyst(Origin::signed(1),));

			assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
				Origin::signed(1),
				GeneticAnalystServiceInfo {
					name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
					prices_by_currency: vec![PriceByCurrency::default()],
					expected_duration: ExpectedDuration::default(),
					description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
					test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
						.as_bytes()
						.to_vec(),
				},
			));

			let _genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

			let _add_genetic_data = GeneticData::add_genetic_data(
				Origin::signed(1),
				"DeBio Genetic Data".as_bytes().to_vec(),
				"DeBio Genetic Data Document Description".as_bytes().to_vec(),
				"DeBio Genetic Data Link".as_bytes().to_vec(),
			);

			let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

			assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
				Origin::signed(1),
				_genetic_data_ids[0],
				_genetic_analyst.services[0],
				0,
				Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
				"DeBio Genetic Genetic Link".as_bytes().to_vec(),
				None,
			));

			let _genetic_analysis_order_id =
				GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();
			let _genetic_analysis =
				GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

			assert_eq!(
				GeneticAnalysisOrders::genetic_analysis_order_by_id(&_genetic_analysis_order_id),
				Some(GeneticAnalysisOrder {
					id: _genetic_analysis_order_id,
					genetic_data_id: _genetic_data_ids[0],
					service_id: _genetic_analyst.services[0],
					customer_id: 1,
					customer_box_public_key: Keccak256::hash(
						"0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()
					),
					seller_id: 1,
					genetic_analysis_tracking_id: _genetic_analysis[0].clone(),
					genetic_link: "DeBio Genetic Genetic Link".as_bytes().to_vec(),
					asset_id: None,
					currency: CurrencyType::default(),
					prices: PriceByCurrency::default().price_components,
					additional_prices: PriceByCurrency::default().additional_prices,
					total_price: PriceByCurrency::default().total_price,
					status: GeneticAnalysisOrderStatus::default(),
					created_at: 0,
					updated_at: 0
				})
			);

			assert_noop!(
				GeneticAnalysts::update_genetic_analyst_verification_status(
					Origin::signed(2),
					1,
					VerificationStatus::Rejected,
				),
				Error::<Test>::GeneticAnalystHasPendingOrders
			);
		})
	}

	#[test]
	fn unstake_genetic_analyst_after_pending_order_is_rejected() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			OrderPalletAccount::<Test>::put(0);

			assert_ok!(Balances::set_balance(
				RawOrigin::Root.into(),
				1,
				60000000000000000000000u128.saturated_into(),
				0
			));

			PalletAccount::<Test>::put(4);
			GeneticAnalystVerifierKey::<Test>::put(2);

			assert_ok!(GeneticAnalysts::update_minimum_stake_amount(
				Origin::signed(2),
				0u128.saturated_into(),
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

			assert_ok!(GeneticAnalysts::update_genetic_analyst_availability_status(
				Origin::signed(1),
				AvailabilityStatus::Available
			));

			assert_ok!(GeneticAnalysts::stake_genetic_analyst(Origin::signed(1),));

			assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
				Origin::signed(1),
				GeneticAnalystServiceInfo {
					name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
					prices_by_currency: vec![PriceByCurrency::default()],
					expected_duration: ExpectedDuration::default(),
					description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
					test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
						.as_bytes()
						.to_vec(),
				},
			));

			let _genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

			let _add_genetic_data = GeneticData::add_genetic_data(
				Origin::signed(1),
				"DeBio Genetic Data".as_bytes().to_vec(),
				"DeBio Genetic Data Document Description".as_bytes().to_vec(),
				"DeBio Genetic Data Link".as_bytes().to_vec(),
			);

			let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

			assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
				Origin::signed(1),
				_genetic_data_ids[0],
				_genetic_analyst.services[0],
				0,
				Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
				"DeBio Genetic Genetic Link".as_bytes().to_vec(),
				None,
			));

			let _genetic_analysis_order_id =
				GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();

			assert_ok!(GeneticAnalysisOrders::set_genetic_analysis_order_paid(
				Origin::signed(1),
				_genetic_analysis_order_id
			));

			let _genetic_analysis =
				GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

			assert_ok!(GeneticAnalysis::reject_genetic_analysis(
				Origin::signed(1),
				_genetic_analysis[0].clone(),
				"Reject DNA Title".as_bytes().to_vec(),
				"Reject DNA Description".as_bytes().to_vec()
			));

			let _genetic_analysis_info =
				GeneticAnalysis::genetic_analysis_by_genetic_analysis_tracking_id(
					_genetic_analysis[0].clone(),
				)
				.unwrap();

			assert_eq!(
				_genetic_analysis_info.get_genetic_analysis_tracking_id(),
				&_genetic_analysis[0]
			);
			assert!(_genetic_analysis_info.is_rejected());

			assert_ok!(GeneticAnalysts::unstake_genetic_analyst(Origin::signed(1),));
		})
	}

	#[test]
	fn unstake_genetic_analyst_after_pending_order_is_result_ready() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			OrderPalletAccount::<Test>::put(0);

			assert_ok!(Balances::set_balance(
				RawOrigin::Root.into(),
				1,
				60000000000000000000000u128.saturated_into(),
				0
			));

			PalletAccount::<Test>::put(4);
			GeneticAnalystVerifierKey::<Test>::put(2);

			assert_ok!(GeneticAnalysts::update_minimum_stake_amount(
				Origin::signed(2),
				0u128.saturated_into(),
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

			assert_ok!(GeneticAnalysts::update_genetic_analyst_availability_status(
				Origin::signed(1),
				AvailabilityStatus::Available
			));

			assert_ok!(GeneticAnalysts::stake_genetic_analyst(Origin::signed(1),));

			assert_ok!(UserProfile::set_eth_address(
				Origin::signed(1),
				EthereumAddress([b'X'; 20])
			));

			assert_ok!(GeneticAnalystServices::create_genetic_analyst_service(
				Origin::signed(1),
				GeneticAnalystServiceInfo {
					name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
					prices_by_currency: vec![PriceByCurrency::default()],
					expected_duration: ExpectedDuration::default(),
					description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
					test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
						.as_bytes()
						.to_vec(),
				}
			));

			let _genetic_analyst = GeneticAnalysts::genetic_analyst_by_account_id(1).unwrap();

			let _add_genetic_data = GeneticData::add_genetic_data(
				Origin::signed(1),
				"DeBio Genetic Data".as_bytes().to_vec(),
				"DeBio Genetic Data Document Description".as_bytes().to_vec(),
				"DeBio Genetic Data Link".as_bytes().to_vec(),
			);

			let _genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

			assert_ok!(GeneticAnalysisOrders::create_genetic_analysis_order(
				Origin::signed(1),
				_genetic_data_ids[0],
				_genetic_analyst.services[0],
				0,
				Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
				"DeBio Genetic Genetic Link".as_bytes().to_vec(),
				None,
			));

			let _genetic_analysis_order_id =
				GeneticAnalysisOrders::last_genetic_analysis_order_by_customer_id(1).unwrap();

			assert_ok!(GeneticAnalysisOrders::set_genetic_analysis_order_paid(
				Origin::signed(1),
				_genetic_analysis_order_id
			));

			let _genetic_analysis =
				GeneticAnalysis::genetic_analysis_by_genetic_analyst_id(1).unwrap();

			assert_ok!(GeneticAnalysis::submit_genetic_analysis(
				Origin::signed(1),
				_genetic_analysis[0].clone(),
				"Genetic Analysis report_link".as_bytes().to_vec(),
				Some("Genetic Analysis comments".as_bytes().to_vec())
			));

			let _genetic_analysis_ =
				GeneticAnalysis::genetic_analysis_by_genetic_analysis_tracking_id(
					_genetic_analysis[0].clone(),
				)
				.unwrap();

			assert_eq!(
				_genetic_analysis_.genetic_analysis_tracking_id,
				_genetic_analysis[0].clone()
			);
			assert_eq!(_genetic_analysis_.genetic_analyst_id, 1);
			assert_eq!(_genetic_analysis_.owner_id, 1);
			assert_eq!(
				_genetic_analysis_.comment,
				Some("Genetic Analysis comments".as_bytes().to_vec())
			);
			assert_eq!(
				_genetic_analysis_.report_link,
				"Genetic Analysis report_link".as_bytes().to_vec()
			);

			assert_ok!(GeneticAnalysis::process_genetic_analysis(
				Origin::signed(1),
				_genetic_analysis[0].clone(),
				GeneticAnalysisStatus::ResultReady
			));

			let _genetic_analysis_info =
				GeneticAnalysis::genetic_analysis_by_genetic_analysis_tracking_id(
					_genetic_analysis[0].clone(),
				)
				.unwrap();

			assert_eq!(
				_genetic_analysis_info.get_genetic_analysis_tracking_id(),
				&_genetic_analysis[0]
			);
			assert!(_genetic_analysis_info.process_success());

			assert_ok!(GeneticAnalysts::unstake_genetic_analyst(Origin::signed(1),));
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
			GeneticAnalystVerifierKey::<Test>::put(2);

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

			assert_ok!(GeneticAnalysts::unstake_genetic_analyst(Origin::signed(1),));

			assert_eq!(
				GeneticAnalysts::genetic_analyst_by_account_id(1),
				Some(GeneticAnalyst {
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					account_id: 1,
					services: Vec::new(),
					qualifications: Vec::new(),
					stake_amount: 50000000000000000000000u128.saturated_into(),
					stake_status: StakeStatus::WaitingForUnstaked,
					verification_status: VerificationStatus::default(),
					availability_status: AvailabilityStatus::Unavailable,
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
						profile_image: Some(
							"DeBio Genetic Analyst profile_image".as_bytes().to_vec()
						),
					}
				})
			);

			assert_ok!(GeneticAnalysts::retrieve_unstake_amount(Origin::signed(2), 1,));

			assert_eq!(
				GeneticAnalysts::genetic_analyst_by_account_id(1),
				Some(GeneticAnalyst {
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 0u64.saturated_into(),
					account_id: 1,
					services: Vec::new(),
					qualifications: Vec::new(),
					stake_amount: 0u128.saturated_into(),
					stake_status: StakeStatus::Unstaked,
					verification_status: VerificationStatus::default(),
					availability_status: AvailabilityStatus::Unavailable,
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
						profile_image: Some(
							"DeBio Genetic Analyst profile_image".as_bytes().to_vec()
						),
					}
				})
			);
		})
	}

	#[test]
	fn cant_retrieve_unstake_amount_when_not_exist() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			GeneticAnalystVerifierKey::<Test>::put(2);

			assert_noop!(
				GeneticAnalysts::retrieve_unstake_amount(Origin::signed(2), 1,),
				Error::<Test>::GeneticAnalystDoesNotExist
			);
		})
	}

	#[test]
	fn cant_retrieve_unstake_amount_when_not_waiting_for_unstake() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			GeneticAnalystVerifierKey::<Test>::put(2);

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
				GeneticAnalysts::retrieve_unstake_amount(Origin::signed(2), 1,),
				Error::<Test>::GeneticAnalystIsNotWaitingForUnstake
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
			GeneticAnalystVerifierKey::<Test>::put(2);

			assert_ok!(GeneticAnalysts::update_unstake_time(
				Origin::signed(2),
				100000u64.saturated_into(),
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

			assert_ok!(GeneticAnalysts::unstake_genetic_analyst(Origin::signed(1),));

			assert_eq!(
				GeneticAnalysts::genetic_analyst_by_account_id(1),
				Some(GeneticAnalyst {
					unstake_at: 0u64.saturated_into(),
					retrieve_unstake_at: 100000u64.saturated_into(),
					account_id: 1,
					services: Vec::new(),
					qualifications: Vec::new(),
					stake_amount: 50000000000000000000000u128.saturated_into(),
					stake_status: StakeStatus::WaitingForUnstaked,
					verification_status: VerificationStatus::default(),
					availability_status: AvailabilityStatus::Unavailable,
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
						profile_image: Some(
							"DeBio Genetic Analyst profile_image".as_bytes().to_vec()
						),
					}
				})
			);

			assert_noop!(
				GeneticAnalysts::retrieve_unstake_amount(Origin::signed(2), 1,),
				Error::<Test>::GeneticAnalystCannotUnstakeBeforeUnstakeTime
			);
		})
	}

	#[test]
	fn cant_unstake_genetic_analyst_when_insufficient_pallet_funds() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			PalletAccount::<Test>::put(0);
			GeneticAnalystVerifierKey::<Test>::put(2);

			assert_ok!(Balances::set_balance(
				RawOrigin::Root.into(),
				1,
				70000000000000000000000u128.saturated_into(),
				0
			));

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

			assert_ok!(GeneticAnalysts::unstake_genetic_analyst(Origin::signed(1),));

			assert_ok!(Balances::set_balance(
				RawOrigin::Root.into(),
				PalletAccount::<Test>::get().unwrap(),
				0u128.saturated_into(),
				0
			));

			assert_noop!(
				GeneticAnalysts::retrieve_unstake_amount(Origin::signed(2), 1,),
				Error::<Test>::InsufficientPalletFunds
			);
		})
	}
}
