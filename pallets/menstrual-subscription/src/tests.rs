use crate::{mock::*, Error};
use frame_support::{
	assert_noop, assert_ok,
	sp_runtime::traits::{Hash, Keccak256},
};

#[test]
fn add_menstrual_data_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MenstrualSubscription::add_menstrual_data(
			Origin::signed(1),
			"DeBio Menstrual Data".as_bytes().to_vec(),
			"DeBio Menstrual Data Document Description".as_bytes().to_vec(),
			"DeBio Menstrual Data Link".as_bytes().to_vec()
		));

		let menstrual_data_ids = MenstrualSubscription::menstrual_data_by_owner_id(1).unwrap();

		assert_eq!(MenstrualSubscription::menstrual_data_count(), Some(1));

		assert_eq!(MenstrualSubscription::menstrual_data_count_by_owner(1), Some(1));

		let menstrual_data = MenstrualSubscription::menstrual_data_by_id(menstrual_data_ids[0]).unwrap();

		assert_eq!(menstrual_data.id, menstrual_data_ids[0]);
		assert_eq!(menstrual_data.owner_id, 1);
		assert_eq!(menstrual_data.title, "DeBio Menstrual Data".as_bytes().to_vec());
		assert_eq!(
			menstrual_data.description,
			"DeBio Menstrual Data Document Description".as_bytes().to_vec()
		);
		assert_eq!(menstrual_data.report_link, "DeBio Menstrual Data Link".as_bytes().to_vec());
	})
}

#[test]
fn remove_menstrual_data_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MenstrualSubscription::add_menstrual_data(
			Origin::signed(1),
			"DeBio Menstrual Data".as_bytes().to_vec(),
			"DeBio Menstrual Data Document Description".as_bytes().to_vec(),
			"DeBio Menstrual Data Link".as_bytes().to_vec()
		));

		let menstrual_data_ids = MenstrualSubscription::menstrual_data_by_owner_id(1).unwrap();

		assert_eq!(MenstrualSubscription::menstrual_data_count(), Some(1));

		assert_eq!(MenstrualSubscription::menstrual_data_count_by_owner(1), Some(1));

		let menstrual_data = MenstrualSubscription::menstrual_data_by_id(menstrual_data_ids[0]).unwrap();

		assert_eq!(menstrual_data.id, menstrual_data_ids[0]);
		assert_eq!(menstrual_data.owner_id, 1);
		assert_eq!(menstrual_data.title, "DeBio Menstrual Data".as_bytes().to_vec());
		assert_eq!(
			menstrual_data.description,
			"DeBio Menstrual Data Document Description".as_bytes().to_vec()
		);
		assert_eq!(menstrual_data.report_link, "DeBio Menstrual Data Link".as_bytes().to_vec());

		assert_ok!(MenstrualSubscription::remove_menstrual_data(Origin::signed(1), menstrual_data_ids[0]));

		assert_eq!(MenstrualSubscription::menstrual_data_count(), Some(0));

		assert_eq!(MenstrualSubscription::menstrual_data_count_by_owner(1), Some(0));
	})
}

#[test]
fn remove_menstrual_data_does_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			MenstrualSubscription::remove_menstrual_data(
				Origin::signed(1),
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes())
			),
			Error::<Test>::MenstrualSubscriptionDoesNotExist
		);
	})
}

#[test]
fn remove_menstrual_data_not_menstrual_data_owner() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MenstrualSubscription::add_menstrual_data(
			Origin::signed(1),
			"DeBio Menstrual Data".as_bytes().to_vec(),
			"DeBio Menstrual Data Document Description".as_bytes().to_vec(),
			"DeBio Menstrual Data Link".as_bytes().to_vec()
		));

		let menstrual_data_ids = MenstrualSubscription::menstrual_data_by_owner_id(1).unwrap();

		assert_eq!(MenstrualSubscription::menstrual_data_count(), Some(1));

		assert_eq!(MenstrualSubscription::menstrual_data_count_by_owner(1), Some(1));

		let menstrual_data = MenstrualSubscription::menstrual_data_by_id(menstrual_data_ids[0]).unwrap();

		assert_eq!(menstrual_data.id, menstrual_data_ids[0]);
		assert_eq!(menstrual_data.owner_id, 1);
		assert_eq!(menstrual_data.title, "DeBio Menstrual Data".as_bytes().to_vec());
		assert_eq!(
			menstrual_data.description,
			"DeBio Menstrual Data Document Description".as_bytes().to_vec()
		);
		assert_eq!(menstrual_data.report_link, "DeBio Menstrual Data Link".as_bytes().to_vec());

		assert_noop!(
			MenstrualSubscription::remove_menstrual_data(Origin::signed(2), menstrual_data_ids[0]),
			Error::<Test>::NotMenstrualSubscriptionOwner
		);
	})
}

