mod mock;

#[cfg(test)]

mod test {
	use std::vec;

	use crate::mock::*;

	use frame_support::{
		assert_noop, assert_ok,
		sp_runtime::traits::{Hash, Keccak256},
	};
	use opinion::{
		Error, Event as OpinionEvent, Opinion as OpinionStruct, OpinionAdminKey, OpinionInfo,
		Status,
	};
	use opinion_requestor::RequestorInfo;
	use primitives_price_and_currency::CurrencyType;

	#[test]
	fn add_opinion_works() {
		ExternalityBuilder::build().execute_with(|| {
			let admin = 1;
			let doctor = 2;
			let customer = 3;

			let info = RequestorInfo::new(
				b"category",
				b"description",
				&Vec::new(),
				&Vec::new(),
				b"myriad_url",
			);

			assert_ok!(OpinionRequestor::request_opinion(RuntimeOrigin::signed(customer), info));

			let requestor_ids = OpinionRequestor::opinion_requestor_by_owner(customer);
			let requestor_id = requestor_ids[0];

			OpinionAdminKey::<Test>::put(admin);

			let info = OpinionInfo::new(
				b"description".to_vec(),
				b"myriad_url".to_vec(),
				None,
				CurrencyType::DBIO,
				1000,
			);

			assert_ok!(Opinion::create(
				RuntimeOrigin::signed(admin),
				requestor_id,
				doctor,
				info.clone()
			));

			let opinion_ids = Opinion::opinion_by_owner(doctor);
			let opinion_id = opinion_ids[0];

			let opinion = OpinionStruct::new(&opinion_id, &requestor_id, &doctor, &info, 0);

			let opinion_requestor =
				OpinionRequestor::opinion_requestor_by_id(requestor_id).unwrap();

			assert_eq!(Opinion::opinion_by_id(opinion_id), Some(opinion));
			assert_eq!(Opinion::opinion_count(), 1);
			assert_eq!(Opinion::opinion_count_by_owner(doctor), 1);
			assert_eq!(opinion_requestor.opinion_ids(), vec![opinion_id]);
		});
	}

	#[test]
	fn update_opinion_info_works() {
		ExternalityBuilder::build().execute_with(|| {
			let admin = 1;
			let doctor = 2;
			let customer = 3;

			let info = RequestorInfo::new(
				b"category",
				b"description",
				&Vec::new(),
				&Vec::new(),
				b"myriad_url",
			);

			assert_ok!(OpinionRequestor::request_opinion(RuntimeOrigin::signed(customer), info));

			let requestor_ids = OpinionRequestor::opinion_requestor_by_owner(customer);
			let requestor_id = requestor_ids[0];

			OpinionAdminKey::<Test>::put(admin);

			let info = OpinionInfo::new(
				b"description".to_vec(),
				b"myriad_url".to_vec(),
				None,
				CurrencyType::DBIO,
				1000,
			);

			assert_ok!(Opinion::create(RuntimeOrigin::signed(admin), requestor_id, doctor, info));

			let opinion_ids = Opinion::opinion_by_owner(doctor);
			let opinion_id = opinion_ids[0];

			let updated_info = OpinionInfo::new(
				b"description".to_vec(),
				b"myriad_url".to_vec(),
				Some(1),
				CurrencyType::USDT,
				1000,
			);

			assert_ok!(Opinion::update(
				RuntimeOrigin::signed(admin),
				opinion_id,
				updated_info.clone()
			));

			let opinion = OpinionStruct::new(&opinion_id, &requestor_id, &doctor, &updated_info, 0);

			assert_eq!(Opinion::opinion_by_id(opinion_id), Some(opinion));
		});
	}

	#[test]
	fn remove_opinion_works() {
		ExternalityBuilder::build().execute_with(|| {
			let admin = 1;
			let doctor = 2;
			let customer = 3;

			let info = RequestorInfo::new(
				b"category",
				b"description",
				&Vec::new(),
				&Vec::new(),
				b"myriad_url",
			);

			assert_ok!(OpinionRequestor::request_opinion(RuntimeOrigin::signed(customer), info));

			let requestor_ids = OpinionRequestor::opinion_requestor_by_owner(customer);
			let requestor_id = requestor_ids[0];

			OpinionAdminKey::<Test>::put(admin);

			let info = OpinionInfo::new(
				b"description".to_vec(),
				b"myriad_url".to_vec(),
				None,
				CurrencyType::DBIO,
				1000,
			);

			assert_ok!(Opinion::create(RuntimeOrigin::signed(admin), requestor_id, doctor, info));

			let opinion_ids = Opinion::opinion_by_owner(doctor);
			let opinion_id = opinion_ids[0];

			assert_ok!(Opinion::delete(RuntimeOrigin::signed(admin), opinion_id));

			let opinion_requestor =
				OpinionRequestor::opinion_requestor_by_id(requestor_id).unwrap();

			assert_eq!(Opinion::opinion_by_id(opinion_id), None);
			assert_eq!(Opinion::opinion_count(), 0);
			assert_eq!(Opinion::opinion_count_by_owner(doctor), 0);
			assert_eq!(opinion_requestor.opinion_ids(), Vec::new());
		});
	}

