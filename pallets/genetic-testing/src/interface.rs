use sp_std::prelude::*;

pub trait GeneticTestingInterface<T: frame_system::Config> {
    type DnaSample;
    type DnaSampleStatus;
    type DnaTestResult;
    type DnaTestResultSubmission;
    type Error;
    type StakedData;
    type DataSubmission;


    fn register_dna_sample(lab_id: &T::AccountId, owner_id: &T::AccountId, order_id: &T::Hash) -> Result<Self::DnaSample, Self::Error>;
    fn receive_dna_sample(lab_id: &T::AccountId, tracking_id: &Vec<u8>) -> Result<Self::DnaSample, Self::Error>;
    fn reject_dna_sample(lab_id: &T::AccountId, tracking_id: &Vec<u8>, rejected_title: &Vec<u8>, rejected_description: &Vec<u8>) -> Result<Self::DnaSample, Self::Error>;
    fn process_dna_sample(lab_id: &T::AccountId, tracking_id: &Vec<u8>, status: Self::DnaSampleStatus) -> Result<Self::DnaSample, Self::Error>;
    fn delete_dna_sample(tracking_id: &Vec<u8>) -> Result<Self::DnaSample, Self::Error>;

    fn submit_test_result(
        lab_id: &T::AccountId,
        tracking_id: &Vec<u8>,
        submission: &Self::DnaTestResultSubmission
    ) -> Result<Self::DnaTestResult, Self::Error>;

    fn submit_independent_test_result(
        owner_id: &T::AccountId,
        submission: &Self::DnaTestResultSubmission
    ) -> Result<Self::DnaTestResult, Self::Error>;

    fn dna_sample_by_tracking_id(tracking_id: &Vec<u8>) -> Option<Self::DnaSample>;
    fn dna_test_result_by_tracking_id(tracking_id: &Vec<u8>) -> Option<Self::DnaTestResult>;
    // Return dna sample tracking ids
    fn dna_samples_by_owner_id(owner_id: &T::AccountId) -> Option<Vec<Vec<u8>>>;
    // Return dna sample tracking ids
    fn dna_samples_by_lab_id(lab_id: &T::AccountId) -> Option<Vec<Vec<u8>>>;
    // Return dna sample tracking ids
    fn dna_test_results_by_owner_id(owner_id: &T::AccountId) -> Option<Vec<Vec<u8>>>;
    // Return dna sample tracking ids
    fn dna_test_results_by_lab_id(lab_id: &T::AccountId) -> Option<Vec<Vec<u8>>>;
    // Submit data staking details
    fn submit_data_staking_details(data_staker: &T::AccountId, data_hash: &T::Hash, data_submission: &Self::DataSubmission) -> Result<Self::StakedData, Self::Error>;
}
