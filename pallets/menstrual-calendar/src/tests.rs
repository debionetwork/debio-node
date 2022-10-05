use crate::{mock::*, Error, MenstrualCycleLog};
use frame_support::{
	assert_noop, assert_ok,
	sp_runtime::traits::{Hash, Keccak256},
};

#[test]
fn add_menstrual_calendar_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MenstrualCalendar::add_menstrual_calendar(Origin::signed(1), 16));

		let menstrual_calendar_ids =
			MenstrualCalendar::menstrual_calendar_by_address_id(1).unwrap();

		assert_eq!(MenstrualCalendar::menstrual_calendar_count(), Some(1));

		assert_eq!(MenstrualCalendar::menstrual_calendar_count_by_owner(1), Some(1));

		let menstrual_calendar =
			MenstrualCalendar::menstrual_calendar_by_id(menstrual_calendar_ids[0]).unwrap();

		assert_eq!(menstrual_calendar.id, menstrual_calendar_ids[0]);
		assert_eq!(menstrual_calendar.address_id, 1);
		assert_eq!(menstrual_calendar.average_cycle, 16);
		assert_eq!(menstrual_calendar.cycle_log, vec![]);
	})
}

#[test]
fn update_menstrual_calendar_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MenstrualCalendar::add_menstrual_calendar(Origin::signed(1), 16));

		let menstrual_calendar_ids =
			MenstrualCalendar::menstrual_calendar_by_address_id(1).unwrap();

		assert_eq!(MenstrualCalendar::menstrual_calendar_count(), Some(1));

		assert_eq!(MenstrualCalendar::menstrual_calendar_count_by_owner(1), Some(1));

		let menstrual_calendar =
			MenstrualCalendar::menstrual_calendar_by_id(menstrual_calendar_ids[0]).unwrap();

		assert_eq!(menstrual_calendar.id, menstrual_calendar_ids[0]);
		assert_eq!(menstrual_calendar.address_id, 1);
		assert_eq!(menstrual_calendar.average_cycle, 16);
		assert_eq!(menstrual_calendar.cycle_log, vec![]);

		assert_ok!(MenstrualCalendar::update_menstrual_calendar(
			Origin::signed(1),
			menstrual_calendar_ids[0],
			16
		));

		let menstrual_calendar_ids =
			MenstrualCalendar::menstrual_calendar_by_address_id(1).unwrap();

		assert_eq!(MenstrualCalendar::menstrual_calendar_count(), Some(1));

		assert_eq!(MenstrualCalendar::menstrual_calendar_count_by_owner(1), Some(1));

		let menstrual_calendar =
			MenstrualCalendar::menstrual_calendar_by_id(menstrual_calendar_ids[0]).unwrap();

		assert_eq!(menstrual_calendar.id, menstrual_calendar_ids[0]);
		assert_eq!(menstrual_calendar.address_id, 1);
		assert_eq!(menstrual_calendar.average_cycle, 16);
		assert_eq!(menstrual_calendar.cycle_log, vec![]);
	})
}

#[test]
fn update_menstrual_calendar_does_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			MenstrualCalendar::update_menstrual_calendar(
				Origin::signed(1),
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
				16
			),
			Error::<Test>::MenstrualCalendarDoesNotExist
		);
	})
}

#[test]
fn update_menstrual_calendar_not_menstrual_calendar_owner() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MenstrualCalendar::add_menstrual_calendar(Origin::signed(1), 16));

		let menstrual_calendar_ids =
			MenstrualCalendar::menstrual_calendar_by_address_id(1).unwrap();

		assert_eq!(MenstrualCalendar::menstrual_calendar_count(), Some(1));

		assert_eq!(MenstrualCalendar::menstrual_calendar_count_by_owner(1), Some(1));

		let menstrual_calendar =
			MenstrualCalendar::menstrual_calendar_by_id(menstrual_calendar_ids[0]).unwrap();

		assert_eq!(menstrual_calendar.id, menstrual_calendar_ids[0]);
		assert_eq!(menstrual_calendar.address_id, 1);
		assert_eq!(menstrual_calendar.average_cycle, 16);
		assert_eq!(menstrual_calendar.cycle_log, vec![]);

		assert_noop!(
			MenstrualCalendar::update_menstrual_calendar(
				Origin::signed(2),
				menstrual_calendar_ids[0],
				16
			),
			Error::<Test>::NotMenstrualCalendarOwner
		);
	})
}

