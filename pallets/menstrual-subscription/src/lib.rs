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
pub use interface::MenstrualSubscriptionInterface;
use sp_std::prelude::*;
use traits_menstrual_subscription::{
	MenstrualSubscription as MenstrualSubscriptionT, MenstrualSubscriptionProvider,
};

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct MenstrualSubscription<AccountId, Hash, Moment> {
	pub id: Hash,
	pub owner_id: AccountId,
	pub title: Vec<u8>,
	pub description: Vec<u8>,
	pub report_link: Vec<u8>,
	pub created_at: Moment,
	pub updated_at: Moment,
}

impl<AccountId, Hash, Moment: Default> MenstrualSubscription<AccountId, Hash, Moment> {
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

impl<T, AccountId, Hash, Moment: Default> MenstrualSubscriptionT<T>
	for MenstrualSubscription<AccountId, Hash, Moment>
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
	use crate::{
		interface::MenstrualSubscriptionInterface, weights::WeightInfo, MenstrualSubscription,
	};
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	pub use sp_std::prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type MenstrualSubscriptionWeightInfo: WeightInfo;
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
	pub type MenstrualSubscriptionOf<T> =
		MenstrualSubscription<AccountIdOf<T>, HashOf<T>, MomentOf<T>>;
	pub type MenstrualSubscriptionIdOf<T> = HashOf<T>;

	// ------- Storage -------------
	#[pallet::storage]
	#[pallet::getter(fn menstrual_subscription_by_owner_id)]
	pub type MenstrualSubscriptionByOwner<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Vec<MenstrualSubscriptionIdOf<T>>>;

