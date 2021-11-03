#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	codec::{Decode, Encode},
	dispatch::DispatchResultWithPostInfo,
	pallet_prelude::*,
	scale_info::TypeInfo,
	sp_std::prelude::*,
	sp_runtime::{RuntimeDebug, traits::{AccountIdConversion, Hash, Zero}},
	traits::{Currency, ExistenceRequirement, WithdrawReasons},
	PalletId,
};
use frame_system::pallet_prelude::*;
use labs::{LabInterface, LabVerificationStatus};
pub use pallet::*;
use traits_services::{ServiceOwner, ServiceOwnerInfo};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod interface;
pub use interface::SeviceRequestInterface;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum RequestStatus {
	Open,
	Processed,
	Claimed,
	WaitingForUnstaked,
	Unstaked,
}
impl Default for RequestStatus {
	fn default() -> Self {
		RequestStatus::Open
	}
}

#[derive(Clone, Decode, Default, Encode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct Request<AccountId, Balance, Hash, Moment> {
	pub hash: Hash,
	pub requester_address: AccountId,
	pub lab_address: Option<AccountId>,
	pub country: Vec<u8>,
	pub region: Vec<u8>,
	pub city: Vec<u8>,
	pub service_category: Vec<u8>,
	pub staking_amount: Balance,
	pub status: RequestStatus,
	pub unstaked_at: Moment,
	pub exists: bool,
}
impl<AccountId, Balance, Hash, Moment> Request<AccountId, Balance, Hash, Moment> {
	pub fn new(
		hash: Hash,
		requester_address: AccountId,
		lab_address: Option<AccountId>,
		country: Vec<u8>,
		region: Vec<u8>,
		city: Vec<u8>,
		service_category: Vec<u8>,
		staking_amount: Balance,
		unstaked_at: Moment,
		exists: bool,
	) -> Self {
		Self {
			hash,
			requester_address,
			lab_address,
			country,
			region,
			city,
			service_category,
			staking_amount,
            status: RequestStatus::default(),
            unstaked_at,
            exists,
        }
    }
}

#[derive(Clone, Decode, Encode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct ServiceOffer<AccountId, Balance, Hash> {
	pub request_hash: Hash,
	pub lab_address: AccountId,
	pub service_id: Hash,
	pub testing_price: Balance,
	pub qc_price: Balance,
	pub exists: bool,
}
impl<AccountId, Balance, Hash> ServiceOffer<AccountId, Balance, Hash> {
	pub fn new(
		request_hash: Hash,
		lab_address: AccountId,
		service_id: Hash,
		testing_price: Balance,
		qc_price: Balance,
		exists: bool,
	) -> Self {
		Self { request_hash, lab_address, service_id, testing_price, qc_price, exists }
	}
}

