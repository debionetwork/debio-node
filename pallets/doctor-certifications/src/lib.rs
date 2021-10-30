#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
pub use pallet::*;
pub use scale_info::TypeInfo;
use traits_doctor_certifications::{
    DoctorCertificationInfo as DoctorCertificationInfoT, DoctorCertificationOwner,
    DoctorCertificationsProvider,
};

pub mod interface;
pub use interface::DoctorCertificationInterface;
use sp_std::prelude::*;

/// DoctorCertificationInfo struct
/// Information that is mutable by user
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct DoctorCertificationInfo {
    pub title: Vec<u8>,
    pub issuer: Vec<u8>,
    pub month: Vec<u8>,
    pub year: Vec<u8>,
    pub description: Vec<u8>,
    pub supporting_document: Option<Vec<u8>>,
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct DoctorCertification<AccountId, Hash> {
    pub id: Hash,
    pub owner_id: AccountId,
    pub info: DoctorCertificationInfo,
}
impl<AccountId, Hash> DoctorCertification<AccountId, Hash> {
    pub fn new(id: Hash, owner_id: AccountId, info: DoctorCertificationInfo) -> Self {
        Self { id, owner_id, info }
    }

    pub fn get_id(&self) -> &Hash {
        &self.id
    }

    pub fn get_owner_id(&self) -> &AccountId {
        &self.owner_id
    }
}

impl<T, AccountId, Hash> DoctorCertificationInfoT<T> for DoctorCertification<AccountId, Hash>
where
    T: frame_system::Config<AccountId = AccountId, Hash = Hash>,
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
    use crate::interface::DoctorCertificationInterface;
    use crate::{DoctorCertification, DoctorCertificationInfo, DoctorCertificationOwner};
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    pub use sp_std::prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type DoctorCertificationOwner: DoctorCertificationOwner<Self>;
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
    pub type DoctorCertificationOf<T> = DoctorCertification<AccountIdOf<T>, HashOf<T>>;
    pub type DoctorCertificationInfoOf = DoctorCertificationInfo;
    pub type DoctorCertificationIdOf<T> = HashOf<T>;

    // ------- Storage -------------
    #[pallet::storage]
    #[pallet::getter(fn certification_by_id)]
    pub type DoctorCertifications<T> =
        StorageMap<_, Blake2_128Concat, HashOf<T>, DoctorCertificationOf<T>>;
    //                                _,  Hasher         ,  Key     ,  Value

    #[pallet::storage]
    #[pallet::getter(fn certifications_count)]
    pub type DoctorCertificationsCount<T> = StorageValue<_, u64>;

    #[pallet::storage]
    #[pallet::getter(fn certification_count_by_owner)]
    pub type DoctorCertificationsCountByOwner<T> =
        StorageMap<_, Blake2_128Concat, AccountIdOf<T>, u64>;
    // -----------------------------

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters, [DoctorCertification, who]
        DoctorCertificationCreated(DoctorCertificationOf<T>, AccountIdOf<T>),
        //// DoctorCertification updated
        /// parameters, [DoctorCertification, who]
        DoctorCertificationUpdated(DoctorCertificationOf<T>, AccountIdOf<T>),
        //// DoctorCertification deleted
        /// parameters, [DoctorCertification, who]
        DoctorCertificationDeleted(DoctorCertificationOf<T>, AccountIdOf<T>),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// User not allowed to create certification
        NotAllowedToCreate,
        /// User is not the owner of a certification
        NotDoctorCertificationOwner,
        /// Ordering a certification that does not exist
        DoctorCertificationDoesNotExist,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(20_000 + T::DbWeight::get().reads_writes(1, 2))]
        pub fn create_certification(
            origin: OriginFor<T>,
            certification_info: DoctorCertificationInfoOf,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as DoctorCertificationInterface<T>>::create_certification(
                &who,
                &certification_info,
            ) {
                Ok(certification) => {
                    Self::deposit_event(Event::DoctorCertificationCreated(
                        certification,
                        who.clone(),
                    ));
                    Ok(().into())
                }
                Err(error) => Err(error)?,
            }
        }

        #[pallet::weight(20_000 + T::DbWeight::get().reads_writes(1, 2))]
        pub fn update_certification(
            origin: OriginFor<T>,
            certification_id: HashOf<T>,
            certification_info: DoctorCertificationInfoOf,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            match <Self as DoctorCertificationInterface<T>>::update_certification(
                &who,
                &certification_id,
                &certification_info,
            ) {
                Ok(certification) => {
                    Self::deposit_event(Event::DoctorCertificationUpdated(
                        certification,
                        who.clone(),
                    ));
                    Ok(().into())
                }
                Err(error) => Err(error)?,
            }
        }

        #[pallet::weight(20_000 + T::DbWeight::get().reads_writes(1, 2))]
        pub fn delete_certification(
            origin: OriginFor<T>,
            certification_id: T::Hash,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            match <Self as DoctorCertificationInterface<T>>::delete_certification(
                &who,
                &certification_id,
            ) {
                Ok(certification) => {
                    Self::deposit_event(Event::DoctorCertificationDeleted(
                        certification,
                        who.clone(),
                    ));
                    Ok(().into())
                }
                Err(error) => Err(error)?,
            }
        }
    }
}

