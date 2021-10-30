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
pub use crate::interface::DoctorInterface;
use frame_support::pallet_prelude::*;
use traits_doctor_certifications::DoctorCertificationOwnerInfo;
use traits_user_profile::UserProfileProvider;

// DoctorInfo Struct
// Used as parameter of dispatchable calls
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct DoctorInfo {
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

// Doctor Struct
// the fields (excluding account_id and certifications) come from DoctorInfo struct
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct Doctor<AccountId, Hash>
where
    Hash: PartialEq + Eq,
{
    pub account_id: AccountId,
    pub certifications: Vec<Hash>,
    pub info: DoctorInfo,
}

impl<AccountId, Hash> Doctor<AccountId, Hash>
where
    Hash: PartialEq + Eq,
{
    pub fn new(account_id: AccountId, info: DoctorInfo) -> Self {
        Self {
            account_id,
            certifications: Vec::<Hash>::new(),
            info,
        }
    }

    fn update_info(&mut self, info: DoctorInfo) -> () {
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

impl<T, AccountId, Hash> DoctorCertificationOwnerInfo<T> for Doctor<AccountId, Hash>
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
    use crate::interface::DoctorInterface;
    use crate::Doctor;
    use crate::DoctorInfo;
    use crate::*;
    use codec::EncodeLike;
    use frame_support::traits::Currency;
    use frame_support::{dispatch::DispatchResultWithPostInfo};
    use frame_system::pallet_prelude::*;
    pub use sp_std::prelude::*;
    pub use traits_doctor_certifications::{
        DoctorCertificationOwner, DoctorCertificationsProvider,
    };

    #[pallet::config]
    /// Configure the pallet by specifying the parameters and types on which it depends.
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: Currency<Self::AccountId>;
        type DoctorCertifications: DoctorCertificationsProvider<Self>;
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
    pub type DoctorOf<T> = Doctor<AccountIdOf<T>, HashOf<T>>;
    pub type CountryRegionCode = Vec<u8>; // country_code-region_code -> XX-YYYY
    pub type CityCode = Vec<u8>; // city_code -> ZZZZ

    // ----- Storage ------------------
    /// Get Doctor by account id
    /// AccountId => Doctor
    #[pallet::storage]
    #[pallet::getter(fn doctor_by_account_id)]
    pub type Doctors<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, DoctorOf<T>>;

    /// Get DoctorId by Country-Region, City
    /// (CountryRegionCode, CityCode) => Vec<AccountId>
    #[pallet::storage]
    #[pallet::getter(fn doctors_by_country_region_city)]
    pub type DoctorsByCountryRegionCity<T> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        CountryRegionCode,
        Blake2_128Concat,
        CityCode,
        Vec<AccountIdOf<T>>,
    >;

    /// Get total doctor count
    /// u32
    #[pallet::storage]
    #[pallet::getter(fn doctor_count)]
    pub type DoctorCount<T> = StorageValue<_, u64>;

    /// Get total doctor count by Country-Region, City
    /// (CountryRegionCode, CityCode) => u32
    #[pallet::storage]
    #[pallet::getter(fn doctor_count_by_country_region_city)]
    pub type DoctorCountByCountryRegionCity<T> =
        StorageDoubleMap<_, Blake2_128Concat, CountryRegionCode, Blake2_128Concat, CityCode, u64>;
    // -----------------------------------

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// User AccountId registered as doctor
        /// parameters. [Doctor, who]
        DoctorRegistered(DoctorOf<T>, AccountIdOf<T>),
        /// Doctor information updated
        /// parameters. [Doctor, who]
        DoctorUpdated(DoctorOf<T>, AccountIdOf<T>),
        /// Doctor deleted
        /// parameters. [Doctor, who]
        DoctorDeleted(DoctorOf<T>, AccountIdOf<T>),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Account already has doctor registered
        DoctorAlreadyRegistered,
        /// Doctor identified by the AccountId does not exist
        DoctorDoesNotExist,
        /// Doctor is not the owner of the certification
        DoctorIsNotOwner,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(20_000 + T::DbWeight::get().reads_writes(1, 2))]
        pub fn register_doctor(
            origin: OriginFor<T>,
            doctor_info: DoctorInfo,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match Self::create_doctor(&who, &doctor_info) {
                Ok(doctor) => {
                    Self::deposit_event(Event::DoctorRegistered(doctor, who.clone()));
                    Ok(().into())
                }
                Err(error) => Err(error)?,
            }
        }

        #[pallet::weight(20_000 + T::DbWeight::get().reads_writes(1, 2))]
        pub fn update_doctor(
            origin: OriginFor<T>,
            doctor_info: DoctorInfo,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as DoctorInterface<T>>::update_doctor(&who, &doctor_info) {
                Ok(doctor) => {
                    Self::deposit_event(Event::DoctorUpdated(doctor, who.clone()));
                    Ok(().into())
                }
                Err(error) => Err(error)?,
            }
        }

        #[pallet::weight(20_000 + T::DbWeight::get().reads_writes(1, 2))]
        pub fn deregister_doctor(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            // Check if user is a doctor
            let doctor = Self::doctor_by_account_id(&who);
            if doctor == None {
                return Err(Error::<T>::DoctorDoesNotExist)?;
            }

            match <Self as DoctorInterface<T>>::delete_doctor(&who) {
                Ok(doctor) => {
                    Self::deposit_event(Event::DoctorDeleted(doctor, who.clone()));
                    Ok(().into())
                }
                Err(error) => Err(error)?,
            }
        }
    }
}

