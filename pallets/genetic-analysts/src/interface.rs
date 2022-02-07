/// Interface for GeneticAnalyst Pallet
/// Defines the functionalities of GeneticAnalyst Pallet
pub trait GeneticAnalystInterface<T: frame_system::Config> {
	type Error;
	type GeneticAnalystInfo;
	type GeneticAnalyst;

	/// Get genetic_analyst by associated account_id
	fn genetic_analyst_by_account_id(account_id: &T::AccountId) -> Option<Self::GeneticAnalyst>;

	/// Store A genetic_analyst with its information
	fn create_genetic_analyst(
		account_id: &T::AccountId,
		genetic_analyst_info: &Self::GeneticAnalystInfo,
	) -> Result<Self::GeneticAnalyst, Self::Error>;
	/// Update a GeneticAnalyst information
	fn update_genetic_analyst(
		account_id: &T::AccountId,
		genetic_analyst_info: &Self::GeneticAnalystInfo,
	) -> Result<Self::GeneticAnalyst, Self::Error>;
	/// Delete GeneticAnalyst
	fn delete_genetic_analyst(account_id: &T::AccountId) -> Result<Self::GeneticAnalyst, Self::Error>;
}
