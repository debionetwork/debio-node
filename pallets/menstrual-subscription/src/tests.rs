use crate::{
	mock::*, AccountKeyType, AdminKey, Error, MenstrualSubscription as MenstrualSubscriptionS,
	MenstrualSubscriptionPrice, TreasuryKey,
};
use frame_support::{
	assert_noop, assert_ok,
	sp_runtime::traits::{Hash, Keccak256},
};
use primitives_duration::MenstrualSubscriptionDuration;
use primitives_menstrual_status::{MenstrualSubscriptionStatus, PaymentStatus};
use primitives_price_and_currency::CurrencyType;

#[test]
fn add_menstrual_subscription_works() {
	ExternalityBuilder::build().execute_with(|| {
		let customer = account_key("customer");
		let admin = account_key("admin");

		AdminKey::<Test>::put(admin);

		assert_ok!(MenstrualSubscription::set_menstrual_subscription_price(
			Origin::signed(admin),
			MenstrualSubscriptionDuration::default(),
			CurrencyType::DBIO,
			10,
			None,
		));

		assert_ok!(MenstrualSubscription::add_menstrual_subscription(
			Origin::signed(customer),
			MenstrualSubscriptionDuration::default(),
			CurrencyType::DBIO,
		));

		let menstrual_subscription_ids =
			MenstrualSubscription::menstrual_subscription_by_address_id(customer).unwrap();

		let menstrual_subscription = MenstrualSubscriptionS {
			id: menstrual_subscription_ids[0],
			address_id: customer,
			currency: CurrencyType::DBIO,
			duration: MenstrualSubscriptionDuration::default(),
			payment_status: PaymentStatus::default(),
			status: MenstrualSubscriptionStatus::InQueue,
			created_at: 0,
			updated_at: 0,
		};

		assert_eq!(
			MenstrualSubscription::menstrual_subscription_by_id(menstrual_subscription_ids[0]),
			Some(menstrual_subscription)
		);
		assert_eq!(MenstrualSubscription::menstrual_subscription_count(), Some(1));

		assert_eq!(MenstrualSubscription::menstrual_subscription_count_by_owner(customer), Some(1));
	})
}

#[test]
fn set_menstrual_subscription_paid_works() {
	ExternalityBuilder::build().execute_with(|| {
		let total_issuance = Balances::total_issuance();

		let customer = account_key("customer");
		let treasure = account_key("treasure");
		let admin = account_key("admin");

		AdminKey::<Test>::put(admin);
		TreasuryKey::<Test>::put(treasure);

		assert_ok!(MenstrualSubscription::set_menstrual_subscription_price(
			Origin::signed(admin),
			MenstrualSubscriptionDuration::default(),
			CurrencyType::DBIO,
			10,
			None,
		));

		assert_ok!(MenstrualSubscription::add_menstrual_subscription(
			Origin::signed(customer),
			MenstrualSubscriptionDuration::default(),
			CurrencyType::DBIO,
		));

		let menstrual_subscription_ids =
			MenstrualSubscription::menstrual_subscription_by_address_id(customer).unwrap();

		assert_ok!(MenstrualSubscription::set_menstrual_subscription_paid(
			Origin::signed(customer),
			menstrual_subscription_ids[0]
		));

		let mut menstrual_subscription = MenstrualSubscriptionS {
			id: menstrual_subscription_ids[0],
			address_id: customer,
			currency: CurrencyType::DBIO,
			duration: MenstrualSubscriptionDuration::default(),
			payment_status: PaymentStatus::Paid,
			status: MenstrualSubscriptionStatus::Active,
			created_at: 0,
			updated_at: 0,
		};

		assert_eq!(
			MenstrualSubscription::active_subscription_by_owner(customer),
			Some(menstrual_subscription_ids[0]),
		);

		assert_eq!(
			MenstrualSubscription::menstrual_subscription_by_id(menstrual_subscription_ids[0]),
			Some(menstrual_subscription.clone())
		);

		assert_eq!(Balances::free_balance(customer), 190);
		assert_eq!(Balances::total_issuance(), total_issuance - 10);

		let total_issuance = Balances::total_issuance();

		assert_ok!(MenstrualSubscription::add_menstrual_subscription(
			Origin::signed(customer),
			MenstrualSubscriptionDuration::default(),
			CurrencyType::DBIO,
		));

		let menstrual_subscription_ids =
			MenstrualSubscription::menstrual_subscription_by_address_id(customer).unwrap();

		assert_ok!(MenstrualSubscription::set_menstrual_subscription_paid(
			Origin::signed(customer),
			menstrual_subscription_ids[1]
		));

		menstrual_subscription.id = menstrual_subscription_ids[1];
		menstrual_subscription.status = MenstrualSubscriptionStatus::InQueue;

		assert_eq!(
			MenstrualSubscription::menstrual_subscription_by_id(menstrual_subscription_ids[1]),
			Some(menstrual_subscription)
		);

		assert_eq!(Balances::free_balance(customer), 180);
		assert_eq!(Balances::total_issuance(), total_issuance - 10);
	})
}