impl<T: Config> DoctorInterface<T> for Pallet<T> {
    type Error = Error<T>;
    type DoctorInfo = DoctorInfo;
    type Doctor = DoctorOf<T>;

    fn create_doctor(
        account_id: &T::AccountId,
        doctor_info: &Self::DoctorInfo,
    ) -> Result<Self::Doctor, Self::Error> {
        if Doctors::<T>::contains_key(account_id) {
            return Err(Error::<T>::DoctorAlreadyRegistered)?;
        }
        let doctor = Doctor::new(account_id.clone(), doctor_info.clone());
        // Insert to Storage
        Doctors::<T>::insert(account_id, &doctor);
        Self::insert_doctor_id_to_location(&doctor);

        // Increment Count
        Self::add_doctor_count();
        Self::add_doctor_count_by_location(&doctor);

        Ok(doctor)
    }

    fn update_doctor(
        account_id: &T::AccountId,
        doctor_info: &Self::DoctorInfo,
    ) -> Result<Self::Doctor, Self::Error> {
        let doctor = Doctors::<T>::get(account_id);
        if doctor == None {
            return Err(Error::<T>::DoctorDoesNotExist)?;
        }
        let mut doctor = doctor.unwrap();

        // If location is updated, remove the doctor from the old location
        // Also update certification locations
        if doctor.get_country() != &doctor_info.country
            || doctor.get_region() != &doctor_info.region
            || doctor.get_city() != &doctor_info.city
        {
            Self::remove_doctor_id_from_location(&doctor);
            Self::sub_doctor_count_by_location(&doctor);
        }

        doctor.update_info(doctor_info.clone());

        Doctors::<T>::insert(account_id, &doctor);
        Self::insert_doctor_id_to_location(&doctor);
        Self::add_doctor_count_by_location(&doctor);

        Ok(doctor)
    }

    fn delete_doctor(account_id: &T::AccountId) -> Result<Self::Doctor, Self::Error> {
        let doctor = Doctors::<T>::get(account_id);
        if doctor == None {
            return Err(Error::<T>::DoctorDoesNotExist)?;
        }
        let doctor = doctor.unwrap();
        // Delete doctor's certifications
        for certification_id in &doctor.certifications {
            let _result =
                T::DoctorCertifications::delete_certification(account_id, &certification_id);
        }
        Self::remove_doctor_id_from_location(&doctor);
        Self::sub_doctor_count_by_location(&doctor);
        Doctors::<T>::remove(&doctor.account_id);
        Self::sub_doctor_count();

        Ok(doctor)
    }

    fn doctors_by_country_region_city(
        country_region_code: &Vec<u8>,
        city_code: &Vec<u8>,
    ) -> Option<Vec<T::AccountId>> {
        Self::doctors_by_country_region_city(country_region_code, city_code)
    }