#[derive(Clone, Decode, Encode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct ServiceInvoice<AccountId, Balance, Hash> {
	pub request_hash: Hash,
	pub order_id: Hash,
	pub service_id: Hash,
	pub customer_address: AccountId,
	pub seller_address: AccountId,
	pub dna_sample_tracking_id: Hash,
	pub testing_price: Balance,
	pub qc_price: Balance,
	pub pay_amount: Balance,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	pub const PALLET_ID: PalletId = PalletId(*b"reqsrvc!");

	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type CurrencyOf<T> = <T as self::Config>::Currency;
	pub type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;
	pub type HashOf<T> = <T as frame_system::Config>::Hash;
	pub type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;
	pub type RequestOf<T> = Request<AccountIdOf<T>, BalanceOf<T>, HashOf<T>, MomentOf<T>>;
	pub type ServiceOfferOf<T> = ServiceOffer<AccountIdOf<T>, BalanceOf<T>, HashOf<T>>;
	pub type ServiceInvoiceOf<T> = ServiceInvoice<AccountIdOf<T>, BalanceOf<T>, HashOf<T>>;
	pub type RequestIdOf<T> = HashOf<T>;
	pub type RequesterIdOf<T> = AccountIdOf<T>;
	pub type LabIdOf<T> = AccountIdOf<T>;
	pub type CountryOf = Vec<u8>;
	pub type RegionOf = Vec<u8>;
	pub type CityOf = Vec<u8>;
	pub type ServiceCategoryOf = Vec<u8>;
	pub type ServiceIdOf<T> = HashOf<T>;
	pub type OrderIdOf<T> = HashOf<T>;
	pub type DNASampleTrackingIdOf<T> = HashOf<T>;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<<Self as frame_system::Config>::AccountId>;
		type Labs: LabInterface<Self>;
	}

	#[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub admin_key: T::AccountId,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                admin_key: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            AdminKey::<T>::put(&self.admin_key);
        }
    }

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ServiceRequestCreated(AccountIdOf<T>, RequestOf<T>),
		ServiceRequestWaitingForUnstaked(AccountIdOf<T>, RequestOf<T>),
		ServiceRequestUnstaked(AccountIdOf<T>, RequestOf<T>),
		UnstakedAmountRetrieved(AccountIdOf<T>, RequestOf<T>),
		ServiceRequestClaimed(AccountIdOf<T>, ServiceOfferOf<T>),
		ServiceRequestProcessed(AccountIdOf<T>, ServiceInvoiceOf<T>),
		ExcessAmountRefunded(AccountIdOf<T>, RequestIdOf<T>, BalanceOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		BadSignatur,
		NotValidAmount,
		NotFound,
		RequestNotFound,
		RequestNotExist,
		RequestAlreadyClaimed,
		RequestAlreadyUnstaked,
		RequestAlreadyInList,
		Unauthorized,
		LabNotExist,
		NoOffer,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
    #[pallet::getter(fn admin_key)]
    pub type AdminKey<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn request_by_id)]
	pub type RequestById<T> = StorageMap<_, Blake2_128Concat, RequestIdOf<T>, RequestOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn requests_by_lab_id)]
	pub type RequestsByLabId<T> = StorageMap<_, Blake2_128Concat, LabIdOf<T>, Vec<RequestIdOf<T>>, ValueQuery>;

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

	#[pallet::storage]
	#[pallet::getter(fn service_offer_by_id)]
	pub type ServiceOfferById<T> = StorageMap<_, Blake2_128Concat, RequestIdOf<T>, ServiceOfferOf<T>>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(20_000 + T::DbWeight::get().reads_writes(1, 2))]
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
				who.clone(),
				country.clone(),
				region.clone(),
				city.clone(),
				service_category.clone(),
				staking_amount.clone(),
			) {
				Ok(request) => {
					Self::deposit_event(Event::ServiceRequestUnstaked(who.clone(), request));
					Ok(().into())
				}
				Err(error) => Err(error)?,
			}
		}

		#[pallet::weight(20_000 + T::DbWeight::get().reads_writes(1, 2))]
		pub fn claim_request(
			origin: OriginFor<T>,
			request_id: T::Hash,
			service_id: T::Hash,
			testing_price: BalanceOf<T>,
			qc_price: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as SeviceRequestInterface<T>>::claim_request(
				who.clone(),
				request_id.clone(),
				service_id.clone(),
				testing_price,
				qc_price,
			) {
				Ok(service_offer) => {
					Self::deposit_event(Event::ServiceRequestClaimed(who.clone(), service_offer));
					Ok(().into())
				}
				Err(error) => Err(error)?,
			}
		}

		#[pallet::weight(20_000 + T::DbWeight::get().reads_writes(1, 2))]
		pub fn unstake(origin: OriginFor<T>, request_id: T::Hash) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as SeviceRequestInterface<T>>::unstake(who.clone(), request_id.clone()) {
				Ok(request) => {
					Self::deposit_event(Event::ServiceRequestCreated(who.clone(), request));
					Ok(().into())
				}
				Err(error) => Err(error)?,
			}
		}
	}
}

/// Pallet Methods
impl<T: Config> Pallet<T> {
	pub fn staking_account_id(service_request_id: ServiceIdOf<T>) -> AccountIdOf<T> {
		PALLET_ID.into_sub_account(service_request_id)
	}
}

