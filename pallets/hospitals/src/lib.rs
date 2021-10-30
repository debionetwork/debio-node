#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
pub use pallet::*;
pub use scale_info::TypeInfo;

#[cfg(test)]
mod mock;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod interface;
pub use crate::interface::HospitalInterface;
use frame_support::pallet_prelude::*;
use traits_hospital_certifications::HospitalCertificationOwnerInfo;
use traits_user_profile::UserProfileProvider;

// HospitalInfo Struct
// Used as parameter of dispatchable calls
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct HospitalInfo {
    pub name: Vec<u8>,
    pub email: Vec<u8>,
    pub country: Vec<u8>,
    pub region: Vec<u8>,
    pub city: Vec<u8>,
    pub address: Vec<u8>,
    pub latitude: Option<Vec<u8>>,
    pub longitude: Option<Vec<u8>>,
    pub profile_image: Option<Vec<u8>>,
}

// Hospital Struct
// the fields (excluding account_id and certifications) come from HospitalInfo struct
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct Hospital<AccountId, Hash>
where
    Hash: PartialEq + Eq,
{
    pub account_id: AccountId,
    pub certifications: Vec<Hash>,
    pub info: HospitalInfo,
}

impl<AccountId, Hash> Hospital<AccountId, Hash>
where
    Hash: PartialEq + Eq,
{
    pub fn new(account_id: AccountId, info: HospitalInfo) -> Self {
        Self {
            account_id,
            certifications: Vec::<Hash>::new(),
            info,
        }
    }

    fn update_info(&mut self, info: HospitalInfo) -> () {
        self.info = info;
    }

    fn get_country(&self) -> &Vec<u8> {
        &self.info.country
    }

    fn get_region(&self) -> &Vec<u8> {
        &self.info.region
    }

    fn get_city(&self) -> &Vec<u8> {
        &self.info.city
    }

    // Returns CountryCode-RegionCode -> XX-YYY
    fn get_country_region(&self) -> Vec<u8> {
        helpers::build_country_region_code(&self.get_country(), &self.get_region())
    }

    pub fn get_account_id(&self) -> &AccountId {
        &self.account_id
    }

    pub fn add_certification(&mut self, certification_id: Hash) -> () {
        &self.certifications.push(certification_id);
    }

    pub fn remove_certification(&mut self, certification_id: Hash) -> () {
        if let Some(pos) = &self
            .certifications
            .iter()
            .position(|x| *x == certification_id)
        {
            &self.certifications.remove(*pos);
        }
    }
}

impl<T, AccountId, Hash> HospitalCertificationOwnerInfo<T> for Hospital<AccountId, Hash>
where
    Hash: PartialEq + Eq,
    T: frame_system::Config<AccountId = AccountId>,
{
    fn get_owner_id(&self) -> &AccountId {
        &self.get_account_id()
    }
}

pub mod helpers {
    use crate::*;

    /// Concatenate CountryCode with RegionCode with a '-'
    pub fn build_country_region_code(country_code: &Vec<u8>, region_code: &Vec<u8>) -> Vec<u8> {
        // container
        let mut country_region_code = Vec::new();
        let mut country_code = country_code.clone();
        // dash character as u8
        let mut dash = ['-'].iter().map(|c| *c as u8).collect::<Vec<u8>>();
        let mut region_code = region_code.clone();

        country_region_code.append(&mut country_code);
        country_region_code.append(&mut dash);
        country_region_code.append(&mut region_code);

        country_region_code
    }
}

#[frame_support::pallet]
pub mod pallet {
    use crate::interface::HospitalInterface;
    use crate::Hospital;
    use crate::HospitalInfo;
    use crate::*;
    use codec::EncodeLike;
    use frame_support::traits::Currency;
    use frame_support::{dispatch::DispatchResultWithPostInfo};
    use frame_system::pallet_prelude::*;
    pub use sp_std::prelude::*;
    pub use traits_hospital_certifications::{
        HospitalCertificationOwner, HospitalCertificationsProvider,
    };

    #[pallet::config]
    /// Configure the pallet by specifying the parameters and types on which it depends.
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: Currency<Self::AccountId>;
        type HospitalCertifications: HospitalCertificationsProvider<Self>;
        type EthereumAddress: Clone
            + Copy
            + PartialEq
            + Eq
            + Encode
            + EncodeLike
            + Decode
            + Default
			+ TypeInfo
            + sp_std::fmt::Debug;
        type UserProfile: UserProfileProvider<Self, Self::EthereumAddress>;
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
    pub type HospitalOf<T> = Hospital<AccountIdOf<T>, HashOf<T>>;
    pub type CountryRegionCode = Vec<u8>; // country_code-region_code -> XX-YYYY
    pub type CityCode = Vec<u8>; // city_code -> ZZZZ

    // ----- Storage ------------------
    /// Get Hospital by account id
    /// AccountId => Hospital
    #[pallet::storage]
    #[pallet::getter(fn hospital_by_account_id)]
    pub type Hospitals<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, HospitalOf<T>>;

