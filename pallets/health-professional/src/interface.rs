use primitives_availability_status::AvailabilityStatus;
use primitives_verification_status::VerificationStatus;

pub trait HealthProfessionalInterface<T: frame_system::Config> {
	type Error;
	type Balance;
	type Moment;
	type HealthProfessional;
	type HealthProfessionalInfo;

	fn create_health_professional(
		account_id: &T::AccountId,
		health_professional_info: &Self::HealthProfessionalInfo,
	) -> Result<Self::HealthProfessional, Self::Error>;

	fn update_health_professional_info(
		account_id: &T::AccountId,
		health_professional_info: &Self::HealthProfessionalInfo,
	) -> Result<Self::HealthProfessionalInfo, Self::Error>;

	fn update_health_professional_verification_status(
		verifier_key: &T::AccountId,
		account_id: &T::AccountId,
		status: &VerificationStatus,
	) -> Result<VerificationStatus, Self::Error>;

	fn update_health_professional_availability_status(
		account_id: &T::AccountId,
		status: &AvailabilityStatus,
	) -> Result<AvailabilityStatus, Self::Error>;

	fn delete_health_professional(account_id: &T::AccountId) -> Result<(), Self::Error>;

	fn stake_health_professional(account_id: &T::AccountId) -> Result<Self::Balance, Self::Error>;

	fn unstake_health_professional(account_id: &T::AccountId) -> Result<Self::Moment, Self::Error>;

	fn retrieve_unstaked_amount(
		account_id: &T::AccountId,
	) -> Result<(Self::Balance, Self::Moment), Self::Error>;

	fn update_stake_amount(
		account_id: &T::AccountId,
		balance: &Self::Balance,
	) -> Result<(), Self::Error>;

	fn update_unstake_time(account_id: &T::AccountId, moment: u128) -> Result<(), Self::Error>;

	fn update_verifier_key(
		verifier_key: &T::AccountId,
		account_id: &T::AccountId,
	) -> Result<(), Self::Error>;
}
