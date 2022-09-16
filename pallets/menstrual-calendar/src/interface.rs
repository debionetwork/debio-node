use sp_std::vec::Vec;

pub trait MentrualCalendarInterface<T: frame_system::Config> {
	type Error;
	type MentrualCalendarId;
	type MentrualCalendar;

	fn generate_menstrual_data_id(
		owner_id: &T::AccountId,
		menstrual_data_count: u64,
	) -> Self::MentrualCalendarId;

	fn add_menstrual_data(
		owner_id: &T::AccountId,
		title: &[u8],
		description: &[u8],
		report_link: &[u8],
	) -> Result<Self::MentrualCalendar, Self::Error>;

	fn update_menstrual_data(
		owner_id: &T::AccountId,
		menstrual_data_id: &T::Hash,
		title: &[u8],
		description: &[u8],
		report_link: &[u8],
	) -> Result<Self::MentrualCalendar, Self::Error>;

	fn remove_menstrual_data(
		owner_id: &T::AccountId,
		menstrual_data_id: &T::Hash,
	) -> Result<Self::MentrualCalendar, Self::Error>;

	fn menstrual_data_count_by_owner(owner_id: &T::AccountId) -> u64;

	fn menstrual_data_by_owner_id(owner_id: &T::AccountId) -> Option<Vec<T::Hash>>;

	fn menstrual_data_by_id(
		menstrual_data_id: &Self::MentrualCalendarId,
	) -> Option<Self::MentrualCalendar>;
}
