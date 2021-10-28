#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use frame_support::traits::Currency;
pub use pallet::*;
use traits_services::{
    types::{ExpectedDuration, PriceByCurrency, ServiceFlow},
    ServiceInfo as ServiceInfoT, ServiceOwner, ServicesProvider,
};

pub mod interface;
pub use interface::ServiceInterface;
use sp_std::prelude::*;

/// ServiceInfo struct
/// Information that is mutable by user
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct ServiceInfo<Balance> {
    pub name: Vec<u8>,
    pub prices_by_currency: Vec<PriceByCurrency<Balance>>,
    pub expected_duration: ExpectedDuration,
    pub category: Vec<u8>,
    pub description: Vec<u8>, // TODO: limit the length
    pub test_result_sample: Vec<u8>,
    pub long_description: Option<Vec<u8>>,
    pub image: Option<Vec<u8>>,
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct Service<AccountId, Hash, Balance> {
    pub id: Hash,
    pub owner_id: AccountId,
    pub info: ServiceInfo<Balance>,
    pub service_flow: ServiceFlow
}
impl<AccountId, Hash, Balance> Service<AccountId, Hash, Balance> {
    pub fn new(
        id: Hash, 
        owner_id: AccountId, 
        info: ServiceInfo<Balance>,
        service_flow: ServiceFlow,
    ) -> Self {
        Self { 
            id, 
            owner_id, 
            info,
            service_flow
        }
    }

    pub fn get_id(&self) -> &Hash {
        &self.id
    }

    pub fn get_owner_id(&self) -> &AccountId {
        &self.owner_id
    }

    pub fn get_service_flow(&self) -> &ServiceFlow {
        &self.service_flow
    }

    pub fn get_prices_by_currency(&self) -> &Vec<PriceByCurrency<Balance>> {
        &self.info.prices_by_currency
    }
}

impl<T, AccountId, Hash, Balance> ServiceInfoT<T, Balance> for Service<AccountId, Hash, Balance>
where
    T: frame_system::Config<AccountId = AccountId, Hash = Hash>,
{
    fn get_id(&self) -> &Hash {
        self.get_id()
    }
    fn get_owner_id(&self) -> &AccountId {
        self.get_owner_id()
    }
    fn get_service_flow(&self) -> &ServiceFlow {
        &self.get_service_flow()
    }
    fn get_prices_by_currency(&self) -> &Vec<PriceByCurrency<Balance>> {
        self.get_prices_by_currency()
    }
}

#[frame_support::pallet]
pub mod pallet {
    use crate::interface::ServiceInterface;
    use traits_services::types::ServiceFlow;
    use crate::{Currency, Service, ServiceInfo, ServiceOwner};
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    pub use sp_std::prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: Currency<<Self as frame_system::Config>::AccountId>;
        type ServiceOwner: ServiceOwner<Self>;
    }

    // ----- This is template code, every pallet needs this ---
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
    // --------------------------------------------------------

    // ----- Types -------
    pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    pub type HashOf<T> = <T as frame_system::Config>::Hash;
    pub type CurrencyOf<T> = <T as self::Config>::Currency;
    pub type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;
    pub type ServiceOf<T> = Service<AccountIdOf<T>, HashOf<T>, BalanceOf<T>>;
    pub type ServiceInfoOf<T> = ServiceInfo<BalanceOf<T>>;
    pub type ServiceIdOf<T> = HashOf<T>;
    
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
    #[pallet::metadata(T::AccountId = "AccountId")]
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
        #[pallet::weight(20_000 + T::DbWeight::get().reads_writes(1, 2))]
        pub fn create_service(
            origin: OriginFor<T>,
            service_info: ServiceInfoOf<T>,
            service_flow: ServiceFlow,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as ServiceInterface<T>>::create_service(&who, &service_info, &service_flow) {
                Ok(service) => {
                    Self::deposit_event(Event::ServiceCreated(service, who.clone()));
                    Ok(().into())
                }
                Err(error) => Err(error)?,
            }
        }

        #[pallet::weight(20_000 + T::DbWeight::get().reads_writes(1, 2))]
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
                }
                Err(error) => Err(error)?,
            }
        }

        #[pallet::weight(20_000 + T::DbWeight::get().reads_writes(1, 2))]
        pub fn delete_service(
            origin: OriginFor<T>,
            service_id: T::Hash,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            match <Self as ServiceInterface<T>>::delete_service(&who, &service_id) {
                Ok(service) => {
                    Self::deposit_event(Event::ServiceDeleted(service, who.clone()));
                    Ok(().into())
                }
                Err(error) => Err(error)?,
            }
        }
    }
}

use frame_support::sp_runtime::traits::Hash;
use traits_services::ServiceOwnerInfo;

/// Service Interface Implementation
impl<T: Config> ServiceInterface<T> for Pallet<T> {
    type Error = Error<T>;
    type ServiceId = T::Hash;
    type Service = ServiceOf<T>;
    type ServiceInfo = ServiceInfoOf<T>;
    type ServiceFlow = ServiceFlow;

    fn generate_service_id(owner_id: &T::AccountId, service_count: u64) -> Self::ServiceId {
        let mut account_id_bytes = owner_id.encode();
        let mut service_count_bytes = service_count.encode();
        account_id_bytes.append(&mut service_count_bytes);

        let seed = &account_id_bytes;
        T::Hashing::hash(seed)
    }

