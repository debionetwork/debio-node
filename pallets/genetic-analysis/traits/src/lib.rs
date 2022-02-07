#![cfg_attr(not(feature = "std"), no_std)]

use primitives_tracking_id::TrackingId;

pub trait GeneticAnalysisTracking {
    fn get_genetic_analysis_tracking_id(&self) -> &TrackingId;
    fn process_success(&self) -> bool;
    fn is_rejected(&self) -> bool;
}

pub trait GeneticAnalysisProvider<T: frame_system::Config> {
    type GeneticAnalysis: GeneticAnalysisTracking + sp_std::fmt::Debug;
    type Error;

    fn genetic_analysis_by_genetic_analysis_tracking_id(tracking_id: &TrackingId) -> Option<Self::GeneticAnalysis>;
}
