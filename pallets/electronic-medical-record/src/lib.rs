#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use traits_electronic_medical_record::{
    ElectronicMedicalRecordsProvider,
    ElectronicMedicalRecordInfo as ElectronicMedicalRecordInfoT
};
use frame_support::codec::{Encode, Decode};
use frame_support::pallet_prelude::*;

pub mod interface;
pub use interface::ElectronicMedicalRecordInterface;
use sp_std::prelude::*;

/// ElectronicMedicalRecordInfo struct
/// Information that is mutable by user
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct ElectronicMedicalRecordInfo {
    title: Vec<u8>,
    description: Vec<u8>, // TODO: limit the length
    record_link: Vec<u8>
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct ElectronicMedicalRecord<AccountId> {
    pub owner_id: AccountId,
    pub info: ElectronicMedicalRecordInfo,
}
impl<AccountId> ElectronicMedicalRecord<AccountId> {
    pub fn new(owner_id: AccountId, info: ElectronicMedicalRecordInfo) -> Self {
        Self {
            owner_id,
            info
        }
    }

    pub fn get_owner_id(&self) -> &AccountId {
        &self.owner_id
    }
}

impl<T, AccountId> ElectronicMedicalRecordInfoT<T> for ElectronicMedicalRecord<AccountId>
    where T: frame_system::Config<AccountId = AccountId>
{
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
    use crate::{ElectronicMedicalRecord, ElectronicMedicalRecordInfo};
    use crate::interface::ElectronicMedicalRecordInterface;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
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
    pub type ElectronicMedicalRecordOf<T> = ElectronicMedicalRecord<AccountIdOf<T>>;
    pub type ElectronicMedicalRecordInfoOf = ElectronicMedicalRecordInfo;

    // ------- Storage -------------
    #[pallet::storage]
    #[pallet::getter(fn electronic_medical_record_by_owner_id)]
    pub type ElectronicMedicalRecordByOwner<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, ElectronicMedicalRecordOf<T>>;
    //                                _,  Hasher         ,  Key     ,  Value
    // -----------------------------


    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters, [ElectronicMedicalRecord, who]
        ElectronicMedicalRecordUploaded(ElectronicMedicalRecordOf<T>, AccountIdOf<T>),
        //// ElectronicMedicalRecord deleted
        /// parameters, [ElectronicMedicalRecord, who]
        ElectronicMedicalRecordRemoved(ElectronicMedicalRecordOf<T>, AccountIdOf<T>),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// User not allowed to create electronic_medical_record
        NotAllowedToCreate,
        /// User is not the owner of a electronic_medical_record
        NotElectronicMedicalRecordOwner,
        /// Ordering a electronic_medical_record that does not exist
        ElectronicMedicalRecordDoesNotExist,
    }
    
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn upload_electronic_medical_record(origin: OriginFor<T>, electronic_medical_record_info: ElectronicMedicalRecordInfoOf) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as ElectronicMedicalRecordInterface<T>>::upload_electronic_medical_record(&who, &electronic_medical_record_info) {
                Ok(electronic_medical_record) => {
                    Self::deposit_event(Event::ElectronicMedicalRecordUploaded(electronic_medical_record, who.clone()));
                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }

        #[pallet::weight(10_1000 + T::DbWeight::get().writes(1))]
        pub fn remove_electronic_medical_record(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            match <Self as ElectronicMedicalRecordInterface<T>>::remove_electronic_medical_record(&who) {
                Ok(electronic_medical_record) => {
                    Self::deposit_event(Event::ElectronicMedicalRecordRemoved(electronic_medical_record, who.clone()));
                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }
    }
}

/// ElectronicMedicalRecord Interface Implementation
impl<T: Config> ElectronicMedicalRecordInterface<T> for Pallet<T> {
    type Error = Error<T>;
    type ElectronicMedicalRecord = ElectronicMedicalRecordOf<T>;
    type ElectronicMedicalRecordInfo = ElectronicMedicalRecordInfoOf;

    /// Upload ElectronicMedicalRecord
    fn upload_electronic_medical_record(owner_id: &T::AccountId, electronic_medical_record_info: &Self::ElectronicMedicalRecordInfo) -> Result<Self::ElectronicMedicalRecord, Self::Error> { 
        let electronic_medical_record = ElectronicMedicalRecord::new(owner_id.clone(), electronic_medical_record_info.clone());
        // Store to ElectronicMedicalRecordById storage
        ElectronicMedicalRecordByOwner::<T>::insert(owner_id, &electronic_medical_record);
        
        Ok(electronic_medical_record) 
    }

    /// Remove ElectronicMedicalRecord
    fn remove_electronic_medical_record(owner_id: &T::AccountId) -> Result<Self::ElectronicMedicalRecord, Self::Error> {
        let electronic_medical_record = ElectronicMedicalRecordByOwner::<T>::get(owner_id);
        if electronic_medical_record == None {
            return Err(Error::<T>::ElectronicMedicalRecordDoesNotExist)?;
        }

        // Remove electronic_medical_record from storage
        let electronic_medical_record = ElectronicMedicalRecordByOwner::<T>::take(owner_id).unwrap();

        Ok(electronic_medical_record)
    }

    fn electronic_medical_record_by_owner_id(owner_id: &T::AccountId) -> Option<Self::ElectronicMedicalRecord> {
        match ElectronicMedicalRecordByOwner::<T>::get(owner_id) {
            None => None,
            Some(electronic_medical_record) => Some(electronic_medical_record)
        }
    }
}

/// ElectronicMedicalRecordsProvider Trait Implementation
impl<T: Config> ElectronicMedicalRecordsProvider<T> for Pallet<T> {
    type Error = Error<T>;
    type ElectronicMedicalRecord = ElectronicMedicalRecordOf<T>;

    fn electronic_medical_record_by_owner_id(owner_id: &T::AccountId) -> Option<ElectronicMedicalRecordOf<T>> {
        <Self as ElectronicMedicalRecordInterface<T>>::electronic_medical_record_by_owner_id(owner_id)
    }

    fn remove_electronic_medical_record(owner_id: &T::AccountId) -> Result<Self::ElectronicMedicalRecord, Self::Error> {
        <Self as ElectronicMedicalRecordInterface<T>>::remove_electronic_medical_record(owner_id)
    }
}

