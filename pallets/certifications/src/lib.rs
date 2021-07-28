#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use traits_certifications::{
    CertificationsProvider,
    CertificationOwner,
    CertificationInfo as CertificationInfoT
};
use frame_support::codec::{Encode, Decode};
use frame_support::pallet_prelude::*;

pub mod interface;
pub use interface::CertificationInterface;
use sp_std::prelude::*;

/// CertificationInfo struct
/// Information that is mutable by user
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct CertificationInfo {
    title: Vec<u8>,
    issuer: Vec<u8>,
    month: Vec<u8>,
    year: Vec<u8>,
    description: Vec<u8>,
    supporting_document: Option<Vec<u8>>,
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct Certification<AccountId, Hash> {
    pub id: Hash,
    pub owner_id: AccountId,
    pub info: CertificationInfo,
}
impl<AccountId, Hash> Certification<AccountId, Hash> {
    pub fn new(id: Hash, owner_id: AccountId, info: CertificationInfo) -> Self {
        Self {
            id,
            owner_id,
            info
        }
    }

    pub fn get_id(&self) -> &Hash {
        &self.id
    }

    pub fn get_owner_id(&self) -> &AccountId {
        &self.owner_id
    }
}

impl<T, AccountId, Hash> CertificationInfoT<T> for Certification<AccountId, Hash>
    where T: frame_system::Config<AccountId = AccountId, Hash = Hash>
{
    fn get_id(&self) -> &Hash {
        self.get_id()
    }
    fn get_owner_id(&self) -> &AccountId {
        self.get_owner_id()
    }
}

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResultWithPostInfo, pallet_prelude::*,
    };
    use frame_system::pallet_prelude::*;
    pub use sp_std::prelude::*;
    use crate::{Certification, CertificationInfo, CertificationOwner};
    use crate::interface::CertificationInterface;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type CertificationOwner: CertificationOwner<Self>;
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
    pub type CertificationOf<T> = Certification<AccountIdOf<T>, HashOf<T>>;
    pub type CertificationInfoOf = CertificationInfo;
    pub type CertificationIdOf<T> = HashOf<T>;

    // ------- Storage -------------
    #[pallet::storage]
    #[pallet::getter(fn certification_by_id)]
    pub type Certifications<T> = StorageMap<_, Blake2_128Concat, HashOf<T>, CertificationOf<T>>;
    //                                _,  Hasher         ,  Key     ,  Value

    #[pallet::storage]
    #[pallet::getter(fn certifications_count)]
    pub type CertificationsCount<T> = StorageValue<_, u64>;

    #[pallet::storage]
    #[pallet::getter(fn certification_count_by_owner)]
    pub type CertificationsCountByOwner<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, u64>;
    // -----------------------------


    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters, [Certification, who]
        CertificationCreated(CertificationOf<T>, AccountIdOf<T>),
        //// Certification updated
        /// parameters, [Certification, who]
        CertificationUpdated(CertificationOf<T>, AccountIdOf<T>),
        //// Certification deleted
        /// parameters, [Certification, who]
        CertificationDeleted(CertificationOf<T>, AccountIdOf<T>),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// User not allowed to create certification
        NotAllowedToCreate,
        /// User is not the owner of a certification
        NotCertificationOwner,
        /// Ordering a certification that does not exist
        CertificationDoesNotExist,
    }
    
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn create_certification(origin: OriginFor<T>, certification_info: CertificationInfoOf) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as CertificationInterface<T>>::create_certification(&who, &certification_info) {
                Ok(certification) => {
                    Self::deposit_event(Event::CertificationCreated(certification, who.clone()));
                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }
        
        #[pallet::weight(10_1000 + T::DbWeight::get().writes(1))]
        pub fn update_certification(origin: OriginFor<T>, certification_id: HashOf<T>, certification_info: CertificationInfoOf) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            match <Self as CertificationInterface<T>>::update_certification(&who, &certification_id, &certification_info) {
                Ok(certification) => {
                    Self::deposit_event(Event::CertificationUpdated(certification, who.clone()));
                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }

        #[pallet::weight(10_1000 + T::DbWeight::get().writes(1))]
        pub fn delete_certification(origin: OriginFor<T>, certification_id: T::Hash) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            match <Self as CertificationInterface<T>>::delete_certification(&who, &certification_id) {
                Ok(certification) => {
                    Self::deposit_event(Event::CertificationDeleted(certification, who.clone()));
                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }
    }
}

use frame_support::sp_runtime::traits::Hash;
use traits_certifications::{CertificationOwnerInfo};

/// Certification Interface Implementation
impl<T: Config> CertificationInterface<T> for Pallet<T> {
    type Error = Error<T>;
    type CertificationId = T::Hash;
    type Certification = CertificationOf<T>;
    type CertificationInfo = CertificationInfoOf;

    fn generate_certification_id(owner_id: &T::AccountId, certification_count: u64) -> Self::CertificationId {
        let mut account_id_bytes = owner_id.encode();
        let mut certification_count_bytes = certification_count.encode();
        account_id_bytes.append(&mut certification_count_bytes);

        let seed = &account_id_bytes;
        T::Hashing::hash(seed)
    }

    /// Create Certification
    /// Add reference to CertificationsByCountryCity storage
    /// Associate certification reference to the owner (creator)
    /// Increment Counts
    fn create_certification(owner_id: &T::AccountId, certification_info: &Self::CertificationInfo) -> Result<Self::Certification, Self::Error> { 
        // Check if user can create_certification
        let can_create_certification = T::CertificationOwner::can_create_certification(owner_id);
        if !can_create_certification {
            return Err(Error::<T>::NotAllowedToCreate)?;
        }

        let owner_certification_count = <Self as CertificationInterface<T>>::certification_count_by_owner(owner_id);
        let certification_id = Self::generate_certification_id(owner_id, owner_certification_count);
        
        let certification = Certification::new(certification_id.clone(), owner_id.clone(), certification_info.clone());
        // Store to Certifications storage
        Certifications::<T>::insert(&certification_id, &certification);

        // Increment Certifications Count
        Self::add_certifications_count();
        // Increment CertificationsCountByOwner
        Self::add_certification_count_by_owner(&certification.owner_id);
        
        // Associate created certification to the owner
        T::CertificationOwner::associate(owner_id, &certification_id);

        Ok(certification) 
    }

    /// Update Certification information
    fn update_certification(owner_id: &T::AccountId, certification_id: &Self::CertificationId, certification_info: &Self::CertificationInfo) -> Result<Self::Certification, Self::Error> {
        let certification = Certifications::<T>::get(certification_id);
        if certification == None {
            return Err(Error::<T>::CertificationDoesNotExist)?;
        }
        let mut certification = certification.unwrap();

        if certification.owner_id != owner_id.clone() {
            return Err(Error::<T>::NotCertificationOwner)?;
        }

        certification.info = certification_info.clone();
        Certifications::<T>::insert(certification_id, &certification);

        Ok(certification)
    }

    /// Delete Certification
    /// Delete from Certifications Storage
    /// Remove the certification id reference in CertificationsByCountryCity storage
    /// Disassociate certification id from the owner
    /// Decrement Counts
    fn delete_certification(owner_id: &T::AccountId, certification_id: &Self::CertificationId) -> Result<Self::Certification, Self::Error> {
        let certification = Certifications::<T>::get(certification_id);
        if certification == None {
            return Err(Error::<T>::CertificationDoesNotExist)?;
        }
        let certification = certification.unwrap();

        if certification.owner_id != owner_id.clone() {
            return Err(Error::<T>::NotCertificationOwner)?;
        }
        // Remove certification from storage
        let certification = Certifications::<T>::take(certification_id).unwrap();

        let owner = T::CertificationOwner::get_owner(owner_id).unwrap();
        // disassociate certification reference from the owner
        T::CertificationOwner::disassociate(owner.get_owner_id(), &certification.id);
        // Decrement counts
        Self::sub_certifications_count();
        Self::sub_certification_count_by_owner(owner.get_owner_id());

        Ok(certification)
    }

    fn certification_by_id(certification_id: &Self::CertificationId) -> Option<Self::Certification> {
        match Certifications::<T>::get(certification_id) {
            None => None,
            Some(certification) => Some(certification)
        }
    }

    fn certification_count_by_owner(owner_id: &T::AccountId) -> u64 {
        Self::certification_count_by_owner(owner_id).unwrap_or(0)
    }
}

/// Pallet Methods
impl<T: Config> Pallet<T> {
    // Certifications Count Addition and Substraction Helpers
    // Add certifications count
    pub fn add_certifications_count() {
        let certifications_count = <CertificationsCount<T>>::get().unwrap_or(0);
        <CertificationsCount<T>>::put(certifications_count.wrapping_add(1));
    }
    // Add certifications count by owner
    pub fn add_certification_count_by_owner(owner_id: &T::AccountId) {
        let certifications_count = CertificationsCountByOwner::<T>::get(owner_id).unwrap_or(0);
        CertificationsCountByOwner::<T>::insert(owner_id, certifications_count.wrapping_add(1))
    }

    // Subtract certifications count
    pub fn sub_certifications_count() {
        let certifications_count = <CertificationsCount<T>>::get().unwrap_or(1);
        CertificationsCount::<T>::put(certifications_count - 1);
    }
    // Subtract certifications count by owner
    pub fn sub_certification_count_by_owner(owner_id: &T::AccountId) {
        let certifications_count = CertificationsCountByOwner::<T>::get(owner_id).unwrap_or(1);
        CertificationsCountByOwner::<T>::insert(owner_id, certifications_count - 1);
    }
}

/// CertificationsProvider Trait Implementation
impl<T: Config> CertificationsProvider<T> for Pallet<T> {
    type Error = Error<T>;
    type Certification = CertificationOf<T>;

    fn certification_by_id(id: &T::Hash) -> Option<CertificationOf<T>> {
        <Self as CertificationInterface<T>>::certification_by_id(id)
    }

    fn delete_certification(owner_id: &T::AccountId, id: &T::Hash) -> Result<Self::Certification, Self::Error> {
        <Self as CertificationInterface<T>>::delete_certification(owner_id, id)
    }
}