use crate::{
    mock::*, 
    Error,
    ElectronicMedicalRecord as ElectronicMedicalRecordStruct,
	ElectronicMedicalRecordFileSubmission
};
use frame_support::{
    assert_noop, assert_ok,
    sp_runtime::traits::{Hash, Keccak256}
};

#[test]
fn add_electronic_medical_record_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(
			ElectronicMedicalRecord::add_electronic_medical_record(
				Origin::signed(1),
                "DeBio EMR".as_bytes().to_vec(),
                "DeBio EMR Category".as_bytes().to_vec(),
				vec![
					ElectronicMedicalRecordFileSubmission {
						title: "DeBio EMR Document Title".as_bytes().to_vec(),
						description: "DeBio EMR Document Description".as_bytes().to_vec(),
						record_link: "DeBio EMR Link".as_bytes().to_vec()
					}
				]
			)
		);

        let emr_ids = ElectronicMedicalRecord::electronic_medical_record_by_owner_id(1).unwrap();

		assert_eq!(ElectronicMedicalRecord::electronic_medical_record_count(), Some(1));

		assert_eq!(ElectronicMedicalRecord::electronic_medical_record_count_by_owner(1), Some(1));

		let emr = ElectronicMedicalRecord::electronic_medical_record_by_id(emr_ids[0]).unwrap();

		assert_eq!(emr.id, emr_ids[0]);
		assert_eq!(emr.owner_id, 1);
		assert_eq!(emr.title, "DeBio EMR".as_bytes().to_vec());
		assert_eq!(emr.category, "DeBio EMR Category".as_bytes().to_vec());

		assert_eq!(
            ElectronicMedicalRecord::electronic_medical_record_by_id(emr_ids[0]),
            Some(ElectronicMedicalRecordStruct {
                id: emr_ids[0],
                owner_id: 1,
                title: "DeBio EMR".as_bytes().to_vec(),
                category: "DeBio EMR Category".as_bytes().to_vec(),
                files: emr.files.clone()
            })
        );

        let emr_file = ElectronicMedicalRecord::electronic_medical_record_file_by_id(emr.files[0]).unwrap();

		assert_eq!(emr_file.title, "DeBio EMR Document Title".as_bytes().to_vec());
		assert_eq!(emr_file.description, "DeBio EMR Document Description".as_bytes().to_vec());
		assert_eq!(emr_file.record_link, "DeBio EMR Link".as_bytes().to_vec());
	})
}

#[test]
fn remove_electronic_medical_record_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(
			ElectronicMedicalRecord::add_electronic_medical_record(
				Origin::signed(1),
                "DeBio EMR".as_bytes().to_vec(),
                "DeBio EMR Category".as_bytes().to_vec(),
				vec![
					ElectronicMedicalRecordFileSubmission {
						title: "DeBio EMR Document Title".as_bytes().to_vec(),
						description: "DeBio EMR Document Description".as_bytes().to_vec(),
						record_link: "DeBio EMR Link".as_bytes().to_vec()
					}
				]
			)
		);

        let emr_ids = ElectronicMedicalRecord::electronic_medical_record_by_owner_id(1).unwrap();

		assert_eq!(ElectronicMedicalRecord::electronic_medical_record_count(), Some(1));

		assert_eq!(ElectronicMedicalRecord::electronic_medical_record_count_by_owner(1), Some(1));

		let emr = ElectronicMedicalRecord::electronic_medical_record_by_id(emr_ids[0]).unwrap();

		assert_eq!(emr.id, emr_ids[0]);
		assert_eq!(emr.owner_id, 1);
		assert_eq!(emr.title, "DeBio EMR".as_bytes().to_vec());
		assert_eq!(emr.category, "DeBio EMR Category".as_bytes().to_vec());

		assert_eq!(
            ElectronicMedicalRecord::electronic_medical_record_by_id(emr_ids[0]),
            Some(ElectronicMedicalRecordStruct {
                id: emr_ids[0],
                owner_id: 1,
                title: "DeBio EMR".as_bytes().to_vec(),
                category: "DeBio EMR Category".as_bytes().to_vec(),
                files: emr.files.clone()
            })
        );

        let emr_file = ElectronicMedicalRecord::electronic_medical_record_file_by_id(emr.files[0]).unwrap();

		assert_eq!(emr_file.title, "DeBio EMR Document Title".as_bytes().to_vec());
		assert_eq!(emr_file.description, "DeBio EMR Document Description".as_bytes().to_vec());
		assert_eq!(emr_file.record_link, "DeBio EMR Link".as_bytes().to_vec());

		assert_ok!(
			ElectronicMedicalRecord::remove_electronic_medical_record(
				Origin::signed(1),
                emr_ids[0]
			)
		);

		assert_eq!(ElectronicMedicalRecord::electronic_medical_record_count(), Some(0));

		assert_eq!(ElectronicMedicalRecord::electronic_medical_record_count_by_owner(1), Some(0));
	})
}