#[test]
fn change_menstrual_subscription_status_works() {
	ExternalityBuilder::build().execute_with(|| {
		let customer = account_key("customer");
		let admin = account_key("admin");
		let treasure = account_key("treasure");

		AdminKey::<Test>::put(admin);
		TreasuryKey::<Test>::put(treasure);

		assert_ok!(MenstrualSubscription::set_menstrual_subscription_price(
			Origin::signed(admin),
			MenstrualSubscriptionDuration::default(),
			CurrencyType::DBIO,
			10,
			None,
		));

		assert_ok!(MenstrualSubscription::add_menstrual_subscription(
			Origin::signed(customer),
			MenstrualSubscriptionDuration::default(),
			CurrencyType::DBIO,
		));

		let menstrual_subscription_ids =
			MenstrualSubscription::menstrual_subscription_by_address_id(customer).unwrap();

		assert_ok!(MenstrualSubscription::set_menstrual_subscription_paid(
			Origin::signed(customer),
			menstrual_subscription_ids[0]
		));

		assert_ok!(MenstrualSubscription::change_menstrual_subscription_status(
			Origin::signed(admin),
			menstrual_subscription_ids[0],
			MenstrualSubscriptionStatus::default(),
		));

		let menstrual_subscription = MenstrualSubscriptionS {
			id: menstrual_subscription_ids[0],
			address_id: customer,
			currency: CurrencyType::DBIO,
			duration: MenstrualSubscriptionDuration::default(),
			payment_status: PaymentStatus::Paid,
			status: MenstrualSubscriptionStatus::default(),
			created_at: 0,
			updated_at: 0,
		};

		assert_eq!(
			MenstrualSubscription::menstrual_subscription_by_id(menstrual_subscription_ids[0]),
			Some(menstrual_subscription)
		);

		assert_eq!(MenstrualSubscription::active_subscription_by_owner(customer), None,);
	})
}

#[test]
fn set_menstrual_subscription_price_works() {
	ExternalityBuilder::build().execute_with(|| {
		let admin = account_key("admin");
		let asset_id = Some(1);

		AdminKey::<Test>::put(admin);

		assert_ok!(MenstrualSubscription::set_menstrual_subscription_price(
			Origin::signed(admin),
			MenstrualSubscriptionDuration::default(),
			CurrencyType::USDT,
			10,
			asset_id,
		));

		let menstrual_subscription_price = MenstrualSubscriptionPrice {
			duration: MenstrualSubscriptionDuration::default(),
			currency: CurrencyType::USDT,
			amount: 10,
			asset_id,
		};

		assert_eq!(
			MenstrualSubscription::menstrual_subscription_prices(
				MenstrualSubscriptionDuration::default(),
				CurrencyType::USDT
			),
			Some(menstrual_subscription_price)
		);
	})
}

#[test]
fn update_key_works() {
	ExternalityBuilder::build().execute_with(|| {
		let admin = account_key("admin");
		let other_admin = account_key("other_admin");

		AdminKey::<Test>::put(admin);

		assert_ok!(MenstrualSubscription::update_key(
			Origin::signed(admin),
			AccountKeyType::AdminKey(other_admin)
		));

		assert_eq!(MenstrualSubscription::admin_key(), Some(other_admin));
	})
}

#[test]
fn sudo_update_key_works() {
	ExternalityBuilder::build().execute_with(|| {
		let treasure = account_key("treasure");

		assert_ok!(MenstrualSubscription::sudo_update_key(
			Origin::root(),
			AccountKeyType::TreasuryKey(treasure)
		));

		assert_eq!(MenstrualSubscription::treasury_key(), Some(treasure));
	})
}