#[test]
fn add_menstrual_cycle_log_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MenstrualCalendar::add_menstrual_cycle_log(
			Origin::signed(1),
			Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			MenstrualCycleLog::default()
		));

		let menstrual_cycle_log_ids = MenstrualCalendar::menstrual_cycle_log_by_owner_id(
			Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
		)
		.unwrap();

		assert_eq!(MenstrualCalendar::menstrual_cycle_log_count(), Some(1));

		let menstrual_cycle_log =
			MenstrualCalendar::menstrual_cycle_log_by_id(menstrual_cycle_log_ids[0]).unwrap();

		assert!(!menstrual_cycle_log.menstruation);
		assert_eq!(menstrual_cycle_log.date, 0);
		assert_eq!(menstrual_cycle_log.symptoms, vec![]);
		assert_eq!(menstrual_cycle_log.created_at, 0);
		assert_eq!(menstrual_cycle_log.updated_at, 0);
	})
}

#[test]
fn remove_menstrual_cycle_log_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MenstrualCalendar::add_menstrual_cycle_log(
			Origin::signed(1),
			Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			MenstrualCycleLog::default()
		));

		let menstrual_cycle_log_ids = MenstrualCalendar::menstrual_cycle_log_by_owner_id(
			Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
		)
		.unwrap();

		assert_eq!(MenstrualCalendar::menstrual_cycle_log_count(), Some(1));

		let menstrual_cycle_log =
			MenstrualCalendar::menstrual_cycle_log_by_id(menstrual_cycle_log_ids[0]).unwrap();

		assert!(!menstrual_cycle_log.menstruation);
		assert_eq!(menstrual_cycle_log.date, 0);
		assert_eq!(menstrual_cycle_log.symptoms, vec![]);
		assert_eq!(menstrual_cycle_log.created_at, 0);
		assert_eq!(menstrual_cycle_log.updated_at, 0);

		assert_ok!(MenstrualCalendar::remove_menstrual_cycle_log(
			Origin::signed(1),
			menstrual_cycle_log.menstrual_calendar_id,
			menstrual_cycle_log_ids[0]
		));

		assert_eq!(MenstrualCalendar::menstrual_cycle_log_count(), Some(0));
	})
}

#[test]
fn remove_menstrual_cycle_log_does_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			MenstrualCalendar::remove_menstrual_cycle_log(
				Origin::signed(1),
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes())
			),
			Error::<Test>::MenstrualCycleLogDoesNotExist
		);
	})
}

#[test]
fn remove_menstrual_cycle_log_not_menstrual_cycle_log_owner() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MenstrualCalendar::add_menstrual_cycle_log(
			Origin::signed(1),
			Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			MenstrualCycleLog::default()
		));

		let menstrual_cycle_log_ids = MenstrualCalendar::menstrual_cycle_log_by_owner_id(
			Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
		)
		.unwrap();

		assert_eq!(MenstrualCalendar::menstrual_cycle_log_count(), Some(1));

		let menstrual_cycle_log =
			MenstrualCalendar::menstrual_cycle_log_by_id(menstrual_cycle_log_ids[0]).unwrap();

		assert!(!menstrual_cycle_log.menstruation);
		assert_eq!(menstrual_cycle_log.date, 0);
		assert_eq!(menstrual_cycle_log.symptoms, vec![]);
		assert_eq!(menstrual_cycle_log.created_at, 0);
		assert_eq!(menstrual_cycle_log.updated_at, 0);

		assert_noop!(
			MenstrualCalendar::remove_menstrual_cycle_log(
				Origin::signed(2),
				Keccak256::hash("0xi2Gh68Fg23ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
				menstrual_cycle_log_ids[0]
			),
			Error::<Test>::NotMenstrualCycleLogOwner
		);
	})
}

