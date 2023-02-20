use crate::{
	mock::*, Error, MenstrualCalendar as MenstrualCalendarS, MenstrualCycleLog, MenstrualInfo,
	Symptom,
};
use frame_support::{
	assert_noop, assert_ok,
	sp_runtime::traits::{Hash, Keccak256},
};

#[test]
fn add_menstrual_calendar_works() {
	ExternalityBuilder::build().execute_with(|| {
		let customer = 1;

		assert_ok!(MenstrualCalendar::add_menstrual_calendar(RuntimeOrigin::signed(customer), 16));

		let ids = MenstrualCalendar::menstrual_calendar_by_owner(customer).unwrap();

		assert_eq!(MenstrualCalendar::menstrual_calendar_count(), Some(1));
		assert_eq!(MenstrualCalendar::menstrual_calendar_count_by_owner(customer), Some(1));
		assert_eq!(
			MenstrualCalendar::menstrual_calendar_by_id(ids[0]),
			Some(MenstrualCalendarS::new(ids[0], customer, 16, 0))
		);
	})
}

#[test]
fn update_menstrual_calendar_works() {
	ExternalityBuilder::build().execute_with(|| {
		let customer = 1;

		assert_ok!(MenstrualCalendar::add_menstrual_calendar(RuntimeOrigin::signed(customer), 16));

		let ids = MenstrualCalendar::menstrual_calendar_by_owner(customer).unwrap();

		assert_ok!(MenstrualCalendar::update_menstrual_calendar(
			RuntimeOrigin::signed(customer),
			ids[0],
			20
		));

		assert_eq!(
			MenstrualCalendar::menstrual_calendar_by_id(ids[0]),
			Some(MenstrualCalendarS::new(ids[0], customer, 20, 0))
		);
	})
}

#[test]
fn add_menstrual_cycle_log_works() {
	ExternalityBuilder::build().execute_with(|| {
		let customer = 1;

		assert_ok!(MenstrualCalendar::add_menstrual_calendar(RuntimeOrigin::signed(customer), 16));

		let menstrual_ids = MenstrualCalendar::menstrual_calendar_by_owner(customer).unwrap();
		let menstrual_info =
			MenstrualInfo { date: 0, symptoms: vec![Symptom::from(b"pain")], menstruation: true };

		assert_ok!(MenstrualCalendar::add_menstrual_cycle_log(
			RuntimeOrigin::signed(customer),
			menstrual_ids[0],
			vec![menstrual_info],
		));

		let cycle_log_ids =
			MenstrualCalendar::menstrual_cycle_log_by_owner_id(menstrual_ids[0]).unwrap();

		assert_eq!(MenstrualCalendar::menstrual_cycle_log_count(), Some(1));
		assert_eq!(
			MenstrualCalendar::menstrual_cycle_log_count_by_owner(menstrual_ids[0]),
			Some(1)
		);
		assert_eq!(
			MenstrualCalendar::menstrual_cycle_log_by_id(cycle_log_ids[0]),
			Some(MenstrualCycleLog::new(
				cycle_log_ids[0],
				menstrual_ids[0],
				0,
				true,
				vec![Symptom::from(b"pain")],
				0,
			)),
		);
	})
}

#[test]
fn update_menstrual_cycle_log_works() {
	ExternalityBuilder::build().execute_with(|| {
		let customer = 1;

		assert_ok!(MenstrualCalendar::add_menstrual_calendar(RuntimeOrigin::signed(customer), 16));

		let menstrual_ids = MenstrualCalendar::menstrual_calendar_by_owner(customer).unwrap();
		let menstrual_info =
			MenstrualInfo { date: 0, symptoms: vec![Symptom::from(b"pain")], menstruation: true };

		assert_ok!(MenstrualCalendar::add_menstrual_cycle_log(
			RuntimeOrigin::signed(customer),
			menstrual_ids[0],
			vec![menstrual_info],
		));

		let cycle_log_ids =
			MenstrualCalendar::menstrual_cycle_log_by_owner_id(menstrual_ids[0]).unwrap();
		let menstrual_cycle_log = MenstrualCycleLog::new(
			cycle_log_ids[0],
			menstrual_ids[0],
			1,
			false,
			vec![Symptom::from(b"headache")],
			0,
		);
		assert_ok!(MenstrualCalendar::update_menstrual_cycle_log(
			RuntimeOrigin::signed(customer),
			vec![menstrual_cycle_log],
		));

		assert_eq!(
			MenstrualCalendar::menstrual_cycle_log_by_id(cycle_log_ids[0]),
			Some(MenstrualCycleLog::new(
				cycle_log_ids[0],
				menstrual_ids[0],
				1,
				false,
				vec![Symptom::from(b"headache")],
				0,
			)),
		);
	})
}

