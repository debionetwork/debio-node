use crate::{mock::*, GeneticAnalyst, GeneticAnalystInfo, StakeStatus, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn register_genetic_analyst_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(1),
			GeneticAnalystInfo {
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				stake_amount: 100,
				stake_status: StakeStatus::default(),
			}
		));

		assert_eq!(
			GeneticAnalysts::genetic_analyst_by_account_id(1),
			Some(GeneticAnalyst {
				account_id: 1,
				qualifications: Vec::new(),
				info: GeneticAnalystInfo {
					first_name: "First Name".as_bytes().to_vec(),
					last_name: "Last Name".as_bytes().to_vec(),
					gender: "Gender".as_bytes().to_vec(),
					date_of_birth: 0,
					email: "Email".as_bytes().to_vec(),
					phone_number: "+6893026516".as_bytes().to_vec(),
					specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
					stake_amount: 100,
					stake_status: StakeStatus::default(),
				}
			})
		);

		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(2),
			GeneticAnalystInfo {
				first_name: "First Name 2".as_bytes().to_vec(),
				last_name: "Last Name 2".as_bytes().to_vec(),
				gender: "Gender 2".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email 2".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst 2".as_bytes().to_vec(),
				stake_amount: 100,
				stake_status: StakeStatus::default(),
			}
		));

		assert_eq!(GeneticAnalysts::genetic_analyst_count(), Some(2),);
	})
}

#[test]
fn update_genetic_analyst_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(1),
			GeneticAnalystInfo {
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				stake_amount: 100,
				stake_status: StakeStatus::default(),
			}
		));

		assert_eq!(
			GeneticAnalysts::genetic_analyst_by_account_id(1),
			Some(GeneticAnalyst {
				account_id: 1,
				qualifications: Vec::new(),
				info: GeneticAnalystInfo {
					first_name: "First Name".as_bytes().to_vec(),
					last_name: "Last Name".as_bytes().to_vec(),
					gender: "Gender".as_bytes().to_vec(),
					date_of_birth: 0,
					email: "Email".as_bytes().to_vec(),
					phone_number: "+6893026516".as_bytes().to_vec(),
					specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
					stake_amount: 100,
					stake_status: StakeStatus::default(),
				}
			})
		);

		assert_ok!(GeneticAnalysts::update_genetic_analyst(
			Origin::signed(1),
			GeneticAnalystInfo {
				first_name: "First Name 2".as_bytes().to_vec(),
				last_name: "Last Name 2".as_bytes().to_vec(),
				gender: "Gender 2".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email 2".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst 2".as_bytes().to_vec(),
				stake_amount: 100,
				stake_status: StakeStatus::default(),
			}
		));

		assert_eq!(
			GeneticAnalysts::genetic_analyst_by_account_id(1),
			Some(GeneticAnalyst {
				account_id: 1,
				qualifications: Vec::new(),
				info: GeneticAnalystInfo {
					first_name: "First Name 2".as_bytes().to_vec(),
					last_name: "Last Name 2".as_bytes().to_vec(),
					gender: "Gender 2".as_bytes().to_vec(),
					date_of_birth: 0,
					email: "Email 2".as_bytes().to_vec(),
					phone_number: "+6893026516".as_bytes().to_vec(),
					specialization: "DeBio Genetic Analyst 2".as_bytes().to_vec(),
					stake_amount: 100,
					stake_status: StakeStatus::default(),
				}
			})
		);
	})
}

#[test]
fn deregister_genetic_analyst_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(1),
			GeneticAnalystInfo {
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				stake_amount: 100,
				stake_status: StakeStatus::default(),
			}
		));

		assert_ok!(GeneticAnalysts::deregister_genetic_analyst(Origin::signed(1),));

		assert_eq!(GeneticAnalysts::genetic_analyst_by_account_id(1), None);

		assert_eq!(GeneticAnalysts::genetic_analyst_count(), Some(0),);
	})
}

#[test]
fn cant_register_genetic_analyst_when_already_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(1),
			GeneticAnalystInfo {
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				stake_amount: 100,
				stake_status: StakeStatus::default(),
			}
		));

		assert_noop!(
			GeneticAnalysts::register_genetic_analyst(
				Origin::signed(1),
				GeneticAnalystInfo {
					first_name: "First Name".as_bytes().to_vec(),
					last_name: "Last Name".as_bytes().to_vec(),
					gender: "Gender".as_bytes().to_vec(),
					date_of_birth: 0,
					email: "Email".as_bytes().to_vec(),
					phone_number: "+6893026516".as_bytes().to_vec(),
					specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
					stake_amount: 100,
					stake_status: StakeStatus::default(),
				}
			),
			Error::<Test>::GeneticAnalystAlreadyRegistered
		);
	})
}

