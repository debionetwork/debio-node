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

pub mod lab_interface;
pub use crate::lab_interface::LabInterface;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResultWithPostInfo, pallet_prelude::*,
    };
    use frame_system::pallet_prelude::*;
    pub use sp_std::prelude::*;

    
    #[pallet::config]
    /// Configure the pallet by specifying the parameters and types on which it depends.
    pub trait Config: frame_system::Config {
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
    pub type LabOf<T> = Lab<AccountIdOf<T>, HashOf<T>>;
    pub type CountryStr = Vec<u8>;
    pub type CityStr = Vec<u8>;

    // ----- Storage ------------------
    /// Get Lab by account id
    /// AccountId => Lab
    #[pallet::storage]
    #[pallet::getter(fn lab_by_account_id)]
    pub type Labs<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, LabOf<T>>;

    /// Get Lab by Country, City
    /// (CountryStr, CityStr) => Vec<Lab>
    #[pallet::storage]
    #[pallet::getter(fn labs_by_country_city)]
    pub type LabsByCountryCity<T> = StorageDoubleMap<_, Blake2_128Concat, CountryStr, Blake2_128Concat, CityStr, Vec<LabOf<T>>>;

    /// Get total lab count
    /// u32
    #[pallet::storage]
    #[pallet::getter(fn lab_count)]
    pub type LabCount<T> = StorageValue<_, u64>;


    /// Get total lab count by Country, City
    /// (CountryStr, CityStr) => u32
    #[pallet::storage]
    #[pallet::getter(fn lab_count_by_country_city)]
    pub type LabCountByCountryCity<T> = StorageDoubleMap<_, Blake2_128Concat, CountryStr, Blake2_128Concat, CityStr, u64>;
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
                country: country.clone(),
                city: city.clone(),
                address: address,
                latitude: latitude,
                longitude: longitude,
                profile_image: profile_image,
                services: services,
                is_verified: is_verified.unwrap_or(false)
            };

            Labs::<T>::insert(&who, &lab);
            match LabsByCountryCity::<T>::get(country.clone(), city.clone()) {
                None => {
                    let mut labs = Vec::new();
                    labs.push(lab.clone());
                    LabsByCountryCity::<T>::insert(country.clone(), city.clone(), labs);
                },
                Some(mut labs) => {
                    labs.push(lab.clone());
                    LabsByCountryCity::<T>::insert(country.clone(), city.clone(), labs);
                }
            }

            // Add lab count
            let lab_count = <LabCount<T>>::get().unwrap_or(0);
            <LabCount<T>>::put(lab_count.wrapping_add(1));

            // Add lab count by country city
            let lab_count_by_country_city = <LabCountByCountryCity<T>>::get(country.clone(), city.clone()).unwrap_or(0);
            <LabCountByCountryCity<T>>::insert(country.clone(), city.clone(), lab_count_by_country_city.wrapping_add(1));

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
            let mut lab = lab.unwrap();

            // Save reference to old location
            let old_country = lab.country.clone();
            let old_city = lab.city.clone();
            
            // Update lab properties
            lab.name = lab_name;
            lab.country = country.clone();
            lab.city = city.clone();
            lab.address = address;
            lab.latitude = latitude;
            lab.longitude = longitude;
            lab.profile_image = profile_image;

            // Update Labs Storage
            Labs::<T>::insert(&who, &lab);

            // Update LabsByCountryCity Storage
            let mut labs_by_old_country_city = LabsByCountryCity::<T>::get(old_country.clone(), old_city.clone()).unwrap_or(Vec::new());
            // If Country and City didnt change
            if old_country == country && old_city == city {
                for l in labs_by_old_country_city.iter_mut() {
                    *l = lab.clone();
                }
                LabsByCountryCity::<T>::insert(country.clone(), city.clone(), labs_by_old_country_city);
            } else {
                // remove the lab from labs list in old_country, old_city
                labs_by_old_country_city.retain(|l| l.id != lab.id);
                LabsByCountryCity::<T>::insert(old_country.clone(), old_city.clone(), labs_by_old_country_city);
                // subtract count
                let old_country_city_lab_count = LabCountByCountryCity::<T>::get(old_country.clone(), old_city.clone()).unwrap_or(1);
                LabCountByCountryCity::<T>::insert(old_country.clone(), old_city.clone(), old_country_city_lab_count - 1);

                // add the lab to a different country, city
                match LabsByCountryCity::<T>::get(country.clone(), city.clone()) {
                    None => {
                        let mut labs = Vec::new();
                        labs.push(lab.clone());
                        LabsByCountryCity::<T>::insert(country.clone(), city.clone(), labs);
                    },
                    Some(mut labs) => {
                        labs.push(lab.clone());
                        LabsByCountryCity::<T>::insert(country.clone(), city.clone(), labs);
                    }
                }
                // add count
                let new_country_city_lab_count = LabCountByCountryCity::<T>::get(country.clone(), city.clone()).unwrap_or(0);
                LabCountByCountryCity::<T>::insert(country.clone(), city.clone(), new_country_city_lab_count.wrapping_add(1));
            }

            Self::deposit_event(Event::LabUpdated(lab, who.clone()));
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

impl<T: Config> LabInterface<T> for Pallet<T> {
    type Error = Error<T>;
    type Lab = LabOf<T>;

    // TODO: 
    fn register_lab() -> Result<(), Self::Error> {
        Ok(())
    }

    // TODO:
    fn update_lab() -> Result<(), Self::Error> {
        Ok(())
    }

    // TODO:
    fn delete_lab() -> Result<(), Self::Error> {
        Ok(())
    }

    // TODO:
    fn labs_by_country_city() -> Option<Vec<Self::Lab>> {
        None
    }

    // TODO:
    fn lab_by_account_id() -> Option<Self::Lab> {
        None
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
}