#[test]
fn remove_menstrual_cycle_log_works() {
	ExternalityBuilder::build().execute_with(|| {
		let customer = 1;

		assert_ok!(MenstrualCalendar::add_menstrual_calendar(RuntimeOrigin::signed(customer), 16));

		let menstrual_ids = MenstrualCalendar::menstrual_calendar_by_owner(customer).unwrap();
		let menstrual_info =
			MenstrualInfo { date: 0, symptoms: vec![Symptom::from(b"pain")], menstruation: true };

		assert_ok!(MenstrualCalendar::add_menstrual_cycle_log(
			RuntimeOrigin::signed(customer),
			menstrual_ids[0],
			vec![menstrual_info],
		));

		let cycle_log_ids =
			MenstrualCalendar::menstrual_cycle_log_by_owner_id(menstrual_ids[0]).unwrap();

		assert_ok!(MenstrualCalendar::remove_menstrual_cycle_log(
			RuntimeOrigin::signed(customer),
			menstrual_ids[0],
			cycle_log_ids[0],
		));

		assert_eq!(MenstrualCalendar::menstrual_cycle_log_by_id(cycle_log_ids[0]), None);
		assert_eq!(
			MenstrualCalendar::menstrual_cycle_log_by_owner_id(menstrual_ids[0]),
			Some(Vec::new())
		);
		assert_eq!(MenstrualCalendar::menstrual_cycle_log_count(), Some(0));
		assert_eq!(
			MenstrualCalendar::menstrual_cycle_log_count_by_owner(menstrual_ids[0]),
			Some(0)
		);
	})
}

#[test]
fn cant_add_menstrual_calendar_when_already_exist() {
	ExternalityBuilder::build().execute_with(|| {
		let customer = 1;

		assert_ok!(MenstrualCalendar::add_menstrual_calendar(RuntimeOrigin::signed(customer), 16));

		assert_noop!(
			MenstrualCalendar::add_menstrual_calendar(RuntimeOrigin::signed(customer), 16),
			Error::<Test>::MenstrualCalendarAlreadyExist,
		);
	})
}

#[test]
fn cant_update_menstrual_calendar_when_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			MenstrualCalendar::update_menstrual_calendar(
				RuntimeOrigin::signed(1),
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
				16
			),
			Error::<Test>::MenstrualCalendarDoesNotExist
		);
	})
}

#[test]
fn cant_update_menstrual_calendar_when_not_owner() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MenstrualCalendar::add_menstrual_calendar(RuntimeOrigin::signed(1), 16));

		let menstrual_calendar_ids = MenstrualCalendar::menstrual_calendar_by_owner(1).unwrap();

		assert_noop!(
			MenstrualCalendar::update_menstrual_calendar(
				RuntimeOrigin::signed(2),
				menstrual_calendar_ids[0],
				16
			),
			Error::<Test>::NotMenstrualCalendarOwner
		);
	})
}

#[test]
fn cant_add_menstrual_cycle_log_when_menstrual_calendar_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		let menstrual_info =
			MenstrualInfo { date: 0, symptoms: vec![Symptom::from(b"pain")], menstruation: true };

		assert_noop!(
			MenstrualCalendar::add_menstrual_cycle_log(
				RuntimeOrigin::signed(1),
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
				vec![menstrual_info],
			),
			Error::<Test>::MenstrualCalendarDoesNotExist
		);
	})
}

#[test]
fn cant_add_menstrual_cycle_log_when_not_owner() {
	ExternalityBuilder::build().execute_with(|| {
		let customer = 1;
		let other_customer = 2;

		assert_ok!(MenstrualCalendar::add_menstrual_calendar(RuntimeOrigin::signed(customer), 16));

		let menstrual_ids = MenstrualCalendar::menstrual_calendar_by_owner(customer).unwrap();
		let menstrual_info =
			MenstrualInfo { date: 0, symptoms: vec![Symptom::from(b"pain")], menstruation: true };

		assert_noop!(
			MenstrualCalendar::add_menstrual_cycle_log(
				RuntimeOrigin::signed(other_customer),
				menstrual_ids[0],
				vec![menstrual_info],
			),
			Error::<Test>::NotMenstrualCalendarOwner
		);
	})
}

#[test]
fn cant_remove_menstrual_cycle_log_when_menstrual_calendar_not_exists() {
	ExternalityBuilder::build().execute_with(|| {
		let customer = 1;

		assert_noop!(
			MenstrualCalendar::remove_menstrual_cycle_log(
				RuntimeOrigin::signed(customer),
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			),
			Error::<Test>::MenstrualCalendarDoesNotExist
		);
	})
}

