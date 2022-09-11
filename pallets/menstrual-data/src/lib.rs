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
pub use interface::MenstrualDataInterface;
use sp_std::prelude::*;
use traits_menstrual_data::{MenstrualData as MenstrualDataT, MenstrualDataProvider};

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct MenstrualData<AccountId, Hash, Moment> {
	pub id: Hash,
	pub owner_id: AccountId,
	pub title: Vec<u8>,
	pub description: Vec<u8>,
	pub report_link: Vec<u8>,
	pub created_at: Moment,
	pub updated_at: Moment,
}

impl<AccountId, Hash, Moment: Default> MenstrualData<AccountId, Hash, Moment> {
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

impl<T, AccountId, Hash, Moment: Default> MenstrualDataT<T>
	for MenstrualData<AccountId, Hash, Moment>
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
	use crate::{interface::MenstrualDataInterface, weights::WeightInfo, MenstrualData};
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	pub use sp_std::prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type MenstrualDataWeightInfo: WeightInfo;
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
	pub type MenstrualDataOf<T> = MenstrualData<AccountIdOf<T>, HashOf<T>, MomentOf<T>>;
	pub type MenstrualDataIdOf<T> = HashOf<T>;

	// ------- Storage -------------
	#[pallet::storage]
	#[pallet::getter(fn menstrual_data_by_owner_id)]
	pub type MenstrualDataByOwner<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Vec<MenstrualDataIdOf<T>>>;

