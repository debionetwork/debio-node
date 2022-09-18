use primitives_duration::MenstrualSubscriptionDuration;
use primitives_menstrual_status::{MenstrualSubscriptionStatus, PaymentStatus};
use sp_std::vec::Vec;

pub trait MenstrualSubscriptionInterface<T: frame_system::Config> {
	type Error;
	type MenstrualSubscriptionId;
	type MenstrualSubscription;

	fn generate_menstrual_subscription_id(
		address_id: &T::AccountId,
		menstrual_subscription_count: u64,
	) -> Self::MenstrualSubscriptionId;

	fn add_menstrual_subscription(
		address_id: &T::AccountId,
		duration: &MenstrualSubscriptionDuration,
		price: &u8,
		payment_status: &PaymentStatus,
		status: &MenstrualSubscriptionStatus,
	) -> Result<Self::MenstrualSubscription, Self::Error>;

	fn change_menstrual_subscription_status(
		address_id: &T::AccountId,
		menstrual_subscription_id: &T::Hash,
		status: &MenstrualSubscriptionStatus,
	) -> Result<Self::MenstrualSubscription, Self::Error>;

	fn set_menstrual_subscription_paid(
		address_id: &T::AccountId,
		menstrual_subscription_id: &T::Hash,
	) -> Result<Self::MenstrualSubscription, Self::Error>;

	fn menstrual_subscription_count_by_owner(address_id: &T::AccountId) -> u64;

	fn menstrual_subscription_by_address_id(address_id: &T::AccountId) -> Option<Vec<T::Hash>>;

	fn menstrual_subscription_by_id(
		menstrual_subscription_id: &Self::MenstrualSubscriptionId,
	) -> Option<Self::MenstrualSubscription>;
}
