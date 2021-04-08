#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use services_trait::{
    ServicesContainer,
    structs::{Service, ServiceInfo}
};
use service_owner_trait::ServiceOwner;
use frame_support::traits::{ Currency, Randomness };
use frame_support::codec::{Encode, Decode};
use frame_support::pallet_prelude::*;

pub mod interface;
pub use interface::ServiceInterface;
use sp_std::prelude::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResultWithPostInfo, pallet_prelude::*,
    };
    use frame_system::pallet_prelude::*;
    pub use sp_std::prelude::*;
    use service_owner_trait::ServiceOwner;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: crate::Currency<<Self as frame_system::Config>::AccountId>;
        type RandomnessSource: crate::Randomness<Self::Hash>;
        type Owner: crate::ServiceOwner<Self>;
    }

    // ----- This is template code, every pallet needs this ---
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
    // --------------------------------------------------------
    

    // ----- Types -------
    type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    type HashOf<T> = <T as frame_system::Config>::Hash;
    type CurrencyOf<T> = <T as self::Config>::Currency;
    pub type BalanceOf<T> = <CurrencyOf<T> as crate::Currency<AccountIdOf<T>>>::Balance;
    pub type ServiceOf<T> = crate::Service<AccountIdOf<T>, HashOf<T>, BalanceOf<T>>;
    pub type ServiceInfoOf<T> = crate::ServiceInfo<BalanceOf<T>>;

    // ------- Storage -------------
    #[pallet::storage]
    #[pallet::getter(fn service_by_id)]
    pub type Services<T> = StorageMap<_, Blake2_128Concat, HashOf<T>, ServiceOf<T>>;
    //                                _,  Hasher         ,  Key     ,  Value

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
        /// User is not the owner of a service
        NotServiceOwner,
        /// Ordering a service that does not exist
        ServiceDoesNotExist,
    }
    
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /*
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn create_service(
            origin: OriginFor<T>,
            name: Vec<u8>,
            price: BalanceOf<T>,
            description: Vec<u8>,
            long_description: Option<Vec<u8>>,
            image: Option<Vec<u8>>,
        )
            -> DispatchResultWithPostInfo
        {
            let who = ensure_signed(origin)?;

            // Check if user can create_service
            let can_create_service = T::Owner::can_create_service(&who);
            if !can_create_service {
                return Err(Error::<T>::NotServiceOwner)?;
            }

            let service_id = Self::generate_hash(&who);
            let service = crate::Service {
                id: service_id,
                lab_id: who.clone(),
                name: name,
                price: price,
                description: description,
                long_description: long_description,
                image: image
            };

            Services::<T>::insert(&service_id, &service);
            T::Owner::associate(&who, &service_id);

            Self::deposit_event(Event::ServiceCreated(service, who.clone()));

            Ok(().into())
        }
        
        #[pallet::weight(10_1000 + T::DbWeight::get().writes(1))]
        pub fn update_service(
            origin: OriginFor<T>,
            service_id: T::Hash,
            name: Vec<u8>,
            price: BalanceOf<T>,
            description: Vec<u8>,
            long_description: Option<Vec<u8>>,
            image: Option<Vec<u8>>,
        )
            -> DispatchResultWithPostInfo
        {
            let who = ensure_signed(origin)?;
            // Check if user is a lab
            let owner = T::Owner::is_owner(&who, &service_id);
            if !owner {
                return Err(Error::<T>::NotServiceOwner)?;
            }

            let service = Services::<T>::mutate(service_id, | service | {
                match service {
                    None => None,
                    Some(service) => {
                        service.name = name;
                        service.price = price;
                        service.description = description;
                        service.long_description = long_description;
                        service.image = image;

                        Some(service.clone())
                    }
                }
            });
            if service == None {
                return Err(Error::<T>::ServiceDoesNotExist)?;
            }

            Self::deposit_event(Event::ServiceUpdated(service.unwrap(), who.clone()));
            Ok(().into())
        }

        #[pallet::weight(10_1000 + T::DbWeight::get().writes(1))]
        pub fn delete_service(
            origin: OriginFor<T>,
            service_id: T::Hash
        )
            -> DispatchResultWithPostInfo
        {
            let who = ensure_signed(origin)?;
            // Check if user is a lab
            let is_owner = T::Owner::is_owner(&who, &service_id);
            if !is_owner {
                return Err(Error::<T>::NotServiceOwner)?;
            }

            let service_exists = Services::<T>::contains_key(&service_id);
            if !service_exists {
                return Err(Error::<T>::ServiceDoesNotExist)?;
            }

            // Remove service_id from associated lab owner
            T::Owner::disassociate(&who, &service_id);

            let service = Services::<T>::take(&service_id);
            let service = service.unwrap();

            Self::deposit_event(Event::ServiceDeleted(service, who.clone()));
            Ok(().into())
        }
        */
    }
}

use frame_support::sp_runtime::traits::Hash;

impl<T: Config> ServiceInterface<T> for Pallet<T> {
    type Error = Error<T>;
    type ServiceId = T::Hash;
    type Service = ServiceOf<T>;
    type ServiceInfo = ServiceInfoOf<T>;

    fn generate_service_id(owner_id: &T::AccountId, service_count: u64) -> Self::ServiceId {
        let mut account_id_bytes = owner_id.encode();
        let mut service_count_bytes = service_count.encode();
        account_id_bytes.append(&mut service_count_bytes);

        let seed = &account_id_bytes;
        T::Hashing::hash(seed)
    }

    fn create_service(owner_id: &T::AccountId, service_info: &Self::ServiceInfo) -> Result<(), Self::Error> { 
        let owner_service_count = <Self as ServiceInterface<T>>::services_count_by_owner(owner_id);
        let service_id = Self::generate_service_id(owner_id, owner_service_count);
        
        Ok(()) 
    }

    fn update_service(service_id: &Self::ServiceId, service: &Self::ServiceInfo) -> Result<(), Self::Error> {
        Ok(())
    }

    fn delete_service(service_id: &Self::ServiceId) -> Result<(), Self::Error> { Ok(()) }


    fn service_by_id(service_id: &Self::ServiceId) -> Option<Self::Service> { None }
    fn services_by_country_city(country: Vec<u8>, city: Vec<u8>) -> Option<Vec<Self::ServiceId>> { None }

    fn services_count_by_owner(owner_id: &T::AccountId) -> u64 {
        Self::services_count_by_owner(owner_id).unwrap_or(0)
    }
}


impl<T: Config> ServicesContainer<T> for Pallet<T> {
  type Balance = pallet::BalanceOf<T>;

  fn service_by_id(id: &T::Hash) -> Option<Service<T::AccountId, T::Hash, Self::Balance>> {
      match Services::<T>::get(id) {
          None => None,
          Some(service) => Some(service)
      }
  }
}

// TODO: Maybe extract this fn as a separate module (this is used by pallet services also)
impl<T: Config> Pallet<T> {
    fn generate_hash(account_id: &T::AccountId)
        -> <T as frame_system::Config>::Hash
    {
        let account_info = frame_system::Module::<T>::account(account_id);
        // debug::info!("account_info.data: {:?}", account_info.data);
        let hash = <T as Config>::RandomnessSource::random(&account_info.nonce.encode());
        // let hash = <T as Trait>::Hashing::hash(&account_info.nonce.encode());
        return hash;
    }
}
