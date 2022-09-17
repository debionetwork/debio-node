use crate::{mock::*, Error};
use frame_support::{
	assert_noop, assert_ok,
	sp_runtime::traits::{Hash, Keccak256},
};
use primitives_menstrual_cycle_log::MenstrualCycleLog;

#[test]
fn add_menstrual_calendar_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MenstrualCalendar::add_menstrual_calendar(
			Origin::signed(1),
			16,
			vec![MenstrualCycleLog::default()]
		));

		let menstrual_calendar_ids = MenstrualCalendar::menstrual_calendar_by_address_id(1).unwrap();

		assert_eq!(MenstrualCalendar::menstrual_calendar_count(), Some(1));

		assert_eq!(MenstrualCalendar::menstrual_calendar_count_by_owner(1), Some(1));

		let menstrual_calendar =
			MenstrualCalendar::menstrual_calendar_by_id(menstrual_calendar_ids[0]).unwrap();

		assert_eq!(menstrual_calendar.id, menstrual_calendar_ids[0]);
		assert_eq!(menstrual_calendar.address_id, 1);
		assert_eq!(menstrual_calendar.average_cycle, 16);
		assert_eq!(menstrual_calendar.cycle_log, vec![MenstrualCycleLog::default()]);
	})
}

#[test]
fn remove_menstrual_calendar_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MenstrualCalendar::add_menstrual_calendar(
			Origin::signed(1),
			16,
			vec![MenstrualCycleLog::default()]
		));

		let menstrual_calendar_ids = MenstrualCalendar::menstrual_calendar_by_address_id(1).unwrap();

		assert_eq!(MenstrualCalendar::menstrual_calendar_count(), Some(1));

		assert_eq!(MenstrualCalendar::menstrual_calendar_count_by_owner(1), Some(1));

		let menstrual_calendar =
			MenstrualCalendar::menstrual_calendar_by_id(menstrual_calendar_ids[0]).unwrap();

		assert_eq!(menstrual_calendar.id, menstrual_calendar_ids[0]);
		assert_eq!(menstrual_calendar.address_id, 1);
		assert_eq!(menstrual_calendar.average_cycle, 16);
		assert_eq!(menstrual_calendar.cycle_log, vec![MenstrualCycleLog::default()]);

		assert_ok!(MenstrualCalendar::remove_menstrual_calendar(
			Origin::signed(1),
			menstrual_calendar_ids[0]
		));

		assert_eq!(MenstrualCalendar::menstrual_calendar_count(), Some(0));

		assert_eq!(MenstrualCalendar::menstrual_calendar_count_by_owner(1), Some(0));
	})
}

#[test]
fn remove_menstrual_calendar_does_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			MenstrualCalendar::remove_menstrual_calendar(
				Origin::signed(1),
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes())
			),
			Error::<Test>::MenstrualCalendarDoesNotExist
		);
	})
}

#[test]
fn remove_menstrual_calendar_not_menstrual_calendar_owner() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MenstrualCalendar::add_menstrual_calendar(
			Origin::signed(1),
			16,
			vec![MenstrualCycleLog::default()]
		));

		let menstrual_calendar_ids = MenstrualCalendar::menstrual_calendar_by_address_id(1).unwrap();

		assert_eq!(MenstrualCalendar::menstrual_calendar_count(), Some(1));

		assert_eq!(MenstrualCalendar::menstrual_calendar_count_by_owner(1), Some(1));

		let menstrual_calendar =
			MenstrualCalendar::menstrual_calendar_by_id(menstrual_calendar_ids[0]).unwrap();

		assert_eq!(menstrual_calendar.id, menstrual_calendar_ids[0]);
		assert_eq!(menstrual_calendar.address_id, 1);
		assert_eq!(menstrual_calendar.average_cycle, 16);
		assert_eq!(menstrual_calendar.cycle_log, vec![MenstrualCycleLog::default()]);

		assert_noop!(
			MenstrualCalendar::remove_menstrual_calendar(
				Origin::signed(2),
				menstrual_calendar_ids[0]
			),
			Error::<Test>::NotMenstrualCalendarOwner
		);
	})
}

#[test]
fn update_menstrual_calendar_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MenstrualCalendar::add_menstrual_calendar(
			Origin::signed(1),
			16,
			vec![MenstrualCycleLog::default()]
		));

		let menstrual_calendar_ids = MenstrualCalendar::menstrual_calendar_by_address_id(1).unwrap();

		assert_eq!(MenstrualCalendar::menstrual_calendar_count(), Some(1));

		assert_eq!(MenstrualCalendar::menstrual_calendar_count_by_owner(1), Some(1));

		let menstrual_calendar =
			MenstrualCalendar::menstrual_calendar_by_id(menstrual_calendar_ids[0]).unwrap();

		assert_eq!(menstrual_calendar.id, menstrual_calendar_ids[0]);
		assert_eq!(menstrual_calendar.address_id, 1);
		assert_eq!(menstrual_calendar.average_cycle, 16);
		assert_eq!(menstrual_calendar.cycle_log, vec![MenstrualCycleLog::default()]);

		assert_ok!(MenstrualCalendar::update_menstrual_calendar(
			Origin::signed(1),
			menstrual_calendar_ids[0],
			16,
			vec![MenstrualCycleLog::default()],
		));

		let menstrual_calendar_ids = MenstrualCalendar::menstrual_calendar_by_address_id(1).unwrap();

		assert_eq!(MenstrualCalendar::menstrual_calendar_count(), Some(1));

		assert_eq!(MenstrualCalendar::menstrual_calendar_count_by_owner(1), Some(1));

		let menstrual_calendar =
			MenstrualCalendar::menstrual_calendar_by_id(menstrual_calendar_ids[0]).unwrap();

		assert_eq!(menstrual_calendar.id, menstrual_calendar_ids[0]);
		assert_eq!(menstrual_calendar.address_id, 1);
		assert_eq!(menstrual_calendar.average_cycle, 16);
		assert_eq!(
			menstrual_calendar.cycle_log,
			vec![MenstrualCycleLog::default()]
		);
	})
}

#[test]
fn update_menstrual_calendar_does_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			MenstrualCalendar::update_menstrual_calendar(
				Origin::signed(1),
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
				16,
				vec![MenstrualCycleLog::default()],
			),
			Error::<Test>::MenstrualCalendarDoesNotExist
		);
	})
}

#[test]
fn update_menstrual_calendar_not_menstrual_calendar_owner() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MenstrualCalendar::add_menstrual_calendar(
			Origin::signed(1),
			16,
			vec![MenstrualCycleLog::default()]
		));

		let menstrual_calendar_ids = MenstrualCalendar::menstrual_calendar_by_address_id(1).unwrap();

		assert_eq!(MenstrualCalendar::menstrual_calendar_count(), Some(1));

		assert_eq!(MenstrualCalendar::menstrual_calendar_count_by_owner(1), Some(1));

		let menstrual_calendar =
			MenstrualCalendar::menstrual_calendar_by_id(menstrual_calendar_ids[0]).unwrap();

		assert_eq!(menstrual_calendar.id, menstrual_calendar_ids[0]);
		assert_eq!(menstrual_calendar.address_id, 1);
		assert_eq!(menstrual_calendar.average_cycle, 16);
		assert_eq!(menstrual_calendar.cycle_log, vec![MenstrualCycleLog::default()]);

		assert_noop!(
			MenstrualCalendar::update_menstrual_calendar(
				Origin::signed(2),
				menstrual_calendar_ids[0],
				16,
				vec![MenstrualCycleLog::default()],
			),
			Error::<Test>::NotMenstrualCalendarOwner
		);
	})
}
