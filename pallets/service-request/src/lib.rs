#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use frame_support::{
	codec::{Decode, Encode},
	dispatch::DispatchResultWithPostInfo,
	pallet_prelude::*,
	sp_runtime::{traits::Hash, RuntimeDebug},
	sp_std::prelude::*,
	traits::{Currency, Get, UnixTime},
};
use frame_system::pallet_prelude::*;
use traits_labs::LabsProvider;
use traits_order::OrderProvider;
use traits_services::ServicesProvider;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod functions;
pub mod impl_service_request;
pub mod interface;
pub mod migrations;
pub mod types;
pub mod weights;

pub use interface::SeviceRequestInterface;
pub use types::*;
pub use weights::WeightInfo;

pub use frame_support::traits::StorageVersion;

/// The current storage version.
const STORAGE_VERSION: StorageVersion = StorageVersion::new(2);

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type TimeProvider: UnixTime;
		type Currency: Currency<<Self as frame_system::Config>::AccountId>;
		type Labs: LabsProvider<Self>;
		type Services: ServicesProvider<Self, BalanceOf<Self>>;
		type Orders: OrderProvider<Self>;
		type ServiceRequestWeightInfo: WeightInfo;

		#[pallet::constant]
		type UnstakePeriode: Get<u64>;
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

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ServiceRequestCreated(AccountIdOf<T>, RequestOf<T>),
		ServiceRequestUpdated(HashOf<T>, RequestStatus, Option<RequestOf<T>>),
		StakingAmountRefunded(AccountIdOf<T>, HashOf<T>, BalanceOf<T>),
		UpdateServiceRequestAdminKeySuccessful(AccountIdOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		BadSignature,
		Unauthorized,
		NotValidAmount,
		RequestNotFound,
		RequestUnableToClaimed,
		RequestUnableToUnstake,
		RequestUnableToRetrieveUnstake,
		RequestUnableToProccess,
		RequestUnableToFinalize,
		RequestWaitingForUnstaked,
		RequestAlreadyUnstaked,
		RequestAlreadyClaimed,
		RequestAlreadyProccessed,
		RequestAlreadyFinalized,
		RequestAlreadyInList,
		LabNotFound,
		Module,
		Other,
		BadOrigin,
		CannotLookup,
		ConsumerRemaining,
		TooManyConsumers,
		NoProviders,
		Token,
		Arithmetic,
		WrongFormat,
		AssetNotExists,
		OrderNotFound,
		ServiceNotFound,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_runtime_upgrade() -> Weight {
			migrations::migrate::<T>()
		}
	}

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn admin_key)]
	pub type AdminKey<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	/// Get Staking Account Id by Request Id
	#[pallet::storage]
	#[pallet::getter(fn staking_account_id_by_request_id)]
	pub type StakingAccountIdByRequestId<T> =
		StorageMap<_, Blake2_128Concat, HashOf<T>, AccountIdOf<T>, OptionQuery>;

	/// Get Request by Account Id
	#[pallet::storage]
	#[pallet::getter(fn request_by_account_id)]
	pub type RequestByAccountId<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Vec<HashOf<T>>, ValueQuery>;

	/// Get Request by RequestId
	#[pallet::storage]
	#[pallet::getter(fn request_by_id)]
	pub type RequestById<T> = StorageMap<_, Blake2_128Concat, HashOf<T>, RequestOf<T>>;

	/// Get Request by LabId
	#[pallet::storage]
	#[pallet::getter(fn requests_by_lab_id)]
	pub type RequestsByLabId<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Vec<HashOf<T>>, ValueQuery>;

	/// Get Request by OrderId
	#[pallet::storage]
	#[pallet::getter(fn request_by_order_id)]
	pub type RequestByOrderId<T> = StorageMap<_, Blake2_128Concat, HashOf<T>, HashOf<T>>;

	/// Get  ServiceCountRequest by Country, Region, City, ServiceCategoryOf
	#[pallet::storage]
	#[pallet::getter(fn service_count_request)]
	pub type ServiceCountRequest<T> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, Country>,
			NMapKey<Blake2_128Concat, Region>,
			NMapKey<Blake2_128Concat, City>,
			NMapKey<Blake2_128Concat, ServiceCategory>,
		),
		u64,
		ValueQuery,
	>;

	// Request by order id

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::ServiceRequestWeightInfo::create_request())]
		pub fn create_request(
			origin: OriginFor<T>,
			country: Vec<u8>,
			region: Vec<u8>,
			city: Vec<u8>,
			service_category: Vec<u8>,
			staking_amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as SeviceRequestInterface<T>>::create_request(
				&who,
				country,
				region,
				city,
				service_category,
				staking_amount,
			) {
				Ok(request) => {
					Self::deposit_event(Event::ServiceRequestCreated(who, request));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::ServiceRequestWeightInfo::unstake())]
		pub fn unstake(origin: OriginFor<T>, request_id: HashOf<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as SeviceRequestInterface<T>>::unstake(&who, &request_id) {
				Ok(request) => {
					Self::deposit_event(Event::ServiceRequestUpdated(
						request_id,
						RequestStatus::WaitingForUnstaked,
						Some(request),
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::ServiceRequestWeightInfo::retrieve_unstaked_amount())]
		pub fn retrieve_unstaked_amount(
			origin: OriginFor<T>,
			request_id: HashOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as SeviceRequestInterface<T>>::retrieve_unstaked_amount(&who, &request_id) {
				Ok(request) => {
					Self::deposit_event(Event::StakingAmountRefunded(
						who,
						request_id,
						request.staking_amount,
					));

					Self::deposit_event(Event::ServiceRequestUpdated(
						request_id,
						RequestStatus::Unstaked,
						Some(request),
					));

					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::ServiceRequestWeightInfo::claim_request())]
		pub fn claim_request(
			origin: OriginFor<T>,
			request_id: HashOf<T>,
			service_id: HashOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as SeviceRequestInterface<T>>::claim_request(&who, &request_id, &service_id)
			{
				Ok(request) => {
					let status = if request.is_some() {
						RequestStatus::Claimed
					} else {
						RequestStatus::InLabList
					};

					Self::deposit_event(Event::ServiceRequestUpdated(request_id, status, request));

					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::ServiceRequestWeightInfo::process_request())]
		pub fn process_request(
			origin: OriginFor<T>,
			request_id: HashOf<T>,
			order_id: HashOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as SeviceRequestInterface<T>>::process_request(&who, &request_id, &order_id)
			{
				Ok(request) => {
					Self::deposit_event(Event::ServiceRequestUpdated(
						request_id,
						RequestStatus::Processed,
						Some(request),
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::ServiceRequestWeightInfo::finalize_request())]
		pub fn finalize_request(
			origin: OriginFor<T>,
			request_id: HashOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as SeviceRequestInterface<T>>::finalize_request(&who, &request_id) {
				Ok(request) => {
					Self::deposit_event(Event::ServiceRequestUpdated(
						request_id,
						RequestStatus::Finalized,
						Some(request),
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::ServiceRequestWeightInfo::update_admin_key())]
		pub fn update_admin_key(
			origin: OriginFor<T>,
			account_id: T::AccountId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			AdminKey::<T>::put(&account_id);

			Self::deposit_event(Event::UpdateServiceRequestAdminKeySuccessful(account_id));

			Ok(().into())
		}
	}
}
