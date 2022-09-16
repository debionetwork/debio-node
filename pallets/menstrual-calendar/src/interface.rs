use sp_std::vec::Vec;

pub trait MenstrualCalendarInterface<T: frame_system::Config> {
	type Error;
	type MenstrualCalendarId;
	type MenstrualCalendar;

	fn generate_menstrual_calendar_id(
		address_id: &T::AccountId,
		menstrual_calendar_count: u64,
	) -> Self::MenstrualCalendarId;

	fn add_menstrual_calendar(
		address_id: &T::AccountId,
		average_cycle: &u8,
		cycle_log: &[u8],
	) -> Result<Self::MenstrualCalendar, Self::Error>;

	fn update_menstrual_calendar(
		address_id: &T::AccountId,
		menstrual_calendar_id: &T::Hash,
		average_cycle: &u8,
		cycle_log: &[u8],
	) -> Result<Self::MenstrualCalendar, Self::Error>;

	fn remove_menstrual_calendar(
		address_id: &T::AccountId,
		menstrual_calendar_id: &T::Hash,
	) -> Result<Self::MenstrualCalendar, Self::Error>;

	fn menstrual_calendar_count_by_owner(address_id: &T::AccountId) -> u64;

	fn menstrual_calendar_by_address_id(address_id: &T::AccountId) -> Option<Vec<T::Hash>>;

	fn menstrual_calendar_by_id(
		menstrual_calendar_id: &Self::MenstrualCalendarId,
	) -> Option<Self::MenstrualCalendar>;
}
