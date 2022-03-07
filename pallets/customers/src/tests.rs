use crate::{mock::*, Customer, CustomerInfo, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn register_customer_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Customers::register_customer(
			Origin::signed(1),
			CustomerInfo {
				name: "DeBio Customer".as_bytes().to_vec(),
				email: "DeBio Email".as_bytes().to_vec(),
				address: "DeBio Address".as_bytes().to_vec(),
				latitude: Some("DeBio Latitude".as_bytes().to_vec()),
				longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
				profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
			}
		));

		assert_eq!(
			Customers::customer_by_account_id(1),
			Some(Customer {
				account_id: 1,
				info: CustomerInfo {
					name: "DeBio Customer".as_bytes().to_vec(),
					email: "DeBio Email".as_bytes().to_vec(),
					address: "DeBio Address".as_bytes().to_vec(),
					latitude: Some("DeBio Latitude".as_bytes().to_vec()),
					longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
					profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				}
			})
		);

		assert_ok!(Customers::register_customer(
			Origin::signed(2),
			CustomerInfo {
				name: "DeBio Customer".as_bytes().to_vec(),
				email: "DeBio Email".as_bytes().to_vec(),
				address: "DeBio Address".as_bytes().to_vec(),
				latitude: Some("DeBio Latitude".as_bytes().to_vec()),
				longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
				profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
			}
		));

		assert_eq!(Customers::customer_count(), Some(2),);
	})
}

#[test]
fn update_customer_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Customers::register_customer(
			Origin::signed(1),
			CustomerInfo {
				name: "DeBio Customer".as_bytes().to_vec(),
				email: "DeBio Email".as_bytes().to_vec(),
				address: "DeBio Address".as_bytes().to_vec(),
				latitude: Some("DeBio Latitude".as_bytes().to_vec()),
				longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
				profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
			}
		));

		assert_ok!(Customers::update_customer(
			Origin::signed(1),
			CustomerInfo {
				name: "Abdul Hakim".as_bytes().to_vec(),
				email: "DeBio Email".as_bytes().to_vec(),
				address: "DeBio Address".as_bytes().to_vec(),
				latitude: Some("DeBio Latitude".as_bytes().to_vec()),
				longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
				profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
			}
		));

		assert_eq!(
			Customers::customer_by_account_id(1),
			Some(Customer {
				account_id: 1,
				info: CustomerInfo {
					name: "Abdul Hakim".as_bytes().to_vec(),
					email: "DeBio Email".as_bytes().to_vec(),
					address: "DeBio Address".as_bytes().to_vec(),
					latitude: Some("DeBio Latitude".as_bytes().to_vec()),
					longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
					profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				}
			})
		);

		assert_ok!(Customers::update_customer(
			Origin::signed(1),
			CustomerInfo {
				name: "Abdul Hakim".as_bytes().to_vec(),
				email: "DeBio Email".as_bytes().to_vec(),
				address: "DeBio Address".as_bytes().to_vec(),
				latitude: Some("DeBio Latitude".as_bytes().to_vec()),
				longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
				profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
			}
		));
	})
}

#[test]
fn deregister_customer_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Customers::register_customer(
			Origin::signed(1),
			CustomerInfo {
				name: "DeBio Customer".as_bytes().to_vec(),
				email: "DeBio Email".as_bytes().to_vec(),
				address: "DeBio Address".as_bytes().to_vec(),
				latitude: Some("DeBio Latitude".as_bytes().to_vec()),
				longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
				profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
			}
		));

		assert_ok!(Customers::deregister_customer(Origin::signed(1),));

		assert_eq!(Customers::customer_by_account_id(1), None);

		assert_eq!(Customers::customer_count(), Some(0),);
	})
}

#[test]
fn cant_register_customer_when_already_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Customers::register_customer(
			Origin::signed(1),
			CustomerInfo {
				name: "DeBio Customer".as_bytes().to_vec(),
				email: "DeBio Email".as_bytes().to_vec(),
				address: "DeBio Address".as_bytes().to_vec(),
				latitude: Some("DeBio Latitude".as_bytes().to_vec()),
				longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
				profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
			}
		));

		assert_noop!(
			Customers::register_customer(
				Origin::signed(1),
				CustomerInfo {
					name: "DeBio Customer".as_bytes().to_vec(),
					email: "DeBio Email".as_bytes().to_vec(),
					address: "DeBio Address".as_bytes().to_vec(),
					latitude: Some("DeBio Latitude".as_bytes().to_vec()),
					longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
					profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				}
			),
			Error::<Test>::CustomerAlreadyRegistered
		);
	})
}

#[test]
fn cant_update_and_deregister_customer_when_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			Customers::update_customer(
				Origin::signed(1),
				CustomerInfo {
					name: "DeBio Customer".as_bytes().to_vec(),
					email: "DeBio Email".as_bytes().to_vec(),
					address: "DeBio Address".as_bytes().to_vec(),
					latitude: Some("DeBio Latitude".as_bytes().to_vec()),
					longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
					profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				}
			),
			Error::<Test>::CustomerDoesNotExist
		);

		assert_noop!(
			Customers::deregister_customer(Origin::signed(1)),
			Error::<Test>::CustomerDoesNotExist
		);
	})
}

#[test]
fn call_event_should_work() {
	ExternalityBuilder::build().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(Customers::register_customer(
			Origin::signed(1),
			CustomerInfo {
				name: "DeBio Customer".as_bytes().to_vec(),
				email: "DeBio Email".as_bytes().to_vec(),
				address: "DeBio Address".as_bytes().to_vec(),
				latitude: Some("DeBio Latitude".as_bytes().to_vec()),
				longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
				profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
			}
		));

		System::assert_last_event(Event::Customers(crate::Event::CustomerRegistered(
			Customer {
				account_id: 1,
				info: CustomerInfo {
					name: "DeBio Customer".as_bytes().to_vec(),
					email: "DeBio Email".as_bytes().to_vec(),
					address: "DeBio Address".as_bytes().to_vec(),
					latitude: Some("DeBio Latitude".as_bytes().to_vec()),
					longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
					profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				},
			},
			1,
		)));

		assert_ok!(Customers::update_customer(
			Origin::signed(1),
			CustomerInfo {
				name: "Abdul Hakim".as_bytes().to_vec(),
				email: "DeBio Email".as_bytes().to_vec(),
				address: "DeBio Address".as_bytes().to_vec(),
				latitude: Some("DeBio Latitude".as_bytes().to_vec()),
				longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
				profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
			}
		));

		System::assert_last_event(Event::Customers(crate::Event::CustomerUpdated(
			Customer {
				account_id: 1,
				info: CustomerInfo {
					name: "Abdul Hakim".as_bytes().to_vec(),
					email: "DeBio Email".as_bytes().to_vec(),
					address: "DeBio Address".as_bytes().to_vec(),
					latitude: Some("DeBio Latitude".as_bytes().to_vec()),
					longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
					profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				},
			},
			1,
		)));

		assert_ok!(Customers::deregister_customer(Origin::signed(1)));

		System::assert_last_event(Event::Customers(crate::Event::CustomerDeleted(
			Customer {
				account_id: 1,
				info: CustomerInfo {
					name: "Abdul Hakim".as_bytes().to_vec(),
					email: "DeBio Email".as_bytes().to_vec(),
					address: "DeBio Address".as_bytes().to_vec(),
					latitude: Some("DeBio Latitude".as_bytes().to_vec()),
					longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
					profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
				},
			},
			1,
		)));
	})
}