use frame_support::sp_runtime::traits::Hash;
use traits_doctor_certifications::DoctorCertificationOwnerInfo;

/// DoctorCertification Interface Implementation
impl<T: Config> DoctorCertificationInterface<T> for Pallet<T> {
    type Error = Error<T>;
    type DoctorCertificationId = T::Hash;
    type DoctorCertification = DoctorCertificationOf<T>;
    type DoctorCertificationInfo = DoctorCertificationInfoOf;

    fn generate_certification_id(
        owner_id: &T::AccountId,
        certification_count: u64,
    ) -> Self::DoctorCertificationId {
        let mut account_id_bytes = owner_id.encode();
        let mut certification_count_bytes = certification_count.encode();
        account_id_bytes.append(&mut certification_count_bytes);

        let seed = &account_id_bytes;
        T::Hashing::hash(seed)
    }

    /// Create DoctorCertification
    /// Add reference to DoctorCertificationsByCountryCity storage
    /// Associate certification reference to the owner (creator)
    /// Increment Counts
    fn create_certification(
        owner_id: &T::AccountId,
        certification_info: &Self::DoctorCertificationInfo,
    ) -> Result<Self::DoctorCertification, Self::Error> {
        // Check if user can create_certification
        let can_create_certification =
            T::DoctorCertificationOwner::can_create_certification(owner_id);
        if !can_create_certification {
            return Err(Error::<T>::NotAllowedToCreate)?;
        }

        let owner_certification_count =
            <Self as DoctorCertificationInterface<T>>::certification_count_by_owner(owner_id);
        let certification_id = Self::generate_certification_id(owner_id, owner_certification_count);

        let certification = DoctorCertification::new(
            certification_id.clone(),
            owner_id.clone(),
            certification_info.clone(),
        );
        // Store to DoctorCertifications storage
        DoctorCertifications::<T>::insert(&certification_id, &certification);

        // Increment DoctorCertifications Count
        Self::add_certifications_count();
        // Increment DoctorCertificationsCountByOwner
        Self::add_certification_count_by_owner(&certification.owner_id);

        // Associate created certification to the owner
        T::DoctorCertificationOwner::associate(owner_id, &certification_id);

        Ok(certification)
    }

    /// Update DoctorCertification information
    fn update_certification(
        owner_id: &T::AccountId,
        certification_id: &Self::DoctorCertificationId,
        certification_info: &Self::DoctorCertificationInfo,
    ) -> Result<Self::DoctorCertification, Self::Error> {
        let certification = DoctorCertifications::<T>::get(certification_id);
        if certification == None {
            return Err(Error::<T>::DoctorCertificationDoesNotExist)?;
        }
        let mut certification = certification.unwrap();

        if certification.owner_id != owner_id.clone() {
            return Err(Error::<T>::NotDoctorCertificationOwner)?;
        }

        certification.info = certification_info.clone();
        DoctorCertifications::<T>::insert(certification_id, &certification);

        Ok(certification)
    }