#[test]
fn cant_add_menstrual_subscription_when_already_inqueue() {
	ExternalityBuilder::build().execute_with(|| {
		let customer = account_key("customer");
		let admin = account_key("admin");

		AdminKey::<Test>::put(admin);

		assert_ok!(MenstrualSubscription::set_menstrual_subscription_price(
			Origin::signed(admin),
			MenstrualSubscriptionDuration::default(),
			CurrencyType::DBIO,
			10,
			None,
		));

		assert_ok!(MenstrualSubscription::add_menstrual_subscription(
			Origin::signed(customer),
			MenstrualSubscriptionDuration::default(),
			CurrencyType::DBIO,
		));

		assert_noop!(
			MenstrualSubscription::add_menstrual_subscription(
				Origin::signed(customer),
				MenstrualSubscriptionDuration::default(),
				CurrencyType::DBIO,
			),
			Error::<Test>::MenstrualSubscriptionAlreadyInQueue,
		);
	})
}

#[test]
fn cant_add_menstrual_subcription_when_price_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		let customer = account_key("customer");

		assert_noop!(
			MenstrualSubscription::add_menstrual_subscription(
				Origin::signed(customer),
				MenstrualSubscriptionDuration::default(),
				CurrencyType::DBIO,
			),
			Error::<Test>::MenstrualSubscriptionPriceNotExist,
		);
	})
}

#[test]
fn cant_change_menstrual_subscription_status_when_not_paid() {
	ExternalityBuilder::build().execute_with(|| {
		let customer = account_key("customer");
		let admin = account_key("admin");

		AdminKey::<Test>::put(admin);

		assert_ok!(MenstrualSubscription::set_menstrual_subscription_price(
			Origin::signed(admin),
			MenstrualSubscriptionDuration::default(),
			CurrencyType::DBIO,
			10,
			None,
		));

		assert_ok!(MenstrualSubscription::add_menstrual_subscription(
			Origin::signed(customer),
			MenstrualSubscriptionDuration::default(),
			CurrencyType::DBIO,
		));

		let menstrual_subscription_ids =
			MenstrualSubscription::menstrual_subscription_by_address_id(customer).unwrap();

		assert_noop!(
			MenstrualSubscription::change_menstrual_subscription_status(
				Origin::signed(admin),
				menstrual_subscription_ids[0],
				MenstrualSubscriptionStatus::default(),
			),
			Error::<Test>::MenstrualSubscriptionNotPaid,
		);
	})
}

#[test]
fn cant_set_menstrual_subscription_paid_when_not_exists() {
	ExternalityBuilder::build().execute_with(|| {
		let treasure = account_key("treasure");
		let customer = account_key("customer");

		TreasuryKey::<Test>::put(treasure);

		assert_noop!(
			MenstrualSubscription::set_menstrual_subscription_paid(
				Origin::signed(customer),
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes())
			),
			Error::<Test>::MenstrualSubscriptionDoesNotExist,
		);
	})
}

#[test]
fn cant_set_menstrual_subscription_paid_when_not_owner() {
	ExternalityBuilder::build().execute_with(|| {
		let customer = account_key("customer");
		let treasure = account_key("treasure");
		let admin = account_key("admin");
		let other = account_key("other");

		AdminKey::<Test>::put(admin);
		TreasuryKey::<Test>::put(treasure);

		assert_ok!(MenstrualSubscription::set_menstrual_subscription_price(
			Origin::signed(admin),
			MenstrualSubscriptionDuration::default(),
			CurrencyType::DBIO,
			10,
			None,
		));

		assert_ok!(MenstrualSubscription::add_menstrual_subscription(
			Origin::signed(customer),
			MenstrualSubscriptionDuration::default(),
			CurrencyType::DBIO,
		));

		let menstrual_subscription_ids =
			MenstrualSubscription::menstrual_subscription_by_address_id(customer).unwrap();

		assert_noop!(
			MenstrualSubscription::set_menstrual_subscription_paid(
				Origin::signed(other),
				menstrual_subscription_ids[0]
			),
			Error::<Test>::NotMenstrualSubscriptionOwner,
		);
	})
}

