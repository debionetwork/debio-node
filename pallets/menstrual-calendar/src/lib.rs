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
pub use interface::MentrualCalendarInterface;
use sp_std::prelude::*;
use traits_menstrual_calendar::{MentrualCalendar as MentrualCalendarT, MentrualCalendarProvider};

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct MentrualCalendar<AccountId, Hash, Moment> {
	pub id: Hash,
	pub owner_id: AccountId,
	pub title: Vec<u8>,
	pub description: Vec<u8>,
	pub report_link: Vec<u8>,
	pub created_at: Moment,
	pub updated_at: Moment,
}

impl<AccountId, Hash, Moment: Default> MentrualCalendar<AccountId, Hash, Moment> {
	pub fn new(
		id: Hash,
		owner_id: AccountId,
		title: Vec<u8>,
		description: Vec<u8>,
		report_link: Vec<u8>,
		created_at: Moment,
	) -> Self {
		Self {
			id,
			owner_id,
			title,
			description,
			report_link,
			created_at,
			updated_at: Moment::default(),
		}
	}

	pub fn get_id(&self) -> &Hash {
		&self.id
	}

	pub fn get_owner_id(&self) -> &AccountId {
		&self.owner_id
	}
}

impl<T, AccountId, Hash, Moment: Default> MentrualCalendarT<T>
	for MentrualCalendar<AccountId, Hash, Moment>
where
	T: frame_system::Config<AccountId = AccountId, Hash = Hash>,
{
	fn get_id(&self) -> &Hash {
		self.get_id()
	}
	fn get_owner_id(&self) -> &AccountId {
		self.get_owner_id()
	}
}

#[frame_support::pallet]
pub mod pallet {
	use crate::{interface::MentrualCalendarInterface, weights::WeightInfo, MentrualCalendar};
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	pub use sp_std::prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type MentrualCalendarWeightInfo: WeightInfo;
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
	pub type MentrualCalendarOf<T> = MentrualCalendar<AccountIdOf<T>, HashOf<T>, MomentOf<T>>;
	pub type MentrualCalendarIdOf<T> = HashOf<T>;

	// ------- Storage -------------
	#[pallet::storage]
	#[pallet::getter(fn menstrual_calendar_by_owner_id)]
	pub type MentrualCalendarByOwner<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Vec<MentrualCalendarIdOf<T>>>;

