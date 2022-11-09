#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub use pallet::*;

pub mod functions;
pub mod impl_menstrual_calendar;
pub mod interface;
pub mod migrations;
pub mod types;
pub mod weights;

use interface::MenstrualCalendarInterface;
use sp_std::prelude::*;
use traits_menstrual_calendar::MenstrualCalendarProvider;
use types::*;
use weights::WeightInfo;

pub use frame_support::traits::StorageVersion;

/// The current storage version.
const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type MenstrualCalendarWeightInfo: WeightInfo;
	}

	// ----- This is template code, every pallet needs this ---
	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_runtime_upgrade() -> Weight {
			migrations::migrate::<T>()
		}
	}
	// --------------------------------------------------------

	// ----- Types -------
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type HashOf<T> = <T as frame_system::Config>::Hash;
	pub type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;
	pub type MenstrualCalendarOf<T> = MenstrualCalendar<AccountIdOf<T>, HashOf<T>, MomentOf<T>>;
	pub type MenstrualCalendarIdOf<T> = HashOf<T>;
	pub type MenstrualCycleLogOf<T> = MenstrualCycleLog<HashOf<T>, MomentOf<T>>;
	pub type MenstrualCycleLogIdOf<T> = HashOf<T>;
	pub type SymptomInfoOf<T> = MenstrualInfo<MomentOf<T>>;

	// ------- Storage -------------
	#[pallet::storage]
	#[pallet::getter(fn menstrual_calendar_by_owner)]
	pub type MenstrualCalendarByOwner<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Vec<MenstrualCalendarIdOf<T>>>;

	#[pallet::storage]
	#[pallet::getter(fn menstrual_calendar_by_id)]
	pub type MenstrualCalendarById<T> =
		StorageMap<_, Blake2_128Concat, MenstrualCalendarIdOf<T>, MenstrualCalendarOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn menstrual_calendar_count_by_owner)]
	pub type MenstrualCalendarCountByOwner<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, u64>;

	#[pallet::storage]
	#[pallet::getter(fn menstrual_calendar_count)]
	pub type MenstrualCalendarCount<T> = StorageValue<_, u64>;

	#[pallet::storage]
	#[pallet::getter(fn menstrual_cycle_log_by_owner_id)]
	pub type MenstrualCycleLogByOwner<T> =
		StorageMap<_, Blake2_128Concat, HashOf<T>, Vec<MenstrualCycleLogIdOf<T>>>;

	#[pallet::storage]
	#[pallet::getter(fn menstrual_cycle_log_by_id)]
	pub type MenstrualCycleLogById<T> =
		StorageMap<_, Blake2_128Concat, HashOf<T>, MenstrualCycleLogOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn menstrual_cycle_log_count)]
	pub type MenstrualCycleLogCount<T> = StorageValue<_, u64>;

	#[pallet::storage]
	#[pallet::getter(fn menstrual_cycle_log_count_by_owner)]
	pub type MenstrualCycleLogCountByOwner<T> = StorageMap<_, Blake2_128Concat, HashOf<T>, u64>;
	//                                _,  Hasher         ,  Key     ,  Value
	// -----------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters, [MenstrualCalendar, who]
		MenstrualCalendarAdded(MenstrualCalendarOf<T>, AccountIdOf<T>),
		//// MenstrualCalendar updated
		/// parameters, [MenstrualCalendar, who]
		MenstrualCalendarUpdated(MenstrualCalendarOf<T>, AccountIdOf<T>),
		//// MenstrualCalendar deleted
		/// parameters, [MenstrualCalendarId, who]
		MenstrualCalendarRemoved(HashOf<T>, AccountIdOf<T>),
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters, [MenstrualCycleLog, who]
		MenstrualCycleLogsAdded(Vec<MenstrualCycleLogOf<T>>, AccountIdOf<T>),
		//// MenstrualCycleLog updated
		/// parameters, [MenstrualCycleLog, who]
		MenstrualCycleLogUpdated(MenstrualCycleLogOf<T>, AccountIdOf<T>),
		//// MenstrualCycleLog deleted
		/// parameters, [MenstrualCycleLogId, who]
		MenstrualCycleLogRemoved(HashOf<T>, AccountIdOf<T>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// User not allowed to create menstrual_calendar
		NotAllowedToCreate,
		/// User is not the owner of a menstrual_calendar
		NotMenstrualCalendarOwner,
		/// Ordering a menstrual_calendar that does not exist
		MenstrualCalendarDoesNotExist,
		/// User is not the owner of a menstrual_cycle_log
		NotMenstrualCycleLogOwner,
		/// Ordering a menstrual_cycle_log that does not exist
		MenstrualCycleLogDoesNotExist,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::MenstrualCalendarWeightInfo::add_menstrual_calendar())]
		pub fn add_menstrual_calendar(
			origin: OriginFor<T>,
			average_cycle: u8,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as MenstrualCalendarInterface<T>>::add_menstrual_calendar(
				&who,
				average_cycle,
			) {
				Ok(menstrual_calendar) => {
					Self::deposit_event(Event::MenstrualCalendarAdded(menstrual_calendar, who));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::MenstrualCalendarWeightInfo::update_menstrual_calendar())]
		pub fn update_menstrual_calendar(
			origin: OriginFor<T>,
			menstrual_calendar_id: HashOf<T>,
			average_cycle: u8,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as MenstrualCalendarInterface<T>>::update_menstrual_calendar(
				&who,
				&menstrual_calendar_id,
				average_cycle,
			) {
				Ok(menstrual_calendar) => {
					Self::deposit_event(Event::MenstrualCalendarUpdated(menstrual_calendar, who));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::MenstrualCalendarWeightInfo::add_menstrual_cycle_log())]
		pub fn add_menstrual_cycle_log(
			origin: OriginFor<T>,
			menstrual_calendar_id: HashOf<T>,
			menstrual_infos: Vec<SymptomInfoOf<T>>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as MenstrualCalendarInterface<T>>::add_menstrual_cycle_log(
				&who,
				&menstrual_calendar_id,
				&menstrual_infos,
			) {
				Ok(menstrual_cycle_logs) => {
					Self::deposit_event(Event::MenstrualCycleLogsAdded(menstrual_cycle_logs, who));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::MenstrualCalendarWeightInfo::update_menstrual_cycle_log())]
		pub fn update_menstrual_cycle_log(
			origin: OriginFor<T>,
			menstrual_calendar_id: HashOf<T>,
			menstrual_cycle_log_id: HashOf<T>,
			date: MomentOf<T>,
			symptoms: Vec<Symptom>,
			menstruation: bool,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as MenstrualCalendarInterface<T>>::update_menstrual_cycle_log(
				&who,
				&menstrual_calendar_id,
				&menstrual_cycle_log_id,
				&date,
				&symptoms,
				menstruation,
			) {
				Ok(menstrual_cycle_log) => {
					Self::deposit_event(Event::MenstrualCycleLogUpdated(menstrual_cycle_log, who));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::MenstrualCalendarWeightInfo::remove_menstrual_cycle_log())]
		pub fn remove_menstrual_cycle_log(
			origin: OriginFor<T>,
			menstrual_calendar_id: HashOf<T>,
			menstrual_cycle_log_id: HashOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as MenstrualCalendarInterface<T>>::remove_menstrual_cycle_log(
				&who,
				&menstrual_calendar_id,
				&menstrual_cycle_log_id,
			) {
				Ok(_) => {
					Self::deposit_event(Event::MenstrualCycleLogRemoved(
						menstrual_cycle_log_id,
						who,
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}
	}
}
