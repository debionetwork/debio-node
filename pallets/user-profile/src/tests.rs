use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

use crate::{AdminKey};

#[test]
fn set_eth_address_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(UserProfile::set_eth_address(
			Origin::signed(1),
			EthereumAddress([b'X';20])
		));


		assert_eq!(
			UserProfile::eth_address_by_account_id(1),
			Some(EthereumAddress([b'X';20]))
		);

		assert_eq!(
			UserProfile::account_id_by_eth_address(EthereumAddress([b'X';20])),
			Some(1)
		);
	});
}

#[test]
fn admin_set_eth_address() {
	ExternalityBuilder::build().execute_with(|| {
		AdminKey::<Test>::put(1);

		assert_ok!(UserProfile::admin_set_eth_address(
			Origin::signed(1),
			2,
			EthereumAddress([b'X';20])
		));

		assert_eq!(
			UserProfile::eth_address_by_account_id(2),
			Some(EthereumAddress([b'X';20]))
		);

		assert_eq!(
			UserProfile::account_id_by_eth_address(EthereumAddress([b'X';20])),
			Some(2)
		);
	})
}

#[test]
fn cant_set_eth_address_when_not_admin() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			UserProfile::admin_set_eth_address(
				Origin::signed(1),
				2,
				EthereumAddress([b'X';20])
			),
			Error::<Test>::Unauthorized
		);
	})
}

#[test]
fn call_event_should_work() {
	ExternalityBuilder::build().execute_with(|| {
		System::set_block_number(1);
		AdminKey::<Test>::put(1);

		assert_ok!(UserProfile::admin_set_eth_address(
			Origin::signed(1),
			2,
			EthereumAddress([b'X';20])
		));

		System::assert_last_event(Event::UserProfile(crate::Event::EthAddressSet(
			EthereumAddress([b'X';20]),
			2
		)));
	})
}