#[test]
fn remove_electronic_medical_record_does_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			ElectronicMedicalRecord::remove_electronic_medical_record(
				Origin::signed(1),
                Keccak256::hash(
                    "0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()
                )
			),
            Error::<Test>::ElectronicMedicalRecordDoesNotExist
		);
	})
}

#[test]
fn remove_electronic_medical_record_not_electronic_medical_record_owner() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(
			ElectronicMedicalRecord::add_electronic_medical_record(
				Origin::signed(1),
                "DeBio EMR".as_bytes().to_vec(),
                "DeBio EMR Category".as_bytes().to_vec(),
				vec![
					ElectronicMedicalRecordFileSubmission {
						title: "DeBio EMR Document Title".as_bytes().to_vec(),
						description: "DeBio EMR Document Description".as_bytes().to_vec(),
						record_link: "DeBio EMR Link".as_bytes().to_vec()
					}
				]
			)
		);

        let emr_ids = ElectronicMedicalRecord::electronic_medical_record_by_owner_id(1).unwrap();

		assert_eq!(ElectronicMedicalRecord::electronic_medical_record_count(), Some(1));

		assert_eq!(ElectronicMedicalRecord::electronic_medical_record_count_by_owner(1), Some(1));

		let emr = ElectronicMedicalRecord::electronic_medical_record_by_id(emr_ids[0]).unwrap();

		assert_eq!(emr.id, emr_ids[0]);
		assert_eq!(emr.owner_id, 1);
		assert_eq!(emr.title, "DeBio EMR".as_bytes().to_vec());
		assert_eq!(emr.category, "DeBio EMR Category".as_bytes().to_vec());

		assert_eq!(
            ElectronicMedicalRecord::electronic_medical_record_by_id(emr_ids[0]),
            Some(ElectronicMedicalRecordStruct {
                id: emr_ids[0],
                owner_id: 1,
                title: "DeBio EMR".as_bytes().to_vec(),
                category: "DeBio EMR Category".as_bytes().to_vec(),
                files: emr.files.clone()
            })
        );

        let emr_file = ElectronicMedicalRecord::electronic_medical_record_file_by_id(emr.files[0]).unwrap();

		assert_eq!(emr_file.title, "DeBio EMR Document Title".as_bytes().to_vec());
		assert_eq!(emr_file.description, "DeBio EMR Document Description".as_bytes().to_vec());
		assert_eq!(emr_file.record_link, "DeBio EMR Link".as_bytes().to_vec());

		assert_noop!(
			ElectronicMedicalRecord::remove_electronic_medical_record(
				Origin::signed(2),
                emr_ids[0]
			),
            Error::<Test>::NotElectronicMedicalRecordOwner
		);
	})
}

