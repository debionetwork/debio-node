use sp_std::prelude::*;

pub use primitives_tracking_id::TrackingId;

pub trait GeneticAnalysisInterface<T: frame_system::Config> {
    type GeneticAnalysis;
    type GeneticAnalysisStatus;
    type Error;


    fn register_genetic_analysis(
        genetic_analyst_id: &T::AccountId,
        owner_id: &T::AccountId,
        genetic_analysis_order_id: &T::Hash,
    ) -> Result<Self::GeneticAnalysis, Self::Error>;
    fn delete_genetic_analysis(
        tracking_id: &TrackingId
    ) -> Result<Self::GeneticAnalysis, Self::Error>;
    fn reject_genetic_analysis(
        genetic_analyst_id: &T::AccountId,
        tracking_id: &TrackingId,
        rejected_title: &[u8],
        rejected_description: &[u8],
    ) -> Result<Self::GeneticAnalysis, Self::Error>;
    fn process_genetic_analysis(
        genetic_analyst_id: &T::AccountId,
        tracking_id: &TrackingId,
        status: Self::GeneticAnalysisStatus,
    ) -> Result<Self::GeneticAnalysis, Self::Error>;
    fn submit_genetic_analysis(
        genetic_analyst_id: &T::AccountId,
        tracking_id: &TrackingId,
        report_link: &[u8],
        comment: &Option<Vec<u8>>,
    ) -> Result<Self::GeneticAnalysis, Self::Error>;

    fn genetic_analysis_by_genetic_analysis_tracking_id(
        tracking_id: &TrackingId
    ) -> Option<Self::GeneticAnalysis>;
    fn genetic_analysis_by_owner_id(
        owner_id: &T::AccountId
    ) -> Option<Vec<TrackingId>>;
    fn genetic_analysis_by_genetic_analyst_id(
        genetic_analyst_id: &T::AccountId
    ) -> Option<Vec<TrackingId>>;
}
