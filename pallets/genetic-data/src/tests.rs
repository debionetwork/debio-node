use crate::{mock::*, Error};
use frame_support::{
	assert_noop, assert_ok,
	sp_runtime::traits::{Hash, Keccak256},
};

#[test]
fn add_genetic_data_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(GeneticData::add_genetic_data(
			Origin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec()
		));

		let genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_eq!(GeneticData::genetic_data_count(), Some(1));

		assert_eq!(GeneticData::genetic_data_count_by_owner(1), Some(1));

		let genetic_data = GeneticData::genetic_data_by_id(genetic_data_ids[0]).unwrap();

		assert_eq!(genetic_data.id, genetic_data_ids[0]);
		assert_eq!(genetic_data.owner_id, 1);
		assert_eq!(genetic_data.title, "DeBio Genetic Data".as_bytes().to_vec());
		assert_eq!(
			genetic_data.description,
			"DeBio Genetic Data Document Description".as_bytes().to_vec()
		);
		assert_eq!(genetic_data.report_link, "DeBio Genetic Data Link".as_bytes().to_vec());
	})
}

#[test]
fn remove_genetic_data_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(GeneticData::add_genetic_data(
			Origin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec()
		));

		let genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_eq!(GeneticData::genetic_data_count(), Some(1));

		assert_eq!(GeneticData::genetic_data_count_by_owner(1), Some(1));

		let genetic_data = GeneticData::genetic_data_by_id(genetic_data_ids[0]).unwrap();

		assert_eq!(genetic_data.id, genetic_data_ids[0]);
		assert_eq!(genetic_data.owner_id, 1);
		assert_eq!(genetic_data.title, "DeBio Genetic Data".as_bytes().to_vec());
		assert_eq!(
			genetic_data.description,
			"DeBio Genetic Data Document Description".as_bytes().to_vec()
		);
		assert_eq!(genetic_data.report_link, "DeBio Genetic Data Link".as_bytes().to_vec());

		assert_ok!(GeneticData::remove_genetic_data(Origin::signed(1), genetic_data_ids[0]));

		assert_eq!(GeneticData::genetic_data_count(), Some(0));

		assert_eq!(GeneticData::genetic_data_count_by_owner(1), Some(0));
	})
}

#[test]
fn remove_genetic_data_does_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			GeneticData::remove_genetic_data(
				Origin::signed(1),
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes())
			),
			Error::<Test>::GeneticDataDoesNotExist
		);
	})
}

#[test]
fn remove_genetic_data_not_genetic_data_owner() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(GeneticData::add_genetic_data(
			Origin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec()
		));

		let genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_eq!(GeneticData::genetic_data_count(), Some(1));

		assert_eq!(GeneticData::genetic_data_count_by_owner(1), Some(1));

		let genetic_data = GeneticData::genetic_data_by_id(genetic_data_ids[0]).unwrap();

		assert_eq!(genetic_data.id, genetic_data_ids[0]);
		assert_eq!(genetic_data.owner_id, 1);
		assert_eq!(genetic_data.title, "DeBio Genetic Data".as_bytes().to_vec());
		assert_eq!(
			genetic_data.description,
			"DeBio Genetic Data Document Description".as_bytes().to_vec()
		);
		assert_eq!(genetic_data.report_link, "DeBio Genetic Data Link".as_bytes().to_vec());

		assert_noop!(
			GeneticData::remove_genetic_data(Origin::signed(2), genetic_data_ids[0]),
			Error::<Test>::NotGeneticDataOwner
		);
	})
}

#[test]
fn update_genetic_data_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(GeneticData::add_genetic_data(
			Origin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec()
		));

		let genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_eq!(GeneticData::genetic_data_count(), Some(1));

		assert_eq!(GeneticData::genetic_data_count_by_owner(1), Some(1));

		let genetic_data = GeneticData::genetic_data_by_id(genetic_data_ids[0]).unwrap();

		assert_eq!(genetic_data.id, genetic_data_ids[0]);
		assert_eq!(genetic_data.owner_id, 1);
		assert_eq!(genetic_data.title, "DeBio Genetic Data".as_bytes().to_vec());
		assert_eq!(
			genetic_data.description,
			"DeBio Genetic Data Document Description".as_bytes().to_vec()
		);
		assert_eq!(genetic_data.report_link, "DeBio Genetic Data Link".as_bytes().to_vec());

		assert_ok!(GeneticData::update_genetic_data(
			Origin::signed(1),
			genetic_data_ids[0],
			"DeBio Genetic Data 2".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description 2".as_bytes().to_vec(),
			"DeBio Genetic Data Link 2".as_bytes().to_vec()
		));

		let genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_eq!(GeneticData::genetic_data_count(), Some(1));

		assert_eq!(GeneticData::genetic_data_count_by_owner(1), Some(1));

		let genetic_data = GeneticData::genetic_data_by_id(genetic_data_ids[0]).unwrap();

		assert_eq!(genetic_data.id, genetic_data_ids[0]);
		assert_eq!(genetic_data.owner_id, 1);
		assert_eq!(genetic_data.title, "DeBio Genetic Data 2".as_bytes().to_vec());
		assert_eq!(
			genetic_data.description,
			"DeBio Genetic Data Document Description 2".as_bytes().to_vec()
		);
		assert_eq!(genetic_data.report_link, "DeBio Genetic Data Link 2".as_bytes().to_vec());
	})
}

#[test]
fn update_genetic_data_does_not_exist() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			GeneticData::update_genetic_data(
				Origin::signed(1),
				Keccak256::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
				"DeBio Genetic Data".as_bytes().to_vec(),
				"DeBio Genetic Data Document Description".as_bytes().to_vec(),
				"DeBio Genetic Data Link".as_bytes().to_vec()
			),
			Error::<Test>::GeneticDataDoesNotExist
		);
	})
}

#[test]
fn update_genetic_data_not_genetic_data_owner() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(GeneticData::add_genetic_data(
			Origin::signed(1),
			"DeBio Genetic Data".as_bytes().to_vec(),
			"DeBio Genetic Data Document Description".as_bytes().to_vec(),
			"DeBio Genetic Data Link".as_bytes().to_vec()
		));

		let genetic_data_ids = GeneticData::genetic_data_by_owner_id(1).unwrap();

		assert_eq!(GeneticData::genetic_data_count(), Some(1));

		assert_eq!(GeneticData::genetic_data_count_by_owner(1), Some(1));

		let genetic_data = GeneticData::genetic_data_by_id(genetic_data_ids[0]).unwrap();

		assert_eq!(genetic_data.id, genetic_data_ids[0]);
		assert_eq!(genetic_data.owner_id, 1);
		assert_eq!(genetic_data.title, "DeBio Genetic Data".as_bytes().to_vec());
		assert_eq!(
			genetic_data.description,
			"DeBio Genetic Data Document Description".as_bytes().to_vec()
		);
		assert_eq!(genetic_data.report_link, "DeBio Genetic Data Link".as_bytes().to_vec());

		assert_noop!(
			GeneticData::update_genetic_data(
				Origin::signed(2),
				genetic_data_ids[0],
				"DeBio Genetic Data 2".as_bytes().to_vec(),
				"DeBio Genetic Data Document Description 2".as_bytes().to_vec(),
				"DeBio Genetic Data Link 2".as_bytes().to_vec()
			),
			Error::<Test>::NotGeneticDataOwner
		);
	})
}