#[test]
fn update_electronic_medical_record_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(
			ElectronicMedicalRecord::add_electronic_medical_record(
				Origin::signed(1),
                "DeBio EMR".as_bytes().to_vec(),
                "DeBio EMR Category".as_bytes().to_vec(),
				vec![
					ElectronicMedicalRecordFileSubmission {
						title: "DeBio EMR Document Title".as_bytes().to_vec(),
						description: "DeBio EMR Document Description".as_bytes().to_vec(),
						record_link: "DeBio EMR Link".as_bytes().to_vec()
					}
				]
			)
		);

        let emr_ids = ElectronicMedicalRecord::electronic_medical_record_by_owner_id(1).unwrap();

		assert_eq!(ElectronicMedicalRecord::electronic_medical_record_count(), Some(1));

		assert_eq!(ElectronicMedicalRecord::electronic_medical_record_count_by_owner(1), Some(1));

		let emr = ElectronicMedicalRecord::electronic_medical_record_by_id(emr_ids[0]).unwrap();

		assert_eq!(emr.id, emr_ids[0]);
		assert_eq!(emr.owner_id, 1);
		assert_eq!(emr.title, "DeBio EMR".as_bytes().to_vec());
		assert_eq!(emr.category, "DeBio EMR Category".as_bytes().to_vec());

		assert_eq!(
            ElectronicMedicalRecord::electronic_medical_record_by_id(emr_ids[0]),
            Some(ElectronicMedicalRecordStruct {
                id: emr_ids[0],
                owner_id: 1,
                title: "DeBio EMR".as_bytes().to_vec(),
                category: "DeBio EMR Category".as_bytes().to_vec(),
                files: emr.files.clone()
            })
        );

        let emr_file = ElectronicMedicalRecord::electronic_medical_record_file_by_id(emr.files[0]).unwrap();

		assert_eq!(emr_file.title, "DeBio EMR Document Title".as_bytes().to_vec());
		assert_eq!(emr_file.description, "DeBio EMR Document Description".as_bytes().to_vec());
		assert_eq!(emr_file.record_link, "DeBio EMR Link".as_bytes().to_vec());

		assert_ok!(
			ElectronicMedicalRecord::update_electronic_medical_record(
				Origin::signed(1),
                emr_ids[0],
                "DeBio EMR 2".as_bytes().to_vec(),
                "DeBio EMR Category 2".as_bytes().to_vec(),
				vec![
					ElectronicMedicalRecordFileSubmission {
						title: "DeBio EMR Document Title 2".as_bytes().to_vec(),
						description: "DeBio EMR Document Description 2".as_bytes().to_vec(),
						record_link: "DeBio EMR Link 2".as_bytes().to_vec()
					}
				]
			)
		);

        let emr_ids = ElectronicMedicalRecord::electronic_medical_record_by_owner_id(1).unwrap();

		assert_eq!(ElectronicMedicalRecord::electronic_medical_record_count(), Some(1));

		assert_eq!(ElectronicMedicalRecord::electronic_medical_record_count_by_owner(1), Some(1));

		let emr = ElectronicMedicalRecord::electronic_medical_record_by_id(emr_ids[0]).unwrap();

		assert_eq!(emr.id, emr_ids[0]);
		assert_eq!(emr.owner_id, 1);
		assert_eq!(emr.title, "DeBio EMR 2".as_bytes().to_vec());
		assert_eq!(emr.category, "DeBio EMR Category 2".as_bytes().to_vec());

		assert_eq!(
            ElectronicMedicalRecord::electronic_medical_record_by_id(emr_ids[0]),
            Some(ElectronicMedicalRecordStruct {
                id: emr_ids[0],
                owner_id: 1,
                title: "DeBio EMR 2".as_bytes().to_vec(),
                category: "DeBio EMR Category 2".as_bytes().to_vec(),
                files: emr.files.clone()
            })
        );

        let emr_file = ElectronicMedicalRecord::electronic_medical_record_file_by_id(emr.files[0]).unwrap();

		assert_eq!(emr_file.title, "DeBio EMR Document Title 2".as_bytes().to_vec());
		assert_eq!(emr_file.description, "DeBio EMR Document Description 2".as_bytes().to_vec());
		assert_eq!(emr_file.record_link, "DeBio EMR Link 2".as_bytes().to_vec());
	})
}