	#[pallet::storage]
	#[pallet::getter(fn menstrual_subscription_by_id)]
	pub type MenstrualSubscriptionById<T> =
		StorageMap<_, Blake2_128Concat, MenstrualSubscriptionIdOf<T>, MenstrualSubscriptionOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn menstrual_subscription_count_by_owner)]
	pub type MenstrualSubscriptionCountByOwner<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, u64>;

	#[pallet::storage]
	#[pallet::getter(fn menstrual_subscription_count)]
	pub type MenstrualSubscriptionCount<T> = StorageValue<_, u64>;
	//                                _,  Hasher         ,  Key     ,  Value
	// -----------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters, [MenstrualSubscription, who]
		MenstrualSubscriptionAdded(MenstrualSubscriptionOf<T>, AccountIdOf<T>),
		//// MenstrualSubscription updated
		/// parameters, [MenstrualSubscription, who]
		MenstrualSubscriptionUpdated(MenstrualSubscriptionOf<T>, AccountIdOf<T>),
		//// MenstrualSubscription deleted
		/// parameters, [MenstrualSubscription, who]
		MenstrualSubscriptionRemoved(MenstrualSubscriptionOf<T>, AccountIdOf<T>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// User not allowed to create menstrual_subscription
		NotAllowedToCreate,
		/// User is not the owner of a menstrual_subscription
		NotMenstrualSubscriptionOwner,
		/// Ordering a menstrual_subscription that does not exist
		MenstrualSubscriptionDoesNotExist,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::MenstrualSubscriptionWeightInfo::add_menstrual_subscription())]
		pub fn add_menstrual_subscription(
			origin: OriginFor<T>,
			title: Vec<u8>,
			description: Vec<u8>,
			report_link: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as MenstrualSubscriptionInterface<T>>::add_menstrual_subscription(
				&who,
				&title,
				&description,
				&report_link,
			) {
				Ok(menstrual_subscription) => {
					Self::deposit_event(Event::MenstrualSubscriptionAdded(
						menstrual_subscription,
						who.clone(),
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::MenstrualSubscriptionWeightInfo::update_menstrual_subscription())]
		pub fn update_menstrual_subscription(
			origin: OriginFor<T>,
			menstrual_subscription_id: HashOf<T>,
			title: Vec<u8>,
			description: Vec<u8>,
			report_link: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as MenstrualSubscriptionInterface<T>>::update_menstrual_subscription(
				&who,
				&menstrual_subscription_id,
				&title,
				&description,
				&report_link,
			) {
				Ok(menstrual_subscription) => {
					Self::deposit_event(Event::MenstrualSubscriptionUpdated(
						menstrual_subscription,
						who.clone(),
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::MenstrualSubscriptionWeightInfo::remove_menstrual_subscription())]
		pub fn remove_menstrual_subscription(
			origin: OriginFor<T>,
			menstrual_subscription_id: HashOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as MenstrualSubscriptionInterface<T>>::remove_menstrual_subscription(
				&who,
				&menstrual_subscription_id,
			) {
				Ok(menstrual_subscription) => {
					Self::deposit_event(Event::MenstrualSubscriptionRemoved(
						menstrual_subscription,
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

/// MenstrualSubscription Interface Implementation
impl<T: Config> MenstrualSubscriptionInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type MenstrualSubscriptionId = T::Hash;
	type MenstrualSubscription = MenstrualSubscriptionOf<T>;

	fn generate_menstrual_subscription_id(
		owner_id: &T::AccountId,
		menstrual_subscription_count: u64,
	) -> Self::MenstrualSubscriptionId {
		let mut account_id_bytes = owner_id.encode();
		let mut menstrual_subscription_count_bytes = menstrual_subscription_count.encode();
		account_id_bytes.append(&mut menstrual_subscription_count_bytes);

		let seed = &account_id_bytes;
		T::Hashing::hash(seed)
	}

	fn add_menstrual_subscription(
		owner_id: &T::AccountId,
		title: &[u8],
		description: &[u8],
		report_link: &[u8],
	) -> Result<Self::MenstrualSubscription, Self::Error> {
		let owner_menstrual_subscription_count =
			<Self as MenstrualSubscriptionInterface<T>>::menstrual_subscription_count_by_owner(
				owner_id,
			);
		let menstrual_subscription_id =
			Self::generate_menstrual_subscription_id(owner_id, owner_menstrual_subscription_count);

		let now = pallet_timestamp::Pallet::<T>::get();

		let menstrual_subscription = MenstrualSubscription::new(
			menstrual_subscription_id,
			owner_id.clone(),
			title.to_vec(),
			description.to_vec(),
			report_link.to_vec(),
			now,
		);

		// Store to MenstrualSubscriptionById storage
		MenstrualSubscriptionById::<T>::insert(menstrual_subscription_id, &menstrual_subscription);

		Self::add_menstrual_subscription_by_owner(owner_id, &menstrual_subscription_id);
		Self::add_menstrual_subscription_count();
		Self::add_menstrual_subscription_count_by_owner(owner_id);

		Ok(menstrual_subscription)
	}

	fn update_menstrual_subscription(
		owner_id: &T::AccountId,
		menstrual_subscription_id: &T::Hash,
		title: &[u8],
		description: &[u8],
		report_link: &[u8],
	) -> Result<Self::MenstrualSubscription, Self::Error> {
		let menstrual_subscription = MenstrualSubscriptionById::<T>::get(menstrual_subscription_id);
		if menstrual_subscription.is_none() {
			return Err(Error::<T>::MenstrualSubscriptionDoesNotExist)
		}

		let mut menstrual_subscription = menstrual_subscription.unwrap();
		if menstrual_subscription.owner_id != owner_id.clone() {
			return Err(Error::<T>::NotMenstrualSubscriptionOwner)
		}

		let now = pallet_timestamp::Pallet::<T>::get();

		menstrual_subscription.title = title.to_vec();
		menstrual_subscription.description = description.to_vec();
		menstrual_subscription.report_link = report_link.to_vec();
		menstrual_subscription.updated_at = now;

		// Store to MenstrualSubscriptionById storage
		MenstrualSubscriptionById::<T>::insert(menstrual_subscription_id, &menstrual_subscription);

		Ok(menstrual_subscription)
	}

	fn remove_menstrual_subscription(
		owner_id: &T::AccountId,
		menstrual_subscription_id: &T::Hash,
	) -> Result<Self::MenstrualSubscription, Self::Error> {
		let menstrual_subscription = MenstrualSubscriptionById::<T>::get(menstrual_subscription_id);
		if menstrual_subscription.is_none() {
			return Err(Error::<T>::MenstrualSubscriptionDoesNotExist)
		}

		let menstrual_subscription = menstrual_subscription.unwrap();
		if menstrual_subscription.owner_id != owner_id.clone() {
			return Err(Error::<T>::NotMenstrualSubscriptionOwner)
		}

		// Remove menstrual_subscription from storage
		MenstrualSubscriptionById::<T>::take(menstrual_subscription_id).unwrap();

		Self::sub_menstrual_subscription_by_owner(
			menstrual_subscription.get_owner_id(),
			menstrual_subscription_id,
		);
		Self::sub_menstrual_subscription_count();
		Self::sub_menstrual_subscription_count_by_owner(menstrual_subscription.get_owner_id());

		Ok(menstrual_subscription)
	}

	fn menstrual_subscription_by_owner_id(owner_id: &T::AccountId) -> Option<Vec<T::Hash>> {
		MenstrualSubscriptionByOwner::<T>::get(owner_id)
	}

	fn menstrual_subscription_count_by_owner(owner_id: &T::AccountId) -> u64 {
		MenstrualSubscriptionCountByOwner::<T>::get(owner_id).unwrap_or(0)
	}

	fn menstrual_subscription_by_id(
		menstrual_subscription_id: &Self::MenstrualSubscriptionId,
	) -> Option<Self::MenstrualSubscription> {
		MenstrualSubscriptionById::<T>::get(menstrual_subscription_id)
	}
}

/// Pallet Methods
impl<T: Config> Pallet<T> {
	// Add menstrual_subscription by owner
	pub fn add_menstrual_subscription_by_owner(
		owner_id: &T::AccountId,
		menstrual_subscription_id: &T::Hash,
	) {
		let mut menstrual_subscription =
			MenstrualSubscriptionByOwner::<T>::get(owner_id).unwrap_or_default();

		menstrual_subscription.push(*menstrual_subscription_id);
		MenstrualSubscriptionByOwner::<T>::insert(owner_id, &menstrual_subscription)
	}

	// Subtract menstrual_subscription by owner
	pub fn sub_menstrual_subscription_by_owner(
		owner_id: &T::AccountId,
		menstrual_subscription_id: &T::Hash,
	) {
		let mut menstrual_subscription =
			MenstrualSubscriptionByOwner::<T>::get(owner_id).unwrap_or_default();
		menstrual_subscription.retain(|&x| x != *menstrual_subscription_id);
		MenstrualSubscriptionByOwner::<T>::insert(owner_id, menstrual_subscription);
	}

	// Add menstrual_subscription count
	pub fn add_menstrual_subscription_count() {
		let menstrual_subscription_count = <MenstrualSubscriptionCount<T>>::get().unwrap_or(0);
		<MenstrualSubscriptionCount<T>>::put(menstrual_subscription_count.wrapping_add(1));
	}

	// Add menstrual_subscription count by owner
	pub fn add_menstrual_subscription_count_by_owner(owner_id: &T::AccountId) {
		let menstrual_subscription_count =
			MenstrualSubscriptionCountByOwner::<T>::get(owner_id).unwrap_or(0);
		MenstrualSubscriptionCountByOwner::<T>::insert(
			owner_id,
			menstrual_subscription_count.wrapping_add(1),
		)
	}

	// Subtract menstrual_subscription count
	pub fn sub_menstrual_subscription_count() {
		let menstrual_subscription_count = <MenstrualSubscriptionCount<T>>::get().unwrap_or(1);
		MenstrualSubscriptionCount::<T>::put(menstrual_subscription_count - 1);
	}

	// Subtract menstrual_subscription count by owner
	pub fn sub_menstrual_subscription_count_by_owner(owner_id: &T::AccountId) {
		let menstrual_subscription_count =
			MenstrualSubscriptionCountByOwner::<T>::get(owner_id).unwrap_or(1);
		MenstrualSubscriptionCountByOwner::<T>::insert(owner_id, menstrual_subscription_count - 1);
	}
}

/// MenstrualSubscriptionProvider Trait Implementation
impl<T: Config> MenstrualSubscriptionProvider<T> for Pallet<T> {
	type Error = Error<T>;
	type MenstrualSubscription = MenstrualSubscriptionOf<T>;

	fn menstrual_subscription_by_id(id: &T::Hash) -> Option<MenstrualSubscriptionOf<T>> {
		<Self as MenstrualSubscriptionInterface<T>>::menstrual_subscription_by_id(id)
	}
}
