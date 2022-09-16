use crate::{mock::*, Error};
use frame_support::{
	assert_noop, assert_ok,
	sp_runtime::traits::{Hash, Keccak256},
};

#[test]
fn add_menstrual_calendar_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MentrualCalendar::add_menstrual_calendar(
			Origin::signed(1),
			"DeBio Menstrual Data".as_bytes().to_vec(),
			"DeBio Menstrual Data Document Description".as_bytes().to_vec(),
			"DeBio Menstrual Data Link".as_bytes().to_vec()
		));

		let menstrual_calendar_ids = MentrualCalendar::menstrual_calendar_by_owner_id(1).unwrap();

		assert_eq!(MentrualCalendar::menstrual_calendar_count(), Some(1));

		assert_eq!(MentrualCalendar::menstrual_calendar_count_by_owner(1), Some(1));

		let menstrual_calendar = MentrualCalendar::menstrual_calendar_by_id(menstrual_calendar_ids[0]).unwrap();

		assert_eq!(menstrual_calendar.id, menstrual_calendar_ids[0]);
		assert_eq!(menstrual_calendar.owner_id, 1);
		assert_eq!(menstrual_calendar.title, "DeBio Menstrual Data".as_bytes().to_vec());
		assert_eq!(
			menstrual_calendar.description,
			"DeBio Menstrual Data Document Description".as_bytes().to_vec()
		);
		assert_eq!(menstrual_calendar.report_link, "DeBio Menstrual Data Link".as_bytes().to_vec());
	})
}

#[test]
fn remove_menstrual_calendar_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MentrualCalendar::add_menstrual_calendar(
			Origin::signed(1),
			"DeBio Menstrual Data".as_bytes().to_vec(),
			"DeBio Menstrual Data Document Description".as_bytes().to_vec(),
			"DeBio Menstrual Data Link".as_bytes().to_vec()
		));

		let menstrual_calendar_ids = MentrualCalendar::menstrual_calendar_by_owner_id(1).unwrap();

		assert_eq!(MentrualCalendar::menstrual_calendar_count(), Some(1));

		assert_eq!(MentrualCalendar::menstrual_calendar_count_by_owner(1), Some(1));

		let menstrual_calendar = MentrualCalendar::menstrual_calendar_by_id(menstrual_calendar_ids[0]).unwrap();

		assert_eq!(menstrual_calendar.id, menstrual_calendar_ids[0]);
		assert_eq!(menstrual_calendar.owner_id, 1);
		assert_eq!(menstrual_calendar.title, "DeBio Menstrual Data".as_bytes().to_vec());
		assert_eq!(
			menstrual_calendar.description,
			"DeBio Menstrual Data Document Description".as_bytes().to_vec()
		);
		assert_eq!(menstrual_calendar.report_link, "DeBio Menstrual Data Link".as_bytes().to_vec());

		assert_ok!(MentrualCalendar::remove_menstrual_calendar(Origin::signed(1), menstrual_calendar_ids[0]));

		assert_eq!(MentrualCalendar::menstrual_calendar_count(), Some(0));

		assert_eq!(MentrualCalendar::menstrual_calendar_count_by_owner(1), Some(0));
	})
}

#[test]
fn remove_menstrual_calendar_does_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			MentrualCalendar::remove_menstrual_calendar(
				Origin::signed(1),
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes())
			),
			Error::<Test>::MentrualCalendarDoesNotExist
		);
	})
}

#[test]
fn remove_menstrual_calendar_not_menstrual_calendar_owner() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MentrualCalendar::add_menstrual_calendar(
			Origin::signed(1),
			"DeBio Menstrual Data".as_bytes().to_vec(),
			"DeBio Menstrual Data Document Description".as_bytes().to_vec(),
			"DeBio Menstrual Data Link".as_bytes().to_vec()
		));

		let menstrual_calendar_ids = MentrualCalendar::menstrual_calendar_by_owner_id(1).unwrap();

		assert_eq!(MentrualCalendar::menstrual_calendar_count(), Some(1));

		assert_eq!(MentrualCalendar::menstrual_calendar_count_by_owner(1), Some(1));

		let menstrual_calendar = MentrualCalendar::menstrual_calendar_by_id(menstrual_calendar_ids[0]).unwrap();

		assert_eq!(menstrual_calendar.id, menstrual_calendar_ids[0]);
		assert_eq!(menstrual_calendar.owner_id, 1);
		assert_eq!(menstrual_calendar.title, "DeBio Menstrual Data".as_bytes().to_vec());
		assert_eq!(
			menstrual_calendar.description,
			"DeBio Menstrual Data Document Description".as_bytes().to_vec()
		);
		assert_eq!(menstrual_calendar.report_link, "DeBio Menstrual Data Link".as_bytes().to_vec());

		assert_noop!(
			MentrualCalendar::remove_menstrual_calendar(Origin::signed(2), menstrual_calendar_ids[0]),
			Error::<Test>::NotMentrualCalendarOwner
		);
	})
}

