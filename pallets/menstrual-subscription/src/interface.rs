use primitives_duration::MenstrualSubscriptionDuration;
use primitives_menstrual_status::MenstrualSubscriptionStatus;
use primitives_price_and_currency::CurrencyType;

pub trait MenstrualSubscriptionInterface<T: frame_system::Config> {
	type Error;
	type Balance;
	type MenstrualSubscription;
	type MenstrualSubscriptionPrice;

	fn add_menstrual_subscription(
		address_id: &T::AccountId,
		duration: &MenstrualSubscriptionDuration,
		currency: &CurrencyType,
	) -> Result<Self::MenstrualSubscription, Self::Error>;

	fn change_menstrual_subscription_status(
		menstrual_subscription_id: &T::Hash,
		status: &MenstrualSubscriptionStatus,
	) -> Result<Self::MenstrualSubscription, Self::Error>;

	fn set_menstrual_subscription_paid(
		address_id: &T::AccountId,
		menstrual_subscription_id: &T::Hash,
	) -> Result<Self::MenstrualSubscription, Self::Error>;

	fn set_menstrual_subscription_price(
		duration: &MenstrualSubscriptionDuration,
		currency: &CurrencyType,
		price: Self::Balance,
		asset_id: Option<u32>,
	) -> Result<Self::MenstrualSubscriptionPrice, Self::Error>;
}