    /// Get HospitalId by Country-Region, City
    /// (CountryRegionCode, CityCode) => Vec<AccountId>
    #[pallet::storage]
    #[pallet::getter(fn hospitals_by_country_region_city)]
    pub type HospitalsByCountryRegionCity<T> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        CountryRegionCode,
        Blake2_128Concat,
        CityCode,
        Vec<AccountIdOf<T>>,
    >;

    /// Get total hospital count
    /// u32
    #[pallet::storage]
    #[pallet::getter(fn hospital_count)]
    pub type HospitalCount<T> = StorageValue<_, u64>;

    /// Get total hospital count by Country-Region, City
    /// (CountryRegionCode, CityCode) => u32
    #[pallet::storage]
    #[pallet::getter(fn hospital_count_by_country_region_city)]
    pub type HospitalCountByCountryRegionCity<T> =
        StorageDoubleMap<_, Blake2_128Concat, CountryRegionCode, Blake2_128Concat, CityCode, u64>;
    // -----------------------------------

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// User AccountId registered as hospital
        /// parameters. [Hospital, who]
        HospitalRegistered(HospitalOf<T>, AccountIdOf<T>),
        /// Hospital information updated
        /// parameters. [Hospital, who]
        HospitalUpdated(HospitalOf<T>, AccountIdOf<T>),
        /// Hospital deleted
        /// parameters. [Hospital, who]
        HospitalDeleted(HospitalOf<T>, AccountIdOf<T>),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Account already has hospital registered
        HospitalAlreadyRegistered,
        /// Hospital identified by the AccountId does not exist
        HospitalDoesNotExist,
        /// Hospital is not the owner of the certification
        HospitalIsNotOwner,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(20_000 + T::DbWeight::get().reads_writes(1, 2))]
        pub fn register_hospital(
            origin: OriginFor<T>,
            hospital_info: HospitalInfo,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match Self::create_hospital(&who, &hospital_info) {
                Ok(hospital) => {
                    Self::deposit_event(Event::HospitalRegistered(hospital, who.clone()));
                    Ok(().into())
                }
                Err(error) => Err(error)?,
            }
        }

        #[pallet::weight(20_000 + T::DbWeight::get().reads_writes(1, 2))]
        pub fn update_hospital(
            origin: OriginFor<T>,
            hospital_info: HospitalInfo,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as HospitalInterface<T>>::update_hospital(&who, &hospital_info) {
                Ok(hospital) => {
                    Self::deposit_event(Event::HospitalUpdated(hospital, who.clone()));
                    Ok(().into())
                }
                Err(error) => Err(error)?,
            }
        }

        #[pallet::weight(20_000 + T::DbWeight::get().reads_writes(1, 2))]
        pub fn deregister_hospital(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            // Check if user is a hospital
            let hospital = Self::hospital_by_account_id(&who);
            if hospital == None {
                return Err(Error::<T>::HospitalDoesNotExist)?;
            }

            match <Self as HospitalInterface<T>>::delete_hospital(&who) {
                Ok(hospital) => {
                    Self::deposit_event(Event::HospitalDeleted(hospital, who.clone()));
                    Ok(().into())
                }
                Err(error) => Err(error)?,
            }
        }
    }
}

impl<T: Config> HospitalInterface<T> for Pallet<T> {
    type Error = Error<T>;
    type HospitalInfo = HospitalInfo;
    type Hospital = HospitalOf<T>;

    fn create_hospital(
        account_id: &T::AccountId,
        hospital_info: &Self::HospitalInfo,
    ) -> Result<Self::Hospital, Self::Error> {
        if Hospitals::<T>::contains_key(account_id) {
            return Err(Error::<T>::HospitalAlreadyRegistered)?;
        }
        let hospital = Hospital::new(account_id.clone(), hospital_info.clone());
        // Insert to Storage
        Hospitals::<T>::insert(account_id, &hospital);
        Self::insert_hospital_id_to_location(&hospital);

        // Increment Count
        Self::add_hospital_count();
        Self::add_hospital_count_by_location(&hospital);

        Ok(hospital)
    }

    fn update_hospital(
        account_id: &T::AccountId,
        hospital_info: &Self::HospitalInfo,
    ) -> Result<Self::Hospital, Self::Error> {
        let hospital = Hospitals::<T>::get(account_id);
        if hospital == None {
            return Err(Error::<T>::HospitalDoesNotExist)?;
        }
        let mut hospital = hospital.unwrap();

        // If location is updated, remove the hospital from the old location
        // Also update certification locations
        if hospital.get_country() != &hospital_info.country
            || hospital.get_region() != &hospital_info.region
            || hospital.get_city() != &hospital_info.city
        {
            Self::remove_hospital_id_from_location(&hospital);
            Self::sub_hospital_count_by_location(&hospital);
        }

        hospital.update_info(hospital_info.clone());

        Hospitals::<T>::insert(account_id, &hospital);
        Self::insert_hospital_id_to_location(&hospital);
        Self::add_hospital_count_by_location(&hospital);

        Ok(hospital)
    }

