#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use frame_support::{
	codec::{Decode, Encode},
	dispatch::DispatchResultWithPostInfo,
	pallet_prelude::*,
	sp_runtime::{traits::Hash, RuntimeDebug},
	sp_std::prelude::*,
	traits::{Currency, UnixTime},
};
use frame_system::pallet_prelude::*;
use primitives_verification_status::VerificationStatusTrait;
use traits_labs::LabsProvider;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod functions;
pub mod impl_service_request;
pub mod interface;
pub mod types;
pub mod weights;

pub use interface::SeviceRequestInterface;
pub use types::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type TimeProvider: UnixTime;
		type Currency: Currency<<Self as frame_system::Config>::AccountId>;
		type Labs: LabsProvider<Self>;
		type ServiceRequestWeightInfo: WeightInfo;
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
		ServiceRequestWaitingForUnstaked(AccountIdOf<T>, RequestOf<T>),
		ServiceRequestUnstaked(AccountIdOf<T>, RequestOf<T>),
		UpdateServiceRequestAdminKeySuccessful(AccountIdOf<T>),
		ServiceRequestWaitingForClaimed(AccountIdOf<T>, ServiceOfferOf<T>),
		ServiceRequestClaimed(AccountIdOf<T>, ServiceOfferOf<T>),
		ServiceRequestProcessed(AccountIdOf<T>, ServiceInvoiceOf<T>),
		ServiceRequestFinalized(AccountIdOf<T>, ServiceInvoiceOf<T>),
		StakingAmountRefunded(AccountIdOf<T>, RequestIdOf<T>, BalanceOf<T>),
		StakingAmountExcessRefunded(AccountIdOf<T>, RequestIdOf<T>, BalanceOf<T>),
		StakingAmountIncreased(AccountIdOf<T>, RequestIdOf<T>, BalanceOf<T>),
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
		RequestAlreadyInList,
		RequestAlreadyClaimed,
		RequestAlreadyProccessed,
		RequestAlreadyFinalized,
		ServiceOfferNotFound,
		ServiceInvoiceNotFound,
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
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::pallet]
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
		StorageMap<_, Blake2_128Concat, RequestIdOf<T>, AccountIdOf<T>, OptionQuery>;

	/// Get Request by Account Id
	#[pallet::storage]
	#[pallet::getter(fn request_by_account_id)]
	pub type RequestByAccountId<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Vec<RequestIdOf<T>>, ValueQuery>;

	/// Get Request by RequestId
	#[pallet::storage]
	#[pallet::getter(fn request_by_id)]
	pub type RequestById<T> = StorageMap<_, Blake2_128Concat, RequestIdOf<T>, RequestOf<T>>;

	/// Get RequestIds by LabId
	#[pallet::storage]
	#[pallet::getter(fn requests_by_lab_id)]
	pub type RequestsByLabId<T> =
		StorageMap<_, Blake2_128Concat, LabIdOf<T>, Vec<RequestIdOf<T>>, ValueQuery>;

	/// Get  ServiceCountRequest by Country, Region, City, ServiceCategoryOf
	#[pallet::storage]
	#[pallet::getter(fn service_count_request)]
	pub type ServiceCountRequest<T> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, CountryOf>,
			NMapKey<Blake2_128Concat, RegionOf>,
			NMapKey<Blake2_128Concat, CityOf>,
			NMapKey<Blake2_128Concat, ServiceCategoryOf>,
		),
		u64,
		ValueQuery,
	>;

	/// Get ServiceOffer by RequestId
	#[pallet::storage]
	#[pallet::getter(fn service_offer_by_id)]
	pub type ServiceOfferById<T> =
		StorageMap<_, Blake2_128Concat, RequestIdOf<T>, ServiceOfferOf<T>>;

	/// Get ServiceInvoice by RequestId
	#[pallet::storage]
	#[pallet::getter(fn service_invoice_by_id)]
	pub type ServiceInvoiceById<T> =
		StorageMap<_, Blake2_128Concat, RequestIdOf<T>, ServiceInvoiceOf<T>>;

	/// Get ServiceInvoice By OrderId
	#[pallet::storage]
	#[pallet::getter(fn service_invoice_by_order_id)]
	pub type ServiceInvoiceByOrderId<T> =
		StorageMap<_, Blake2_128Concat, OrderIdOf<T>, ServiceInvoiceOf<T>>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::ServiceRequestWeightInfo::create_request())]
		pub fn create_request(
			origin: OriginFor<T>,
			country: CountryOf,
			region: RegionOf,
			city: CityOf,
			service_category: ServiceCategoryOf,
			staking_amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as SeviceRequestInterface<T>>::create_request(
				who.clone(),
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

			match <Self as SeviceRequestInterface<T>>::unstake(who.clone(), request_id) {
				Ok(request) => {
					Self::deposit_event(Event::ServiceRequestWaitingForUnstaked(who, request));
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

			match <Self as SeviceRequestInterface<T>>::retrieve_unstaked_amount(
				who.clone(),
				request_id,
			) {
				Ok(request) => {
					Self::deposit_event(Event::ServiceRequestUnstaked(who, request));
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
			testing_price: BalanceOf<T>,
			qc_price: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as SeviceRequestInterface<T>>::claim_request(
				who.clone(),
				request_id,
				service_id,
				testing_price,
				qc_price,
			) {
				Ok((request, service_offer)) => {
					if request.status == RequestStatus::Claimed {
						Self::deposit_event(Event::ServiceRequestClaimed(who, service_offer));
					} else {
						Self::deposit_event(Event::ServiceRequestWaitingForClaimed(
							who,
							service_offer,
						));
					}
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::ServiceRequestWeightInfo::process_request())]
		pub fn process_request(
			origin: OriginFor<T>,
			lab_id: LabIdOf<T>,
			request_id: HashOf<T>,
			order_id: HashOf<T>,
			dna_sample_tracking_id: DNASampleTrackingIdOf,
			additional_staking_amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as SeviceRequestInterface<T>>::process_request(
				who.clone(),
				lab_id,
				request_id,
				order_id,
				dna_sample_tracking_id,
				additional_staking_amount,
			) {
				Ok(service_invoice) => {
					Self::deposit_event(Event::ServiceRequestProcessed(who, service_invoice));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::ServiceRequestWeightInfo::finalize_request())]
		pub fn finalize_request(
			origin: OriginFor<T>,
			request_id: HashOf<T>,
			test_result_success: bool,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as SeviceRequestInterface<T>>::finalize_request(
				who.clone(),
				request_id,
				test_result_success,
			) {
				Ok(service_invoice) => {
					Self::deposit_event(Event::ServiceRequestFinalized(who, service_invoice));
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
			let who = ensure_signed(origin)?;

			match <Self as SeviceRequestInterface<T>>::update_admin_key(&who, &account_id) {
				Ok(_) => {
					Self::deposit_event(Event::UpdateServiceRequestAdminKeySuccessful(who.clone()));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}
	}
}
