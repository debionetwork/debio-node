#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::prelude::*;

use frame_support::{
	codec::{Encode, Decode},
	scale_info::TypeInfo,
    sp_std::convert::TryInto,
	sp_runtime::{RuntimeDebug},
};
use primitives_tracking_id::TrackingId;

pub trait GeneticAnalysisTracking {
    fn get_genetic_analysis_tracking_id(&self) -> &TrackingId;
    fn process_success(&self) -> bool;
    fn is_rejected(&self) -> bool;
}

pub trait GeneticAnalysisProvider<T: frame_system::Config> {
    type GeneticAnalysis: GeneticAnalysisTracking + sp_std::fmt::Debug;
    type Error;

    fn register_genetic_analysis(
        lab_id: &T::AccountId,
        owner_id: &T::AccountId,
        order_id: &T::Hash,
    ) -> Result<Self::GeneticAnalysis, Self::Error>;
    fn genetic_analysis_by_genetic_analysis_tracking_id(tracking_id: &TrackingId) -> Option<Self::GeneticAnalysis>;
    fn delete_genetic_analysis(tracking_id: &TrackingId) -> Result<Self::GeneticAnalysis, Self::Error>;
}
