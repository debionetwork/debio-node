use sp_std::prelude::*;

pub use traits_genetic_testing::DnaSampleTrackingId;

pub trait GeneticTestingInterface<T: frame_system::Config> {
	type DnaSample;
	type DnaSampleStatus;
	type DnaTestResult;
	type DnaTestResultSubmission;
	type Error;
	type StakedData;

	fn register_dna_sample(
		lab_id: &T::AccountId,
		owner_id: &T::AccountId,
		order_id: &T::Hash,
	) -> Result<Self::DnaSample, Self::Error>;
	fn reject_dna_sample(
		lab_id: &T::AccountId,
		tracking_id: &DnaSampleTrackingId,
		rejected_title: &[u8],
		rejected_description: &[u8],
	) -> Result<Self::DnaSample, Self::Error>;
	fn process_dna_sample(
		lab_id: &T::AccountId,
		tracking_id: &DnaSampleTrackingId,
		status: Self::DnaSampleStatus,
	) -> Result<Self::DnaSample, Self::Error>;
	fn delete_dna_sample(tracking_id: &DnaSampleTrackingId)
		-> Result<Self::DnaSample, Self::Error>;

	fn submit_test_result(
		lab_id: &T::AccountId,
		tracking_id: &DnaSampleTrackingId,
		submission: &Self::DnaTestResultSubmission,
	) -> Result<Self::DnaTestResult, Self::Error>;

	fn submit_independent_test_result(
		owner_id: &T::AccountId,
		submission: &Self::DnaTestResultSubmission,
	) -> Result<Self::DnaTestResult, Self::Error>;

	fn dna_sample_by_tracking_id(tracking_id: &DnaSampleTrackingId) -> Option<Self::DnaSample>;
	fn dna_test_result_by_tracking_id(
		tracking_id: &DnaSampleTrackingId,
	) -> Option<Self::DnaTestResult>;
	// Return dna sample tracking ids
	fn dna_samples_by_owner_id(owner_id: &T::AccountId) -> Option<Vec<DnaSampleTrackingId>>;
	// Return dna sample tracking ids
	fn dna_samples_by_lab_id(lab_id: &T::AccountId) -> Option<Vec<DnaSampleTrackingId>>;
	// Return dna sample tracking ids
	fn dna_test_results_by_owner_id(owner_id: &T::AccountId) -> Option<Vec<DnaSampleTrackingId>>;
	// Return dna sample tracking ids
	fn dna_test_results_by_lab_id(lab_id: &T::AccountId) -> Option<Vec<DnaSampleTrackingId>>;
	// Submit data bounty details
	fn submit_data_bounty_details(
		data_staker: &T::AccountId,
		data_hash: &T::Hash,
		order_id: &T::Hash,
	) -> Result<Self::StakedData, Self::Error>;
}