#[test]
fn cant_remove_menstrual_cycle_log_when_not_owner() {
	ExternalityBuilder::build().execute_with(|| {
		let customer = 1;
		let other_customer = 2;

		assert_ok!(MenstrualCalendar::add_menstrual_calendar(RuntimeOrigin::signed(customer), 16));

		let menstrual_ids = MenstrualCalendar::menstrual_calendar_by_owner(customer).unwrap();
		let menstrual_info =
			MenstrualInfo { date: 0, symptoms: vec![Symptom::from(b"pain")], menstruation: true };

		assert_ok!(MenstrualCalendar::add_menstrual_cycle_log(
			RuntimeOrigin::signed(customer),
			menstrual_ids[0],
			vec![menstrual_info],
		));

		let cycle_log_ids =
			MenstrualCalendar::menstrual_cycle_log_by_owner_id(menstrual_ids[0]).unwrap();

		assert_noop!(
			MenstrualCalendar::remove_menstrual_cycle_log(
				RuntimeOrigin::signed(other_customer),
				menstrual_ids[0],
				cycle_log_ids[0],
			),
			Error::<Test>::NotMenstrualCalendarOwner,
		);
	})
}

#[test]
fn cant_remove_menstrual_cycle_log_when_menstrual_cycle_log_not_exists() {
	ExternalityBuilder::build().execute_with(|| {
		let customer = 1;

		assert_ok!(MenstrualCalendar::add_menstrual_calendar(RuntimeOrigin::signed(customer), 16));

		let menstrual_ids = MenstrualCalendar::menstrual_calendar_by_owner(customer).unwrap();

		assert_noop!(
			MenstrualCalendar::remove_menstrual_cycle_log(
				RuntimeOrigin::signed(customer),
				menstrual_ids[0],
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			),
			Error::<Test>::MenstrualCycleLogDoesNotExist,
		);
	})
}

#[test]
fn call_event_should_works() {
	ExternalityBuilder::build().execute_with(|| {
		System::set_block_number(1);

		let customer = 1;

		assert_ok!(MenstrualCalendar::add_menstrual_calendar(RuntimeOrigin::signed(customer), 16));

		let menstrual_ids = MenstrualCalendar::menstrual_calendar_by_owner(customer).unwrap();

		System::assert_last_event(RuntimeEvent::MenstrualCalendar(
			crate::Event::MenstrualCalendarAdded(
				MenstrualCalendarS::new(menstrual_ids[0], customer, 16, 0),
				customer,
			),
		));

		assert_ok!(MenstrualCalendar::update_menstrual_calendar(
			RuntimeOrigin::signed(customer),
			menstrual_ids[0],
			20
		));

		System::assert_last_event(RuntimeEvent::MenstrualCalendar(
			crate::Event::MenstrualCalendarUpdated(
				MenstrualCalendarS::new(menstrual_ids[0], customer, 20, 0),
				customer,
			),
		));

		let menstrual_info =
			MenstrualInfo { date: 0, symptoms: vec![Symptom::from(b"pain")], menstruation: true };

		assert_ok!(MenstrualCalendar::add_menstrual_cycle_log(
			RuntimeOrigin::signed(customer),
			menstrual_ids[0],
			vec![menstrual_info],
		));

		let cycle_log_ids =
			MenstrualCalendar::menstrual_cycle_log_by_owner_id(menstrual_ids[0]).unwrap();

		System::assert_last_event(RuntimeEvent::MenstrualCalendar(
			crate::Event::MenstrualCycleLogsAdded(
				vec![MenstrualCycleLog::new(
					cycle_log_ids[0],
					menstrual_ids[0],
					0,
					true,
					vec![Symptom::from(b"pain")],
					0,
				)],
				customer,
			),
		));

		let menstrual_cycle_log = MenstrualCycleLog::new(
			cycle_log_ids[0],
			menstrual_ids[0],
			1,
			false,
			vec![Symptom::from(b"headache")],
			0,
		);

		assert_ok!(MenstrualCalendar::update_menstrual_cycle_log(
			RuntimeOrigin::signed(customer),
			vec![menstrual_cycle_log.clone()],
		));

		System::assert_last_event(RuntimeEvent::MenstrualCalendar(
			crate::Event::MenstrualCycleLogUpdated(vec![menstrual_cycle_log], customer),
		));

		assert_ok!(MenstrualCalendar::remove_menstrual_cycle_log(
			RuntimeOrigin::signed(customer),
			menstrual_ids[0],
			cycle_log_ids[0],
		));

		System::assert_last_event(RuntimeEvent::MenstrualCalendar(
			crate::Event::MenstrualCycleLogRemoved(cycle_log_ids[0], customer),
		));
	})
}