#[test]
fn update_menstrual_data_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MenstrualSubscription::add_menstrual_data(
			Origin::signed(1),
			"DeBio Menstrual Data".as_bytes().to_vec(),
			"DeBio Menstrual Data Document Description".as_bytes().to_vec(),
			"DeBio Menstrual Data Link".as_bytes().to_vec()
		));

		let menstrual_data_ids = MenstrualSubscription::menstrual_data_by_owner_id(1).unwrap();

		assert_eq!(MenstrualSubscription::menstrual_data_count(), Some(1));

		assert_eq!(MenstrualSubscription::menstrual_data_count_by_owner(1), Some(1));

		let menstrual_data = MenstrualSubscription::menstrual_data_by_id(menstrual_data_ids[0]).unwrap();

		assert_eq!(menstrual_data.id, menstrual_data_ids[0]);
		assert_eq!(menstrual_data.owner_id, 1);
		assert_eq!(menstrual_data.title, "DeBio Menstrual Data".as_bytes().to_vec());
		assert_eq!(
			menstrual_data.description,
			"DeBio Menstrual Data Document Description".as_bytes().to_vec()
		);
		assert_eq!(menstrual_data.report_link, "DeBio Menstrual Data Link".as_bytes().to_vec());

		assert_ok!(MenstrualSubscription::update_menstrual_data(
			Origin::signed(1),
			menstrual_data_ids[0],
			"DeBio Menstrual Data 2".as_bytes().to_vec(),
			"DeBio Menstrual Data Document Description 2".as_bytes().to_vec(),
			"DeBio Menstrual Data Link 2".as_bytes().to_vec()
		));

		let menstrual_data_ids = MenstrualSubscription::menstrual_data_by_owner_id(1).unwrap();

		assert_eq!(MenstrualSubscription::menstrual_data_count(), Some(1));

		assert_eq!(MenstrualSubscription::menstrual_data_count_by_owner(1), Some(1));

		let menstrual_data = MenstrualSubscription::menstrual_data_by_id(menstrual_data_ids[0]).unwrap();

		assert_eq!(menstrual_data.id, menstrual_data_ids[0]);
		assert_eq!(menstrual_data.owner_id, 1);
		assert_eq!(menstrual_data.title, "DeBio Menstrual Data 2".as_bytes().to_vec());
		assert_eq!(
			menstrual_data.description,
			"DeBio Menstrual Data Document Description 2".as_bytes().to_vec()
		);
		assert_eq!(menstrual_data.report_link, "DeBio Menstrual Data Link 2".as_bytes().to_vec());
	})
}

#[test]
fn update_menstrual_data_does_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			MenstrualSubscription::update_menstrual_data(
				Origin::signed(1),
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
				"DeBio Menstrual Data".as_bytes().to_vec(),
				"DeBio Menstrual Data Document Description".as_bytes().to_vec(),
				"DeBio Menstrual Data Link".as_bytes().to_vec()
			),
			Error::<Test>::MenstrualSubscriptionDoesNotExist
		);
	})
}

#[test]
fn update_menstrual_data_not_menstrual_data_owner() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MenstrualSubscription::add_menstrual_data(
			Origin::signed(1),
			"DeBio Menstrual Data".as_bytes().to_vec(),
			"DeBio Menstrual Data Document Description".as_bytes().to_vec(),
			"DeBio Menstrual Data Link".as_bytes().to_vec()
		));

		let menstrual_data_ids = MenstrualSubscription::menstrual_data_by_owner_id(1).unwrap();

		assert_eq!(MenstrualSubscription::menstrual_data_count(), Some(1));

		assert_eq!(MenstrualSubscription::menstrual_data_count_by_owner(1), Some(1));

		let menstrual_data = MenstrualSubscription::menstrual_data_by_id(menstrual_data_ids[0]).unwrap();

		assert_eq!(menstrual_data.id, menstrual_data_ids[0]);
		assert_eq!(menstrual_data.owner_id, 1);
		assert_eq!(menstrual_data.title, "DeBio Menstrual Data".as_bytes().to_vec());
		assert_eq!(
			menstrual_data.description,
			"DeBio Menstrual Data Document Description".as_bytes().to_vec()
		);
		assert_eq!(menstrual_data.report_link, "DeBio Menstrual Data Link".as_bytes().to_vec());

		assert_noop!(
			MenstrualSubscription::update_menstrual_data(
				Origin::signed(2),
				menstrual_data_ids[0],
				"DeBio Menstrual Data 2".as_bytes().to_vec(),
				"DeBio Menstrual Data Document Description 2".as_bytes().to_vec(),
				"DeBio Menstrual Data Link 2".as_bytes().to_vec()
			),
			Error::<Test>::NotMenstrualSubscriptionOwner
		);
	})
}
