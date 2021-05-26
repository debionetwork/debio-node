#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use traits_electrical_medical_record::{
    ElectricalMedicalRecordsProvider,
    ElectricalMedicalRecordInfo as ElectricalMedicalRecordInfoT
};
use frame_support::codec::{Encode, Decode};
use frame_support::pallet_prelude::*;

pub mod interface;
pub use interface::ElectricalMedicalRecordInterface;
use sp_std::prelude::*;

/// ElectricalMedicalRecordInfo struct
/// Information that is mutable by user
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct ElectricalMedicalRecordInfo {
    description: Vec<u8>, // TODO: limit the length
    record_link: Vec<u8>
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct ElectricalMedicalRecord<AccountId> {
    pub owner_id: AccountId,
    pub info: ElectricalMedicalRecordInfo,
}
impl<AccountId> ElectricalMedicalRecord<AccountId> {
    pub fn new(owner_id: AccountId, info: ElectricalMedicalRecordInfo) -> Self {
        Self {
            owner_id,
            info
        }
    }

    pub fn get_owner_id(&self) -> &AccountId {
        &self.owner_id
    }
}

impl<T, AccountId> ElectricalMedicalRecordInfoT<T> for ElectricalMedicalRecord<AccountId>
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
    use crate::{ElectricalMedicalRecord, ElectricalMedicalRecordInfo};
    use crate::interface::ElectricalMedicalRecordInterface;

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
    pub type ElectricalMedicalRecordOf<T> = ElectricalMedicalRecord<AccountIdOf<T>>;
    pub type ElectricalMedicalRecordInfoOf = ElectricalMedicalRecordInfo;

    // ------- Storage -------------
    #[pallet::storage]
    #[pallet::getter(fn electrical_medical_record_by_owner_id)]
    pub type ElectricalMedicalRecordByOwner<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, ElectricalMedicalRecordOf<T>>;
    //                                _,  Hasher         ,  Key     ,  Value
    // -----------------------------


    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters, [ElectricalMedicalRecord, who]
        ElectricalMedicalRecordUploaded(ElectricalMedicalRecordOf<T>, AccountIdOf<T>),
        //// ElectricalMedicalRecord deleted
        /// parameters, [ElectricalMedicalRecord, who]
        ElectricalMedicalRecordRemoved(ElectricalMedicalRecordOf<T>, AccountIdOf<T>),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// User not allowed to create electrical_medical_record
        NotAllowedToCreate,
        /// User is not the owner of a electrical_medical_record
        NotElectricalMedicalRecordOwner,
        /// Ordering a electrical_medical_record that does not exist
        ElectricalMedicalRecordDoesNotExist,
    }
    
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn upload_electrical_medical_record(origin: OriginFor<T>, electrical_medical_record_info: ElectricalMedicalRecordInfoOf) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as ElectricalMedicalRecordInterface<T>>::upload_electrical_medical_record(&who, &electrical_medical_record_info) {
                Ok(electrical_medical_record) => {
                    Self::deposit_event(Event::ElectricalMedicalRecordUploaded(electrical_medical_record, who.clone()));
                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }

        #[pallet::weight(10_1000 + T::DbWeight::get().writes(1))]
        pub fn remove_electrical_medical_record(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            match <Self as ElectricalMedicalRecordInterface<T>>::remove_electrical_medical_record(&who) {
                Ok(electrical_medical_record) => {
                    Self::deposit_event(Event::ElectricalMedicalRecordRemoved(electrical_medical_record, who.clone()));
                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }
    }
}

/// ElectricalMedicalRecord Interface Implementation
impl<T: Config> ElectricalMedicalRecordInterface<T> for Pallet<T> {
    type Error = Error<T>;
    type ElectricalMedicalRecord = ElectricalMedicalRecordOf<T>;
    type ElectricalMedicalRecordInfo = ElectricalMedicalRecordInfoOf;

    /// Upload ElectricalMedicalRecord
    fn upload_electrical_medical_record(owner_id: &T::AccountId, electrical_medical_record_info: &Self::ElectricalMedicalRecordInfo) -> Result<Self::ElectricalMedicalRecord, Self::Error> { 
        let electrical_medical_record = ElectricalMedicalRecord::new(owner_id.clone(), electrical_medical_record_info.clone());
        // Store to ElectricalMedicalRecordById storage
        ElectricalMedicalRecordByOwner::<T>::insert(owner_id, &electrical_medical_record);
        
        Ok(electrical_medical_record) 
    }

    /// Remove ElectricalMedicalRecord
    fn remove_electrical_medical_record(owner_id: &T::AccountId) -> Result<Self::ElectricalMedicalRecord, Self::Error> {
        let electrical_medical_record = ElectricalMedicalRecordByOwner::<T>::get(owner_id);
        if electrical_medical_record == None {
            return Err(Error::<T>::ElectricalMedicalRecordDoesNotExist)?;
        }

        // Remove electrical_medical_record from storage
        let electrical_medical_record = ElectricalMedicalRecordByOwner::<T>::take(owner_id).unwrap();

        Ok(electrical_medical_record)
    }

    fn electrical_medical_record_by_owner_id(owner_id: &T::AccountId) -> Option<Self::ElectricalMedicalRecord> {
        match ElectricalMedicalRecordByOwner::<T>::get(owner_id) {
            None => None,
            Some(electrical_medical_record) => Some(electrical_medical_record)
        }
    }
}

/// ElectricalMedicalRecordsProvider Trait Implementation
impl<T: Config> ElectricalMedicalRecordsProvider<T> for Pallet<T> {
    type Error = Error<T>;
    type ElectricalMedicalRecord = ElectricalMedicalRecordOf<T>;

    fn electrical_medical_record_by_owner_id(owner_id: &T::AccountId) -> Option<ElectricalMedicalRecordOf<T>> {
        <Self as ElectricalMedicalRecordInterface<T>>::electrical_medical_record_by_owner_id(owner_id)
    }

    fn remove_electrical_medical_record(owner_id: &T::AccountId) -> Result<Self::ElectricalMedicalRecord, Self::Error> {
        <Self as ElectricalMedicalRecordInterface<T>>::remove_electrical_medical_record(owner_id)
    }
}