/// Service Request Interface Implementation
impl<T: Config + pallet_timestamp::Config> SeviceRequestInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type Balance = BalanceOf<T>;
	type Request = RequestOf<T>;
	type ServiceOffer = ServiceOfferOf<T>;
	type ServiceInvoice = ServiceInvoiceOf<T>;
	type RequestId = RequestIdOf<T>;
	type RequesterId = RequesterIdOf<T>;
	type LabId = LabIdOf<T>;
	type Country = CountryOf;
	type Region = RegionOf;
	type City = CityOf;
	type ServiceCategory = ServiceCategoryOf;
	type ServiceId = ServiceIdOf<T>;
	type OrderId = OrderIdOf<T>;
	type DNASampleTrackingId = DNASampleTrackingIdOf<T>;

	fn generate_service_request_id(
		requester_id: Self::RequesterId,
		country: Self::Country,
		region: Self::Region,
		city: Self::City,
		service_category: Self::ServiceCategory,
	) -> Self::RequestId {
		let mut seed = requester_id.encode();
		let account_info = frame_system::Pallet::<T>::account(requester_id);

		seed.append(&mut account_info.nonce.encode());
		seed.append(&mut country.encode());
		seed.append(&mut region.encode());
		seed.append(&mut city.encode());
		seed.append(&mut service_category.encode());

		T::Hashing::hash(&seed)
	}

	fn create_request(
		requester_id: Self::RequesterId,
		country: Self::Country,
		region: Self::Region,
		city: Self::City,
		service_category: Self::ServiceCategory,
		staking_amount: Self::Balance,
    ) -> Result<Self::Request, Self::Error> {
		if staking_amount.is_zero() {
            return Err(Error::<T>::NotValidAmount);
        }

		let now = pallet_timestamp::Pallet::<T>::get();
		let service_request_id = Self::generate_service_request_id(
			requester_id.clone(),
			country.clone(),
			region.clone(),
			city.clone(),
			service_category.clone(),
		);

		match CurrencyOf::<T>::withdraw(
			&requester_id,
			staking_amount.clone(),
			WithdrawReasons::TRANSFER,
			ExistenceRequirement::KeepAlive,
		) {
			Ok(imb) => {
				CurrencyOf::<T>::resolve_creating(&Self::staking_account_id(service_request_id), imb);

				let request = Request::new(
					service_request_id.clone(),
					requester_id.clone(),
					None,
					country.clone(),
					region.clone(),
					city.clone(),
					service_category.clone(),
					staking_amount.clone(),
					now.clone(),
					true
				);

				RequestById::<T>::insert(service_request_id.clone(), request.clone());

				let service_count = ServiceCountRequest::<T>::get((
					country.clone(),
					region.clone(),
					city.clone(),
					service_category.clone(),
				));

				ServiceCountRequest::<T>::insert(
					(country, region, city, service_category),
					service_count.wrapping_add(1),
				);

				Ok(request.clone())
			},
			_ => Err(Error::<T>::BadSignatur),
		}
	}

	fn unstake(
		requester_id: Self::RequesterId,
		request_id: Self::RequestId,
	) -> Result<Self::Request, Self::Error> {
		let request = RequestById::<T>::get(request_id.clone());

		if request.is_none() {
			return Err(Error::<T>::RequestNotFound);
		}

		let mut request = request.unwrap();

		if request.exists == false {
			return Err(Error::<T>::RequestNotExist);
		}

		if request.requester_address != requester_id {
			return Err(Error::<T>::Unauthorized);
		}

		request.status = RequestStatus::Unstaked;
		request.unstaked_at = pallet_timestamp::Pallet::<T>::get();

		RequestById::<T>::insert(request_id, request.clone());

		Ok(request)
	}

	// fn retrieve_unstaked_amount(
	// 	requester_id: Self::RequesterId,
	// 	request_id: Self::RequestId,
	// ) -> Result<Self::Request, Self::Error> {
	// 	let request = RequestById::<T>::get(request_id.clone());

	// 	if request.is_none() {
	// 		return Err(Error::<T>::RequestNotFound);
	// 	}

	// 	let request = request.unwrap();

	// 	if request.exists == false {
	// 		return Err(Error::<T>::RequestNotExist);
	// 	}

	// 	if request.requester_address != requester_id {
	// 		return Err(Error::<T>::Unauthorized);
	// 	}

	// 	let now: u128 = pallet_timestamp::Pallet::<T>::get().into() as u128;
	// 	let six_days = 3600 * 144 * 1000;
	// 	let unstaked_at = request.unstaked_at.unique_saturated_into();

	// 	if (now - request.unstaked_at.unique_saturated_into()) == six_days {
	// 		// TODO: change error event
	// 		return Err(Error::<T>::NotFound);
	// 	}

	// 	// TODO: transfer to requester_id

	// 	RequestById::<T>::insert(request_id, request.clone());

	// 	Ok(request)
	// }

	fn claim_request(
		lab_id: Self::LabId,
		request_id: Self::RequestId,
		service_id: Self::ServiceId,
		testing_price: Self::Balance,
		qc_price: Self::Balance,
	) -> Result<Self::ServiceOffer, Self::Error> {
		let request = RequestById::<T>::get(request_id.clone());

		if request.is_none() {
			return Err(Error::<T>::RequestNotFound);
		}

		let mut request = request.unwrap();

		if request.exists == false {
			return Err(Error::<T>::RequestNotExist);
		}

		if request.status == RequestStatus::Claimed {
			return Err(Error::<T>::RequestAlreadyClaimed);
		}

		// let lab_status = T::Labs::lab_verification_status(&lab_id);

		// if lab_status.is_none() {
		// 	return Err(Error::<T>::LabNotExist);
		// }

		let service_offer = ServiceOffer::new(
			request_id.clone(),
			lab_id.clone(),
			service_id,
			testing_price,
			qc_price,
			true,
		);

		// let lab_status = lab_status.unwrap();

		// // TODO: validation if lab verification status is verified
		// if lab_status == LabVerificationStatus::Verified {
		// 	request.status = RequestStatus::Claimed;
		// 	request.lab_address = Some(lab_id);

		// 	RequestById::<T>::insert(request_id, request.clone());
		// 	ServiceOfferById::<T>::insert(request_id, service_offer.clone());
		// } else {
		// 	let mut request_ids = RequestsByLabId::<T>::get(lab_id.clone());
		// 	let found = request_ids.iter().find(|x| x == &&request_id);

		// 	if found.is_some() {
		// 		return Err(Error::<T>::RequestAlreadyInList);
		// 	}
		// 	request_ids.push(request_id.clone());

		// 	RequestsByLabId::<T>::insert(lab_id.clone(), request_ids);
		// }

		Ok(service_offer)
	}

	// fn process_request(
	// 	requester_id: Self::RequesterId,
	// 	lab_id: Self::LabId,
	// 	request_id: Self::RequestId,
	// 	order_id: Self::OrderId,
	// 	dna_sample_tracking_id: Self::DNASampleTrackingId,
	// ) -> Result<Self::ServiceInvoice, Self::Error> {
	// 	let request = RequestById::<T>::get(request_id.clone());

	// 	if request.is_none() {
	// 		return Err(Error::<T>::RequestNotFound);
	// 	}

	// 	let request = request.unwrap();

	// 	if requester_id != request.requester_address {
	// 		return Err(Error::<T>::Unauthorized);
	// 	}

	// 	if request.status == RequestStatus::Unstaked {
	// 		return Err(Error::<T>::RequestAlreadyUnstaked);
	// 	}

	// 	if request.status == false {
	// 		return Err(Error::<T>::NoOffer);
	// 	}

	// 	let pay_amount = request.staking_amount;
	// 	let testing_price = request.testing_price;
	// 	let qc_price = request.qc_price;

	// 	// Refund excess payment
	// 	let total_price = testing_price + qc_price;

	// 	// TODO: transfer

	// 	Ok(())
	// }
}