#[test]
fn update_electronic_medical_record_does_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			ElectronicMedicalRecord::update_electronic_medical_record(
				Origin::signed(1),
                Keccak256::hash(
                    "0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()
                ),
                "DeBio EMR".as_bytes().to_vec(),
                "DeBio EMR Category".as_bytes().to_vec(),
				vec![
					ElectronicMedicalRecordFileSubmission {
						title: "DeBio EMR Document Title".as_bytes().to_vec(),
						description: "DeBio EMR Document Description".as_bytes().to_vec(),
						record_link: "DeBio EMR Link".as_bytes().to_vec()
					}
				]
			),
            Error::<Test>::ElectronicMedicalRecordDoesNotExist
		);
	})
}

#[test]
fn update_electronic_medical_record_not_electronic_medical_record_owner() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(
			ElectronicMedicalRecord::add_electronic_medical_record(
				Origin::signed(1),
                "DeBio EMR".as_bytes().to_vec(),
                "DeBio EMR Category".as_bytes().to_vec(),
				vec![
					ElectronicMedicalRecordFileSubmission {
						title: "DeBio EMR Document Title".as_bytes().to_vec(),
						description: "DeBio EMR Document Description".as_bytes().to_vec(),
						record_link: "DeBio EMR Link".as_bytes().to_vec()
					}
				]
			)
		);

        let emr_ids = ElectronicMedicalRecord::electronic_medical_record_by_owner_id(1).unwrap();

		assert_eq!(ElectronicMedicalRecord::electronic_medical_record_count(), Some(1));

		assert_eq!(ElectronicMedicalRecord::electronic_medical_record_count_by_owner(1), Some(1));

		let emr = ElectronicMedicalRecord::electronic_medical_record_by_id(emr_ids[0]).unwrap();

		assert_eq!(emr.id, emr_ids[0]);
		assert_eq!(emr.owner_id, 1);
		assert_eq!(emr.title, "DeBio EMR".as_bytes().to_vec());
		assert_eq!(emr.category, "DeBio EMR Category".as_bytes().to_vec());

		assert_eq!(
            ElectronicMedicalRecord::electronic_medical_record_by_id(emr_ids[0]),
            Some(ElectronicMedicalRecordStruct {
                id: emr_ids[0],
                owner_id: 1,
                title: "DeBio EMR".as_bytes().to_vec(),
                category: "DeBio EMR Category".as_bytes().to_vec(),
                files: emr.files.clone()
            })
        );

        let emr_file = ElectronicMedicalRecord::electronic_medical_record_file_by_id(emr.files[0]).unwrap();

		assert_eq!(emr_file.title, "DeBio EMR Document Title".as_bytes().to_vec());
		assert_eq!(emr_file.description, "DeBio EMR Document Description".as_bytes().to_vec());
		assert_eq!(emr_file.record_link, "DeBio EMR Link".as_bytes().to_vec());

		assert_noop!(
			ElectronicMedicalRecord::update_electronic_medical_record(
				Origin::signed(2),
                emr_ids[0],
                "DeBio EMR".as_bytes().to_vec(),
                "DeBio EMR Category".as_bytes().to_vec(),
				vec![
					ElectronicMedicalRecordFileSubmission {
						title: "DeBio EMR Document Title".as_bytes().to_vec(),
						description: "DeBio EMR Document Description".as_bytes().to_vec(),
						record_link: "DeBio EMR Link".as_bytes().to_vec()
					}
				]
			),
            Error::<Test>::NotElectronicMedicalRecordOwner
		);
	})
}