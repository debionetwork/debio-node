#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
pub use pallet::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub mod interface;
pub use crate::interface::LabInterface;
use frame_support::pallet_prelude::*;
use traits_services::{ServiceOwnerInfo};

// LabInfo Struct
// Used as parameter of dispatchable calls
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct LabInfo {
    name: Vec<u8>,
    email: Vec<u8>,
    country: Vec<u8>,
    city: Vec<u8>,
    address: Vec<u8>,
    latitude: Option<Vec<u8>>,
    longitude: Option<Vec<u8>>,
    profile_image: Option<Vec<u8>>,
}

// Lab Struct
// the fields (excluding account_id and services) come from LabInfo struct
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct Lab<AccountId, Hash>
    where Hash: PartialEq + Eq
{
    account_id: AccountId,
    services: Vec<Hash>,
    info: LabInfo,
}

impl<AccountId, Hash> Lab<AccountId, Hash>
    where Hash: PartialEq + Eq
{
    pub fn new (
        account_id: AccountId,
        info: LabInfo,
    ) -> Self {
        Self {
            account_id,
            services: Vec::<Hash>::new(),
            info,
        }
    }

    fn update_info(&mut self, info: LabInfo) -> () {
        self.info = info;
    }

    fn get_country(&self) -> &Vec<u8> {
        &self.info.country
    }

    fn get_city(&self) -> &Vec<u8> {
        &self.info.city
    }

    pub fn get_account_id(&self) -> &AccountId {
        &self.account_id
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

impl<T, AccountId, Hash> ServiceOwnerInfo<T> for Lab<AccountId, Hash>
    where
        Hash: PartialEq + Eq,
        T: frame_system::Config<AccountId = AccountId>
{
    fn get_id(&self) -> &AccountId {
        &self.get_account_id()
    }

    fn get_country(&self) -> &Vec<u8> {
        &self.get_country()
    }

    fn get_city(&self) -> &Vec<u8> {
        &self.get_city()
    }
}

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResultWithPostInfo, pallet_prelude::*,
    };
    use frame_system::pallet_prelude::*;
    pub use sp_std::prelude::*;
    use crate::interface::LabInterface;
    use crate::Lab;
    use crate::LabInfo;
    pub use traits_services::{ServicesProvider, ServiceOwner};
    use frame_support::traits::Currency;


    #[pallet::config]
    /// Configure the pallet by specifying the parameters and types on which it depends.
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: Currency<Self::AccountId>;
        type Services: ServicesProvider<Self>;
    }

    // ----- This is template code, every pallet needs this ---
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
    // --------------------------------------------------------

    
    // ---- Types ----------------------
    pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    pub type HashOf<T> = <T as frame_system::Config>::Hash;
    pub type LabOf<T> = Lab<AccountIdOf<T>, HashOf<T>>;
    pub type CountryStr = Vec<u8>;
    pub type CityStr = Vec<u8>;

    // ----- Storage ------------------
    /// Get Lab by account id
    /// AccountId => Lab
    #[pallet::storage]
    #[pallet::getter(fn lab_by_account_id)]
    pub type Labs<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, LabOf<T>>;

    /// Get LabId by Country, City
    /// (CountryStr, CityStr) => Vec<AccountId>
    #[pallet::storage]
    #[pallet::getter(fn labs_by_country_city)]
    pub type LabsByCountryCity<T> = StorageDoubleMap<_, Blake2_128Concat, CountryStr, Blake2_128Concat, CityStr, Vec<AccountIdOf<T>>>;

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
    #[pallet::metadata(T::AccountId = "AccountId", LabOf<T> = "Lab")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// User AccountId registered as lab
        /// parameters. [Lab, who]
        LabRegistered(LabOf<T>, AccountIdOf<T>),
        /// Lab information updated
        /// parameters. [Lab, who]
        LabUpdated(LabOf<T>, AccountIdOf<T>),
        /// Lab deleted
        /// parameters. [Lab, who]
        LabDeleted(LabOf<T>, AccountIdOf<T>),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Account already has lab registered
        LabAlreadyRegistered,
        /// Lab identified by the AccountId does not exist
        LabDoesNotExist,
        /// Lab is not the owner of the service
        LabIsNotOwner
    }


    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn register_lab(origin: OriginFor<T>, lab_info: LabInfo) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match Self::create_lab(&who, &lab_info) {
                Ok(lab) => {
                    Self::deposit_event(Event::LabRegistered(lab, who.clone()));
                    Ok(().into())
                },
                Err(error) => Err(error)? 
            }
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn update_lab(origin: OriginFor<T>, lab_info: LabInfo) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as LabInterface<T>>::update_lab(&who, &lab_info) {
                Ok(lab) => {
                    Self::deposit_event(Event::LabUpdated(lab, who.clone()));
                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn deregister_lab(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            // Check if user is a lab
            let lab = Self::lab_by_account_id(&who);
            if lab == None {
                return Err(Error::<T>::LabDoesNotExist)?;
            }

            match <Self as LabInterface<T>>::delete_lab(&who) {
                Ok(lab) => {
                    Self::deposit_event(Event::LabDeleted(lab, who.clone()));
                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }

    }

}

impl<T: Config> LabInterface<T> for Pallet<T> {
    type Error = Error<T>;
    type LabInfo = LabInfo;
    type Lab = LabOf<T>;

    fn create_lab(account_id: &T::AccountId, lab_info: &Self::LabInfo) -> Result<Self::Lab, Self::Error> {
        if Labs::<T>::contains_key(account_id) {
            return Err(Error::<T>::LabAlreadyRegistered)?;
        }
        let lab = Lab::new(account_id.clone(), lab_info.clone());
        // Insert to Storage
        Labs::<T>::insert(account_id, &lab);
        Self::insert_lab_id_to_country_city(lab.get_country(), lab.get_city(), lab.get_account_id());

        // Increment Count
        Self::add_lab_count();
        Self::add_lab_count_by_country_city(lab.get_country(), lab.get_city());

        Ok(lab)
    }

    fn update_lab(account_id: &T::AccountId, lab_info: &Self::LabInfo) -> Result<Self::Lab, Self::Error> {
        let lab = Labs::<T>::get(account_id);
        if lab == None {
            return Err(Error::<T>::LabDoesNotExist)?;
        }
        let mut lab = lab.unwrap();

        // If location is updated, remove the lab from the old location
        if lab.get_country() != &lab_info.country && lab.get_city() != &lab_info.city {
            Self::remove_lab_id_from_country_city(lab.get_country(), lab.get_city(), lab.get_account_id());
            Self::sub_lab_count_by_country_city(lab.get_country(), lab.get_city());
        }

        lab.update_info(lab_info.clone());

        Labs::<T>::insert(account_id, &lab);
        Self::insert_lab_id_to_country_city(lab.get_country(), lab.get_city(), lab.get_account_id());
        Self::add_lab_count_by_country_city(lab.get_country(), lab.get_city());

        Ok(lab)
    }

    fn delete_lab(account_id: &T::AccountId) -> Result<Self::Lab, Self::Error> {
        let lab = Labs::<T>::get(account_id);
        if lab == None {
            return Err(Error::<T>::LabDoesNotExist)?;
        }
        let lab = lab.unwrap();
        // Delete lab's services
        for service_id in &lab.services {
            let _result = T::Services::delete_service(account_id, &service_id);
        }
        Self::remove_lab_id_from_country_city(lab.get_country(), lab.get_city(), &lab.account_id);
        Self::sub_lab_count_by_country_city(lab.get_country(), lab.get_city());
        Labs::<T>::remove(&lab.account_id);
        Self::sub_lab_count();

        Ok(lab)
    }

    fn labs_by_country_city(country: &Vec<u8>, city: &Vec<u8>) -> Option<Vec<T::AccountId>> {
        Self::labs_by_country_city(country, city)
    }

    fn lab_by_account_id(account_id: &T::AccountId) -> Option<Self::Lab> {
        Self::lab_by_account_id(account_id)
    }
}

impl<T: Config> Pallet<T> {
    pub fn insert_lab_id_to_country_city(country: &Vec<u8>, city: &Vec<u8>, lab_account_id: &T::AccountId) -> () {
        match LabsByCountryCity::<T>::get(country, city) {
            None => {
                let mut labs = Vec::new();
                labs.push(lab_account_id);
                LabsByCountryCity::<T>::insert(country, city, labs);
            },
            Some(mut labs) => {
                labs.push(lab_account_id.clone());
                LabsByCountryCity::<T>::insert(country, city, labs);
            }
        }
    }

    pub fn remove_lab_id_from_country_city(country: &Vec<u8>, city: &Vec<u8>, lab_account_id: &T::AccountId) -> () {
        // Get the lab_account_id list
        let mut labs_by_country_city = LabsByCountryCity::<T>::get(country, city).unwrap_or(Vec::new());
        // Remove id from the list
        labs_by_country_city.retain(|l_id| l_id != lab_account_id);
        //  Put back the list to storage
        LabsByCountryCity::<T>::insert(country, city, labs_by_country_city);
    }

    // Add lab count
    pub fn add_lab_count() {
        let lab_count = <LabCount<T>>::get().unwrap_or(0);
        <LabCount<T>>::put(lab_count.wrapping_add(1));
    }

    // Add lab count by country city
    pub fn add_lab_count_by_country_city(country: &Vec<u8>, city: &Vec<u8>) {
        let lab_count = <LabCountByCountryCity<T>>::get(country.clone(), city.clone()).unwrap_or(0);
        <LabCountByCountryCity<T>>::insert(country.clone(), city.clone(), lab_count.wrapping_add(1));
    }

    // Subtract lab count
    pub fn sub_lab_count() {
        let lab_count = <LabCount<T>>::get().unwrap_or(1);
        LabCount::<T>::put(lab_count - 1);
    }

    // Subtract lab count by country city
    pub fn sub_lab_count_by_country_city(country: &Vec<u8>, city: &Vec<u8>) {
        let lab_count = LabCountByCountryCity::<T>::get(country.clone(), city.clone()).unwrap_or(1);
        LabCountByCountryCity::<T>::insert(country.clone(), city.clone(), lab_count - 1);
    }
}

impl<T: Config> ServiceOwner<T> for Pallet<T> {
    type Owner = Lab<T::AccountId, T::Hash>;

    fn can_create_service(user_id: &T::AccountId) -> bool {
        return Labs::<T>::contains_key(user_id);
    }

    fn get_owner(id: &T::AccountId) -> Option<Self::Owner> {
        let lab = Labs::<T>::get(id);
        lab
    }

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
}