#[test]
fn cant_set_menstrual_subscription_paid_when_already_paid() {
	ExternalityBuilder::build().execute_with(|| {
		let customer = account_key("customer");
		let treasure = account_key("treasure");
		let admin = account_key("admin");

		AdminKey::<Test>::put(admin);
		TreasuryKey::<Test>::put(treasure);

		assert_ok!(MenstrualSubscription::set_menstrual_subscription_price(
			Origin::signed(admin),
			MenstrualSubscriptionDuration::default(),
			CurrencyType::DBIO,
			10,
			None,
		));

		assert_ok!(MenstrualSubscription::add_menstrual_subscription(
			Origin::signed(customer),
			MenstrualSubscriptionDuration::default(),
			CurrencyType::DBIO,
		));

		let menstrual_subscription_ids =
			MenstrualSubscription::menstrual_subscription_by_address_id(customer).unwrap();

		assert_ok!(MenstrualSubscription::set_menstrual_subscription_paid(
			Origin::signed(customer),
			menstrual_subscription_ids[0]
		));

		assert_noop!(
			MenstrualSubscription::set_menstrual_subscription_paid(
				Origin::signed(customer),
				menstrual_subscription_ids[0]
			),
			Error::<Test>::MenstrualSubscriptionAlreadyPaid,
		);
	})
}

#[test]
fn cant_set_menstrual_subscription_paid_when_balance_not_enough() {
	ExternalityBuilder::build().execute_with(|| {
		let customer = account_key("customer");
		let treasure = account_key("treasure");
		let admin = account_key("admin");

		AdminKey::<Test>::put(admin);
		TreasuryKey::<Test>::put(treasure);

		assert_ok!(MenstrualSubscription::set_menstrual_subscription_price(
			Origin::signed(admin),
			MenstrualSubscriptionDuration::default(),
			CurrencyType::DBIO,
			1000,
			None,
		));

		assert_ok!(MenstrualSubscription::add_menstrual_subscription(
			Origin::signed(customer),
			MenstrualSubscriptionDuration::default(),
			CurrencyType::DBIO,
		));

		let menstrual_subscription_ids =
			MenstrualSubscription::menstrual_subscription_by_address_id(customer).unwrap();

		assert_noop!(
			MenstrualSubscription::set_menstrual_subscription_paid(
				Origin::signed(customer),
				menstrual_subscription_ids[0]
			),
			Error::<Test>::InsufficientBalance,
		);
	})
}

#[test]
fn cant_change_menstrual_subscription_status_when_unauthorized() {
	ExternalityBuilder::build().execute_with(|| {
		let other = account_key("other");
		let admin = account_key("admin");

		assert_noop!(
			MenstrualSubscription::change_menstrual_subscription_status(
				Origin::signed(other),
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
				MenstrualSubscriptionStatus::default()
			),
			Error::<Test>::Unauthorized
		);

		AdminKey::<Test>::put(admin);

		assert_noop!(
			MenstrualSubscription::change_menstrual_subscription_status(
				Origin::signed(other),
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
				MenstrualSubscriptionStatus::default()
			),
			Error::<Test>::Unauthorized
		);
	})
}

#[test]
fn cant_change_menstrual_subscription_status_when_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		let admin = account_key("admin");

		AdminKey::<Test>::put(admin);

		assert_noop!(
			MenstrualSubscription::change_menstrual_subscription_status(
				Origin::signed(admin),
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
				MenstrualSubscriptionStatus::default()
			),
			Error::<Test>::MenstrualSubscriptionDoesNotExist
		);
	})
}

#[test]
fn cant_set_menstrual_subscription_price_when_unauthorized() {
	ExternalityBuilder::build().execute_with(|| {
		let admin = account_key("admin");
		let other = account_key("other");
		let asset_id = Some(0);

		assert_noop!(
			MenstrualSubscription::set_menstrual_subscription_price(
				Origin::signed(other),
				MenstrualSubscriptionDuration::default(),
				CurrencyType::USDT,
				10,
				asset_id,
			),
			Error::<Test>::Unauthorized,
		);

		AdminKey::<Test>::put(admin);

		assert_noop!(
			MenstrualSubscription::set_menstrual_subscription_price(
				Origin::signed(other),
				MenstrualSubscriptionDuration::default(),
				CurrencyType::USDT,
				10,
				asset_id,
			),
			Error::<Test>::Unauthorized,
		);
	})
}

