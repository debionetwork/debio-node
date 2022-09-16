use sp_std::vec::Vec;

pub trait MenstrualCalendarInterface<T: frame_system::Config> {
	type Error;
	type MenstrualCalendarId;
	type MenstrualCalendar;

	fn generate_menstrual_calendar_id(
		owner_id: &T::AccountId,
		menstrual_calendar_count: u64,
	) -> Self::MenstrualCalendarId;

	fn add_menstrual_calendar(
		owner_id: &T::AccountId,
		title: &[u8],
		description: &[u8],
		report_link: &[u8],
	) -> Result<Self::MenstrualCalendar, Self::Error>;

	fn update_menstrual_calendar(
		owner_id: &T::AccountId,
		menstrual_calendar_id: &T::Hash,
		title: &[u8],
		description: &[u8],
		report_link: &[u8],
	) -> Result<Self::MenstrualCalendar, Self::Error>;

	fn remove_menstrual_calendar(
		owner_id: &T::AccountId,
		menstrual_calendar_id: &T::Hash,
	) -> Result<Self::MenstrualCalendar, Self::Error>;

	fn menstrual_calendar_count_by_owner(owner_id: &T::AccountId) -> u64;

	fn menstrual_calendar_by_owner_id(owner_id: &T::AccountId) -> Option<Vec<T::Hash>>;

	fn menstrual_calendar_by_id(
		menstrual_calendar_id: &Self::MenstrualCalendarId,
	) -> Option<Self::MenstrualCalendar>;
}
