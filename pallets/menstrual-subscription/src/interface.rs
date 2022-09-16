use sp_std::vec::Vec;

pub trait MenstrualSubscriptionInterface<T: frame_system::Config> {
	type Error;
	type MenstrualSubscriptionId;
	type MenstrualSubscription;

	fn generate_menstrual_data_id(
		owner_id: &T::AccountId,
		menstrual_data_count: u64,
	) -> Self::MenstrualSubscriptionId;

	fn add_menstrual_data(
		owner_id: &T::AccountId,
		title: &[u8],
		description: &[u8],
		report_link: &[u8],
	) -> Result<Self::MenstrualSubscription, Self::Error>;

	fn update_menstrual_data(
		owner_id: &T::AccountId,
		menstrual_data_id: &T::Hash,
		title: &[u8],
		description: &[u8],
		report_link: &[u8],
	) -> Result<Self::MenstrualSubscription, Self::Error>;

	fn remove_menstrual_data(
		owner_id: &T::AccountId,
		menstrual_data_id: &T::Hash,
	) -> Result<Self::MenstrualSubscription, Self::Error>;

	fn menstrual_data_count_by_owner(owner_id: &T::AccountId) -> u64;

	fn menstrual_data_by_owner_id(owner_id: &T::AccountId) -> Option<Vec<T::Hash>>;

	fn menstrual_data_by_id(
		menstrual_data_id: &Self::MenstrualSubscriptionId,
	) -> Option<Self::MenstrualSubscription>;
}
