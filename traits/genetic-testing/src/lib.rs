#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::prelude::*;

pub trait DnaSampleTracking {
    fn get_tracking_id(&self) -> &Vec<u8>;
}

pub trait GeneticTestingProvider<T: frame_system::Config> {
    type DnaSample: DnaSampleTracking + sp_std::fmt::Debug;
    type Error;

    fn create_dna_sample(lab_id: &T::AccountId, owner_id: &T::AccountId) -> Result<Self::DnaSample, Self::Error>;
}
