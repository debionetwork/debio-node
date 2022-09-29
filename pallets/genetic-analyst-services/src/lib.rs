#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	codec::{Decode, Encode},
	pallet_prelude::*,
	sp_runtime::SaturatedConversion,
	traits::{Currency, StorageVersion},
};
pub use pallet::*;
use primitives_duration::ExpectedDuration;
use primitives_price_and_currency::PriceByCurrency;
pub use scale_info::TypeInfo;
use traits_genetic_analyst_services::{
	GeneticAnalystServiceInfo as GeneticAnalystServiceInfoT, GeneticAnalystServiceOwner,
	GeneticAnalystServicesProvider,
};

pub mod interface;
pub mod migrations;
pub mod weights;

pub use interface::GeneticAnalystServiceInterface;
use sp_std::prelude::*;

/// GeneticAnalystServiceInfo struct
/// Information that is mutable by user
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct GeneticAnalystServiceInfo<Balance> {
	pub name: Vec<u8>,
	pub prices_by_currency: Vec<PriceByCurrency<Balance>>,
	pub expected_duration: ExpectedDuration,
	pub description: Vec<u8>,
	pub test_result_sample: Vec<u8>,
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct GeneticAnalystService<AccountId, Hash, Balance> {
	pub id: Hash,
	pub owner_id: AccountId,
	pub info: GeneticAnalystServiceInfo<Balance>,
}
impl<AccountId, Hash, Balance> GeneticAnalystService<AccountId, Hash, Balance> {
	pub fn new(id: Hash, owner_id: AccountId, info: GeneticAnalystServiceInfo<Balance>) -> Self {
		Self { id, owner_id, info }
	}

	pub fn get_id(&self) -> &Hash {
		&self.id
	}

	pub fn get_owner_id(&self) -> &AccountId {
		&self.owner_id
	}

	pub fn get_prices_by_currency(&self) -> &Vec<PriceByCurrency<Balance>> {
		&self.info.prices_by_currency
	}
}

impl<T, AccountId, Hash, Balance> GeneticAnalystServiceInfoT<T, Balance>
	for GeneticAnalystService<AccountId, Hash, Balance>
where
	T: frame_system::Config<AccountId = AccountId, Hash = Hash>,
{
	fn get_id(&self) -> &Hash {
		self.get_id()
	}
	fn get_owner_id(&self) -> &AccountId {
		self.get_owner_id()
	}
	fn get_prices_by_currency(&self) -> &Vec<PriceByCurrency<Balance>> {
		self.get_prices_by_currency()
	}
}

/// The current storage version.
const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use crate::{
		interface::GeneticAnalystServiceInterface, weights::WeightInfo, Currency,
		GeneticAnalystService, GeneticAnalystServiceInfo, GeneticAnalystServiceOwner,
	};
	use frame_support::dispatch::DispatchResultWithPostInfo;
	use frame_system::pallet_prelude::*;
	pub use sp_std::prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<<Self as frame_system::Config>::AccountId>;
		type GeneticAnalystServiceOwner: GeneticAnalystServiceOwner<Self>;
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

	// ----- Types -------
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type HashOf<T> = <T as frame_system::Config>::Hash;
	pub type CurrencyOf<T> = <T as self::Config>::Currency;
	pub type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;
	pub type GeneticAnalystServiceOf<T> =
		GeneticAnalystService<AccountIdOf<T>, HashOf<T>, BalanceOf<T>>;
	pub type GeneticAnalystServiceInfoOf<T> = GeneticAnalystServiceInfo<BalanceOf<T>>;
	pub type GeneticAnalystServiceIdOf<T> = HashOf<T>;

	// ------- Storage -------------
	#[pallet::storage]
	#[pallet::getter(fn genetic_analyst_service_by_id)]
	pub type GeneticAnalystServices<T> =
		StorageMap<_, Blake2_128Concat, HashOf<T>, GeneticAnalystServiceOf<T>>;
	//                                _,  Hasher         ,  Key     ,  Value

	#[pallet::storage]
	#[pallet::getter(fn genetic_analyst_services_count)]
	pub type GeneticAnalystServicesCount<T> = StorageValue<_, u64>;

	#[pallet::storage]
	#[pallet::getter(fn genetic_analyst_services_count_by_owner)]
	pub type GeneticAnalystServicesCountByOwner<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, u64>;
	// -----------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters, [GeneticAnalystService, who]
		GeneticAnalystServiceCreated(GeneticAnalystServiceOf<T>, AccountIdOf<T>),
		//// GeneticAnalystService updated
		/// parameters, [GeneticAnalystService, who]
		GeneticAnalystServiceUpdated(GeneticAnalystServiceOf<T>, AccountIdOf<T>),
		//// GeneticAnalystService deleted
		/// parameters, [GeneticAnalystService, who]
		GeneticAnalystServiceDeleted(GeneticAnalystServiceOf<T>, AccountIdOf<T>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// User not allowed to create genetic_analyst_service
		NotAllowedToCreate,
		/// User is not the owner of a genetic_analyst_service
		NotGeneticAnalystServiceOwner,
		/// Ordering a genetic_analyst_service that does not exist
		GeneticAnalystServiceDoesNotExist,
		// Cannot create more than twenty services at once
		CannotCreateMoreThanTwentyServicesAtOnce,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::create_genetic_analyst_service())]
		pub fn create_genetic_analyst_service(
			origin: OriginFor<T>,
			genetic_analyst_service_info: GeneticAnalystServiceInfoOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as GeneticAnalystServiceInterface<T>>::create_genetic_analyst_service(
				&who,
				&[genetic_analyst_service_info],
			) {
				Ok(genetic_analyst_services) => {
					Self::deposit_event(Event::GeneticAnalystServiceCreated(
						genetic_analyst_services.first().unwrap().clone(),
						who.clone(),
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::bulk_create_genetic_analyst_service())]
		pub fn bulk_create_genetic_analyst_service(
			origin: OriginFor<T>,
			genetic_analyst_service_infos: Vec<GeneticAnalystServiceInfoOf<T>>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as GeneticAnalystServiceInterface<T>>::create_genetic_analyst_service(
				&who,
				&genetic_analyst_service_infos,
			) {
				Ok(genetic_analyst_services) => {
					for genetic_analyst_service in genetic_analyst_services {
						Self::deposit_event(Event::GeneticAnalystServiceCreated(
							genetic_analyst_service,
							who.clone(),
						));
					}
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::update_genetic_analyst_service())]
		pub fn update_genetic_analyst_service(
			origin: OriginFor<T>,
			genetic_analyst_service_id: HashOf<T>,
			genetic_analyst_service_info: GeneticAnalystServiceInfoOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			match <Self as GeneticAnalystServiceInterface<T>>::update_genetic_analyst_service(
				&who,
				&genetic_analyst_service_id,
				&genetic_analyst_service_info,
			) {
				Ok(genetic_analyst_service) => {
					Self::deposit_event(Event::GeneticAnalystServiceUpdated(
						genetic_analyst_service,
						who.clone(),
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::delete_genetic_analyst_service())]
		pub fn delete_genetic_analyst_service(
			origin: OriginFor<T>,
			genetic_analyst_service_id: T::Hash,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			match <Self as GeneticAnalystServiceInterface<T>>::delete_genetic_analyst_service(
				&who,
				&genetic_analyst_service_id,
			) {
				Ok(genetic_analyst_service) => {
					Self::deposit_event(Event::GeneticAnalystServiceDeleted(
						genetic_analyst_service,
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
use traits_genetic_analyst_services::GeneticAnalystServiceOwnerInfo;

/// GeneticAnalystService Interface Implementation
impl<T: Config> GeneticAnalystServiceInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type GeneticAnalystServiceId = T::Hash;
	type GeneticAnalystService = GeneticAnalystServiceOf<T>;
	type GeneticAnalystServiceInfo = GeneticAnalystServiceInfoOf<T>;

	fn generate_genetic_analyst_service_id(
		owner_id: &T::AccountId,
		genetic_analyst_service_count: u64,
	) -> Self::GeneticAnalystServiceId {
		let mut account_id_bytes = owner_id.encode();
		let mut genetic_analyst_service_count_bytes = genetic_analyst_service_count.encode();
		account_id_bytes.append(&mut genetic_analyst_service_count_bytes);

		let seed = &account_id_bytes;
		T::Hashing::hash(seed)
	}

	/// Create GeneticAnalystService
	/// Add reference to GeneticAnalystServicesByCountryCity storage
	/// Associate genetic_analyst_service reference to the owner (creator)
	/// Increment Counts
	fn create_genetic_analyst_service(
		owner_id: &T::AccountId,
		genetic_analyst_service_infos: &[Self::GeneticAnalystServiceInfo],
	) -> Result<Vec<Self::GeneticAnalystService>, Self::Error> {
		// Check if user can create_genetic_analyst_service
		let can_create_genetic_analyst_service =
			T::GeneticAnalystServiceOwner::can_create_genetic_analyst_service(owner_id);
		if !can_create_genetic_analyst_service {
			return Err(Error::<T>::NotAllowedToCreate)
		}

		if genetic_analyst_service_infos.len() > 20 {
			return Err(Error::<T>::CannotCreateMoreThanTwentyServicesAtOnce)
		}

		// Create vector
		let mut genetic_analyst_services = vec![];
		for genetic_analyst_service_info in genetic_analyst_service_infos {
			let owner_genetic_analyst_service_count =
				<Self as GeneticAnalystServiceInterface<T>>::genetic_analyst_services_count_by_owner(owner_id);
			let genetic_analyst_service_id = Self::generate_genetic_analyst_service_id(
				owner_id,
				owner_genetic_analyst_service_count,
			);

			// Calculate total price
			let mut genetic_analyst_service_info_mut = genetic_analyst_service_info.clone();
			for (idx, price_by_currency) in
				genetic_analyst_service_info.prices_by_currency.iter().enumerate()
			{
				// Remove total price before sum
				genetic_analyst_service_info_mut.prices_by_currency[idx].total_price =
					0u128.saturated_into();

				for price_component in price_by_currency.price_components.iter() {
					genetic_analyst_service_info_mut.prices_by_currency[idx].total_price +=
						price_component.value;
				}

				for additional_price in price_by_currency.additional_prices.iter() {
					genetic_analyst_service_info_mut.prices_by_currency[idx].total_price +=
						additional_price.value;
				}
			}

			let genetic_analyst_service = GeneticAnalystService::new(
				genetic_analyst_service_id,
				owner_id.clone(),
				genetic_analyst_service_info_mut,
			);
			// Store to GeneticAnalystServices storage
			GeneticAnalystServices::<T>::insert(
				genetic_analyst_service_id,
				&genetic_analyst_service,
			);

			// Increment GeneticAnalystServices Count
			Self::add_genetic_analyst_services_count();
			// Increment GeneticAnalystServicesCountByOwner
			Self::add_genetic_analyst_services_count_by_owner(&genetic_analyst_service.owner_id);

			// Associate created genetic_analyst_service to the owner
			T::GeneticAnalystServiceOwner::associate(owner_id, &genetic_analyst_service_id);

			genetic_analyst_services.push(genetic_analyst_service)
		}

		Ok(genetic_analyst_services)
	}

	/// Update GeneticAnalystService information
	fn update_genetic_analyst_service(
		owner_id: &T::AccountId,
		genetic_analyst_service_id: &Self::GeneticAnalystServiceId,
		genetic_analyst_service_info: &Self::GeneticAnalystServiceInfo,
	) -> Result<Self::GeneticAnalystService, Self::Error> {
		let genetic_analyst_service = GeneticAnalystServices::<T>::get(genetic_analyst_service_id);
		if genetic_analyst_service.is_none() {
			return Err(Error::<T>::GeneticAnalystServiceDoesNotExist)
		}
		let mut genetic_analyst_service = genetic_analyst_service.unwrap();

		if genetic_analyst_service.owner_id != owner_id.clone() {
			return Err(Error::<T>::NotGeneticAnalystServiceOwner)
		}

		// Calculate total price
		let mut genetic_analyst_service_info_mut = genetic_analyst_service_info.clone();
		for (idx, price_by_currency) in
			genetic_analyst_service_info.prices_by_currency.iter().enumerate()
		{
			// Remove total price before sum
			genetic_analyst_service_info_mut.prices_by_currency[idx].total_price =
				0u128.saturated_into();

			for price_component in price_by_currency.price_components.iter() {
				genetic_analyst_service_info_mut.prices_by_currency[idx].total_price +=
					price_component.value;
			}

			for additional_price in price_by_currency.additional_prices.iter() {
				genetic_analyst_service_info_mut.prices_by_currency[idx].total_price +=
					additional_price.value;
			}
		}

		genetic_analyst_service.info = genetic_analyst_service_info_mut;
		GeneticAnalystServices::<T>::insert(genetic_analyst_service_id, &genetic_analyst_service);

		Ok(genetic_analyst_service)
	}

	/// Delete GeneticAnalystService
	/// Delete from GeneticAnalystServices Storage
	/// Remove the genetic_analyst_service id reference in GeneticAnalystServicesByCountryCity
	/// storage
	/// Disassociate genetic_analyst_service id from the owner
	/// Decrement Counts
	fn delete_genetic_analyst_service(
		owner_id: &T::AccountId,
		genetic_analyst_service_id: &Self::GeneticAnalystServiceId,
	) -> Result<Self::GeneticAnalystService, Self::Error> {
		let genetic_analyst_service = GeneticAnalystServices::<T>::get(genetic_analyst_service_id);
		if genetic_analyst_service.is_none() {
			return Err(Error::<T>::GeneticAnalystServiceDoesNotExist)
		}
		let genetic_analyst_service = genetic_analyst_service.unwrap();

		if genetic_analyst_service.owner_id != owner_id.clone() {
			return Err(Error::<T>::NotGeneticAnalystServiceOwner)
		}
		// Remove genetic_analyst_service from storage
		let genetic_analyst_service =
			GeneticAnalystServices::<T>::take(genetic_analyst_service_id).unwrap();

		let owner = T::GeneticAnalystServiceOwner::get_owner(owner_id).unwrap();
		// disassociate genetic_analyst_service reference from the owner
		T::GeneticAnalystServiceOwner::disassociate(owner.get_id(), &genetic_analyst_service.id);
		// Decrement counts
		Self::sub_genetic_analyst_services_count();
		Self::sub_genetic_analyst_services_count_by_owner(owner.get_id());

		Ok(genetic_analyst_service)
	}

	fn genetic_analyst_service_by_id(
		genetic_analyst_service_id: &Self::GeneticAnalystServiceId,
	) -> Option<Self::GeneticAnalystService> {
		GeneticAnalystServices::<T>::get(genetic_analyst_service_id)
	}

	fn genetic_analyst_services_count_by_owner(owner_id: &T::AccountId) -> u64 {
		Self::genetic_analyst_services_count_by_owner(owner_id).unwrap_or(0)
	}
}

/// Pallet Methods
impl<T: Config> Pallet<T> {
	// GeneticAnalystServices Count Addition and Substraction Helpers
	// Add genetic_analyst_services count
	pub fn add_genetic_analyst_services_count() {
		let genetic_analyst_services_count = <GeneticAnalystServicesCount<T>>::get().unwrap_or(0);
		<GeneticAnalystServicesCount<T>>::put(genetic_analyst_services_count.wrapping_add(1));
	}

	// Add genetic_analyst_services count by owner
	pub fn add_genetic_analyst_services_count_by_owner(owner_id: &T::AccountId) {
		let genetic_analyst_services_count =
			GeneticAnalystServicesCountByOwner::<T>::get(owner_id).unwrap_or(0);
		GeneticAnalystServicesCountByOwner::<T>::insert(
			owner_id,
			genetic_analyst_services_count.wrapping_add(1),
		)
	}

	// Subtract genetic_analyst_services count
	pub fn sub_genetic_analyst_services_count() {
		let genetic_analyst_services_count = <GeneticAnalystServicesCount<T>>::get().unwrap_or(1);
		GeneticAnalystServicesCount::<T>::put(genetic_analyst_services_count - 1);
	}

	// Subtract genetic_analyst_services count by owner
	pub fn sub_genetic_analyst_services_count_by_owner(owner_id: &T::AccountId) {
		let genetic_analyst_services_count =
			GeneticAnalystServicesCountByOwner::<T>::get(owner_id).unwrap_or(1);
		GeneticAnalystServicesCountByOwner::<T>::insert(
			owner_id,
			genetic_analyst_services_count - 1,
		);
	}
}

/// GeneticAnalystServicesProvider Trait Implementation
impl<T: Config, Balance> GeneticAnalystServicesProvider<T, Balance> for Pallet<T>
where
	GeneticAnalystServiceOf<T>:
		traits_genetic_analyst_services::GeneticAnalystServiceInfo<T, Balance>,
{
	type Error = Error<T>;
	type GeneticAnalystService = GeneticAnalystServiceOf<T>;

	fn genetic_analyst_service_by_id(id: &T::Hash) -> Option<GeneticAnalystServiceOf<T>> {
		<Self as GeneticAnalystServiceInterface<T>>::genetic_analyst_service_by_id(id)
	}

	fn delete_genetic_analyst_service(
		owner_id: &T::AccountId,
		id: &T::Hash,
	) -> Result<Self::GeneticAnalystService, Self::Error> {
		<Self as GeneticAnalystServiceInterface<T>>::delete_genetic_analyst_service(owner_id, id)
	}
}