#[test]
fn update_menstrual_calendar_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MentrualCalendar::add_menstrual_calendar(
			Origin::signed(1),
			"DeBio Menstrual Data".as_bytes().to_vec(),
			"DeBio Menstrual Data Document Description".as_bytes().to_vec(),
			"DeBio Menstrual Data Link".as_bytes().to_vec()
		));

		let menstrual_calendar_ids = MentrualCalendar::menstrual_calendar_by_owner_id(1).unwrap();

		assert_eq!(MentrualCalendar::menstrual_calendar_count(), Some(1));

		assert_eq!(MentrualCalendar::menstrual_calendar_count_by_owner(1), Some(1));

		let menstrual_calendar = MentrualCalendar::menstrual_calendar_by_id(menstrual_calendar_ids[0]).unwrap();

		assert_eq!(menstrual_calendar.id, menstrual_calendar_ids[0]);
		assert_eq!(menstrual_calendar.owner_id, 1);
		assert_eq!(menstrual_calendar.title, "DeBio Menstrual Data".as_bytes().to_vec());
		assert_eq!(
			menstrual_calendar.description,
			"DeBio Menstrual Data Document Description".as_bytes().to_vec()
		);
		assert_eq!(menstrual_calendar.report_link, "DeBio Menstrual Data Link".as_bytes().to_vec());

		assert_ok!(MentrualCalendar::update_menstrual_calendar(
			Origin::signed(1),
			menstrual_calendar_ids[0],
			"DeBio Menstrual Data 2".as_bytes().to_vec(),
			"DeBio Menstrual Data Document Description 2".as_bytes().to_vec(),
			"DeBio Menstrual Data Link 2".as_bytes().to_vec()
		));

		let menstrual_calendar_ids = MentrualCalendar::menstrual_calendar_by_owner_id(1).unwrap();

		assert_eq!(MentrualCalendar::menstrual_calendar_count(), Some(1));

		assert_eq!(MentrualCalendar::menstrual_calendar_count_by_owner(1), Some(1));

		let menstrual_calendar = MentrualCalendar::menstrual_calendar_by_id(menstrual_calendar_ids[0]).unwrap();

		assert_eq!(menstrual_calendar.id, menstrual_calendar_ids[0]);
		assert_eq!(menstrual_calendar.owner_id, 1);
		assert_eq!(menstrual_calendar.title, "DeBio Menstrual Data 2".as_bytes().to_vec());
		assert_eq!(
			menstrual_calendar.description,
			"DeBio Menstrual Data Document Description 2".as_bytes().to_vec()
		);
		assert_eq!(menstrual_calendar.report_link, "DeBio Menstrual Data Link 2".as_bytes().to_vec());
	})
}

#[test]
fn update_menstrual_calendar_does_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			MentrualCalendar::update_menstrual_calendar(
				Origin::signed(1),
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
				"DeBio Menstrual Data".as_bytes().to_vec(),
				"DeBio Menstrual Data Document Description".as_bytes().to_vec(),
				"DeBio Menstrual Data Link".as_bytes().to_vec()
			),
			Error::<Test>::MentrualCalendarDoesNotExist
		);
	})
}

#[test]
fn update_menstrual_calendar_not_menstrual_calendar_owner() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MentrualCalendar::add_menstrual_calendar(
			Origin::signed(1),
			"DeBio Menstrual Data".as_bytes().to_vec(),
			"DeBio Menstrual Data Document Description".as_bytes().to_vec(),
			"DeBio Menstrual Data Link".as_bytes().to_vec()
		));

		let menstrual_calendar_ids = MentrualCalendar::menstrual_calendar_by_owner_id(1).unwrap();

		assert_eq!(MentrualCalendar::menstrual_calendar_count(), Some(1));

		assert_eq!(MentrualCalendar::menstrual_calendar_count_by_owner(1), Some(1));

		let menstrual_calendar = MentrualCalendar::menstrual_calendar_by_id(menstrual_calendar_ids[0]).unwrap();

		assert_eq!(menstrual_calendar.id, menstrual_calendar_ids[0]);
		assert_eq!(menstrual_calendar.owner_id, 1);
		assert_eq!(menstrual_calendar.title, "DeBio Menstrual Data".as_bytes().to_vec());
		assert_eq!(
			menstrual_calendar.description,
			"DeBio Menstrual Data Document Description".as_bytes().to_vec()
		);
		assert_eq!(menstrual_calendar.report_link, "DeBio Menstrual Data Link".as_bytes().to_vec());

		assert_noop!(
			MentrualCalendar::update_menstrual_calendar(
				Origin::signed(2),
				menstrual_calendar_ids[0],
				"DeBio Menstrual Data 2".as_bytes().to_vec(),
				"DeBio Menstrual Data Document Description 2".as_bytes().to_vec(),
				"DeBio Menstrual Data Link 2".as_bytes().to_vec()
			),
			Error::<Test>::NotMentrualCalendarOwner
		);
	})
}
