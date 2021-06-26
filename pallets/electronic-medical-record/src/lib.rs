#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use traits_electronic_medical_record::{
    ElectronicMedicalRecordInfosProvider,
    ElectronicMedicalRecordInfoOwner,
    ElectronicMedicalRecordInfo as ElectronicMedicalRecordInfoT
};
use frame_support::codec::{Encode, Decode};
use frame_support::pallet_prelude::*;

pub mod interface;
pub use interface::ElectronicMedicalRecordInterface;
use sp_std::prelude::*;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct ElectronicMedicalRecord<AccountId, Hash> 
    where Hash: PartialEq + Eq 
{
    pub owner_id: AccountId,
    pub info: Vec<Hash>,
}

impl<AccountId, Hash> ElectronicMedicalRecord<AccountId, Hash> 
    where Hash: PartialEq + Eq 
{
        
    pub fn new (
        owner_id: AccountId
    ) -> Self {
        Self {
            owner_id,
            info: Vec::<Hash>::new(),
        }
    }

    pub fn get_owner_id(&self) -> &AccountId {
        &self.owner_id
    }

    pub fn add_info(&mut self, emr_info_id: Hash) -> () {
        &self.info.push(emr_info_id);
    }

    pub fn remove_info(&mut self, emr_info_id: Hash) -> () {
        if let Some(pos) = &self.info.iter().position(|x| *x == emr_info_id) {
            &self.info.remove(*pos);
        }
    }
}

/// ElectronicMedicalRecordInfo struct
/// Information that is mutable by user
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct ElectronicMedicalRecordInfo<AccountId, Hash, Moment>
    where Hash: PartialEq + Eq 
{
    pub id: Hash,
    pub owner_id: AccountId,
    pub title: Vec<u8>,
    pub description: Vec<u8>, // TODO: limit the length
    pub record_link: Vec<u8>,
    pub uploaded_at: Moment,
}

impl<AccountId, Hash, Moment> ElectronicMedicalRecordInfo<AccountId, Hash, Moment> 
    where Hash: PartialEq + Eq 
{
    pub fn new (
        id: Hash,
        owner_id: AccountId,
        title: Vec<u8>,
        description: Vec<u8>,
        record_link: Vec<u8>,
        uploaded_at: Moment
    ) -> Self {
        Self {
            id,
            owner_id,
            title,
            description,
            record_link,
            uploaded_at,
        }
    }

    pub fn get_id(&self) -> &Hash {
        &self.id
    }

    pub fn get_uploaded_at(&self) -> &Moment {
        &self.uploaded_at
    }

    pub fn get_owner_id(&self) -> &AccountId {
        &self.owner_id
    }
}