#[test]
fn cant_update_and_deregister_genetic_analyst_when_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			GeneticAnalysts::update_genetic_analyst(
				Origin::signed(1),
				GeneticAnalystInfo {
					first_name: "First Name".as_bytes().to_vec(),
					last_name: "Last Name".as_bytes().to_vec(),
					gender: "Gender".as_bytes().to_vec(),
					date_of_birth: 0,
					email: "Email".as_bytes().to_vec(),
					phone_number: "+6893026516".as_bytes().to_vec(),
					specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
					stake_amount: 100,
					stake_status: StakeStatus::default(),
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
	ExternalityBuilder::build().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(GeneticAnalysts::register_genetic_analyst(
			Origin::signed(1),
			GeneticAnalystInfo {
				first_name: "First Name".as_bytes().to_vec(),
				last_name: "Last Name".as_bytes().to_vec(),
				gender: "Gender".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
				stake_amount: 100,
				stake_status: StakeStatus::default(),
			}
		));

		System::assert_last_event(Event::GeneticAnalysts(crate::Event::GeneticAnalystRegistered(
			GeneticAnalyst {
				account_id: 1,
				qualifications: Vec::new(),
				info: GeneticAnalystInfo {
					first_name: "First Name".as_bytes().to_vec(),
					last_name: "Last Name".as_bytes().to_vec(),
					gender: "Gender".as_bytes().to_vec(),
					date_of_birth: 0,
					email: "Email".as_bytes().to_vec(),
					phone_number: "+6893026516".as_bytes().to_vec(),
					specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
					stake_amount: 100,
					stake_status: StakeStatus::default(),
				},
			},
			1,
		)));

		assert_ok!(GeneticAnalysts::update_genetic_analyst(
			Origin::signed(1),
			GeneticAnalystInfo {
				first_name: "First Name 2".as_bytes().to_vec(),
				last_name: "Last Name 2".as_bytes().to_vec(),
				gender: "Gender 2".as_bytes().to_vec(),
				date_of_birth: 0,
				email: "Email 2".as_bytes().to_vec(),
				phone_number: "+6893026516".as_bytes().to_vec(),
				specialization: "DeBio Genetic Analyst 2".as_bytes().to_vec(),
				stake_amount: 100,
				stake_status: StakeStatus::default(),
			}
		));

		System::assert_last_event(Event::GeneticAnalysts(crate::Event::GeneticAnalystUpdated(
			GeneticAnalyst {
				account_id: 1,
				qualifications: Vec::new(),
				info: GeneticAnalystInfo {
					first_name: "First Name 2".as_bytes().to_vec(),
					last_name: "Last Name 2".as_bytes().to_vec(),
					gender: "Gender 2".as_bytes().to_vec(),
					date_of_birth: 0,
					email: "Email 2".as_bytes().to_vec(),
					phone_number: "+6893026516".as_bytes().to_vec(),
					specialization: "DeBio Genetic Analyst 2".as_bytes().to_vec(),
					stake_amount: 100,
					stake_status: StakeStatus::default(),
				},
			},
			1,
		)));

		assert_ok!(GeneticAnalysts::deregister_genetic_analyst(Origin::signed(1)));

		System::assert_last_event(Event::GeneticAnalysts(crate::Event::GeneticAnalystDeleted(
			GeneticAnalyst {
				account_id: 1,
				qualifications: Vec::new(),
				info: GeneticAnalystInfo {
					first_name: "First Name 2".as_bytes().to_vec(),
					last_name: "Last Name 2".as_bytes().to_vec(),
					gender: "Gender 2".as_bytes().to_vec(),
					date_of_birth: 0,
					email: "Email 2".as_bytes().to_vec(),
					phone_number: "+6893026516".as_bytes().to_vec(),
					specialization: "DeBio Genetic Analyst 2".as_bytes().to_vec(),
					stake_amount: 100,
					stake_status: StakeStatus::default(),
				},
			},
			1,
		)));
	})
}
