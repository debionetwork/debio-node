#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_module, decl_storage, decl_event, decl_error,
    dispatch, debug,
    traits::{
        Get, Randomness, Currency, ExistenceRequirement,
    }, 
};
use frame_system::ensure_signed;
use frame_support::codec::{Encode, Decode};
use frame_support::sp_runtime::{RuntimeDebug, traits::Hash};
use frame_support::sp_std::prelude::*;

pub trait Trait: frame_system::Trait + debio_labs::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type RandomnessSource: Randomness<Self::Hash>;
    type Hashing: Hash<Output = Self::Hash>;
    type Currency: Currency<Self::AccountId>;
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
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// Lab identified by the AccountId does not exist
        LabDoesNotExist,
        /// Ordering a service that does not exist
        ServiceDoesNotExist
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
        pub fn add_service(
            origin,
            name: Vec<u8>,
            price: BalanceOf<T>,
            description: Vec<u8>,
            long_description: Option<Vec<u8>>,
            image: Option<Vec<u8>>,
        ) -> dispatch::DispatchResult
        {
            let who = ensure_signed(origin)?;

            // Check if lab exists
            let lab_exists = debio_labs::Module::<T>::lab_by_account_id(&who);
            if lab_exists == None {
                return Err(Error::<T>::LabDoesNotExist)?;
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
            debio_labs::Module::<T>::associate_service_to_lab(&who, service_id);

            Self::deposit_event(RawEvent::ServiceCreated(service, who.clone()));

            Ok(())
        }

        /**
         * Order Service
         * */
        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn order_service(origin, service_id: <T as frame_system::Trait>::Hash)
            -> dispatch::DispatchResult
        {
            let customer = ensure_signed(origin)?;

            let service_exists = <Services<T>>::contains_key(&service_id);
            if !service_exists {
                return Err(Error::<T>::ServiceDoesNotExist)?;
            }

            let service = <Services<T>>::get(&service_id);
            match service {
                None => (), // TODO: Error
                Some(service) => {
                    let lab = debio_labs::Module::<T>::lab_by_account_id(&service.lab_id);
                    match lab {
                        None => (), // TODO: Error
                        Some(lab) => {
                            <T as Trait>::Currency::transfer(
                                &customer,
                                lab.get_id(),
                                service.price,
                                ExistenceRequirement::KeepAlive
                            );
                        }
                    }
                }
            }

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