    /// Create Service
    /// Add reference to ServicesByCountryCity storage
    /// Associate service reference to the owner (creator)
    /// Increment Counts
    fn create_service(
        owner_id: &T::AccountId,
        service_info: &Self::ServiceInfo,
        service_flow: &Self::ServiceFlow,
    ) -> Result<Self::Service, Self::Error> {
        // Check if user can create_service
        let can_create_service = T::ServiceOwner::can_create_service(owner_id);
        if !can_create_service {
            return Err(Error::<T>::NotAllowedToCreate)?;
        }

        let owner_service_count = <Self as ServiceInterface<T>>::services_count_by_owner(owner_id);
        let service_id = Self::generate_service_id(owner_id, owner_service_count);

        // Calculate total price
        let mut service_info_mut = service_info.clone();
        for (idx, price_by_currency) in service_info.prices_by_currency.iter().enumerate() {
            service_info_mut.prices_by_currency[idx].total_price -= price_by_currency.total_price;
            for price_component in price_by_currency.price_components.iter() {
                service_info_mut.prices_by_currency[idx].total_price += price_component.value;
            }

            for additional_price in price_by_currency.additional_prices.iter() {
                service_info_mut.prices_by_currency[idx].total_price += additional_price.value;
            }
        }

        let service = Service::new(service_id.clone(), owner_id.clone(), service_info_mut, service_flow.clone());
        // Store to Services storage
        Services::<T>::insert(&service_id, &service);

        // Increment Services Count
        Self::add_services_count();
        // Increment ServicesCountByOwner
        Self::add_services_count_by_owner(&service.owner_id);

        // Associate created service to the owner
        T::ServiceOwner::associate(owner_id, &service_id);

        Ok(service)
    }

    /// Update Service information
    fn update_service(
        owner_id: &T::AccountId,
        service_id: &Self::ServiceId,
        service_info: &Self::ServiceInfo,
    ) -> Result<Self::Service, Self::Error> {
        let service = Services::<T>::get(service_id);
        if service == None {
            return Err(Error::<T>::ServiceDoesNotExist)?;
        }
        let mut service = service.unwrap();

        if service.owner_id != owner_id.clone() {
            return Err(Error::<T>::NotServiceOwner)?;
        }

        // Calculate total price
        let mut service_info_mut = service_info.clone();
        for (idx, price_by_currency) in service_info.prices_by_currency.iter().enumerate() {
            service_info_mut.prices_by_currency[idx].total_price -= price_by_currency.total_price;
            for price_component in price_by_currency.price_components.iter() {
                service_info_mut.prices_by_currency[idx].total_price += price_component.value;
            }

            for additional_price in price_by_currency.additional_prices.iter() {
                service_info_mut.prices_by_currency[idx].total_price += additional_price.value;
            }
        }

        service.info = service_info_mut;
        Services::<T>::insert(service_id, &service);

        Ok(service)
    }

    /// Delete Service
    /// Delete from Services Storage
    /// Remove the service id reference in ServicesByCountryCity storage
    /// Disassociate service id from the owner
    /// Decrement Counts
    fn delete_service(
        owner_id: &T::AccountId,
        service_id: &Self::ServiceId,
    ) -> Result<Self::Service, Self::Error> {
        let service = Services::<T>::get(service_id);
        if service == None {
            return Err(Error::<T>::ServiceDoesNotExist)?;
        }
        let service = service.unwrap();

        if service.owner_id != owner_id.clone() {
            return Err(Error::<T>::NotServiceOwner)?;
        }
        // Remove service from storage
        let service = Services::<T>::take(service_id).unwrap();

        let owner = T::ServiceOwner::get_owner(owner_id).unwrap();
        // disassociate service reference from the owner
        T::ServiceOwner::disassociate(owner.get_id(), &service.id);
        // Decrement counts
        Self::sub_services_count();
        Self::sub_services_count_by_owner(owner.get_id());

        Ok(service)
    }

    fn service_by_id(service_id: &Self::ServiceId) -> Option<Self::Service> {
        match Services::<T>::get(service_id) {
            None => None,
            Some(service) => Some(service),
        }
    }

    fn services_count_by_owner(owner_id: &T::AccountId) -> u64 {
        Self::services_count_by_owner(owner_id).unwrap_or(0)
    }
}

/// Pallet Methods
impl<T: Config> Pallet<T> {
    // Services Count Addition and Substraction Helpers
    // Add services count
    pub fn add_services_count() {
        let services_count = <ServicesCount<T>>::get().unwrap_or(0);
        <ServicesCount<T>>::put(services_count.wrapping_add(1));
    }
    // Add services count by owner
    pub fn add_services_count_by_owner(owner_id: &T::AccountId) {
        let services_count = ServicesCountByOwner::<T>::get(owner_id).unwrap_or(0);
        ServicesCountByOwner::<T>::insert(owner_id, services_count.wrapping_add(1))
    }

    // Subtract services count
    pub fn sub_services_count() {
        let services_count = <ServicesCount<T>>::get().unwrap_or(1);
        ServicesCount::<T>::put(services_count - 1);
    }
    // Subtract services count by owner
    pub fn sub_services_count_by_owner(owner_id: &T::AccountId) {
        let services_count = ServicesCountByOwner::<T>::get(owner_id).unwrap_or(1);
        ServicesCountByOwner::<T>::insert(owner_id, services_count - 1);
    }
}

/// ServicesProvider Trait Implementation
impl<T: Config, Balance> ServicesProvider<T, Balance> for Pallet<T>
where
    ServiceOf<T>: traits_services::ServiceInfo<T, Balance>,
{
    type Error = Error<T>;
    type Service = ServiceOf<T>;

    fn service_by_id(id: &T::Hash) -> Option<ServiceOf<T>> {
        <Self as ServiceInterface<T>>::service_by_id(id)
    }

    fn delete_service(owner_id: &T::AccountId, id: &T::Hash) -> Result<Self::Service, Self::Error> {
        <Self as ServiceInterface<T>>::delete_service(owner_id, id)
    }
}