    fn doctor_by_account_id(account_id: &T::AccountId) -> Option<Self::Doctor> {
        Self::doctor_by_account_id(account_id)
    }
}

impl<T: Config> Pallet<T> {
    pub fn insert_doctor_id_to_location(doctor: &DoctorOf<T>) -> () {
        let country_region_code = doctor.get_country_region();
        let city_code = doctor.get_city();
        let doctor_account_id = doctor.get_account_id();

        match DoctorsByCountryRegionCity::<T>::get(&country_region_code, city_code) {
            None => {
                let mut doctors = Vec::new();
                doctors.push(doctor_account_id.clone());
                DoctorsByCountryRegionCity::<T>::insert(&country_region_code, city_code, doctors);
            }
            Some(mut doctors) => {
                doctors.push(doctor_account_id.clone());
                DoctorsByCountryRegionCity::<T>::insert(&country_region_code, city_code, doctors);
            }
        }
    }

    pub fn remove_doctor_id_from_location(doctor: &DoctorOf<T>) -> () {
        let country_region_code = doctor.get_country_region();
        let city_code = doctor.get_city();
        let doctor_account_id = doctor.get_account_id();

        // Get the doctor_account_id list
        let mut doctors_by_location =
            DoctorsByCountryRegionCity::<T>::get(&country_region_code, city_code)
                .unwrap_or(Vec::new());
        // Remove id from the list
        doctors_by_location.retain(|l_id| l_id != doctor_account_id);
        //  Put back the list to storage
        DoctorsByCountryRegionCity::<T>::insert(
            &country_region_code,
            city_code,
            doctors_by_location,
        );
    }

    // Add doctor count
    pub fn add_doctor_count() {
        let doctor_count = <DoctorCount<T>>::get().unwrap_or(0);
        <DoctorCount<T>>::put(doctor_count.wrapping_add(1));
    }

    // Add doctor count by location
    pub fn add_doctor_count_by_location(doctor: &DoctorOf<T>) {
        let country_region_code = doctor.get_country_region();
        let city_code = doctor.get_city();

        let doctor_count = <DoctorCountByCountryRegionCity<T>>::get(
            country_region_code.clone(),
            city_code.clone(),
        )
        .unwrap_or(0);
        <DoctorCountByCountryRegionCity<T>>::insert(
            country_region_code.clone(),
            city_code.clone(),
            doctor_count.wrapping_add(1),
        );
    }

    // Subtract doctor count
    pub fn sub_doctor_count() {
        let doctor_count = <DoctorCount<T>>::get().unwrap_or(1);
        DoctorCount::<T>::put(doctor_count - 1);
    }

    // Subtract doctor count by location
    pub fn sub_doctor_count_by_location(doctor: &DoctorOf<T>) {
        let country_region_code = doctor.get_country_region();
        let city_code = doctor.get_city();

        let doctor_count = DoctorCountByCountryRegionCity::<T>::get(
            country_region_code.clone(),
            city_code.clone(),
        )
        .unwrap_or(1);
        DoctorCountByCountryRegionCity::<T>::insert(
            country_region_code.clone(),
            city_code.clone(),
            doctor_count - 1,
        );
    }
}

impl<T: Config> DoctorCertificationOwner<T> for Pallet<T> {
    type Owner = Doctor<T::AccountId, T::Hash>;

    /// User can create certification if he/she is a doctor
    fn can_create_certification(user_id: &T::AccountId) -> bool {
        return Doctors::<T>::contains_key(user_id);
    }

    fn get_owner(id: &T::AccountId) -> Option<Self::Owner> {
        let doctor = Doctors::<T>::get(id);
        doctor
    }

    fn associate(owner_id: &T::AccountId, certification_id: &T::Hash) -> () {
        <Doctors<T>>::mutate(owner_id, |doctor| {
            match doctor {
                None => (), // If doctor does not exist, do nothing
                Some(doctor) => {
                    doctor.add_certification(*certification_id);
                }
            }
        });
    }

    fn disassociate(owner_id: &T::AccountId, certification_id: &T::Hash) -> () {
        Doctors::<T>::mutate(owner_id, |doctor| match doctor {
            None => (),
            Some(doctor) => {
                doctor.remove_certification(*certification_id);
            }
        });
    }
}
