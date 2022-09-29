#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod functions;
pub mod impl_services;
pub mod interface;
pub mod migrations;
pub mod types;
pub mod weights;

pub use interface::ServiceInterface;
pub use traits_services::{types::ServiceFlow, ServiceOwner, ServicesProvider};
pub use types::*;
pub use weights::WeightInfo;

pub use frame_support::traits::StorageVersion;

/// The current storage version.
const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use frame_support::{
		dispatch::DispatchResultWithPostInfo, pallet_prelude::*, traits::Currency,
	};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<<Self as frame_system::Config>::AccountId>;
		type ServiceOwner: ServiceOwner<Self>;
		type WeightInfo: WeightInfo;
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

	// ------- Storage -------------
	#[pallet::storage]
	#[pallet::getter(fn service_by_id)]
	pub type Services<T> = StorageMap<_, Blake2_128Concat, HashOf<T>, ServiceOf<T>>;
	//                                _,  Hasher         ,  Key     ,  Value

	#[pallet::storage]
	#[pallet::getter(fn services_count)]
	pub type ServicesCount<T> = StorageValue<_, u64>;

	#[pallet::storage]
	#[pallet::getter(fn services_count_by_owner)]
	pub type ServicesCountByOwner<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, u64>;
	// -----------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters, [Service, who]
		ServiceCreated(ServiceOf<T>, AccountIdOf<T>),
		//// Service updated
		/// parameters, [Service, who]
		ServiceUpdated(ServiceOf<T>, AccountIdOf<T>),
		//// Service deleted
		/// parameters, [Service, who]
		ServiceDeleted(ServiceOf<T>, AccountIdOf<T>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// User not allowed to create service
		NotAllowedToCreate,
		/// User is not the owner of a service
		NotServiceOwner,
		/// Ordering a service that does not exist
		ServiceDoesNotExist,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::create_service())]
		pub fn create_service(
			origin: OriginFor<T>,
			service_info: ServiceInfoOf<T>,
			service_flow: ServiceFlow,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as ServiceInterface<T>>::create_service(&who, &service_info, &service_flow)
			{
				Ok(service) => {
					Self::deposit_event(Event::ServiceCreated(service, who.clone()));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::update_service())]
		pub fn update_service(
			origin: OriginFor<T>,
			service_id: HashOf<T>,
			service_info: ServiceInfoOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			match <Self as ServiceInterface<T>>::update_service(&who, &service_id, &service_info) {
				Ok(service) => {
					Self::deposit_event(Event::ServiceUpdated(service, who.clone()));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::delete_service())]
		pub fn delete_service(
			origin: OriginFor<T>,
			service_id: T::Hash,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			match <Self as ServiceInterface<T>>::delete_service(&who, &service_id) {
				Ok(service) => {
					Self::deposit_event(Event::ServiceDeleted(service, who.clone()));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}
	}
}
