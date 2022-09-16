#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;

pub trait MentrualCalendar<T: Config> {
	fn get_id(&self) -> &T::Hash;
	fn get_owner_id(&self) -> &T::AccountId;
}

pub trait MentrualCalendarProvider<T: Config> {
	type Error;
	type MentrualCalendar: MentrualCalendar<T> + sp_std::fmt::Debug;

	fn menstrual_calendar_by_id(id: &T::Hash) -> Option<Self::MentrualCalendar>;
}
