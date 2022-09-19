use sp_std::vec::Vec;

pub trait MenstrualCalendarInterface<T: frame_system::Config> {
	type Error;
	type MenstrualCycleLog;
	type MenstrualCalendarId;
	type MenstrualCycleLogId;
	type MenstrualCalendar;

	fn generate_menstrual_calendar_id(
		address_id: &T::AccountId,
		menstrual_calendar_count: u64,
	) -> Self::MenstrualCalendarId;

	fn add_menstrual_calendar(
		address_id: &T::AccountId,
		average_cycle: &u8,
	) -> Result<Self::MenstrualCalendar, Self::Error>;

	fn update_menstrual_calendar(
		address_id: &T::AccountId,
		menstrual_calendar_id: &T::Hash,
		average_cycle: &u8,
	) -> Result<Self::MenstrualCalendar, Self::Error>;

	fn generate_menstrual_cycle_log_id(
		menstrual_calendar_id: &T::Hash,
		menstrual_cycle_log_count: u64,
	) -> Self::MenstrualCycleLogId;

	fn add_menstrual_cycle_log(
		menstrual_calendar_id: &T::Hash,
		menstrual_cycle_log: &Self::MenstrualCycleLog,
	) -> Result<Self::MenstrualCycleLog, Self::Error>;

	fn update_menstrual_cycle_log(
		menstrual_calendar_id: &T::Hash,
		menstrual_cycle_log_id: &T::Hash,
		menstrual_cycle_log: &Self::MenstrualCycleLog,
	) -> Result<Self::MenstrualCycleLog, Self::Error>;

	fn remove_menstrual_cycle_log(
		menstrual_calendar_id: &T::Hash,
		menstrual_cycle_log_id: &T::Hash,
	) -> Result<Self::MenstrualCycleLog, Self::Error>;

	fn menstrual_calendar_count_by_owner(address_id: &T::AccountId) -> u64;

	fn menstrual_calendar_by_address_id(address_id: &T::AccountId) -> Option<Vec<T::Hash>>;

	fn menstrual_calendar_by_id(
		menstrual_calendar_id: &Self::MenstrualCalendarId,
	) -> Option<Self::MenstrualCalendar>;

	fn menstrual_cycle_log_count_by_owner(menstrual_cycle_log_id: &T::Hash) -> u64;

	fn menstrual_cycle_log_by_id(
		menstrual_cycle_log_id: &Self::MenstrualCycleLogId,
	) -> Option<Self::MenstrualCycleLog>;
}
