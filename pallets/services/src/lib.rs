#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use services_trait::{
    ServicesContainer, structs::Service
};
use service_owner_trait::ServiceOwner;
use frame_support::traits::{ Currency, Randomness };
use frame_support::codec::{Encode, Decode};


#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResultWithPostInfo, pallet_prelude::*,
    };
    use frame_system::pallet_prelude::*;
    use sp_std::prelude::*;
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
    type ServiceOf<T> = crate::Service<AccountIdOf<T>, HashOf<T>, BalanceOf<T>>;

    // ------- Storage -------------
    #[pallet::storage]
    #[pallet::getter(fn service_by_id)]
    //                                _,  Hasher         ,  Key     ,  Value
    pub type Services<T> = StorageMap<_, Blake2_128Concat, HashOf<T>, ServiceOf<T>>;
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
/*
use frame_support::{
    decl_module, decl_storage, decl_event, decl_error,
    dispatch, debug,
    traits::{
        Get, Randomness, Currency, // ExistenceRequirement,
    }, 
};
use frame_system::ensure_signed;
use frame_support::codec::{Encode, Decode};
use frame_support::sp_runtime::{
    RuntimeDebug, // traits::Hash
};
use frame_support::sp_std::prelude::*;
use service_owner::ServiceOwner;

pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type RandomnessSource: Randomness<Self::Hash>;
    // type Hashing: Hash<Output = Self::Hash>;
    type Currency: Currency<Self::AccountId>;
    type Owner: ServiceOwner<Self>;
}

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct Service<AccountId, Hash, Balance> {
    id: Hash,
    lab_id: AccountId,
    name: Vec<u8>,
    price: Balance,
    description: Vec<u8>, // TODO: limit the length
    long_description: Option<Vec<u8>>,
    image: Option<Vec<u8>>
}

impl<AccountId, Hash, Balance> Service<AccountId, Hash, Balance> {
    pub fn get_id(&self) -> &Hash {
        &self.id
    }

    pub fn get_lab_id(&self) -> &AccountId {
        &self.lab_id
    }

    pub fn get_price(&self) -> &Balance {
        &self.price
    }
}

decl_storage! {
    trait Store for Module<T: Trait> as ServicesStorage {
        Services get(fn service_by_id):
            map hasher(blake2_128_concat) T::Hash
                => Option<Service<T::AccountId, T::Hash, BalanceOf<T>>>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        Hash = <T as frame_system::Trait>::Hash,
        Balance = BalanceOf<T>,
    {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters, [Service, who]
        ServiceCreated(Service<AccountId, Hash, Balance>, AccountId),
        //// Service updated
        /// parameters, [Service, who]
        ServiceUpdated(Service<AccountId, Hash, Balance>, AccountId),
        //// Service deleted
        /// parameters, [Service, who]
        ServiceDeleted(Service<AccountId, Hash, Balance>, AccountId),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// User is not the owner of a service
        NotServiceOwner,
        /// Ordering a service that does not exist
        ServiceDoesNotExist,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Errors must be initialized if they are used by the pallet.
        type Error = Error<T>;
        // Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;

        /**
         * Add Service
         * */
        #[weight = 10_1000 + T::DbWeight::get().writes(1)]
        pub fn create_service(
            origin,
            name: Vec<u8>,
            price: BalanceOf<T>,
            description: Vec<u8>,
            long_description: Option<Vec<u8>>,
            image: Option<Vec<u8>>,
        )
            -> dispatch::DispatchResult
        {
            let who = ensure_signed(origin)?;

            // Check if user can create_service
            let can_create_service = T::Owner::can_create_service(&who);
            if !can_create_service {
                return Err(Error::<T>::NotServiceOwner)?;
            }

            let service_id = Self::generate_hash(&who);
            let service = Service {
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

            Self::deposit_event(RawEvent::ServiceCreated(service, who.clone()));

            Ok(())
        }

        #[weight = 10_1000 + T::DbWeight::get().writes(1)]
        pub fn update_service(
            origin,
            service_id: T::Hash,
            name: Vec<u8>,
            price: BalanceOf<T>,
            description: Vec<u8>,
            long_description: Option<Vec<u8>>,
            image: Option<Vec<u8>>,
        )
            -> dispatch::DispatchResult
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

            Self::deposit_event(RawEvent::ServiceUpdated(service.unwrap(), who.clone()));
            Ok(())
        }

        #[weight = 10_1000 + T::DbWeight::get().writes(1)]
        pub fn delete_service(
            origin,
            service_id: T::Hash
        )
            -> dispatch::DispatchResult
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

            Self::deposit_event(RawEvent::ServiceDeleted(service, who.clone()));
            Ok(())
        }
    }
}


// TODO: Maybe extract this fn as a separate module (this is used by pallet services also)
impl<T: Trait> Module<T> {
    fn generate_hash(account_id: &T::AccountId)
        -> <T as frame_system::Trait>::Hash
    {
        let account_info = frame_system::Module::<T>::account(account_id);
        debug::info!("account_info.data: {:?}", account_info.data);
        let hash = <T as Trait>::RandomnessSource::random(&account_info.nonce.encode());
        // let hash = <T as Trait>::Hashing::hash(&account_info.nonce.encode());
        return hash;
    }
}
*/
