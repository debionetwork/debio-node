use sp_std::prelude::*;

pub trait GeneticTestingInterface<T: frame_system::Config> {
    type DnaSample;
    type DnaTestResult;
    type DnaTestResultSubmission;
    type Error;

    fn create_dna_sample(lab_id: &T::AccountId, owner_id: &T::AccountId) -> Result<Self::DnaSample, Self::Error>;
    fn receive_dna_sample(lab_id: &T::AccountId, tracking_id: &Vec<u8>) -> Result<Self::DnaSample, Self::Error>;
    fn reject_dna_sample(lab_id: &T::AccountId, tracking_id: &Vec<u8>) -> Result<Self::DnaSample, Self::Error>;
    fn process_dna_sample(lab_id: &T::AccountId, tracking_id: &Vec<u8>) -> Result<Self::DnaSample, Self::Error>;
    fn submit_test_result(lab_id: &T::AccountId, tracking_id: &Vec<u8>, submission: &Self::DnaTestResultSubmission) -> Result<Self::DnaTestResult, Self::Error>;
}
