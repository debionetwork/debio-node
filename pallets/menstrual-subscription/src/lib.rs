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
use primitives_duration::MenstrualSubscriptionDuration;
use primitives_menstrual_status::{MenstrualSubscriptionStatus, PaymentStatus};
use sp_std::prelude::*;
use traits_menstrual_subscription::{
	MenstrualSubscription as MenstrualSubscriptionT, MenstrualSubscriptionProvider,
};

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct MenstrualSubscription<AccountId, Hash, Moment> {
	pub id: Hash,
	pub address_id: AccountId,
	pub duration: MenstrualSubscriptionDuration,
	pub price: u8,
	pub payment_status: PaymentStatus,
	pub status: MenstrualSubscriptionStatus,
	pub created_at: Moment,
	pub updated_at: Moment,
}

impl<AccountId, Hash, Moment: Default> MenstrualSubscription<AccountId, Hash, Moment> {
	pub fn new(
		id: Hash,
		address_id: AccountId,
		duration: MenstrualSubscriptionDuration,
		price: u8,
		payment_status: PaymentStatus,
		status: MenstrualSubscriptionStatus,
		created_at: Moment,
	) -> Self {
		Self {
			id,
			address_id,
			duration,
			price,
			payment_status,
			status,
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

impl<T, AccountId, Hash, Moment: Default> MenstrualSubscriptionT<T>
	for MenstrualSubscription<AccountId, Hash, Moment>
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
	use crate::{
		interface::MenstrualSubscriptionInterface, weights::WeightInfo, MenstrualSubscription,
		MenstrualSubscriptionDuration, MenstrualSubscriptionStatus, PaymentStatus,
	};
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	pub use sp_std::prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type MenstrualSubscriptionWeightInfo: WeightInfo;
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub admin_key: Option<T::AccountId>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { admin_key: None }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			if let Some(ref admin_key) = self.admin_key {
				AdminKey::<T>::put(admin_key);
			}
		}
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
	#[pallet::getter(fn admin_key)]
	pub type AdminKey<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn menstrual_subscription_by_address_id)]
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
		/// Update menstrual subscription admin key successful
		/// parameters. [who]
		UpdateMenstrualSubscriptionAdminKeySuccessful(AccountIdOf<T>),
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
		// Unauthorized access of an Admin key
		Unauthorized,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::MenstrualSubscriptionWeightInfo::add_menstrual_subscription())]
		pub fn add_menstrual_subscription(
			origin: OriginFor<T>,
			duration: MenstrualSubscriptionDuration,
			price: u8,
			payment_status: PaymentStatus,
			status: MenstrualSubscriptionStatus,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as MenstrualSubscriptionInterface<T>>::add_menstrual_subscription(
				&who,
				&duration,
				&price,
				&payment_status,
				&status,
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

		#[pallet::weight(T::MenstrualSubscriptionWeightInfo::change_menstrual_subscription_status())]
		pub fn change_menstrual_subscription_status(
			origin: OriginFor<T>,
			account_id: T::AccountId,
			menstrual_subscription_id: HashOf<T>,
			status: MenstrualSubscriptionStatus,
		) -> DispatchResultWithPostInfo {
			let admin = ensure_signed(origin)?;

			ensure!(admin == AdminKey::<T>::get().unwrap(), Error::<T>::Unauthorized);

			match <Self as MenstrualSubscriptionInterface<T>>::change_menstrual_subscription_status(
				&account_id,
				&menstrual_subscription_id,
				&status,
			) {
				Ok(menstrual_subscription) => {
					Self::deposit_event(Event::MenstrualSubscriptionUpdated(
						menstrual_subscription,
						account_id.clone(),
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::MenstrualSubscriptionWeightInfo::set_menstrual_subscription_paid())]
		pub fn set_menstrual_subscription_paid(
			origin: OriginFor<T>,
			account_id: T::AccountId,
			menstrual_subscription_id: HashOf<T>,
		) -> DispatchResultWithPostInfo {
			let admin = ensure_signed(origin)?;

			ensure!(admin == AdminKey::<T>::get().unwrap(), Error::<T>::Unauthorized);

			match <Self as MenstrualSubscriptionInterface<T>>::set_menstrual_subscription_paid(
				&account_id,
				&menstrual_subscription_id,
			) {
				Ok(menstrual_subscription) => {
					Self::deposit_event(Event::MenstrualSubscriptionRemoved(
						menstrual_subscription,
						account_id.clone(),
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::MenstrualSubscriptionWeightInfo::set_menstrual_subscription_paid())]
		pub fn update_admin_key(
			origin: OriginFor<T>,
			account_id: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			ensure!(who == AdminKey::<T>::get().unwrap(), Error::<T>::Unauthorized);

			AdminKey::<T>::put(&account_id);

			Self::deposit_event(Event::UpdateMenstrualSubscriptionAdminKeySuccessful(account_id));

			Ok(Pays::No.into())
		}

		#[pallet::weight(0)]
		pub fn sudo_update_admin_key(
			origin: OriginFor<T>,
			account_id: T::AccountId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			AdminKey::<T>::put(&account_id);

			Self::deposit_event(Event::UpdateMenstrualSubscriptionAdminKeySuccessful(account_id));

			Ok(Pays::No.into())
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
		address_id: &T::AccountId,
		menstrual_subscription_count: u64,
	) -> Self::MenstrualSubscriptionId {
		let mut account_id_bytes = address_id.encode();
		let mut menstrual_subscription_count_bytes = menstrual_subscription_count.encode();
		account_id_bytes.append(&mut menstrual_subscription_count_bytes);

		let seed = &account_id_bytes;
		T::Hashing::hash(seed)
	}

	fn add_menstrual_subscription(
		address_id: &T::AccountId,
		duration: &MenstrualSubscriptionDuration,
		price: &u8,
		payment_status: &PaymentStatus,
		status: &MenstrualSubscriptionStatus,
	) -> Result<Self::MenstrualSubscription, Self::Error> {
		let owner_menstrual_subscription_count =
			<Self as MenstrualSubscriptionInterface<T>>::menstrual_subscription_count_by_owner(
				address_id,
			);
		let menstrual_subscription_id = Self::generate_menstrual_subscription_id(
			address_id,
			owner_menstrual_subscription_count,
		);

		let now = pallet_timestamp::Pallet::<T>::get();

		let menstrual_subscription = MenstrualSubscription::new(
			menstrual_subscription_id,
			address_id.clone(),
			duration.clone(),
			*price,
			payment_status.clone(),
			status.clone(),
			now,
		);

		// Store to MenstrualSubscriptionById storage
		MenstrualSubscriptionById::<T>::insert(menstrual_subscription_id, &menstrual_subscription);

		Self::add_menstrual_subscription_by_owner(address_id, &menstrual_subscription_id);
		Self::add_menstrual_subscription_count();
		Self::add_menstrual_subscription_count_by_owner(address_id);

		Ok(menstrual_subscription)
	}

	fn change_menstrual_subscription_status(
		address_id: &T::AccountId,
		menstrual_subscription_id: &T::Hash,
		status: &MenstrualSubscriptionStatus,
	) -> Result<Self::MenstrualSubscription, Self::Error> {
		let menstrual_subscription = MenstrualSubscriptionById::<T>::get(menstrual_subscription_id);
		if menstrual_subscription.is_none() {
			return Err(Error::<T>::MenstrualSubscriptionDoesNotExist)
		}

		let mut menstrual_subscription = menstrual_subscription.unwrap();
		if menstrual_subscription.address_id != address_id.clone() {
			return Err(Error::<T>::NotMenstrualSubscriptionOwner)
		}

		let now = pallet_timestamp::Pallet::<T>::get();

		menstrual_subscription.status = status.clone();
		menstrual_subscription.updated_at = now;

		// Store to MenstrualSubscriptionById storage
		MenstrualSubscriptionById::<T>::insert(menstrual_subscription_id, &menstrual_subscription);

		Ok(menstrual_subscription)
	}

	fn set_menstrual_subscription_paid(
		address_id: &T::AccountId,
		menstrual_subscription_id: &T::Hash,
	) -> Result<Self::MenstrualSubscription, Self::Error> {
		let menstrual_subscription = MenstrualSubscriptionById::<T>::get(menstrual_subscription_id);
		if menstrual_subscription.is_none() {
			return Err(Error::<T>::MenstrualSubscriptionDoesNotExist)
		}

		let mut menstrual_subscription = menstrual_subscription.unwrap();
		if menstrual_subscription.address_id != address_id.clone() {
			return Err(Error::<T>::NotMenstrualSubscriptionOwner)
		}

		let now = pallet_timestamp::Pallet::<T>::get();

		menstrual_subscription.payment_status = PaymentStatus::Paid;
		menstrual_subscription.updated_at = now;

		// Store to MenstrualSubscriptionById storage
		MenstrualSubscriptionById::<T>::insert(menstrual_subscription_id, &menstrual_subscription);

		Ok(menstrual_subscription)
	}

	fn menstrual_subscription_by_address_id(address_id: &T::AccountId) -> Option<Vec<T::Hash>> {
		MenstrualSubscriptionByOwner::<T>::get(address_id)
	}

	fn menstrual_subscription_count_by_owner(address_id: &T::AccountId) -> u64 {
		MenstrualSubscriptionCountByOwner::<T>::get(address_id).unwrap_or(0)
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
		address_id: &T::AccountId,
		menstrual_subscription_id: &T::Hash,
	) {
		let mut menstrual_subscription =
			MenstrualSubscriptionByOwner::<T>::get(address_id).unwrap_or_default();

		menstrual_subscription.push(*menstrual_subscription_id);
		MenstrualSubscriptionByOwner::<T>::insert(address_id, &menstrual_subscription)
	}

	// Subtract menstrual_subscription by owner
	pub fn sub_menstrual_subscription_by_owner(
		address_id: &T::AccountId,
		menstrual_subscription_id: &T::Hash,
	) {
		let mut menstrual_subscription =
			MenstrualSubscriptionByOwner::<T>::get(address_id).unwrap_or_default();
		menstrual_subscription.retain(|&x| x != *menstrual_subscription_id);
		MenstrualSubscriptionByOwner::<T>::insert(address_id, menstrual_subscription);
	}

	// Add menstrual_subscription count
	pub fn add_menstrual_subscription_count() {
		let menstrual_subscription_count = <MenstrualSubscriptionCount<T>>::get().unwrap_or(0);
		<MenstrualSubscriptionCount<T>>::put(menstrual_subscription_count.wrapping_add(1));
	}

	// Add menstrual_subscription count by owner
	pub fn add_menstrual_subscription_count_by_owner(address_id: &T::AccountId) {
		let menstrual_subscription_count =
			MenstrualSubscriptionCountByOwner::<T>::get(address_id).unwrap_or(0);
		MenstrualSubscriptionCountByOwner::<T>::insert(
			address_id,
			menstrual_subscription_count.wrapping_add(1),
		)
	}

	// Subtract menstrual_subscription count
	pub fn sub_menstrual_subscription_count() {
		let menstrual_subscription_count = <MenstrualSubscriptionCount<T>>::get().unwrap_or(1);
		MenstrualSubscriptionCount::<T>::put(menstrual_subscription_count - 1);
	}

	// Subtract menstrual_subscription count by owner
	pub fn sub_menstrual_subscription_count_by_owner(address_id: &T::AccountId) {
		let menstrual_subscription_count =
			MenstrualSubscriptionCountByOwner::<T>::get(address_id).unwrap_or(1);
		MenstrualSubscriptionCountByOwner::<T>::insert(
			address_id,
			menstrual_subscription_count - 1,
		);
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
