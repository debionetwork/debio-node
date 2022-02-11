use crate::{mock::*, Error, PalletAccount, RewarderKey};
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;

#[test]
fn reward_funds_works() {
	ExternalityBuilder::build().execute_with(|| {
		System::set_block_number(1);

		assert_eq!(RewarderKey::<Test>::put(1), ());

		assert_ok!(Balances::set_balance(
			RawOrigin::Root.into(),
			PalletAccount::<Test>::get(),
			100,
			0
		));
		assert_ok!(Rewards::reward_funds(Origin::signed(1), 2, 10));

		System::assert_last_event(Event::Rewards(crate::Event::RewardFunds(2, 10, 1)));

		assert_eq!(Balances::free_balance(PalletAccount::<Test>::get()), 90);
	})
}

#[test]
fn reward_funds_bad_signature() {
	ExternalityBuilder::build().execute_with(|| {
		System::set_block_number(1);

		assert_eq!(RewarderKey::<Test>::put(1), ());

		assert_ok!(Balances::set_balance(
			RawOrigin::Root.into(),
			PalletAccount::<Test>::get(),
			1,
			0
		));
		assert_noop!(Rewards::reward_funds(Origin::signed(1), 2, 2), Error::<Test>::BadSignature);
	})
}

#[test]
fn cant_reward_funds_when_not_admin() {
	ExternalityBuilder::build().execute_with(|| {
		System::set_block_number(1);

		assert_eq!(RewarderKey::<Test>::put(1), ());

		assert_noop!(Rewards::reward_funds(Origin::signed(2), 2, 1), Error::<Test>::Unauthorized);
	})
}

#[test]
fn update_admin_key_works() {
	ExternalityBuilder::build().execute_with(|| {
		RewarderKey::<Test>::put(2);

		assert_eq!(
			Rewards::admin_key(),
			2
		);

		assert_ok!(Rewards::update_admin_key(
			Origin::signed(2),
			1,
		));

		assert_eq!(
			Rewards::admin_key(),
			1
		);
	})
}
