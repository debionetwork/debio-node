pub trait GeneticAnalysisOrderInterface<T: frame_system::Config> {
	type GeneticAnalysisOrder;
	type Error;

	fn create_genetic_analysis_order(
		customer_id: &T::AccountId,
		genetic_data_id: &T::Hash,
		service_id: &T::Hash,
		price_index: u32,
		customer_box_public_key: &T::Hash,
		genetic_link: &[u8],
		asset_id: Option<u32>,
	) -> Result<Self::GeneticAnalysisOrder, Self::Error>;
	fn cancel_genetic_analysis_order(
		customer_id: &T::AccountId,
		genetic_analysis_order_id: &T::Hash,
	) -> Result<Self::GeneticAnalysisOrder, Self::Error>;
	fn set_genetic_analysis_order_paid(
		customer_id: &T::AccountId,
		genetic_analysis_order_id: &T::Hash,
	) -> Result<Self::GeneticAnalysisOrder, Self::Error>;
	fn fulfill_genetic_analysis_order(
		seller_id: &T::AccountId,
		genetic_analysis_order_id: &T::Hash,
	) -> Result<Self::GeneticAnalysisOrder, Self::Error>;
	fn set_genetic_analysis_order_refunded(
		escrow_account_id: &T::AccountId,
		genetic_analysis_order_id: &T::Hash,
	) -> Result<Self::GeneticAnalysisOrder, Self::Error>;
}
