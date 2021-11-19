use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

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
