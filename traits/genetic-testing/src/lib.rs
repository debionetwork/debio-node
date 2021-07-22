#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::prelude::*;

pub trait DnaSampleTracking {
    fn get_tracking_id(&self) -> &Vec<u8>;
    fn process_success(&self) -> bool;
    fn process_failed(&self) -> bool;
    fn is_rejected(&self) -> bool;
}

pub trait GeneticTestingProvider<T: frame_system::Config> {
    type DnaSample: DnaSampleTracking + sp_std::fmt::Debug;
    type Error;

    fn register_dna_sample(lab_id: &T::AccountId, owner_id: &T::AccountId, order_id: &T::Hash) -> Result<Self::DnaSample, Self::Error>;
    fn dna_sample_by_tracking_id(tracking_id: &Vec<u8>) -> Option<Self::DnaSample>;
    fn delete_dna_sample(tracking_id: &Vec<u8>) -> Result<Self::DnaSample, Self::Error>;
}

pub trait DnaSampleStatus {
    fn prepared(&self) -> &Vec<u8>;
    fn extracted(&self) -> &Vec<u8>;
    fn genotyped(&self) -> &Vec<u8>;
    fn reviewed(&self) -> &Vec<u8>;
    fn computed(&self) -> &Vec<u8>;
}
