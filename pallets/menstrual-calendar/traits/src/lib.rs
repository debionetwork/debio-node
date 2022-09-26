#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;

pub trait MenstrualCalendar<T: Config> {
	fn get_id(&self) -> &T::Hash;
	fn get_address_id(&self) -> &T::AccountId;
}

pub trait MenstrualCycleLog<T: Config> {
	fn get_id(&self) -> &T::Hash;
	fn get_menstrual_calendar_id(&self) -> &T::Hash;
}

pub trait MenstrualCalendarProvider<T: Config> {
	type Error;
	type MenstrualCalendar: MenstrualCalendar<T> + sp_std::fmt::Debug;

	fn menstrual_calendar_by_id(id: &T::Hash) -> Option<Self::MenstrualCalendar>;
}
