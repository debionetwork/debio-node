use crate::*;

pub trait MenstrualCalendarInterface<T: frame_system::Config> {
	type Error;
	type MenstrualCycleLog;
	type MenstrualCalendar;
	type MenstrualInfo;
	type Date;

	fn add_menstrual_calendar(
		address_id: &T::AccountId,
		average_cycle: u8,
	) -> Result<Self::MenstrualCalendar, Self::Error>;

	fn update_menstrual_calendar(
		address_id: &T::AccountId,
		menstrual_calendar_id: &T::Hash,
		average_cycle: u8,
	) -> Result<Self::MenstrualCalendar, Self::Error>;

	fn add_menstrual_cycle_log(
		address_id: &T::AccountId,
		menstrual_calendar_id: &T::Hash,
		menstrual_infos: &[Self::MenstrualInfo],
	) -> Result<Vec<Self::MenstrualCycleLog>, Self::Error>;

	fn update_menstrual_cycle_log(
		address_id: &T::AccountId,
		menstrual_cycle_logs: &[Self::MenstrualCycleLog],
	) -> Result<Vec<Self::MenstrualCycleLog>, Self::Error>;

	fn remove_menstrual_cycle_log(
		address_id: &T::AccountId,
		menstrual_calendar_id: &T::Hash,
		menstrual_cycle_log_id: &T::Hash,
	) -> Result<(), Self::Error>;
}
