#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
pub use pallet::*;
use service_owner::ServiceOwner;
use services_trait::ServicesContainer;
use services_trait::structs::Service;
use frame_support::traits::Currency;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;


#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResultWithPostInfo, pallet_prelude::*,
    };
    use frame_system::pallet_prelude::*;
    use sp_std::prelude::*;

    
    #[pallet::config]
    /// Configure the pallet by specifying the parameters and types on which it depends.
    pub trait Config: frame_system::Config /*+ services::Config*/ {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: crate::Currency<Self::AccountId>;
        type Services: crate::ServicesContainer<Self>;
    }

    // ----- This is template code, every pallet needs this ---
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
    // --------------------------------------------------------
    
    // Lab Struct
    #[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
    pub struct Lab<AccountId, Hash: PartialEq + Eq> {
        id: AccountId,
        name: Vec<u8>,
        services: Vec<Hash>,
        country: Vec<u8>,
        city: Vec<u8>,
        address: Vec<u8>,
        latitude: Option<Vec<u8>>,
        longitude: Option<Vec<u8>>,
        profile_image: Option<Vec<u8>>,
        is_verified: bool,
    }
    impl<AccountId, Hash: PartialEq + Eq> Lab<AccountId, Hash> {
        pub fn get_id(&self) -> &AccountId {
            &self.id
        }

        pub fn add_service(&mut self, service_id: Hash) -> () {
            &self.services.push(service_id);
        }

        pub fn remove_service(&mut self, service_id: Hash) -> () {
          if let Some(pos) = &self.services.iter().position(|x| *x == service_id) {
              &self.services.remove(*pos);
          }
        }
    }

    // ---- Types ----------------------
    pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    pub type HashOf<T> = <T as frame_system::Config>::Hash;
    pub type BalanceOf<T> = <<T as self::Config>::Services as crate::ServicesContainer<T>>::Balance;
    pub type ServiceOf<T> = crate::Service<AccountIdOf<T>, HashOf<T>, BalanceOf<T>>;

    // ----- Storage ------------------
    #[pallet::storage]
    #[pallet::getter(fn lab_by_account_id)]
    pub type Labs<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Lab<AccountIdOf<T>, HashOf<T>>>;

    #[pallet::storage]
    #[pallet::getter(fn lab_count)]
    pub type LabCount<T> = StorageValue<_, u32>;
    // -----------------------------------


    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// User AccountId registered as lab
        /// parameters. [Lab, who]
        LabRegistered(Lab<AccountIdOf<T>, HashOf<T>>, AccountIdOf<T>),
        /// Lab information updated
        /// parameters. [Lab, who]
        LabUpdated(Lab<AccountIdOf<T>, HashOf<T>>, AccountIdOf<T>),
        /// Lab deleted
        /// parameters. [Lab, who]
        LabDeleted(Lab<AccountIdOf<T>, HashOf<T>>, AccountIdOf<T>),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Account already has lab registered
        LabAlreadyRegistered,
        /// Lab identified by the AccountId does not exist
        LabDoesNotExist,
        /// Service does not exist
        ServiceDoesNotExist,
    }


    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn register_lab(
            origin: OriginFor<T>,
            lab_name: Vec<u8>,
            country: Vec<u8>,
            city: Vec<u8>,
            address: Vec<u8>,
            latitude: Option<Vec<u8>>,
            longitude: Option<Vec<u8>>,
            profile_image: Option<Vec<u8>>,
            is_verified: Option<bool>
        )
            -> DispatchResultWithPostInfo
        {
            let who = ensure_signed(origin)?;

            let lab_exists = <Labs<T>>::contains_key(&who);
            if lab_exists {
                return Err(Error::<T>::LabAlreadyRegistered)?;
            }

            let services: Vec<<T as frame_system::Config>::Hash> = Vec::new();
            let lab = Lab {
                id: who.clone(),
                name: lab_name,
                country: country,
                city: city,
                address: address,
                latitude: latitude,
                longitude: longitude,
                profile_image: profile_image,
                services: services,
                is_verified: is_verified.unwrap_or(false)
            };
            <Labs<T>>::insert(&who, &lab);

            // Add lab count
            let lab_count = <LabCount<T>>::get().unwrap_or(0);
            <LabCount<T>>::put(lab_count.wrapping_add(1));

            Self::deposit_event(Event::LabRegistered(lab, who.clone()));

            Ok(().into())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn update_lab(
            origin: OriginFor<T>,
            lab_name: Vec<u8>,
            country: Vec<u8>,
            city: Vec<u8>,
            address: Vec<u8>,
            latitude: Option<Vec<u8>>,
            longitude: Option<Vec<u8>>,
            profile_image: Option<Vec<u8>>,
        )
            -> DispatchResultWithPostInfo
        {
            let who = ensure_signed(origin)?;

            let lab = Labs::<T>::get(&who);
            if lab == None {
                return Err(Error::<T>::LabDoesNotExist)?;
            }

            let lab = Labs::<T>::mutate(&who, | lab | {
                match lab {
                    None => None,
                    Some(lab) => {
                        lab.name = lab_name;
                        lab.country = country;
                        lab.city = city;
                        lab.address = address;
                        lab.latitude = latitude;
                        lab.longitude = longitude;
                        lab.profile_image = profile_image;

                        Some(lab.clone())
                    }
                }
            });

            Self::deposit_event(Event::LabUpdated(lab.unwrap(), who.clone()));

            Ok(().into())
        }

        /* TODO: Delete Lab
        #[weight = 10_1000 + T::DbWeight::get().writes(1)]
        pub fn delete_lab(
            origin,
            lab_id: T::Hash
        )
            -> dispatch::DispatchResult
        {
            let who = ensure_signed(origin)?;
            // Check if user is a lab
            let lab = Self::lab_by_account_id(&who);
            if lab == None {
                return Err(Error::<T>::LabDoesNotExist)?;
            }

            /*
            let service_exists = Services::<T>::contains_key(&service_id);
            if !service_exists {
                return Err(Error::<T>::ServiceDoesNotExist)?;
            }

            let service = Services::<T>::take(&service_id);
            let service = service.unwrap();
            */

            /*
            Self::deposit_event(RawEvent::ServiceDeleted(service, who.clone()));
            */
            Ok(())
        }
        */

    }

}

