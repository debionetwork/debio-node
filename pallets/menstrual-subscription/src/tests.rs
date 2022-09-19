use crate::{mock::*, AdminKey, Error};
use frame_support::{
	assert_noop, assert_ok,
	sp_runtime::traits::{Hash, Keccak256},
};
use primitives_duration::MenstrualSubscriptionDuration;
use primitives_menstrual_status::{MenstrualSubscriptionStatus, PaymentStatus};

#[test]
fn add_menstrual_subscription_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MenstrualSubscription::add_menstrual_subscription(
			Origin::signed(1),
			MenstrualSubscriptionDuration::default(),
			1,
			PaymentStatus::default(),
			MenstrualSubscriptionStatus::default(),
		));

		let menstrual_subscription_ids =
			MenstrualSubscription::menstrual_subscription_by_address_id(1).unwrap();

		assert_eq!(MenstrualSubscription::menstrual_subscription_count(), Some(1));

		assert_eq!(MenstrualSubscription::menstrual_subscription_count_by_owner(1), Some(1));

		let menstrual_subscription =
			MenstrualSubscription::menstrual_subscription_by_id(menstrual_subscription_ids[0])
				.unwrap();

		assert_eq!(menstrual_subscription.id, menstrual_subscription_ids[0]);
		assert_eq!(menstrual_subscription.address_id, 1);
		assert_eq!(menstrual_subscription.duration, MenstrualSubscriptionDuration::default());
		assert_eq!(menstrual_subscription.price, 1);
		assert_eq!(menstrual_subscription.payment_status, PaymentStatus::default(),);
		assert_eq!(menstrual_subscription.status, MenstrualSubscriptionStatus::default(),);
	})
}

#[test]
fn set_menstrual_subscription_paid_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MenstrualSubscription::add_menstrual_subscription(
			Origin::signed(1),
			MenstrualSubscriptionDuration::default(),
			1,
			PaymentStatus::default(),
			MenstrualSubscriptionStatus::default(),
		));

		let menstrual_subscription_ids =
			MenstrualSubscription::menstrual_subscription_by_address_id(1).unwrap();

		assert_eq!(MenstrualSubscription::menstrual_subscription_count(), Some(1));

		assert_eq!(MenstrualSubscription::menstrual_subscription_count_by_owner(1), Some(1));

		let menstrual_subscription =
			MenstrualSubscription::menstrual_subscription_by_id(menstrual_subscription_ids[0])
				.unwrap();

		assert_eq!(menstrual_subscription.id, menstrual_subscription_ids[0]);
		assert_eq!(menstrual_subscription.address_id, 1);
		assert_eq!(menstrual_subscription.duration, MenstrualSubscriptionDuration::default());
		assert_eq!(menstrual_subscription.price, 1);
		assert_eq!(menstrual_subscription.payment_status, PaymentStatus::default(),);
		assert_eq!(menstrual_subscription.status, MenstrualSubscriptionStatus::default(),);

		AdminKey::<Test>::put(1);

		assert_ok!(MenstrualSubscription::set_menstrual_subscription_paid(
			Origin::signed(1),
			1,
			menstrual_subscription_ids[0]
		));

		assert_eq!(MenstrualSubscription::menstrual_subscription_count(), Some(1));

		assert_eq!(MenstrualSubscription::menstrual_subscription_count_by_owner(1), Some(1));
	})
}

#[test]
fn set_menstrual_subscription_paid_does_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(1);

		assert_noop!(
			MenstrualSubscription::set_menstrual_subscription_paid(
				Origin::signed(1),
				1,
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes())
			),
			Error::<Test>::MenstrualSubscriptionDoesNotExist
		);
	})
}

#[test]
fn set_menstrual_subscription_paid_not_menstrual_subscription_owner() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MenstrualSubscription::add_menstrual_subscription(
			Origin::signed(1),
			MenstrualSubscriptionDuration::default(),
			1,
			PaymentStatus::default(),
			MenstrualSubscriptionStatus::default(),
		));

		let menstrual_subscription_ids =
			MenstrualSubscription::menstrual_subscription_by_address_id(1).unwrap();

		assert_eq!(MenstrualSubscription::menstrual_subscription_count(), Some(1));

		assert_eq!(MenstrualSubscription::menstrual_subscription_count_by_owner(1), Some(1));

		let menstrual_subscription =
			MenstrualSubscription::menstrual_subscription_by_id(menstrual_subscription_ids[0])
				.unwrap();

		assert_eq!(menstrual_subscription.id, menstrual_subscription_ids[0]);
		assert_eq!(menstrual_subscription.address_id, 1);
		assert_eq!(menstrual_subscription.duration, MenstrualSubscriptionDuration::default());
		assert_eq!(menstrual_subscription.price, 1);
		assert_eq!(menstrual_subscription.payment_status, PaymentStatus::default(),);
		assert_eq!(menstrual_subscription.status, MenstrualSubscriptionStatus::default(),);

		AdminKey::<Test>::put(2);

		assert_noop!(
			MenstrualSubscription::set_menstrual_subscription_paid(
				Origin::signed(2),
				2,
				menstrual_subscription_ids[0]
			),
			Error::<Test>::NotMenstrualSubscriptionOwner
		);
	})
}

