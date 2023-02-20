#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod functions;
pub mod impl_opinion;
pub mod interface;
pub mod types;
pub mod weights;

pub use types::*;

use frame_support::traits::StorageVersion;
use interface::OpinionInterface;
use weights::WeightInfo;

/// The current storage version.
const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use frame_support::{
		pallet_prelude::*,
		traits::{fungibles, Currency},
	};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;
	use traits_opinion_requestor::OpinionRequestorProvider;

	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type HashOf<T> = <T as frame_system::Config>::Hash;
	pub type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;
	pub type OpinionOf<T> = Opinion<AccountIdOf<T>, HashOf<T>, MomentOf<T>>;

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Currency: Currency<<Self as frame_system::Config>::AccountId>;
		type Assets: fungibles::InspectMetadata<
			<Self as frame_system::Config>::AccountId,
			AssetId = u32,
			Balance = u128,
		>;
		type OpinionRequestor: OpinionRequestorProvider<Self>;
		type OpinionWeightInfo: WeightInfo;
	}

	#[pallet::storage]
	#[pallet::getter(fn opinion_by_id)]
	pub type Opinions<T> = StorageMap<_, Blake2_128Concat, HashOf<T>, OpinionOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn opinion_by_owner)]
	pub type OpinionByOwner<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Vec<HashOf<T>>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn opinion_count)]
	pub type OpinionCount<T> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn opinion_count_by_owner)]
	pub type OpinionCountByOwner<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn admin_key)]
	pub type OpinionAdminKey<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		OpinionAdded(AccountIdOf<T>, OpinionOf<T>),
		OpinionUpdated(AccountIdOf<T>, OpinionOf<T>),
		OpinionRemoved(AccountIdOf<T>, HashOf<T>),
		OpinionStatusUpdated(AccountIdOf<T>, HashOf<T>, Status),
		AdminKeyUpdated(AccountIdOf<T>),
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
				OpinionAdminKey::<T>::put(admin_key);
			}
		}
	}

	#[pallet::error]
	pub enum Error<T> {
		NotFound,
		Unauthorized,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::OpinionWeightInfo::create())]
		pub fn create(
			origin: OriginFor<T>,
			requestor_id: HashOf<T>,
			account_id: AccountIdOf<T>,
			info: OpinionInfo,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as OpinionInterface<T>>::add_opinion(
				&who,
				&requestor_id,
				&account_id,
				&info,
			) {
				Ok(opinion) => {
					Self::deposit_event(Event::OpinionAdded(who, opinion));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::OpinionWeightInfo::update())]
		pub fn update(
			origin: OriginFor<T>,
			opinion_id: HashOf<T>,
			info: OpinionInfo,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as OpinionInterface<T>>::update_opinion(&who, &opinion_id, &info) {
				Ok(opinion) => {
					Self::deposit_event(Event::OpinionUpdated(who, opinion));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::OpinionWeightInfo::delete())]
		pub fn delete(origin: OriginFor<T>, opinion_id: HashOf<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as OpinionInterface<T>>::remove_opinion(&who, &opinion_id) {
				Ok(()) => {
					Self::deposit_event(Event::OpinionRemoved(who, opinion_id));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::OpinionWeightInfo::update_status())]
		pub fn update_status(
			origin: OriginFor<T>,
			opinion_id: HashOf<T>,
			status: Status,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as OpinionInterface<T>>::update_status(&who, &opinion_id, &status) {
				Ok(()) => {
					Self::deposit_event(Event::OpinionStatusUpdated(who, opinion_id, status));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::OpinionWeightInfo::update_admin_key())]
		pub fn update_admin_key(
			origin: OriginFor<T>,
			account_id: AccountIdOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as OpinionInterface<T>>::update_admin_key(&who, &account_id) {
				Ok(_) => {
					Self::deposit_event(Event::AdminKeyUpdated(account_id));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(0)]
		pub fn sudo_update_admin_key(
			origin: OriginFor<T>,
			account_id: T::AccountId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			OpinionAdminKey::<T>::put(&account_id);

			Self::deposit_event(Event::AdminKeyUpdated(account_id));

			Ok(Pays::No.into())
		}
	}
}