    /// Delete DoctorCertification
    /// Delete from DoctorCertifications Storage
    /// Remove the certification id reference in DoctorCertificationsByCountryCity storage
    /// Disassociate certification id from the owner
    /// Decrement Counts
    fn delete_certification(
        owner_id: &T::AccountId,
        certification_id: &Self::DoctorCertificationId,
    ) -> Result<Self::DoctorCertification, Self::Error> {
        let certification = DoctorCertifications::<T>::get(certification_id);
        if certification == None {
            return Err(Error::<T>::DoctorCertificationDoesNotExist)?;
        }
        let certification = certification.unwrap();

        if certification.owner_id != owner_id.clone() {
            return Err(Error::<T>::NotDoctorCertificationOwner)?;
        }
        // Remove certification from storage
        let certification = DoctorCertifications::<T>::take(certification_id).unwrap();

        let owner = T::DoctorCertificationOwner::get_owner(owner_id).unwrap();
        // disassociate certification reference from the owner
        T::DoctorCertificationOwner::disassociate(owner.get_owner_id(), &certification.id);
        // Decrement counts
        Self::sub_certifications_count();
        Self::sub_certification_count_by_owner(owner.get_owner_id());

        Ok(certification)
    }

    fn certification_by_id(
        certification_id: &Self::DoctorCertificationId,
    ) -> Option<Self::DoctorCertification> {
        match DoctorCertifications::<T>::get(certification_id) {
            None => None,
            Some(certification) => Some(certification),
        }
    }

    fn certification_count_by_owner(owner_id: &T::AccountId) -> u64 {
        Self::certification_count_by_owner(owner_id).unwrap_or(0)
    }
}

/// Pallet Methods
impl<T: Config> Pallet<T> {
    // DoctorCertifications Count Addition and Substraction Helpers
    // Add certifications count
    pub fn add_certifications_count() {
        let certifications_count = <DoctorCertificationsCount<T>>::get().unwrap_or(0);
        <DoctorCertificationsCount<T>>::put(certifications_count.wrapping_add(1));
    }
    // Add certifications count by owner
    pub fn add_certification_count_by_owner(owner_id: &T::AccountId) {
        let certifications_count =
            DoctorCertificationsCountByOwner::<T>::get(owner_id).unwrap_or(0);
        DoctorCertificationsCountByOwner::<T>::insert(
            owner_id,
            certifications_count.wrapping_add(1),
        )
    }

    // Subtract certifications count
    pub fn sub_certifications_count() {
        let certifications_count = <DoctorCertificationsCount<T>>::get().unwrap_or(1);
        DoctorCertificationsCount::<T>::put(certifications_count - 1);
    }
    // Subtract certifications count by owner
    pub fn sub_certification_count_by_owner(owner_id: &T::AccountId) {
        let certifications_count =
            DoctorCertificationsCountByOwner::<T>::get(owner_id).unwrap_or(1);
        DoctorCertificationsCountByOwner::<T>::insert(owner_id, certifications_count - 1);
    }
}

/// DoctorCertificationsProvider Trait Implementation
impl<T: Config> DoctorCertificationsProvider<T> for Pallet<T> {
    type Error = Error<T>;
    type DoctorCertification = DoctorCertificationOf<T>;

    fn certification_by_id(id: &T::Hash) -> Option<DoctorCertificationOf<T>> {
        <Self as DoctorCertificationInterface<T>>::certification_by_id(id)
    }

    fn delete_certification(
        owner_id: &T::AccountId,
        id: &T::Hash,
    ) -> Result<Self::DoctorCertification, Self::Error> {
        <Self as DoctorCertificationInterface<T>>::delete_certification(owner_id, id)
    }
}