	#[test]
	fn update_opinion_status_works() {
		ExternalityBuilder::build().execute_with(|| {
			let admin = 1;
			let doctor = 2;
			let customer = 3;

			let info = RequestorInfo::new(
				b"category",
				b"description",
				&Vec::new(),
				&Vec::new(),
				b"myriad_url",
			);

			assert_ok!(OpinionRequestor::request_opinion(RuntimeOrigin::signed(customer), info));

			let requestor_ids = OpinionRequestor::opinion_requestor_by_owner(customer);
			let requestor_id = requestor_ids[0];

			OpinionAdminKey::<Test>::put(admin);

			let info = OpinionInfo::new(
				b"description".to_vec(),
				b"myriad_url".to_vec(),
				None,
				CurrencyType::DBIO,
				1000,
			);

			assert_ok!(Opinion::create(
				RuntimeOrigin::signed(admin),
				requestor_id,
				doctor,
				info.clone()
			));

			let opinion_ids = Opinion::opinion_by_owner(doctor);
			let opinion_id = opinion_ids[0];

			let mut opinion = OpinionStruct::new(&opinion_id, &requestor_id, &doctor, &info, 0);

			assert_ok!(Opinion::update_status(
				RuntimeOrigin::signed(admin),
				opinion_id,
				Status::Paid
			));

			opinion.update_status(&Status::Paid);

			assert_eq!(Opinion::opinion_by_id(opinion_id), Some(opinion));
		});
	}

	#[test]
	fn update_admin_key_works() {
		ExternalityBuilder::build().execute_with(|| {
			let admin = 1;
			let other_admin = 2;

			OpinionAdminKey::<Test>::put(admin);

			assert_ok!(Opinion::update_admin_key(RuntimeOrigin::signed(admin), other_admin));

			assert_eq!(Opinion::admin_key(), Some(other_admin));
		});
	}

	#[test]
	fn sudo_update_admin_key_works() {
		ExternalityBuilder::build().execute_with(|| {
			let admin = 1;

			assert_ok!(Opinion::sudo_update_admin_key(RuntimeOrigin::root(), admin));

			assert_eq!(Opinion::admin_key(), Some(admin));
		});
	}

	#[test]
	fn cant_add_opinion_when_not_admin() {
		ExternalityBuilder::build().execute_with(|| {
			let admin = 1;
			let doctor = 2;
			let customer = 3;

			let info = RequestorInfo::new(
				b"category",
				b"description",
				&Vec::new(),
				&Vec::new(),
				b"myriad_url",
			);

			assert_ok!(OpinionRequestor::request_opinion(RuntimeOrigin::signed(customer), info));

			let requestor_ids = OpinionRequestor::opinion_requestor_by_owner(customer);
			let requestor_id = requestor_ids[0];

			let info = OpinionInfo::new(
				b"description".to_vec(),
				b"myriad_url".to_vec(),
				None,
				CurrencyType::DBIO,
				1000,
			);

			assert_noop!(
				Opinion::create(RuntimeOrigin::signed(admin), requestor_id, doctor, info),
				Error::<Test>::Unauthorized,
			);
		});
	}