impl<T, AccountId, Hash, Moment> ElectronicMedicalRecordInfoT<T> for ElectronicMedicalRecordInfo<AccountId, Hash, Moment>
    where 
        Hash: PartialEq + Eq,
        T: frame_system::Config<AccountId = AccountId, Hash = Hash> + pallet_timestamp::Config<Moment = Moment>
{
    fn get_id(&self) -> &Hash {
        self.get_id()
    }

    fn get_uploaded_at(&self) -> &Moment {
        self.get_uploaded_at()
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
    use crate::{ElectronicMedicalRecord, ElectronicMedicalRecordInfoOwner, ElectronicMedicalRecordInfo};
    use crate::interface::ElectronicMedicalRecordInterface;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type ElectronicMedicalRecord: ElectronicMedicalRecordInfoOwner<Self>;
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
    pub type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;
    pub type ElectronicMedicalRecordOf<T> = ElectronicMedicalRecord<AccountIdOf<T>, HashOf<T>>;
    pub type ElectronicMedicalRecordInfoOf<T> = ElectronicMedicalRecordInfo<AccountIdOf<T>, HashOf<T>, MomentOf<T>>;
    pub type ElectronicMedicalRecordInfoIdOf<T> = HashOf<T>;

    // ------- Storage -------------
    #[pallet::storage]
    #[pallet::getter(fn electronic_medical_record_by_owner_id)]
    pub type ElectronicMedicalRecordByOwner<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, ElectronicMedicalRecordOf<T>>;
    
    #[pallet::storage]
    #[pallet::getter(fn electronic_medical_record_info_by_id)]
    pub type ElectronicMedicalRecordInfoById<T> = StorageMap<_, Blake2_128Concat, ElectronicMedicalRecordInfoIdOf<T>, ElectronicMedicalRecordInfoOf<T>>;

    #[pallet::storage]
    #[pallet::getter(fn electronic_medical_record_info_count)]
    pub type ElectronicMedicalRecordInfoCount<T> = StorageValue<_, u64>;

    #[pallet::storage]
    #[pallet::getter(fn electronic_medical_record_info_count_by_owner)]
    pub type ElectronicMedicalRecordInfoCountByOwner<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, u64>;
    //                                _,  Hasher         ,  Key     ,  Value
    // -----------------------------


    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters, [ElectronicMedicalRecord, who]
        ElectronicMedicalRecordAdded(ElectronicMedicalRecordOf<T>, AccountIdOf<T>),
        //// ElectronicMedicalRecord deleted
        /// parameters, [ElectronicMedicalRecord, who]
        ElectronicMedicalRecordRemoved(ElectronicMedicalRecordOf<T>, AccountIdOf<T>),
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters, [ElectronicMedicalRecordInfo, who]
        ElectronicMedicalRecordInfoAdded(ElectronicMedicalRecordInfoOf<T>, AccountIdOf<T>),
        //// ElectronicMedicalRecordInfo deleted
        /// parameters, [ElectronicMedicalRecordInfo, who]
        ElectronicMedicalRecordInfoRemoved(ElectronicMedicalRecordInfoOf<T>, AccountIdOf<T>),
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
        pub fn add_electronic_medical_record(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as ElectronicMedicalRecordInterface<T>>::add_electronic_medical_record(&who) {
                Ok(electronic_medical_record) => {
                    Self::deposit_event(Event::ElectronicMedicalRecordAdded(electronic_medical_record, who.clone()));
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
        
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn add_electronic_medical_record_info(origin: OriginFor<T>, mut title: Vec<u8>, mut description: Vec<u8>, mut record_link: Vec<u8>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as ElectronicMedicalRecordInterface<T>>::add_electronic_medical_record_info(&who, &mut title, &mut description, &mut record_link) {
                Ok(electronic_medical_record_info) => {
                    Self::deposit_event(Event::ElectronicMedicalRecordInfoAdded(electronic_medical_record_info, who.clone()));
                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }

        #[pallet::weight(10_1000 + T::DbWeight::get().writes(1))]
        pub fn remove_electronic_medical_record_info(origin: OriginFor<T>, electronic_medical_record_info_id: HashOf<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            match <Self as ElectronicMedicalRecordInterface<T>>::remove_electronic_medical_record_info(&who, &electronic_medical_record_info_id) {
                Ok(electronic_medical_record_info) => {
                    Self::deposit_event(Event::ElectronicMedicalRecordInfoRemoved(electronic_medical_record_info, who.clone()));
                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }
    }
}

use frame_support::sp_runtime::traits::Hash;
use traits_electronic_medical_record::{ElectronicMedicalRecordInfoOwnerInfo};

/// ElectronicMedicalRecord Interface Implementation
impl<T: Config> ElectronicMedicalRecordInterface<T> for Pallet<T> {
    type Error = Error<T>;
    type ElectronicMedicalRecordInfoId = T::Hash;
    type ElectronicMedicalRecord = ElectronicMedicalRecordOf<T>;
    type ElectronicMedicalRecordInfo = ElectronicMedicalRecordInfoOf<T>;

    fn generate_electronic_medical_record_info_id(owner_id: &T::AccountId, electronic_medical_record_info_count: u64) -> Self::ElectronicMedicalRecordInfoId {
        let mut account_id_bytes = owner_id.encode();
        let mut electronic_medical_record_info_count_bytes = electronic_medical_record_info_count.encode();
        account_id_bytes.append(&mut electronic_medical_record_info_count_bytes);

        let seed = &account_id_bytes;
        return T::Hashing::hash(seed);
    }

    fn add_electronic_medical_record(owner_id: &T::AccountId) -> Result<Self::ElectronicMedicalRecord, Self::Error> { 
        let electronic_medical_record = ElectronicMedicalRecord::new(owner_id.clone());
        // Store to ElectronicMedicalRecordById storage
        ElectronicMedicalRecordByOwner::<T>::insert(owner_id, &electronic_medical_record);
        
        Ok(electronic_medical_record) 
    }

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

    fn add_electronic_medical_record_info(owner_id: &T::AccountId, title: &mut Vec<u8>, description: &mut Vec<u8>, record_link: &mut Vec<u8>) -> Result<Self::ElectronicMedicalRecordInfo, Self::Error> { 
        let owner_electronic_medical_record_info_count = <Self as ElectronicMedicalRecordInterface<T>>::electronic_medical_record_info_count_by_owner(owner_id);
        let electronic_medical_record_info_id = Self::generate_electronic_medical_record_info_id(owner_id, owner_electronic_medical_record_info_count);
        let now = pallet_timestamp::Pallet::<T>::get();
        
        let electronic_medical_record_info = ElectronicMedicalRecordInfo::new(
            electronic_medical_record_info_id.clone(),
            owner_id.clone(),
            title.clone(),
            description.clone(),
            record_link.clone(),
            now
        );
        // Store to ElectronicMedicalRecordInfos storage
        ElectronicMedicalRecordInfoById::<T>::insert(&electronic_medical_record_info_id, &electronic_medical_record_info);

        // Increment ElectronicMedicalRecordInfos Count
        Self::add_electronic_medical_record_info_count();
        // Increment ElectronicMedicalRecordInfoCountByOwner
        Self::add_electronic_medical_record_info_count_by_owner(&electronic_medical_record_info.owner_id);
        
        // Associate created electronic_medical_record_info to the owner
        T::ElectronicMedicalRecord::associate(owner_id, &electronic_medical_record_info_id);

        Ok(electronic_medical_record_info) 
    }

    fn remove_electronic_medical_record_info(owner_id: &T::AccountId, electronic_medical_record_info_id: &Self::ElectronicMedicalRecordInfoId) -> Result<Self::ElectronicMedicalRecordInfo, Self::Error> {
        let electronic_medical_record_info = ElectronicMedicalRecordInfoById::<T>::get(electronic_medical_record_info_id);
        if electronic_medical_record_info == None {
            return Err(Error::<T>::ElectronicMedicalRecordDoesNotExist)?;
        }
        let electronic_medical_record_info = electronic_medical_record_info.unwrap();

        if electronic_medical_record_info.owner_id != owner_id.clone() {
            return Err(Error::<T>::NotElectronicMedicalRecordOwner)?;
        }
        // Remove electronic_medical_record_info from storage
        let electronic_medical_record_info = ElectronicMedicalRecordInfoById::<T>::take(electronic_medical_record_info_id).unwrap();

        let owner = T::ElectronicMedicalRecord::get_owner(owner_id).unwrap();
        // disassociate electronic_medical_record_info reference from the owner
        T::ElectronicMedicalRecord::disassociate(owner.get_owner_id(), &electronic_medical_record_info.id);
        // Decrement counts
        Self::sub_electronic_medical_record_info_count();
        Self::sub_electronic_medical_record_info_count_by_owner(owner.get_owner_id());

        Ok(electronic_medical_record_info)
    }

    fn electronic_medical_record_info_count_by_owner(owner_id: &T::AccountId) -> u64 { 
        let electronic_medical_record_info_count = ElectronicMedicalRecordInfoCountByOwner::<T>::get(owner_id).unwrap_or(1);
        return electronic_medical_record_info_count;
    }

    fn electronic_medical_record_info_by_id(electronic_medical_record_info_id: &Self::ElectronicMedicalRecordInfoId) -> Option<Self::ElectronicMedicalRecordInfo> {
        match ElectronicMedicalRecordInfoById::<T>::get(electronic_medical_record_info_id) {
            None => None,
            Some(electronic_medical_record_info) => Some(electronic_medical_record_info)
        }
    }
}

/// Pallet Methods
impl<T: Config> Pallet<T> {
    // ElectronicMedicalRecordInfo Count Addition and Substraction Helpers
    // Add electronic_medical_record_info count
    pub fn add_electronic_medical_record_info_count() {
        let electronic_medical_record_info_count = <ElectronicMedicalRecordInfoCount<T>>::get().unwrap_or(0);
        <ElectronicMedicalRecordInfoCount<T>>::put(electronic_medical_record_info_count.wrapping_add(1));
    }
    // Add electronic_medical_record_info count by owner
    pub fn add_electronic_medical_record_info_count_by_owner(owner_id: &T::AccountId) {
        let electronic_medical_record_info_count = ElectronicMedicalRecordInfoCountByOwner::<T>::get(owner_id).unwrap_or(0);
        ElectronicMedicalRecordInfoCountByOwner::<T>::insert(owner_id, electronic_medical_record_info_count.wrapping_add(1))
    }

    // Subtract electronic_medical_record_info count
    pub fn sub_electronic_medical_record_info_count() {
        let electronic_medical_record_info_count = <ElectronicMedicalRecordInfoCount<T>>::get().unwrap_or(1);
        ElectronicMedicalRecordInfoCount::<T>::put(electronic_medical_record_info_count - 1);
    }
    // Subtract electronic_medical_record_info count by owner
    pub fn sub_electronic_medical_record_info_count_by_owner(owner_id: &T::AccountId) {
        let electronic_medical_record_info_count = ElectronicMedicalRecordInfoCountByOwner::<T>::get(owner_id).unwrap_or(1);
        ElectronicMedicalRecordInfoCountByOwner::<T>::insert(owner_id, electronic_medical_record_info_count - 1);
    }
}

/// ElectronicMedicalRecordInfosProvider Trait Implementation
impl<T: Config> ElectronicMedicalRecordInfosProvider<T> for Pallet<T> {
    type Error = Error<T>;
    type Moment = MomentOf<T>;
    type ElectronicMedicalRecordInfo = ElectronicMedicalRecordInfoOf<T>;

    fn electronic_medical_record_info_by_id(electronic_medical_record_info_id: &T::Hash) -> Option<ElectronicMedicalRecordInfoOf<T>> {
        <Self as ElectronicMedicalRecordInterface<T>>::electronic_medical_record_info_by_id(electronic_medical_record_info_id)
    }

    fn remove_electronic_medical_record_info(owner_id: &T::AccountId, electronic_medical_record_info_id: &T::Hash) -> Result<Self::ElectronicMedicalRecordInfo, Self::Error> {
        <Self as ElectronicMedicalRecordInterface<T>>::remove_electronic_medical_record_info(owner_id, electronic_medical_record_info_id)
    }
}

impl<T, AccountId, Hash> ElectronicMedicalRecordInfoOwnerInfo<T> for ElectronicMedicalRecord<AccountId, Hash>
    where
        Hash: PartialEq + Eq,
        T: frame_system::Config<AccountId = AccountId>
{
    fn get_owner_id(&self) -> &AccountId {
        &self.get_owner_id()
    }
}

impl<T: Config> ElectronicMedicalRecordInfoOwner<T> for Pallet<T> {
    type Owner = ElectronicMedicalRecord<T::AccountId, T::Hash>;

    fn get_owner(id: &T::AccountId) -> Option<Self::Owner> {
        let electronic_medical_record = ElectronicMedicalRecordByOwner::<T>::get(id);
        electronic_medical_record
    }

    fn associate(owner_id: &T::AccountId, electronic_medical_record_info_id: &T::Hash) -> () {
        <ElectronicMedicalRecordByOwner<T>>::mutate(owner_id, | electronic_medical_record | {
            match electronic_medical_record {
                None => (), // If electronic_medical_record does not exist, do nothing
                Some(electronic_medical_record) => {
                    electronic_medical_record.add_info(*electronic_medical_record_info_id);
                }
            }
        });
    }

    fn disassociate(owner_id: &T::AccountId, electronic_medical_record_info_id: &T::Hash) -> () {
        ElectronicMedicalRecordByOwner::<T>::mutate(owner_id, | electronic_medical_record | {
            match electronic_medical_record {
                None => (),
                Some(electronic_medical_record) => {
                    electronic_medical_record.remove_info(*electronic_medical_record_info_id);
                }
            }
        });
    }
}

