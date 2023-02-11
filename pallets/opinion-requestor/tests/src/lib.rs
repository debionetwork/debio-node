mod mock;

#[cfg(test)]

mod test {
	use crate::mock::*;

	use frame_support::{
		assert_noop, assert_ok,
		sp_runtime::traits::{Hash, Keccak256},
	};
	use opinion_requestor::{
		Error, Event as OpinionRequestorEvent, OpinionRequestor as OpinionRequestorStruct,
		RequestorInfo,
	};

	#[test]
	fn request_opinion_works() {
		ExternalityBuilder::build().execute_with(|| {
			assert_ok!(ElectronicMedicalRecord::add_electronic_medical_record(
				Origin::signed(1),
				"Electronic Medical Record Title".as_bytes().to_vec(),
				"Electronic Medical Record Category".as_bytes().to_vec(),
				Vec::new(),
			));

			let result = ElectronicMedicalRecord::electronic_medical_record_by_owner_id(1);
			let electronic_medical_record_ids = result.unwrap();
			let electronical_medical_record_id = electronic_medical_record_ids[0];
			let random_electronical_medical_record_id =
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes());

			let info = RequestorInfo::new(
				b"category",
				b"description",
				&[electronical_medical_record_id, random_electronical_medical_record_id],
				&Vec::new(),
				b"myriad_url",
			);

			assert_ok!(OpinionRequestor::request_opinion(Origin::signed(1), info));

			let info = RequestorInfo::new(
				b"category",
				b"description",
				&[electronical_medical_record_id],
				&Vec::new(),
				b"myriad_url",
			);

			let requestor_ids = OpinionRequestor::opinion_requestor_by_owner(1);
			let requestor_id = requestor_ids[0];

			let requestor = OpinionRequestorStruct::new(&requestor_id, &1, &info, 0);

			assert_eq!(OpinionRequestor::opinion_requestor_by_id(requestor_id), Some(requestor));
		});
	}

	#[test]
	fn update_requestor_info_works() {
		ExternalityBuilder::build().execute_with(|| {
			let info = RequestorInfo::new(
				b"category",
				b"description",
				&Vec::new(),
				&Vec::new(),
				b"myriad_url",
			);

			assert_ok!(OpinionRequestor::request_opinion(Origin::signed(1), info));

			let requestor_ids = OpinionRequestor::opinion_requestor_by_owner(1);
			let requestor_id = requestor_ids[0];

			assert_ok!(ElectronicMedicalRecord::add_electronic_medical_record(
				Origin::signed(1),
				"Electronic Medical Record Title".as_bytes().to_vec(),
				"Electronic Medical Record Category".as_bytes().to_vec(),
				Vec::new(),
			));

			let result = ElectronicMedicalRecord::electronic_medical_record_by_owner_id(1);
			let electronic_medical_record_ids = result.unwrap();
			let electronical_medical_record_id = electronic_medical_record_ids[0];

			let updated_info = RequestorInfo::new(
				b"category",
				b"description",
				&[electronical_medical_record_id],
				&Vec::new(),
				b"myriad_url",
			);

			let requestor = OpinionRequestorStruct::new(&requestor_id, &1, &updated_info, 0);

			assert_ok!(OpinionRequestor::update_requestor_info(
				Origin::signed(1),
				requestor_id,
				updated_info
			));
			assert_eq!(OpinionRequestor::opinion_requestor_by_id(requestor_id), Some(requestor));
		});
	}

	#[test]
	fn cant_update_requestor_info_when_not_found() {
		ExternalityBuilder::build().execute_with(|| {
			let requestor_id =
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes());
			let info = RequestorInfo::new(
				b"category",
				b"description",
				&Vec::new(),
				&Vec::new(),
				b"myriad_url",
			);

			assert_noop!(
				OpinionRequestor::update_requestor_info(Origin::signed(1), requestor_id, info),
				Error::<Test>::NotFound,
			);
		});
	}

	#[test]
	fn cant_update_requestor_info_when_not_owner() {
		ExternalityBuilder::build().execute_with(|| {
			let info = RequestorInfo::new(
				b"category",
				b"description",
				&Vec::new(),
				&Vec::new(),
				b"myriad_url",
			);

			assert_ok!(OpinionRequestor::request_opinion(Origin::signed(1), info));

			let requestor_ids = OpinionRequestor::opinion_requestor_by_owner(1);
			let requestor_id = requestor_ids[0];

			assert_ok!(ElectronicMedicalRecord::add_electronic_medical_record(
				Origin::signed(1),
				"Electronic Medical Record Title".as_bytes().to_vec(),
				"Electronic Medical Record Category".as_bytes().to_vec(),
				Vec::new(),
			));

			let result = ElectronicMedicalRecord::electronic_medical_record_by_owner_id(1);
			let electronic_medical_record_ids = result.unwrap();
			let electronical_medical_record_id = electronic_medical_record_ids[0];

			let updated_info = RequestorInfo::new(
				b"category",
				b"description",
				&[electronical_medical_record_id],
				&Vec::new(),
				b"myriad_url",
			);

			assert_noop!(
				OpinionRequestor::update_requestor_info(
					Origin::signed(2),
					requestor_id,
					updated_info
				),
				Error::<Test>::Unauthorized,
			);
		});
	}

	#[test]
	fn call_event_should_work() {
		ExternalityBuilder::build().execute_with(|| {
			System::set_block_number(1);

			let info = RequestorInfo::new(
				b"category",
				b"description",
				&Vec::new(),
				&Vec::new(),
				b"myriad_url",
			);

			assert_ok!(OpinionRequestor::request_opinion(Origin::signed(1), info.clone()));

			let requestor_ids = OpinionRequestor::opinion_requestor_by_owner(1);
			let requestor_id = requestor_ids[0];
			let opinion_requestor = OpinionRequestorStruct::new(&requestor_id, &1, &info, 0);

			System::assert_last_event(Event::OpinionRequestor(
				OpinionRequestorEvent::OpinionRequested(1, opinion_requestor),
			));

			let updated_info = RequestorInfo::new(
				b"category",
				b"description",
				&Vec::new(),
				&Vec::new(),
				b"myriad_url",
			);

			assert_ok!(OpinionRequestor::update_requestor_info(
				Origin::signed(1),
				requestor_id,
				updated_info.clone()
			));

			System::assert_last_event(Event::OpinionRequestor(
				OpinionRequestorEvent::OpinionRequestorInfoUpdated(1, updated_info),
			));
		});
	}
}