#[test]
fn update_menstrual_cycle_log_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MenstrualCalendar::add_menstrual_cycle_log(
			Origin::signed(1),
			Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			MenstrualCycleLog::default()
		));

		let menstrual_cycle_log_ids = MenstrualCalendar::menstrual_cycle_log_by_owner_id(
			Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
		)
		.unwrap();

		assert_eq!(MenstrualCalendar::menstrual_cycle_log_count(), Some(1));

		let menstrual_cycle_log =
			MenstrualCalendar::menstrual_cycle_log_by_id(menstrual_cycle_log_ids[0]).unwrap();

		assert!(!menstrual_cycle_log.menstruation);
		assert_eq!(menstrual_cycle_log.date, 0);
		assert_eq!(menstrual_cycle_log.symptoms, vec![]);
		assert_eq!(menstrual_cycle_log.created_at, 0);
		assert_eq!(menstrual_cycle_log.updated_at, 0);

		assert_ok!(MenstrualCalendar::update_menstrual_cycle_log(
			Origin::signed(1),
			menstrual_cycle_log.menstrual_calendar_id,
			menstrual_cycle_log_ids[0],
			MenstrualCycleLog::default()
		));

		let menstrual_cycle_log_ids = MenstrualCalendar::menstrual_cycle_log_by_owner_id(
			Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
		)
		.unwrap();

		assert_eq!(MenstrualCalendar::menstrual_cycle_log_count(), Some(1));

		let menstrual_cycle_log =
			MenstrualCalendar::menstrual_cycle_log_by_id(menstrual_cycle_log_ids[0]).unwrap();

		assert!(!menstrual_cycle_log.menstruation);
		assert_eq!(menstrual_cycle_log.date, 0);
		assert_eq!(menstrual_cycle_log.symptoms, vec![]);
		assert_eq!(menstrual_cycle_log.created_at, 0);
		assert_eq!(menstrual_cycle_log.updated_at, 0);
	})
}

#[test]
fn update_menstrual_cycle_log_does_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			MenstrualCalendar::update_menstrual_cycle_log(
				Origin::signed(1),
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
				MenstrualCycleLog::default()
			),
			Error::<Test>::MenstrualCycleLogDoesNotExist
		);
	})
}

#[test]
fn update_menstrual_cycle_log_not_menstrual_cycle_log_owner() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MenstrualCalendar::add_menstrual_cycle_log(
			Origin::signed(1),
			Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			MenstrualCycleLog::default()
		));

		let menstrual_cycle_log_ids = MenstrualCalendar::menstrual_cycle_log_by_owner_id(
			Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
		)
		.unwrap();

		assert_eq!(MenstrualCalendar::menstrual_cycle_log_count(), Some(1));

		let menstrual_cycle_log =
			MenstrualCalendar::menstrual_cycle_log_by_id(menstrual_cycle_log_ids[0]).unwrap();

		assert!(!menstrual_cycle_log.menstruation);
		assert_eq!(menstrual_cycle_log.date, 0);
		assert_eq!(menstrual_cycle_log.symptoms, vec![]);
		assert_eq!(menstrual_cycle_log.created_at, 0);
		assert_eq!(menstrual_cycle_log.updated_at, 0);

		assert_noop!(
			MenstrualCalendar::update_menstrual_cycle_log(
				Origin::signed(2),
				Keccak256::hash("0xi2Gh68Fg23ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
				menstrual_cycle_log_ids[0],
				MenstrualCycleLog::default()
			),
			Error::<Test>::NotMenstrualCycleLogOwner
		);
	})
}
