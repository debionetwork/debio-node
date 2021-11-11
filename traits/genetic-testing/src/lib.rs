#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::prelude::*;

use codec::EncodeLike;

use frame_support::{
	codec::{Encode, Decode},
	scale_info::TypeInfo,
    sp_std::convert::TryInto,
	sp_runtime::{RuntimeDebug},
};

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct DnaSampleTrackingId([u8; 21]);
impl EncodeLike<DnaSampleTrackingId> for [u8; 21] {}
impl EncodeLike<DnaSampleTrackingId> for &[u8; 21] {}
impl DnaSampleTrackingId {
    pub fn from_vec(_vec_id: Vec<u8>) -> Self {
        let _array = _vec_id.try_into()
            .unwrap_or_else(|_vec_id: Vec<u8>| panic!("Expected a Vec of length {} but it was {}", 21, _vec_id.len()));
        
        Self(_array)
    }
}

pub trait DnaSampleTracking {
    fn get_tracking_id(&self) -> &DnaSampleTrackingId;
    fn process_success(&self) -> bool;
    fn is_rejected(&self) -> bool;
}

pub trait GeneticTestingProvider<T: frame_system::Config> {
    type DnaSample: DnaSampleTracking + sp_std::fmt::Debug;
    type Error;

    fn register_dna_sample(
        lab_id: &T::AccountId,
        owner_id: &T::AccountId,
        order_id: &T::Hash,
    ) -> Result<Self::DnaSample, Self::Error>;
    fn dna_sample_by_tracking_id(tracking_id: &DnaSampleTrackingId) -> Option<Self::DnaSample>;
    fn delete_dna_sample(tracking_id: &DnaSampleTrackingId) -> Result<Self::DnaSample, Self::Error>;
}
