#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	codec::{Decode, Encode},
	pallet_prelude::*,
};
pub use pallet::*;
pub use scale_info::TypeInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod interface;
pub mod weights;
pub use interface::MenstrualCalendarInterface;
use sp_std::prelude::*;
use traits_menstrual_calendar::{
	MenstrualCalendar as MenstrualCalendarT, MenstrualCalendarProvider,
};
use primitives_menstrual_cycle_log::MenstrualCycleLog;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct MenstrualCalendar<AccountId, Hash, Moment> {
	pub id: Hash,
	pub address_id: AccountId,
	pub average_cycle: u8,
	pub cycle_log: Vec<MenstrualCycleLog>,
	pub created_at: Moment,
	pub updated_at: Moment,
}

impl<AccountId, Hash, Moment: Default> MenstrualCalendar<AccountId, Hash, Moment> {
	pub fn new(
		id: Hash,
		address_id: AccountId,
		average_cycle: u8,
		cycle_log: Vec<MenstrualCycleLog>,
		created_at: Moment,
	) -> Self {
		Self {
			id,
			address_id,
			average_cycle,
			cycle_log,
			created_at,
			updated_at: Moment::default(),
		}
	}

	pub fn get_id(&self) -> &Hash {
		&self.id
	}

	pub fn get_address_id(&self) -> &AccountId {
		&self.address_id
	}
}

impl<T, AccountId, Hash, Moment: Default> MenstrualCalendarT<T>
	for MenstrualCalendar<AccountId, Hash, Moment>
where
	T: frame_system::Config<AccountId = AccountId, Hash = Hash>,
{
	fn get_id(&self) -> &Hash {
		self.get_id()
	}
	fn get_address_id(&self) -> &AccountId {
		self.get_address_id()
	}
}