	#[pallet::storage]
	#[pallet::getter(fn menstrual_data_by_id)]
	pub type MenstrualDataById<T> =
		StorageMap<_, Blake2_128Concat, MenstrualDataIdOf<T>, MenstrualDataOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn menstrual_data_count_by_owner)]
	pub type MenstrualDataCountByOwner<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, u64>;

	#[pallet::storage]
	#[pallet::getter(fn menstrual_data_count)]
	pub type MenstrualDataCount<T> = StorageValue<_, u64>;
	//                                _,  Hasher         ,  Key     ,  Value
	// -----------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters, [MenstrualData, who]
		MenstrualDataAdded(MenstrualDataOf<T>, AccountIdOf<T>),
		//// MenstrualData updated
		/// parameters, [MenstrualData, who]
		MenstrualDataUpdated(MenstrualDataOf<T>, AccountIdOf<T>),
		//// MenstrualData deleted
		/// parameters, [MenstrualData, who]
		MenstrualDataRemoved(MenstrualDataOf<T>, AccountIdOf<T>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// User not allowed to create menstrual_data
		NotAllowedToCreate,
		/// User is not the owner of a menstrual_data
		NotMenstrualDataOwner,
		/// Ordering a menstrual_data that does not exist
		MenstrualDataDoesNotExist,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::MenstrualDataWeightInfo::add_menstrual_data())]
		pub fn add_menstrual_data(
			origin: OriginFor<T>,
			title: Vec<u8>,
			description: Vec<u8>,
			report_link: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as MenstrualDataInterface<T>>::add_menstrual_data(
				&who,
				&title,
				&description,
				&report_link,
			) {
				Ok(menstrual_data) => {
					Self::deposit_event(Event::MenstrualDataAdded(menstrual_data, who.clone()));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::MenstrualDataWeightInfo::update_menstrual_data())]
		pub fn update_menstrual_data(
			origin: OriginFor<T>,
			menstrual_data_id: HashOf<T>,
			title: Vec<u8>,
			description: Vec<u8>,
			report_link: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as MenstrualDataInterface<T>>::update_menstrual_data(
				&who,
				&menstrual_data_id,
				&title,
				&description,
				&report_link,
			) {
				Ok(menstrual_data) => {
					Self::deposit_event(Event::MenstrualDataUpdated(menstrual_data, who.clone()));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::MenstrualDataWeightInfo::remove_menstrual_data())]
		pub fn remove_menstrual_data(
			origin: OriginFor<T>,
			menstrual_data_id: HashOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as MenstrualDataInterface<T>>::remove_menstrual_data(
				&who,
				&menstrual_data_id,
			) {
				Ok(menstrual_data) => {
					Self::deposit_event(Event::MenstrualDataRemoved(menstrual_data, who.clone()));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}
	}
}

use frame_support::sp_runtime::traits::Hash;

/// MenstrualData Interface Implementation
impl<T: Config> MenstrualDataInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type MenstrualDataId = T::Hash;
	type MenstrualData = MenstrualDataOf<T>;

	fn generate_menstrual_data_id(
		owner_id: &T::AccountId,
		menstrual_data_count: u64,
	) -> Self::MenstrualDataId {
		let mut account_id_bytes = owner_id.encode();
		let mut menstrual_data_count_bytes = menstrual_data_count.encode();
		account_id_bytes.append(&mut menstrual_data_count_bytes);

		let seed = &account_id_bytes;
		T::Hashing::hash(seed)
	}

	fn add_menstrual_data(
		owner_id: &T::AccountId,
		title: &[u8],
		description: &[u8],
		report_link: &[u8],
	) -> Result<Self::MenstrualData, Self::Error> {
		let owner_menstrual_data_count =
			<Self as MenstrualDataInterface<T>>::menstrual_data_count_by_owner(owner_id);
		let menstrual_data_id =
			Self::generate_menstrual_data_id(owner_id, owner_menstrual_data_count);

		let now = pallet_timestamp::Pallet::<T>::get();

		let menstrual_data = MenstrualData::new(
			menstrual_data_id,
			owner_id.clone(),
			title.to_vec(),
			description.to_vec(),
			report_link.to_vec(),
			now,
		);

		// Store to MenstrualDataById storage
		MenstrualDataById::<T>::insert(menstrual_data_id, &menstrual_data);

		Self::add_menstrual_data_by_owner(owner_id, &menstrual_data_id);
		Self::add_menstrual_data_count();
		Self::add_menstrual_data_count_by_owner(owner_id);

		Ok(menstrual_data)
	}

	fn update_menstrual_data(
		owner_id: &T::AccountId,
		menstrual_data_id: &T::Hash,
		title: &[u8],
		description: &[u8],
		report_link: &[u8],
	) -> Result<Self::MenstrualData, Self::Error> {
		let menstrual_data = MenstrualDataById::<T>::get(menstrual_data_id);
		if menstrual_data.is_none() {
			return Err(Error::<T>::MenstrualDataDoesNotExist)
		}

		let mut menstrual_data = menstrual_data.unwrap();
		if menstrual_data.owner_id != owner_id.clone() {
			return Err(Error::<T>::NotMenstrualDataOwner)
		}

		let now = pallet_timestamp::Pallet::<T>::get();

		menstrual_data.title = title.to_vec();
		menstrual_data.description = description.to_vec();
		menstrual_data.report_link = report_link.to_vec();
		menstrual_data.updated_at = now;

		// Store to MenstrualDataById storage
		MenstrualDataById::<T>::insert(menstrual_data_id, &menstrual_data);

		Ok(menstrual_data)
	}

	fn remove_menstrual_data(
		owner_id: &T::AccountId,
		menstrual_data_id: &T::Hash,
	) -> Result<Self::MenstrualData, Self::Error> {
		let menstrual_data = MenstrualDataById::<T>::get(menstrual_data_id);
		if menstrual_data.is_none() {
			return Err(Error::<T>::MenstrualDataDoesNotExist)
		}

		let menstrual_data = menstrual_data.unwrap();
		if menstrual_data.owner_id != owner_id.clone() {
			return Err(Error::<T>::NotMenstrualDataOwner)
		}

		// Remove menstrual_data from storage
		MenstrualDataById::<T>::take(menstrual_data_id).unwrap();

		Self::sub_menstrual_data_by_owner(menstrual_data.get_owner_id(), menstrual_data_id);
		Self::sub_menstrual_data_count();
		Self::sub_menstrual_data_count_by_owner(menstrual_data.get_owner_id());

		Ok(menstrual_data)
	}

	fn menstrual_data_by_owner_id(owner_id: &T::AccountId) -> Option<Vec<T::Hash>> {
		MenstrualDataByOwner::<T>::get(owner_id)
	}

	fn menstrual_data_count_by_owner(owner_id: &T::AccountId) -> u64 {
		MenstrualDataCountByOwner::<T>::get(owner_id).unwrap_or(0)
	}

	fn menstrual_data_by_id(
		menstrual_data_id: &Self::MenstrualDataId,
	) -> Option<Self::MenstrualData> {
		MenstrualDataById::<T>::get(menstrual_data_id)
	}
}

/// Pallet Methods
impl<T: Config> Pallet<T> {
	// Add menstrual_data by owner
	pub fn add_menstrual_data_by_owner(owner_id: &T::AccountId, menstrual_data_id: &T::Hash) {
		let mut menstrual_data = MenstrualDataByOwner::<T>::get(owner_id).unwrap_or_default();

		menstrual_data.push(*menstrual_data_id);
		MenstrualDataByOwner::<T>::insert(owner_id, &menstrual_data)
	}

	// Subtract menstrual_data by owner
	pub fn sub_menstrual_data_by_owner(owner_id: &T::AccountId, menstrual_data_id: &T::Hash) {
		let mut menstrual_data = MenstrualDataByOwner::<T>::get(owner_id).unwrap_or_default();
		menstrual_data.retain(|&x| x != *menstrual_data_id);
		MenstrualDataByOwner::<T>::insert(owner_id, menstrual_data);
	}

	// Add menstrual_data count
	pub fn add_menstrual_data_count() {
		let menstrual_data_count = <MenstrualDataCount<T>>::get().unwrap_or(0);
		<MenstrualDataCount<T>>::put(menstrual_data_count.wrapping_add(1));
	}

	// Add menstrual_data count by owner
	pub fn add_menstrual_data_count_by_owner(owner_id: &T::AccountId) {
		let menstrual_data_count = MenstrualDataCountByOwner::<T>::get(owner_id).unwrap_or(0);
		MenstrualDataCountByOwner::<T>::insert(owner_id, menstrual_data_count.wrapping_add(1))
	}

	// Subtract menstrual_data count
	pub fn sub_menstrual_data_count() {
		let menstrual_data_count = <MenstrualDataCount<T>>::get().unwrap_or(1);
		MenstrualDataCount::<T>::put(menstrual_data_count - 1);
	}

	// Subtract menstrual_data count by owner
	pub fn sub_menstrual_data_count_by_owner(owner_id: &T::AccountId) {
		let menstrual_data_count = MenstrualDataCountByOwner::<T>::get(owner_id).unwrap_or(1);
		MenstrualDataCountByOwner::<T>::insert(owner_id, menstrual_data_count - 1);
	}
}

/// MenstrualDataProvider Trait Implementation
impl<T: Config> MenstrualDataProvider<T> for Pallet<T> {
	type Error = Error<T>;
	type MenstrualData = MenstrualDataOf<T>;

	fn menstrual_data_by_id(id: &T::Hash) -> Option<MenstrualDataOf<T>> {
		<Self as MenstrualDataInterface<T>>::menstrual_data_by_id(id)
	}
}