    fn delete_hospital(account_id: &T::AccountId) -> Result<Self::Hospital, Self::Error> {
        let hospital = Hospitals::<T>::get(account_id);
        if hospital == None {
            return Err(Error::<T>::HospitalDoesNotExist)?;
        }
        let hospital = hospital.unwrap();
        // Delete hospital's certifications
        for certification_id in &hospital.certifications {
            let _result =
                T::HospitalCertifications::delete_certification(account_id, &certification_id);
        }
        Self::remove_hospital_id_from_location(&hospital);
        Self::sub_hospital_count_by_location(&hospital);
        Hospitals::<T>::remove(&hospital.account_id);
        Self::sub_hospital_count();

        Ok(hospital)
    }

    fn hospitals_by_country_region_city(
        country_region_code: &Vec<u8>,
        city_code: &Vec<u8>,
    ) -> Option<Vec<T::AccountId>> {
        Self::hospitals_by_country_region_city(country_region_code, city_code)
    }

    fn hospital_by_account_id(account_id: &T::AccountId) -> Option<Self::Hospital> {
        Self::hospital_by_account_id(account_id)
    }
}

impl<T: Config> Pallet<T> {
    pub fn insert_hospital_id_to_location(hospital: &HospitalOf<T>) -> () {
        let country_region_code = hospital.get_country_region();
        let city_code = hospital.get_city();
        let hospital_account_id = hospital.get_account_id();

        match HospitalsByCountryRegionCity::<T>::get(&country_region_code, city_code) {
            None => {
                let mut hospitals = Vec::new();
                hospitals.push(hospital_account_id.clone());
                HospitalsByCountryRegionCity::<T>::insert(
                    &country_region_code,
                    city_code,
                    hospitals,
                );
            }
            Some(mut hospitals) => {
                hospitals.push(hospital_account_id.clone());
                HospitalsByCountryRegionCity::<T>::insert(
                    &country_region_code,
                    city_code,
                    hospitals,
                );
            }
        }
    }

    pub fn remove_hospital_id_from_location(hospital: &HospitalOf<T>) -> () {
        let country_region_code = hospital.get_country_region();
        let city_code = hospital.get_city();
        let hospital_account_id = hospital.get_account_id();

        // Get the hospital_account_id list
        let mut hospitals_by_location =
            HospitalsByCountryRegionCity::<T>::get(&country_region_code, city_code)
                .unwrap_or(Vec::new());
        // Remove id from the list
        hospitals_by_location.retain(|l_id| l_id != hospital_account_id);
        //  Put back the list to storage
        HospitalsByCountryRegionCity::<T>::insert(
            &country_region_code,
            city_code,
            hospitals_by_location,
        );
    }

    // Add hospital count
    pub fn add_hospital_count() {
        let hospital_count = <HospitalCount<T>>::get().unwrap_or(0);
        <HospitalCount<T>>::put(hospital_count.wrapping_add(1));
    }

    // Add hospital count by location
    pub fn add_hospital_count_by_location(hospital: &HospitalOf<T>) {
        let country_region_code = hospital.get_country_region();
        let city_code = hospital.get_city();

        let hospital_count = <HospitalCountByCountryRegionCity<T>>::get(
            country_region_code.clone(),
            city_code.clone(),
        )
        .unwrap_or(0);
        <HospitalCountByCountryRegionCity<T>>::insert(
            country_region_code.clone(),
            city_code.clone(),
            hospital_count.wrapping_add(1),
        );
    }

    // Subtract hospital count
    pub fn sub_hospital_count() {
        let hospital_count = <HospitalCount<T>>::get().unwrap_or(1);
        HospitalCount::<T>::put(hospital_count - 1);
    }

    // Subtract hospital count by location
    pub fn sub_hospital_count_by_location(hospital: &HospitalOf<T>) {
        let country_region_code = hospital.get_country_region();
        let city_code = hospital.get_city();

        let hospital_count = HospitalCountByCountryRegionCity::<T>::get(
            country_region_code.clone(),
            city_code.clone(),
        )
        .unwrap_or(1);
        HospitalCountByCountryRegionCity::<T>::insert(
            country_region_code.clone(),
            city_code.clone(),
            hospital_count - 1,
        );
    }
}

impl<T: Config> HospitalCertificationOwner<T> for Pallet<T> {
    type Owner = Hospital<T::AccountId, T::Hash>;

    /// User can create certification if he/she is a hospital
    fn can_create_certification(user_id: &T::AccountId) -> bool {
        return Hospitals::<T>::contains_key(user_id);
    }

    fn get_owner(id: &T::AccountId) -> Option<Self::Owner> {
        let hospital = Hospitals::<T>::get(id);
        hospital
    }

    fn associate(owner_id: &T::AccountId, certification_id: &T::Hash) -> () {
        <Hospitals<T>>::mutate(owner_id, |hospital| {
            match hospital {
                None => (), // If hospital does not exist, do nothing
                Some(hospital) => {
                    hospital.add_certification(*certification_id);
                }
            }
        });
    }

    fn disassociate(owner_id: &T::AccountId, certification_id: &T::Hash) -> () {
        Hospitals::<T>::mutate(owner_id, |hospital| match hospital {
            None => (),
            Some(hospital) => {
                hospital.remove_certification(*certification_id);
            }
        });
    }
}