#[frame_support::pallet]
pub mod pallet {
	use crate::{interface::MenstrualCalendarInterface, weights::WeightInfo, MenstrualCalendar, MenstrualCycleLog};
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	pub use sp_std::prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type MenstrualCalendarWeightInfo: WeightInfo;
	}

	// ----- This is template code, every pallet needs this ---
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
	// --------------------------------------------------------

	// ----- Types -------
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type HashOf<T> = <T as frame_system::Config>::Hash;
	pub type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;
	pub type MenstrualCalendarOf<T> = MenstrualCalendar<AccountIdOf<T>, HashOf<T>, MomentOf<T>>;
	pub type MenstrualCalendarIdOf<T> = HashOf<T>;

	// ------- Storage -------------
	#[pallet::storage]
	#[pallet::getter(fn menstrual_calendar_by_address_id)]
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
		/// parameters, [MenstrualCalendar, who]
		MenstrualCalendarRemoved(MenstrualCalendarOf<T>, AccountIdOf<T>),
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
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::MenstrualCalendarWeightInfo::add_menstrual_calendar())]
		pub fn add_menstrual_calendar(
			origin: OriginFor<T>,
			average_cycle: u8,
			cycle_log: Vec<MenstrualCycleLog>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as MenstrualCalendarInterface<T>>::add_menstrual_calendar(
				&who,
				&average_cycle,
				&cycle_log,
			) {
				Ok(menstrual_calendar) => {
					Self::deposit_event(Event::MenstrualCalendarAdded(
						menstrual_calendar,
						who.clone(),
					));
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
			cycle_log: Vec<MenstrualCycleLog>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as MenstrualCalendarInterface<T>>::update_menstrual_calendar(
				&who,
				&menstrual_calendar_id,
				&average_cycle,
				&cycle_log,
			) {
				Ok(menstrual_calendar) => {
					Self::deposit_event(Event::MenstrualCalendarUpdated(
						menstrual_calendar,
						who.clone(),
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::MenstrualCalendarWeightInfo::remove_menstrual_calendar())]
		pub fn remove_menstrual_calendar(
			origin: OriginFor<T>,
			menstrual_calendar_id: HashOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as MenstrualCalendarInterface<T>>::remove_menstrual_calendar(
				&who,
				&menstrual_calendar_id,
			) {
				Ok(menstrual_calendar) => {
					Self::deposit_event(Event::MenstrualCalendarRemoved(
						menstrual_calendar,
						who.clone(),
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}
	}
}

use frame_support::sp_runtime::traits::Hash;

/// MenstrualCalendar Interface Implementation
impl<T: Config> MenstrualCalendarInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type MenstrualCalendarId = T::Hash;
	type MenstrualCalendar = MenstrualCalendarOf<T>;

	fn generate_menstrual_calendar_id(
		address_id: &T::AccountId,
		menstrual_calendar_count: u64,
	) -> Self::MenstrualCalendarId {
		let mut account_id_bytes = address_id.encode();
		let mut menstrual_calendar_count_bytes = menstrual_calendar_count.encode();
		account_id_bytes.append(&mut menstrual_calendar_count_bytes);

		let seed = &account_id_bytes;
		T::Hashing::hash(seed)
	}

	fn add_menstrual_calendar(
		address_id: &T::AccountId,
		average_cycle: &u8,
		cycle_log: &[MenstrualCycleLog],
	) -> Result<Self::MenstrualCalendar, Self::Error> {
		let owner_menstrual_calendar_count =
			<Self as MenstrualCalendarInterface<T>>::menstrual_calendar_count_by_owner(address_id);
		let menstrual_calendar_id =
			Self::generate_menstrual_calendar_id(address_id, owner_menstrual_calendar_count);

		let now = pallet_timestamp::Pallet::<T>::get();

		let menstrual_calendar = MenstrualCalendar::new(
			menstrual_calendar_id,
			address_id.clone(),
			average_cycle.clone(),
			cycle_log.to_vec(),
			now,
		);

		// Store to MenstrualCalendarById storage
		MenstrualCalendarById::<T>::insert(menstrual_calendar_id, &menstrual_calendar);

		Self::add_menstrual_calendar_by_owner(address_id, &menstrual_calendar_id);
		Self::add_menstrual_calendar_count();
		Self::add_menstrual_calendar_count_by_owner(address_id);

		Ok(menstrual_calendar)
	}

	fn update_menstrual_calendar(
		address_id: &T::AccountId,
		menstrual_calendar_id: &T::Hash,
		average_cycle: &u8,
		cycle_log: &[MenstrualCycleLog],
	) -> Result<Self::MenstrualCalendar, Self::Error> {
		let menstrual_calendar = MenstrualCalendarById::<T>::get(menstrual_calendar_id);
		if menstrual_calendar.is_none() {
			return Err(Error::<T>::MenstrualCalendarDoesNotExist)
		}

		let mut menstrual_calendar = menstrual_calendar.unwrap();
		if menstrual_calendar.address_id != address_id.clone() {
			return Err(Error::<T>::NotMenstrualCalendarOwner)
		}

		let now = pallet_timestamp::Pallet::<T>::get();

		menstrual_calendar.average_cycle = average_cycle.clone();
		menstrual_calendar.cycle_log = cycle_log.to_vec();
		menstrual_calendar.updated_at = now;

		// Store to MenstrualCalendarById storage
		MenstrualCalendarById::<T>::insert(menstrual_calendar_id, &menstrual_calendar);

		Ok(menstrual_calendar)
	}

	fn remove_menstrual_calendar(
		address_id: &T::AccountId,
		menstrual_calendar_id: &T::Hash,
	) -> Result<Self::MenstrualCalendar, Self::Error> {
		let menstrual_calendar = MenstrualCalendarById::<T>::get(menstrual_calendar_id);
		if menstrual_calendar.is_none() {
			return Err(Error::<T>::MenstrualCalendarDoesNotExist)
		}

		let menstrual_calendar = menstrual_calendar.unwrap();
		if menstrual_calendar.address_id != address_id.clone() {
			return Err(Error::<T>::NotMenstrualCalendarOwner)
		}

		// Remove menstrual_calendar from storage
		MenstrualCalendarById::<T>::take(menstrual_calendar_id).unwrap();

		Self::sub_menstrual_calendar_by_owner(
			menstrual_calendar.get_address_id(),
			menstrual_calendar_id,
		);
		Self::sub_menstrual_calendar_count();
		Self::sub_menstrual_calendar_count_by_owner(menstrual_calendar.get_address_id());

		Ok(menstrual_calendar)
	}

	fn menstrual_calendar_by_address_id(address_id: &T::AccountId) -> Option<Vec<T::Hash>> {
		MenstrualCalendarByOwner::<T>::get(address_id)
	}

	fn menstrual_calendar_count_by_owner(address_id: &T::AccountId) -> u64 {
		MenstrualCalendarCountByOwner::<T>::get(address_id).unwrap_or(0)
	}

	fn menstrual_calendar_by_id(
		menstrual_calendar_id: &Self::MenstrualCalendarId,
	) -> Option<Self::MenstrualCalendar> {
		MenstrualCalendarById::<T>::get(menstrual_calendar_id)
	}
}

/// Pallet Methods
impl<T: Config> Pallet<T> {
	// Add menstrual_calendar by owner
	pub fn add_menstrual_calendar_by_owner(
		address_id: &T::AccountId,
		menstrual_calendar_id: &T::Hash,
	) {
		let mut menstrual_calendar =
			MenstrualCalendarByOwner::<T>::get(address_id).unwrap_or_default();

		menstrual_calendar.push(*menstrual_calendar_id);
		MenstrualCalendarByOwner::<T>::insert(address_id, &menstrual_calendar)
	}

	// Subtract menstrual_calendar by owner
	pub fn sub_menstrual_calendar_by_owner(
		address_id: &T::AccountId,
		menstrual_calendar_id: &T::Hash,
	) {
		let mut menstrual_calendar =
			MenstrualCalendarByOwner::<T>::get(address_id).unwrap_or_default();
		menstrual_calendar.retain(|&x| x != *menstrual_calendar_id);
		MenstrualCalendarByOwner::<T>::insert(address_id, menstrual_calendar);
	}

	// Add menstrual_calendar count
	pub fn add_menstrual_calendar_count() {
		let menstrual_calendar_count = <MenstrualCalendarCount<T>>::get().unwrap_or(0);
		<MenstrualCalendarCount<T>>::put(menstrual_calendar_count.wrapping_add(1));
	}

	// Add menstrual_calendar count by owner
	pub fn add_menstrual_calendar_count_by_owner(address_id: &T::AccountId) {
		let menstrual_calendar_count =
			MenstrualCalendarCountByOwner::<T>::get(address_id).unwrap_or(0);
		MenstrualCalendarCountByOwner::<T>::insert(
			address_id,
			menstrual_calendar_count.wrapping_add(1),
		)
	}

	// Subtract menstrual_calendar count
	pub fn sub_menstrual_calendar_count() {
		let menstrual_calendar_count = <MenstrualCalendarCount<T>>::get().unwrap_or(1);
		MenstrualCalendarCount::<T>::put(menstrual_calendar_count - 1);
	}

	// Subtract menstrual_calendar count by owner
	pub fn sub_menstrual_calendar_count_by_owner(address_id: &T::AccountId) {
		let menstrual_calendar_count =
			MenstrualCalendarCountByOwner::<T>::get(address_id).unwrap_or(1);
		MenstrualCalendarCountByOwner::<T>::insert(address_id, menstrual_calendar_count - 1);
	}
}

/// MenstrualCalendarProvider Trait Implementation
impl<T: Config> MenstrualCalendarProvider<T> for Pallet<T> {
	type Error = Error<T>;
	type MenstrualCalendar = MenstrualCalendarOf<T>;

	fn menstrual_calendar_by_id(id: &T::Hash) -> Option<MenstrualCalendarOf<T>> {
		<Self as MenstrualCalendarInterface<T>>::menstrual_calendar_by_id(id)
	}
}