	#[pallet::storage]
	#[pallet::getter(fn menstrual_calendar_by_id)]
	pub type MentrualCalendarById<T> =
		StorageMap<_, Blake2_128Concat, MentrualCalendarIdOf<T>, MentrualCalendarOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn menstrual_calendar_count_by_owner)]
	pub type MentrualCalendarCountByOwner<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, u64>;

	#[pallet::storage]
	#[pallet::getter(fn menstrual_calendar_count)]
	pub type MentrualCalendarCount<T> = StorageValue<_, u64>;
	//                                _,  Hasher         ,  Key     ,  Value
	// -----------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters, [MentrualCalendar, who]
		MentrualCalendarAdded(MentrualCalendarOf<T>, AccountIdOf<T>),
		//// MentrualCalendar updated
		/// parameters, [MentrualCalendar, who]
		MentrualCalendarUpdated(MentrualCalendarOf<T>, AccountIdOf<T>),
		//// MentrualCalendar deleted
		/// parameters, [MentrualCalendar, who]
		MentrualCalendarRemoved(MentrualCalendarOf<T>, AccountIdOf<T>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// User not allowed to create menstrual_calendar
		NotAllowedToCreate,
		/// User is not the owner of a menstrual_calendar
		NotMentrualCalendarOwner,
		/// Ordering a menstrual_calendar that does not exist
		MentrualCalendarDoesNotExist,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::MentrualCalendarWeightInfo::add_menstrual_calendar())]
		pub fn add_menstrual_calendar(
			origin: OriginFor<T>,
			title: Vec<u8>,
			description: Vec<u8>,
			report_link: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as MentrualCalendarInterface<T>>::add_menstrual_calendar(
				&who,
				&title,
				&description,
				&report_link,
			) {
				Ok(menstrual_calendar) => {
					Self::deposit_event(Event::MentrualCalendarAdded(menstrual_calendar, who.clone()));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::MentrualCalendarWeightInfo::update_menstrual_calendar())]
		pub fn update_menstrual_calendar(
			origin: OriginFor<T>,
			menstrual_calendar_id: HashOf<T>,
			title: Vec<u8>,
			description: Vec<u8>,
			report_link: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as MentrualCalendarInterface<T>>::update_menstrual_calendar(
				&who,
				&menstrual_calendar_id,
				&title,
				&description,
				&report_link,
			) {
				Ok(menstrual_calendar) => {
					Self::deposit_event(Event::MentrualCalendarUpdated(menstrual_calendar, who.clone()));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::MentrualCalendarWeightInfo::remove_menstrual_calendar())]
		pub fn remove_menstrual_calendar(
			origin: OriginFor<T>,
			menstrual_calendar_id: HashOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as MentrualCalendarInterface<T>>::remove_menstrual_calendar(
				&who,
				&menstrual_calendar_id,
			) {
				Ok(menstrual_calendar) => {
					Self::deposit_event(Event::MentrualCalendarRemoved(menstrual_calendar, who.clone()));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}
	}
}

use frame_support::sp_runtime::traits::Hash;

/// MentrualCalendar Interface Implementation
impl<T: Config> MentrualCalendarInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type MentrualCalendarId = T::Hash;
	type MentrualCalendar = MentrualCalendarOf<T>;

	fn generate_menstrual_calendar_id(
		owner_id: &T::AccountId,
		menstrual_calendar_count: u64,
	) -> Self::MentrualCalendarId {
		let mut account_id_bytes = owner_id.encode();
		let mut menstrual_calendar_count_bytes = menstrual_calendar_count.encode();
		account_id_bytes.append(&mut menstrual_calendar_count_bytes);

		let seed = &account_id_bytes;
		T::Hashing::hash(seed)
	}

	fn add_menstrual_calendar(
		owner_id: &T::AccountId,
		title: &[u8],
		description: &[u8],
		report_link: &[u8],
	) -> Result<Self::MentrualCalendar, Self::Error> {
		let owner_menstrual_calendar_count =
			<Self as MentrualCalendarInterface<T>>::menstrual_calendar_count_by_owner(owner_id);
		let menstrual_calendar_id =
			Self::generate_menstrual_calendar_id(owner_id, owner_menstrual_calendar_count);

		let now = pallet_timestamp::Pallet::<T>::get();

		let menstrual_calendar = MentrualCalendar::new(
			menstrual_calendar_id,
			owner_id.clone(),
			title.to_vec(),
			description.to_vec(),
			report_link.to_vec(),
			now,
		);

		// Store to MentrualCalendarById storage
		MentrualCalendarById::<T>::insert(menstrual_calendar_id, &menstrual_calendar);

		Self::add_menstrual_calendar_by_owner(owner_id, &menstrual_calendar_id);
		Self::add_menstrual_calendar_count();
		Self::add_menstrual_calendar_count_by_owner(owner_id);

		Ok(menstrual_calendar)
	}

	fn update_menstrual_calendar(
		owner_id: &T::AccountId,
		menstrual_calendar_id: &T::Hash,
		title: &[u8],
		description: &[u8],
		report_link: &[u8],
	) -> Result<Self::MentrualCalendar, Self::Error> {
		let menstrual_calendar = MentrualCalendarById::<T>::get(menstrual_calendar_id);
		if menstrual_calendar.is_none() {
			return Err(Error::<T>::MentrualCalendarDoesNotExist)
		}

		let mut menstrual_calendar = menstrual_calendar.unwrap();
		if menstrual_calendar.owner_id != owner_id.clone() {
			return Err(Error::<T>::NotMentrualCalendarOwner)
		}

		let now = pallet_timestamp::Pallet::<T>::get();

		menstrual_calendar.title = title.to_vec();
		menstrual_calendar.description = description.to_vec();
		menstrual_calendar.report_link = report_link.to_vec();
		menstrual_calendar.updated_at = now;

		// Store to MentrualCalendarById storage
		MentrualCalendarById::<T>::insert(menstrual_calendar_id, &menstrual_calendar);

		Ok(menstrual_calendar)
	}

	fn remove_menstrual_calendar(
		owner_id: &T::AccountId,
		menstrual_calendar_id: &T::Hash,
	) -> Result<Self::MentrualCalendar, Self::Error> {
		let menstrual_calendar = MentrualCalendarById::<T>::get(menstrual_calendar_id);
		if menstrual_calendar.is_none() {
			return Err(Error::<T>::MentrualCalendarDoesNotExist)
		}

		let menstrual_calendar = menstrual_calendar.unwrap();
		if menstrual_calendar.owner_id != owner_id.clone() {
			return Err(Error::<T>::NotMentrualCalendarOwner)
		}

		// Remove menstrual_calendar from storage
		MentrualCalendarById::<T>::take(menstrual_calendar_id).unwrap();

		Self::sub_menstrual_calendar_by_owner(menstrual_calendar.get_owner_id(), menstrual_calendar_id);
		Self::sub_menstrual_calendar_count();
		Self::sub_menstrual_calendar_count_by_owner(menstrual_calendar.get_owner_id());

		Ok(menstrual_calendar)
	}

	fn menstrual_calendar_by_owner_id(owner_id: &T::AccountId) -> Option<Vec<T::Hash>> {
		MentrualCalendarByOwner::<T>::get(owner_id)
	}

	fn menstrual_calendar_count_by_owner(owner_id: &T::AccountId) -> u64 {
		MentrualCalendarCountByOwner::<T>::get(owner_id).unwrap_or(0)
	}

	fn menstrual_calendar_by_id(
		menstrual_calendar_id: &Self::MentrualCalendarId,
	) -> Option<Self::MentrualCalendar> {
		MentrualCalendarById::<T>::get(menstrual_calendar_id)
	}
}

/// Pallet Methods
impl<T: Config> Pallet<T> {
	// Add menstrual_calendar by owner
	pub fn add_menstrual_calendar_by_owner(owner_id: &T::AccountId, menstrual_calendar_id: &T::Hash) {
		let mut menstrual_calendar = MentrualCalendarByOwner::<T>::get(owner_id).unwrap_or_default();

		menstrual_calendar.push(*menstrual_calendar_id);
		MentrualCalendarByOwner::<T>::insert(owner_id, &menstrual_calendar)
	}

	// Subtract menstrual_calendar by owner
	pub fn sub_menstrual_calendar_by_owner(owner_id: &T::AccountId, menstrual_calendar_id: &T::Hash) {
		let mut menstrual_calendar = MentrualCalendarByOwner::<T>::get(owner_id).unwrap_or_default();
		menstrual_calendar.retain(|&x| x != *menstrual_calendar_id);
		MentrualCalendarByOwner::<T>::insert(owner_id, menstrual_calendar);
	}

	// Add menstrual_calendar count
	pub fn add_menstrual_calendar_count() {
		let menstrual_calendar_count = <MentrualCalendarCount<T>>::get().unwrap_or(0);
		<MentrualCalendarCount<T>>::put(menstrual_calendar_count.wrapping_add(1));
	}

	// Add menstrual_calendar count by owner
	pub fn add_menstrual_calendar_count_by_owner(owner_id: &T::AccountId) {
		let menstrual_calendar_count = MentrualCalendarCountByOwner::<T>::get(owner_id).unwrap_or(0);
		MentrualCalendarCountByOwner::<T>::insert(owner_id, menstrual_calendar_count.wrapping_add(1))
	}

	// Subtract menstrual_calendar count
	pub fn sub_menstrual_calendar_count() {
		let menstrual_calendar_count = <MentrualCalendarCount<T>>::get().unwrap_or(1);
		MentrualCalendarCount::<T>::put(menstrual_calendar_count - 1);
	}

	// Subtract menstrual_calendar count by owner
	pub fn sub_menstrual_calendar_count_by_owner(owner_id: &T::AccountId) {
		let menstrual_calendar_count = MentrualCalendarCountByOwner::<T>::get(owner_id).unwrap_or(1);
		MentrualCalendarCountByOwner::<T>::insert(owner_id, menstrual_calendar_count - 1);
	}
}

/// MentrualCalendarProvider Trait Implementation
impl<T: Config> MentrualCalendarProvider<T> for Pallet<T> {
	type Error = Error<T>;
	type MentrualCalendar = MentrualCalendarOf<T>;

	fn menstrual_calendar_by_id(id: &T::Hash) -> Option<MentrualCalendarOf<T>> {
		<Self as MentrualCalendarInterface<T>>::menstrual_calendar_by_id(id)
	}
}