#[test]
fn change_menstrual_subscription_status_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MenstrualSubscription::add_menstrual_subscription(
			Origin::signed(1),
			MenstrualSubscriptionDuration::default(),
			1,
			PaymentStatus::default(),
			MenstrualSubscriptionStatus::default(),
		));

		let menstrual_subscription_ids =
			MenstrualSubscription::menstrual_subscription_by_address_id(1).unwrap();

		assert_eq!(MenstrualSubscription::menstrual_subscription_count(), Some(1));

		assert_eq!(MenstrualSubscription::menstrual_subscription_count_by_owner(1), Some(1));

		let menstrual_subscription =
			MenstrualSubscription::menstrual_subscription_by_id(menstrual_subscription_ids[0])
				.unwrap();

		assert_eq!(menstrual_subscription.id, menstrual_subscription_ids[0]);
		assert_eq!(menstrual_subscription.address_id, 1);
		assert_eq!(menstrual_subscription.duration, MenstrualSubscriptionDuration::default());
		assert_eq!(menstrual_subscription.price, 1);
		assert_eq!(menstrual_subscription.payment_status, PaymentStatus::default(),);
		assert_eq!(menstrual_subscription.status, MenstrualSubscriptionStatus::default(),);

		AdminKey::<Test>::put(1);

		assert_ok!(MenstrualSubscription::change_menstrual_subscription_status(
			Origin::signed(1),
			1,
			menstrual_subscription_ids[0],
			MenstrualSubscriptionStatus::default(),
		));

		let menstrual_subscription_ids =
			MenstrualSubscription::menstrual_subscription_by_address_id(1).unwrap();

		assert_eq!(MenstrualSubscription::menstrual_subscription_count(), Some(1));

		assert_eq!(MenstrualSubscription::menstrual_subscription_count_by_owner(1), Some(1));

		let menstrual_subscription =
			MenstrualSubscription::menstrual_subscription_by_id(menstrual_subscription_ids[0])
				.unwrap();

		assert_eq!(menstrual_subscription.id, menstrual_subscription_ids[0]);
		assert_eq!(menstrual_subscription.address_id, 1);
		assert_eq!(menstrual_subscription.duration, MenstrualSubscriptionDuration::default());
		assert_eq!(menstrual_subscription.price, 1);
		assert_eq!(menstrual_subscription.payment_status, PaymentStatus::default(),);
		assert_eq!(menstrual_subscription.status, MenstrualSubscriptionStatus::default(),);
	})
}

#[test]
fn change_menstrual_subscription_status_does_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(1);

		assert_noop!(
			MenstrualSubscription::change_menstrual_subscription_status(
				Origin::signed(1),
				1,
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
				MenstrualSubscriptionStatus::default()
			),
			Error::<Test>::MenstrualSubscriptionDoesNotExist
		);
	})
}

#[test]
fn change_menstrual_subscription_status_not_menstrual_subscription_owner() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MenstrualSubscription::add_menstrual_subscription(
			Origin::signed(1),
			MenstrualSubscriptionDuration::default(),
			1,
			PaymentStatus::default(),
			MenstrualSubscriptionStatus::default(),
		));

		let menstrual_subscription_ids =
			MenstrualSubscription::menstrual_subscription_by_address_id(1).unwrap();

		assert_eq!(MenstrualSubscription::menstrual_subscription_count(), Some(1));

		assert_eq!(MenstrualSubscription::menstrual_subscription_count_by_owner(1), Some(1));

		let menstrual_subscription =
			MenstrualSubscription::menstrual_subscription_by_id(menstrual_subscription_ids[0])
				.unwrap();

		assert_eq!(menstrual_subscription.id, menstrual_subscription_ids[0]);
		assert_eq!(menstrual_subscription.address_id, 1);
		assert_eq!(menstrual_subscription.duration, MenstrualSubscriptionDuration::default());
		assert_eq!(menstrual_subscription.price, 1);
		assert_eq!(menstrual_subscription.payment_status, PaymentStatus::default(),);
		assert_eq!(menstrual_subscription.status, MenstrualSubscriptionStatus::default(),);

		AdminKey::<Test>::put(2);

		assert_noop!(
			MenstrualSubscription::change_menstrual_subscription_status(
				Origin::signed(2),
				2,
				menstrual_subscription_ids[0],
				MenstrualSubscriptionStatus::default(),
			),
			Error::<Test>::NotMenstrualSubscriptionOwner
		);
	})
}