impl<T: Config> ServiceOwner<T> for Pallet<T> {
    fn associate(owner_id: &T::AccountId, service_id: &T::Hash) -> () {
        <Labs<T>>::mutate(owner_id, | lab | {
            match lab {
                None => (), // If lab does not exist, do nothing
                Some(lab) => {
                    lab.add_service(*service_id);
                }
            }
        });
    }

    fn disassociate(owner_id: &T::AccountId, service_id: &T::Hash) -> () {
        Labs::<T>::mutate(owner_id, | lab | {
            match lab {
                None => (),
                Some(lab) => {
                    lab.remove_service(*service_id);
                }
            }
        });
    }

    fn is_owner(owner_id: &T::AccountId, service_id: &T::Hash) -> bool {
        let service: Option<pallet::ServiceOf<T>> = T::Services::service_by_id(service_id);

        match service {
            None => false,
            Some(service) => {
                return *service.get_lab_id() == *owner_id;
            }
        }
    }

    fn can_create_service(user_id: &T::AccountId) -> bool {
        return Labs::<T>::contains_key(user_id);
    }

    // Reads the nonce from storage, increments the stored nonce, and returns
    // the encoded nonce to the caller.
    //fn encode_and_update_nonce() -> Vec<u8> {
    //	let nonce = Nonce::get();
    //	Nonce::put(nonce.wrapping_add(1));
    //	nonce.encode()
    //}
}

