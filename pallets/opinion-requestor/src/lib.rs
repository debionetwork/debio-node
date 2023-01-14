#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod functions;
pub mod impl_opinion_requestor;
pub mod interface;
pub mod migrations;
pub mod types;
pub mod weights;

pub use types::*;

use frame_support::traits::StorageVersion;
use interface::OpinionRequestorInterface;
use weights::WeightInfo;

/// The current storage version.
const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;
	use traits_genetic_data::GeneticDataProvider;

	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type HashOf<T> = <T as frame_system::Config>::Hash;
	pub type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;
	pub type OpinionRequestorOf<T> = OpinionRequestor<AccountIdOf<T>, HashOf<T>, MomentOf<T>>;
	pub type RequestorInfoOf<T> = RequestorInfo<HashOf<T>>;

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

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type GeneticData: GeneticDataProvider<Self>;
		type OpinionRequestorWeightInfo: WeightInfo;
	}

	#[pallet::storage]
	#[pallet::getter(fn opinion_requestor_by_id)]
	pub type OpinionRequestors<T> =
		StorageMap<_, Blake2_128Concat, HashOf<T>, OpinionRequestorOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn opinion_requestor_by_owner)]
	pub type OpinionRequestorByOwner<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Vec<HashOf<T>>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn opinion_requestor_count)]
	pub type OpinionRequestorCount<T> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn opinion_requestor_count_by_owner)]
	pub type OpinionRequestorCountByOwner<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, u64, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		OpinionRequested(AccountIdOf<T>, OpinionRequestorOf<T>),
		OpinionRequestorInfoUpdated(AccountIdOf<T>, RequestorInfoOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		NotFound,
		Unauthorized,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::OpinionRequestorWeightInfo::request_opinion())]
		pub fn request_opinion(
			origin: OriginFor<T>,
			info: RequestorInfoOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as OpinionRequestorInterface<T>>::request_opinion(&who, info) {
				Ok(requestor) => {
					Self::deposit_event(Event::OpinionRequested(who, requestor));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::OpinionRequestorWeightInfo::update_requestor_info())]
		pub fn update_requestor_info(
			origin: OriginFor<T>,
			requestor_id: HashOf<T>,
			info: RequestorInfoOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as OpinionRequestorInterface<T>>::update_requestor_info(
				&requestor_id,
				&who,
				info,
			) {
				Ok(info) => {
					Self::deposit_event(Event::OpinionRequestorInfoUpdated(who, info));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}
	}
}
