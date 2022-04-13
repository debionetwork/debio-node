#![cfg_attr(not(feature = "std"), no_std)]

use primitives_tracking_id::TrackingId;

pub trait GeneticAnalysisTracking {
	fn get_genetic_analysis_tracking_id(&self) -> &TrackingId;
	fn process_success(&self) -> bool;
	fn is_registered(&self) -> bool;
	fn is_rejected(&self) -> bool;
	fn is_empty(&self) -> bool;
}

pub trait GeneticAnalysisProvider<T: frame_system::Config> {
	type GeneticAnalysis: GeneticAnalysisTracking + sp_std::fmt::Debug;
	type Error;

	fn register_genetic_analysis(
		genetic_analyst_id: &T::AccountId,
		owner_id: &T::AccountId,
		genetic_analysis_order_id: &T::Hash,
	) -> Result<Self::GeneticAnalysis, Self::Error>;
	fn delete_genetic_analysis(
		tracking_id: &TrackingId,
	) -> Result<Self::GeneticAnalysis, Self::Error>;
	fn genetic_analysis_by_genetic_analysis_tracking_id(
		tracking_id: &TrackingId,
	) -> Option<Self::GeneticAnalysis>;
}