	#[test]
	fn cant_add_opinion_when_not_found() {
		ExternalityBuilder::build().execute_with(|| {
			let doctor = 1;
			let admin = 2;

			let requestor_id =
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes());
			let info = OpinionInfo::new(
				b"description".to_vec(),
				b"myriad_url".to_vec(),
				None,
				CurrencyType::DBIO,
				1000,
			);

			OpinionAdminKey::<Test>::put(admin);

			assert_noop!(
				Opinion::create(RuntimeOrigin::signed(admin), requestor_id, doctor, info),
				Error::<Test>::NotFound,
			);
		});
	}

	#[test]
	fn cant_add_opinion_when_asset_not_found() {
		ExternalityBuilder::build().execute_with(|| {
			let admin = 1;
			let doctor = 2;
			let customer = 3;

			let info = RequestorInfo::new(
				b"category",
				b"description",
				&Vec::new(),
				&Vec::new(),
				b"myriad_url",
			);

			assert_ok!(OpinionRequestor::request_opinion(RuntimeOrigin::signed(customer), info));

			let requestor_ids = OpinionRequestor::opinion_requestor_by_owner(customer);
			let requestor_id = requestor_ids[0];

			OpinionAdminKey::<Test>::put(admin);

			let info = OpinionInfo::new(
				b"description".to_vec(),
				b"myriad_url".to_vec(),
				None,
				CurrencyType::USDT,
				1000,
			);

			assert_noop!(
				Opinion::create(RuntimeOrigin::signed(admin), requestor_id, doctor, info),
				Error::<Test>::NotFound,
			);
		});
	}

	#[test]
	fn cant_update_opinion_info_when_not_admin() {
		ExternalityBuilder::build().execute_with(|| {
			let admin = 1;
			let doctor = 2;
			let customer = 3;
			let other_admin = 4;

			let info = RequestorInfo::new(
				b"category",
				b"description",
				&Vec::new(),
				&Vec::new(),
				b"myriad_url",
			);

			assert_ok!(OpinionRequestor::request_opinion(RuntimeOrigin::signed(customer), info));

			let requestor_ids = OpinionRequestor::opinion_requestor_by_owner(customer);
			let requestor_id = requestor_ids[0];

			OpinionAdminKey::<Test>::put(admin);

			let info = OpinionInfo::new(
				b"description".to_vec(),
				b"myriad_url".to_vec(),
				None,
				CurrencyType::DBIO,
				1000,
			);

			assert_ok!(Opinion::create(RuntimeOrigin::signed(admin), requestor_id, doctor, info));

			let opinion_ids = Opinion::opinion_by_owner(doctor);
			let opinion_id = opinion_ids[0];

			let updated_info = OpinionInfo::new(
				b"description".to_vec(),
				b"myriad_url".to_vec(),
				Some(1),
				CurrencyType::USDT,
				1000,
			);

			assert_noop!(
				Opinion::update(RuntimeOrigin::signed(other_admin), opinion_id, updated_info),
				Error::<Test>::Unauthorized,
			);
		});
	}

	#[test]
	fn cant_update_opinion_info_when_not_found() {
		ExternalityBuilder::build().execute_with(|| {
			let admin = 2;

			let opinion_id =
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes());
			let info = OpinionInfo::new(
				b"description".to_vec(),
				b"myriad_url".to_vec(),
				None,
				CurrencyType::DBIO,
				1000,
			);

			OpinionAdminKey::<Test>::put(admin);

			assert_noop!(
				Opinion::update(RuntimeOrigin::signed(admin), opinion_id, info),
				Error::<Test>::NotFound,
			);
		});
	}

	#[test]
	fn cant_update_opinion_info_when_asset_not_found() {
		ExternalityBuilder::build().execute_with(|| {
			let admin = 1;
			let doctor = 2;
			let customer = 3;

			let info = RequestorInfo::new(
				b"category",
				b"description",
				&Vec::new(),
				&Vec::new(),
				b"myriad_url",
			);

			assert_ok!(OpinionRequestor::request_opinion(RuntimeOrigin::signed(customer), info));

			let requestor_ids = OpinionRequestor::opinion_requestor_by_owner(customer);
			let requestor_id = requestor_ids[0];

			OpinionAdminKey::<Test>::put(admin);

			let info = OpinionInfo::new(
				b"description".to_vec(),
				b"myriad_url".to_vec(),
				None,
				CurrencyType::DBIO,
				1000,
			);

			assert_ok!(Opinion::create(RuntimeOrigin::signed(admin), requestor_id, doctor, info));

			let opinion_ids = Opinion::opinion_by_owner(doctor);
			let opinion_id = opinion_ids[0];

			let updated_info = OpinionInfo::new(
				b"description".to_vec(),
				b"myriad_url".to_vec(),
				None,
				CurrencyType::USDT,
				1000,
			);

			assert_noop!(
				Opinion::update(RuntimeOrigin::signed(admin), opinion_id, updated_info),
				Error::<Test>::NotFound,
			);
		});
	}

	#[test]
	fn cant_remove_opinion_when_not_admin() {
		ExternalityBuilder::build().execute_with(|| {
			let admin = 1;

			let opinion_id =
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes());

			assert_noop!(
				Opinion::delete(RuntimeOrigin::signed(admin), opinion_id),
				Error::<Test>::Unauthorized,
			);
		});
	}

	#[test]
	fn cant_remove_opinion_when_not_found() {
		ExternalityBuilder::build().execute_with(|| {
			let admin = 1;

			OpinionAdminKey::<Test>::put(admin);

			let opinion_id =
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes());

			assert_noop!(
				Opinion::delete(RuntimeOrigin::signed(admin), opinion_id),
				Error::<Test>::NotFound,
			);
		});
	}

	#[test]
	fn cant_update_opinion_status_when_not_admin() {
		ExternalityBuilder::build().execute_with(|| {
			let admin = 1;
			let doctor = 2;
			let customer = 3;

			let info = RequestorInfo::new(
				b"category",
				b"description",
				&Vec::new(),
				&Vec::new(),
				b"myriad_url",
			);

			assert_ok!(OpinionRequestor::request_opinion(RuntimeOrigin::signed(customer), info));

			let requestor_ids = OpinionRequestor::opinion_requestor_by_owner(customer);
			let requestor_id = requestor_ids[0];

			let info = OpinionInfo::new(
				b"description".to_vec(),
				b"myriad_url".to_vec(),
				None,
				CurrencyType::DBIO,
				1000,
			);

			OpinionAdminKey::<Test>::put(admin);

			assert_ok!(Opinion::create(RuntimeOrigin::signed(admin), requestor_id, doctor, info));

			let opinion_ids = Opinion::opinion_by_owner(doctor);
			let opinion_id = opinion_ids[0];
			let other_admin = 4;

			assert_noop!(
				Opinion::update_status(
					RuntimeOrigin::signed(other_admin),
					opinion_id,
					Status::Paid,
				),
				Error::<Test>::Unauthorized,
			);
		});
	}

	#[test]
	fn cant_update_opinion_status_when_not_found() {
		ExternalityBuilder::build().execute_with(|| {
			let admin = 1;
			let opinion_id =
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes());

			OpinionAdminKey::<Test>::put(admin);

			assert_noop!(
				Opinion::update_status(RuntimeOrigin::signed(admin), opinion_id, Status::Paid,),
				Error::<Test>::NotFound,
			);
		});
	}

	#[test]
	fn cant_update_admin_key_when_not_admin() {
		ExternalityBuilder::build().execute_with(|| {
			let admin = 1;
			let other_admin = 2;

			assert_noop!(
				Opinion::update_admin_key(RuntimeOrigin::signed(admin), other_admin),
				Error::<Test>::Unauthorized,
			);
		});
	}

	#[test]
	fn call_event_should_work() {
		ExternalityBuilder::build().execute_with(|| {
			System::set_block_number(1);

			let admin = 1;
			let doctor = 2;
			let customer = 3;

			OpinionAdminKey::<Test>::put(admin);

			let info = RequestorInfo::new(
				b"category",
				b"description",
				&Vec::new(),
				&Vec::new(),
				b"myriad_url",
			);

			assert_ok!(OpinionRequestor::request_opinion(RuntimeOrigin::signed(customer), info));

			let requestor_ids = OpinionRequestor::opinion_requestor_by_owner(customer);
			let requestor_id = requestor_ids[0];

			let info = OpinionInfo::new(
				b"description".to_vec(),
				b"myriad_url".to_vec(),
				None,
				CurrencyType::DBIO,
				1000,
			);

			assert_ok!(Opinion::create(
				RuntimeOrigin::signed(admin),
				requestor_id,
				doctor,
				info.clone()
			));

			let opinion_ids = Opinion::opinion_by_owner(doctor);
			let opinion_id = opinion_ids[0];

			let opinion = OpinionStruct::new(&opinion_id, &requestor_id, &doctor, &info, 0);

			System::assert_last_event(RuntimeEvent::Opinion(OpinionEvent::OpinionAdded(
				admin, opinion,
			)));

			let updated_info = OpinionInfo::new(
				b"description".to_vec(),
				b"myriad_url".to_vec(),
				Some(1),
				CurrencyType::USDT,
				1000,
			);

			let opinion = OpinionStruct::new(&opinion_id, &requestor_id, &doctor, &updated_info, 0);

			assert_ok!(Opinion::update(RuntimeOrigin::signed(admin), opinion_id, updated_info));

			System::assert_last_event(RuntimeEvent::Opinion(OpinionEvent::OpinionUpdated(
				admin, opinion,
			)));

			assert_ok!(Opinion::update_status(
				RuntimeOrigin::signed(admin),
				opinion_id,
				Status::Paid
			));

			System::assert_last_event(RuntimeEvent::Opinion(OpinionEvent::OpinionStatusUpdated(
				admin,
				opinion_id,
				Status::Paid,
			)));

			assert_ok!(Opinion::delete(RuntimeOrigin::signed(admin), opinion_id));

			System::assert_last_event(RuntimeEvent::Opinion(OpinionEvent::OpinionRemoved(
				admin, opinion_id,
			)));

			let new_admin = 4;

			assert_ok!(Opinion::update_admin_key(RuntimeOrigin::signed(admin), new_admin));

			System::assert_last_event(RuntimeEvent::Opinion(OpinionEvent::AdminKeyUpdated(
				new_admin,
			)))
		});
	}
}