#[test]
fn cant_set_menstrual_subscription_price_when_asset_id_not_found() {
	ExternalityBuilder::build().execute_with(|| {
		let admin = account_key("admin");
		let asset_id = Some(0);

		AdminKey::<Test>::put(admin);

		assert_noop!(
			MenstrualSubscription::set_menstrual_subscription_price(
				Origin::signed(admin),
				MenstrualSubscriptionDuration::default(),
				CurrencyType::USDT,
				10,
				asset_id,
			),
			Error::<Test>::AssetIdNotFound,
		);
	})
}

#[test]
fn cant_update_key_when_unauthorized() {
	ExternalityBuilder::build().execute_with(|| {
		let treasure = account_key("treasure");
		let other_treasure = account_key("other_treasure");

		TreasuryKey::<Test>::put(treasure);

		assert_noop!(
			MenstrualSubscription::update_key(
				Origin::signed(other_treasure),
				AccountKeyType::TreasuryKey(treasure)
			),
			Error::<Test>::Unauthorized,
		);
	})
}

#[test]
fn call_event_should_work() {
	ExternalityBuilder::build().execute_with(|| {
		System::set_block_number(1);

		let customer = account_key("customer");
		let admin = account_key("admin");

		AdminKey::<Test>::put(admin);

		assert_ok!(MenstrualSubscription::set_menstrual_subscription_price(
			Origin::signed(admin),
			MenstrualSubscriptionDuration::default(),
			CurrencyType::DBIO,
			10,
			None,
		));

		let menstrual_subscription_price = MenstrualSubscriptionPrice {
			duration: MenstrualSubscriptionDuration::default(),
			currency: CurrencyType::DBIO,
			amount: 10,
			asset_id: None,
		};

		System::assert_last_event(Event::MenstrualSubscription(
			crate::Event::MenstrualSubscriptionPriceAdded(menstrual_subscription_price),
		));

		assert_ok!(MenstrualSubscription::add_menstrual_subscription(
			Origin::signed(customer),
			MenstrualSubscriptionDuration::default(),
			CurrencyType::DBIO,
		));

		let menstrual_subscription_ids =
			MenstrualSubscription::menstrual_subscription_by_address_id(customer).unwrap();

		let mut menstrual_subscription = MenstrualSubscriptionS {
			id: menstrual_subscription_ids[0],
			address_id: customer,
			currency: CurrencyType::DBIO,
			duration: MenstrualSubscriptionDuration::default(),
			payment_status: PaymentStatus::default(),
			status: MenstrualSubscriptionStatus::InQueue,
			created_at: 0,
			updated_at: 0,
		};

		System::assert_last_event(Event::MenstrualSubscription(
			crate::Event::MenstrualSubscriptionAdded(menstrual_subscription.clone(), customer),
		));

		let treasure = account_key("treasure");

		assert_ok!(MenstrualSubscription::sudo_update_key(
			Origin::root(),
			AccountKeyType::TreasuryKey(treasure)
		));

		System::assert_last_event(Event::MenstrualSubscription(
			crate::Event::UpdateMenstrualSubscriptionKeySuccessful(AccountKeyType::TreasuryKey(
				treasure,
			)),
		));

		menstrual_subscription.payment_status = PaymentStatus::Paid;
		menstrual_subscription.status = MenstrualSubscriptionStatus::Active;

		assert_ok!(MenstrualSubscription::set_menstrual_subscription_paid(
			Origin::signed(customer),
			menstrual_subscription_ids[0]
		));

		System::assert_last_event(Event::MenstrualSubscription(
			crate::Event::MenstrualSubscriptionPaid(menstrual_subscription.clone(), customer),
		));

		menstrual_subscription.payment_status = PaymentStatus::Paid;
		menstrual_subscription.status = MenstrualSubscriptionStatus::Inactive;

		assert_ok!(MenstrualSubscription::change_menstrual_subscription_status(
			Origin::signed(admin),
			menstrual_subscription_ids[0],
			MenstrualSubscriptionStatus::default(),
		));

		System::assert_last_event(Event::MenstrualSubscription(
			crate::Event::MenstrualSubscriptionUpdated(menstrual_subscription),
		));

		let other_admin = account_key("other_admin");

		assert_ok!(MenstrualSubscription::update_key(
			Origin::signed(admin),
			AccountKeyType::AdminKey(other_admin)
		));

		System::assert_last_event(Event::MenstrualSubscription(
			crate::Event::UpdateMenstrualSubscriptionKeySuccessful(AccountKeyType::AdminKey(
				other_admin,
			)),
		));
	})
}
