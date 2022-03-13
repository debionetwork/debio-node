/// Interface for GeneticAnalyst Pallet
/// Defines the functionalities of GeneticAnalyst Pallet
pub trait GeneticAnalystInterface<T: frame_system::Config> {
	type Error;
	type Balance;
	type GeneticAnalystInfo;
	type GeneticAnalyst;
	type VerificationStatus;
	type AvailabilityStatus;

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
	/// Update a GeneticAnalyst verification status
	fn update_genetic_analyst_verification_status(
		genetic_analyst_verifier_key: &T::AccountId,
		account_id: &T::AccountId,
		status: &Self::VerificationStatus,
	) -> Result<Self::GeneticAnalyst, Self::Error>;
	/// Update a GeneticAnalyst availability status
	fn update_genetic_analyst_availability_status(
		account_id: &T::AccountId,
		status: &Self::AvailabilityStatus,
	) -> Result<Self::GeneticAnalyst, Self::Error>;
	/// Stake GeneticAnalyst
	fn stake_genetic_analyst(
		account_id: &T::AccountId,
	) -> Result<Self::GeneticAnalyst, Self::Error>;
	/// Unstake GeneticAnalyst
	fn unstake_genetic_analyst(
		account_id: &T::AccountId,
	) -> Result<Self::GeneticAnalyst, Self::Error>;
	/// Retrieve Unstake Amount
	fn retrieve_unstake_amount(
		admin_key: &T::AccountId,
		account_id: &T::AccountId,
	) -> Result<Self::GeneticAnalyst, Self::Error>;
	/// Delete GeneticAnalyst
	fn delete_genetic_analyst(
		account_id: &T::AccountId,
	) -> Result<Self::GeneticAnalyst, Self::Error>;
	/// Update genetic analyst minimum stake amount
	fn update_minimum_stake_amount(
		account_id: &T::AccountId,
		amount: Self::Balance,
	) -> Result<(), Self::Error>;
	/// Update genetic analyst admin key
	fn update_admin_key(
		account_id: &T::AccountId,
		admin_key: &T::AccountId,
	) -> Result<(), Self::Error>;
}
