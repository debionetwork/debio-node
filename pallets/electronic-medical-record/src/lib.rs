#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
pub use pallet::*;
pub use scale_info::TypeInfo;

#[cfg(test)]
mod mock;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use traits_electronic_medical_record::{
    ElectronicMedicalRecordFile as ElectronicMedicalRecordFileT,
    ElectronicMedicalRecordFileByElectronicMedicalRecord,
    ElectronicMedicalRecordFilesProvider,
};

pub mod weights;
pub mod interface;
pub use interface::ElectronicMedicalRecordInterface;
use sp_std::prelude::*;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct ElectronicMedicalRecord<AccountId, Hash>
where
    Hash: PartialEq + Eq,
{
    pub id: Hash,
    pub owner_id: AccountId,
    pub title: Vec<u8>,
    pub category: Vec<u8>,
    pub files: Vec<Hash>,
}

impl<AccountId, Hash> ElectronicMedicalRecord<AccountId, Hash>
where
    Hash: PartialEq + Eq,
{
    pub fn new(
        id: Hash,
        owner_id: AccountId,
        title: Vec<u8>,
        category: Vec<u8>,
    ) -> Self {
        Self {
            id,
            owner_id,
            title,
            category,
            files: Vec::<Hash>::new(),
        }
    }

    pub fn get_id(&self) -> &Hash {
        &self.id
    }

    pub fn get_owner_id(&self) -> &AccountId {
        &self.owner_id
    }

    pub fn add_file(&mut self, emr_files_id: Hash) -> () {
        self.files.push(emr_files_id);
    }

    pub fn remove_file(&mut self, emr_files_id: Hash) -> () {
        if let Some(pos) = &self.files.iter().position(|x| *x == emr_files_id) {
            self.files.remove(*pos);
        }
    }
}

/// ElectronicMedicalRecordFile struct
/// Information that is mutable by user
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct ElectronicMedicalRecordFile<Hash, Moment>
where
    Hash: PartialEq + Eq,
{
    pub id: Hash,
    pub electronic_medical_record_id: Hash,
    pub title: Vec<u8>,
    pub description: Vec<u8>, // TODO: limit the length
    pub record_link: Vec<u8>,
    pub uploaded_at: Moment,
}

impl<Hash, Moment> ElectronicMedicalRecordFile<Hash, Moment>
where
    Hash: PartialEq + Eq,
{
    pub fn new(
        id: Hash,
        electronic_medical_record_id: Hash,
        title: Vec<u8>,
        description: Vec<u8>,
        record_link: Vec<u8>,
        uploaded_at: Moment,
    ) -> Self {
        Self {
            id,
            electronic_medical_record_id,
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

    pub fn get_electronic_medical_record_id(&self) -> &Hash {
        &self.electronic_medical_record_id
    }
}

impl<T, Hash, Moment> ElectronicMedicalRecordFileT<T>
    for ElectronicMedicalRecordFile<Hash, Moment>
where
    Hash: PartialEq + Eq,
    T: frame_system::Config<Hash = Hash>
        + pallet_timestamp::Config<Moment = Moment>,
{
    fn get_id(&self) -> &T::Hash {
        self.get_id()
    }

    fn get_electronic_medical_record_id(&self) -> &T::Hash {
        self.get_electronic_medical_record_id()
    }

    fn get_uploaded_at(&self) -> &Moment {
        self.get_uploaded_at()
    }
}

#[frame_support::pallet]
pub mod pallet {
    use crate::weights::WeightInfo;
    use crate::interface::ElectronicMedicalRecordInterface;
    use crate::{
        ElectronicMedicalRecord, ElectronicMedicalRecordFile, ElectronicMedicalRecordFileByElectronicMedicalRecord
    };
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    pub use sp_std::prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type ElectronicMedicalRecord: ElectronicMedicalRecordFileByElectronicMedicalRecord<Self>;
        type ElectronicMedicalRecordWeightInfo: WeightInfo;
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
    pub type ElectronicMedicalRecordFileOf<T> =
        ElectronicMedicalRecordFile<HashOf<T>, MomentOf<T>>;
    pub type ElectronicMedicalRecordIdOf<T> = HashOf<T>;
    pub type ElectronicMedicalRecordFileIdOf<T> = HashOf<T>;

    // ------- Storage -------------
    #[pallet::storage]
    #[pallet::getter(fn electronic_medical_record_by_owner_id)]
    pub type ElectronicMedicalRecordByOwner<T> =
        StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Vec<ElectronicMedicalRecordIdOf<T>>>;

    #[pallet::storage]
    #[pallet::getter(fn electronic_medical_record_by_id)]
    pub type ElectronicMedicalRecordById<T> =
        StorageMap<_, Blake2_128Concat, ElectronicMedicalRecordIdOf<T>, ElectronicMedicalRecordOf<T>>;

    #[pallet::storage]
    #[pallet::getter(fn electronic_medical_record_file_by_id)]
    pub type ElectronicMedicalRecordFileById<T> = StorageMap<
        _,
        Blake2_128Concat,
        ElectronicMedicalRecordFileIdOf<T>,
        ElectronicMedicalRecordFileOf<T>,
    >;

    #[pallet::storage]
    #[pallet::getter(fn electronic_medical_record_count_by_owner)]
    pub type ElectronicMedicalRecordCountByOwner<T> =
        StorageMap<_, Blake2_128Concat, AccountIdOf<T>, u64>;

    #[pallet::storage]
    #[pallet::getter(fn electronic_medical_record_file_count_by_electronic_medical_record)]
    pub type ElectronicMedicalRecordFileCountByElectronicMedicalRecordId<T> =
        StorageMap<_, Blake2_128Concat, ElectronicMedicalRecordIdOf<T>, u64>;

    #[pallet::storage]
    #[pallet::getter(fn electronic_medical_record_count)]
    pub type ElectronicMedicalRecordCount<T> = StorageValue<_, u64>;

    #[pallet::storage]
    #[pallet::getter(fn electronic_medical_record_file_count)]
    pub type ElectronicMedicalRecordFileCount<T> = StorageValue<_, u64>;
    //                                _,  Hasher         ,  Key     ,  Value
    // -----------------------------

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters, [ElectronicMedicalRecord, who]
        ElectronicMedicalRecordAdded(ElectronicMedicalRecordOf<T>, AccountIdOf<T>),
        //// ElectronicMedicalRecord deleted
        /// parameters, [ElectronicMedicalRecord, who]
        ElectronicMedicalRecordRemoved(ElectronicMedicalRecordOf<T>, AccountIdOf<T>),
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters, [ElectronicMedicalRecordFile, who]
        ElectronicMedicalRecordFileAdded(ElectronicMedicalRecordFileOf<T>, AccountIdOf<T>),
        //// ElectronicMedicalRecordFile deleted
        /// parameters, [ElectronicMedicalRecordFile, who]
        ElectronicMedicalRecordFileRemoved(ElectronicMedicalRecordFileOf<T>, AccountIdOf<T>),
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
        #[pallet::weight(T::ElectronicMedicalRecordWeightInfo::add_electronic_medical_record())]
        pub fn add_electronic_medical_record(
            origin: OriginFor<T>,
            title: Vec<u8>,
            category: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as ElectronicMedicalRecordInterface<T>>::add_electronic_medical_record(&who, &title, &category)
            {
                Ok(electronic_medical_record) => {
                    Self::deposit_event(Event::ElectronicMedicalRecordAdded(
                        electronic_medical_record,
                        who.clone(),
                    ));
                    Ok(().into())
                }
                Err(error) => Err(error)?,
            }
        }

        #[pallet::weight(T::ElectronicMedicalRecordWeightInfo::remove_electronic_medical_record())]
        pub fn remove_electronic_medical_record(
            origin: OriginFor<T>,
            electronic_medical_record_id: HashOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            
            match <Self as ElectronicMedicalRecordInterface<T>>::remove_electronic_medical_record(
                &who,
                &electronic_medical_record_id,
            ) {
                Ok(electronic_medical_record) => {
                    Self::deposit_event(Event::ElectronicMedicalRecordRemoved(
                        electronic_medical_record,
                        who.clone(),
                    ));
                    Ok(().into())
                }
                Err(error) => Err(error)?,
            }
        }

        #[pallet::weight(T::ElectronicMedicalRecordWeightInfo::add_electronic_medical_record_file())]
        pub fn add_electronic_medical_record_file(
            origin: OriginFor<T>,
            electronic_medical_record_id: HashOf<T>,
            mut title: Vec<u8>,
            mut description: Vec<u8>,
            mut record_link: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as ElectronicMedicalRecordInterface<T>>::add_electronic_medical_record_file(
                &who,
                &electronic_medical_record_id,
                &mut title,
                &mut description,
                &mut record_link,
            ) {
                Ok(electronic_medical_record_file) => {
                    Self::deposit_event(Event::ElectronicMedicalRecordFileAdded(
                        electronic_medical_record_file,
                        who.clone(),
                    ));
                    Ok(().into())
                }
                Err(error) => Err(error)?,
            }
        }

        #[pallet::weight(T::ElectronicMedicalRecordWeightInfo::remove_electronic_medical_record_file())]
        pub fn remove_electronic_medical_record_file(
            origin: OriginFor<T>,
            electronic_medical_record_file_id: HashOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as ElectronicMedicalRecordInterface<T>>::remove_electronic_medical_record_file(&who, &electronic_medical_record_file_id) {
                Ok(electronic_medical_record_file) => {
                    Self::deposit_event(Event::ElectronicMedicalRecordFileRemoved(electronic_medical_record_file, who.clone()));
                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }
    }
}

use frame_support::sp_runtime::traits::Hash;
use traits_electronic_medical_record::ElectronicMedicalRecordFileOwnerInfo;

/// ElectronicMedicalRecord Interface Implementation
impl<T: Config> ElectronicMedicalRecordInterface<T> for Pallet<T> {
    type Error = Error<T>;
    type ElectronicMedicalRecordId = T::Hash;
    type ElectronicMedicalRecord = ElectronicMedicalRecordOf<T>;

    type ElectronicMedicalRecordFileId = T::Hash;
    type ElectronicMedicalRecordFile = ElectronicMedicalRecordFileOf<T>;

    fn generate_electronic_medical_record_id(
        owner_id: &T::AccountId,
        electronic_medical_record_count: u64,
    ) -> Self::ElectronicMedicalRecordId {
        let mut account_id_bytes = owner_id.encode();
        let mut electronic_medical_record_count_bytes =
        electronic_medical_record_count.encode();
        account_id_bytes.append(&mut electronic_medical_record_count_bytes);

        let seed = &account_id_bytes;
        return T::Hashing::hash(seed);
    }

    fn generate_electronic_medical_record_file_id(
        electronic_medical_record_id: &T::Hash,
        electronic_medical_record_file_count: u64,
    ) -> Self::ElectronicMedicalRecordFileId {
        let mut electronic_medical_record_id_bytes = electronic_medical_record_id.encode();
        let mut electronic_medical_record_file_count_bytes =
            electronic_medical_record_file_count.encode();
        electronic_medical_record_id_bytes.append(&mut electronic_medical_record_file_count_bytes);

        let seed = &electronic_medical_record_id_bytes;
        return T::Hashing::hash(seed);
    }

    fn add_electronic_medical_record(
        owner_id: &T::AccountId,
        title: &Vec<u8>,
        category: &Vec<u8>,
    ) -> Result<Self::ElectronicMedicalRecord, Self::Error> {
        let owner_electronic_medical_record_count = <Self as ElectronicMedicalRecordInterface<T>>::electronic_medical_record_count_by_owner(owner_id);
        let electronic_medical_record_id = Self::generate_electronic_medical_record_id(
            owner_id,
            owner_electronic_medical_record_count,
        );

        let electronic_medical_record = ElectronicMedicalRecord::new(
            electronic_medical_record_id.clone(),
            owner_id.clone(),
            title.clone(),
            category.clone()
        );

        // Store to ElectronicMedicalRecordById storage
        ElectronicMedicalRecordById::<T>::insert(electronic_medical_record_id, &electronic_medical_record);

        Self::add_electronic_medical_record_by_owner(
            &owner_id,
            &electronic_medical_record_id
        );
        Self::add_electronic_medical_record_count();
        Self::add_electronic_medical_record_count_by_owner(&owner_id);

        Ok(electronic_medical_record)
    }

    fn remove_electronic_medical_record(
        owner_id: &T::AccountId,
        electronic_medical_record_id: &T::Hash,
    ) -> Result<Self::ElectronicMedicalRecord, Self::Error> {
        let electronic_medical_record = ElectronicMedicalRecordById::<T>::get(electronic_medical_record_id);
        if electronic_medical_record == None {
            return Err(Error::<T>::ElectronicMedicalRecordDoesNotExist);
        }

        if electronic_medical_record.unwrap().owner_id != owner_id.clone() {
            return Err(Error::<T>::NotElectronicMedicalRecordOwner);
        }

        // Remove electronic_medical_record from storage
        let electronic_medical_record =
            ElectronicMedicalRecordById::<T>::take(electronic_medical_record_id).unwrap();

        Self::sub_electronic_medical_record_by_owner(
            electronic_medical_record.get_owner_id(),
            &electronic_medical_record_id
        );
        Self::sub_electronic_medical_record_count();
        Self::sub_electronic_medical_record_count_by_owner(electronic_medical_record.get_owner_id());

        Ok(electronic_medical_record)
    }

    fn electronic_medical_record_by_owner_id(
        owner_id: &T::AccountId,
    ) -> Option<Vec<T::Hash>> {
        match ElectronicMedicalRecordByOwner::<T>::get(owner_id) {
            None => None,
            Some(electronic_medical_record_vec) => Some(electronic_medical_record_vec),
        }
    }

    fn add_electronic_medical_record_file(
        owner_id: &T::AccountId,
        electronic_medical_record_id: &T::Hash,
        title: &mut Vec<u8>,
        description: &mut Vec<u8>,
        record_link: &mut Vec<u8>,
    ) -> Result<Self::ElectronicMedicalRecordFile, Self::Error> {
        let electronic_medical_record =
            ElectronicMedicalRecordById::<T>::get(electronic_medical_record_id).unwrap();

        if electronic_medical_record.owner_id != owner_id.clone() {
            return Err(Error::<T>::NotElectronicMedicalRecordOwner);
        }

        let owner_electronic_medical_record_file_count = <Self as ElectronicMedicalRecordInterface<T>>::electronic_medical_record_file_count_by_electronic_medical_record_id(electronic_medical_record_id);
        let electronic_medical_record_file_id = Self::generate_electronic_medical_record_file_id(
            electronic_medical_record_id,
            owner_electronic_medical_record_file_count,
        );
        let now = pallet_timestamp::Pallet::<T>::get();

        let electronic_medical_record_file = ElectronicMedicalRecordFile::new(
            electronic_medical_record_file_id.clone(),
            electronic_medical_record_id.clone(),
            title.clone(),
            description.clone(),
            record_link.clone(),
            now,
        );

        // Store to ElectronicMedicalRecordFiles storage
        ElectronicMedicalRecordFileById::<T>::insert(
            &electronic_medical_record_file_id,
            &electronic_medical_record_file,
        );

        // Increment ElectronicMedicalRecordFiles Count
        Self::add_electronic_medical_record_file_count();
        // Increment ElectronicMedicalRecordFileCountByElectronicMedicalRecord
        Self::add_electronic_medical_record_file_count_by_electronic_medical_record(
            &electronic_medical_record_id,
        );

        // Associate created electronic_medical_record_file to the electronic_medical_record
        T::ElectronicMedicalRecord::associate(electronic_medical_record_id, &electronic_medical_record_file_id);

        Ok(electronic_medical_record_file)
    }

    fn remove_electronic_medical_record_file(
        owner_id: &T::AccountId,
        electronic_medical_record_file_id: &Self::ElectronicMedicalRecordFileId,
    ) -> Result<Self::ElectronicMedicalRecordFile, Self::Error> {
        let _emr_file =
            ElectronicMedicalRecordFileById::<T>::get(electronic_medical_record_file_id);
        if _emr_file == None {
            return Err(Error::<T>::ElectronicMedicalRecordDoesNotExist)?;
        }

        let electronic_medical_record =
            ElectronicMedicalRecordById::<T>::get(_emr_file.unwrap().electronic_medical_record_id).unwrap();

        if electronic_medical_record.owner_id != owner_id.clone() {
            return Err(Error::<T>::NotElectronicMedicalRecordOwner)?;
        }

        // Remove electronic_medical_record_file from storage
        let electronic_medical_record_file =
            ElectronicMedicalRecordFileById::<T>::take(electronic_medical_record_file_id).unwrap();

        // disassociate electronic_medical_record_file reference from the electronic_medical_record
        T::ElectronicMedicalRecord::disassociate(
            &electronic_medical_record_file.electronic_medical_record_id,
            &electronic_medical_record_file.id,
        );
            
        // Decrement counts
        Self::sub_electronic_medical_record_file_count();
        Self::sub_electronic_medical_record_file_count_by_electronic_medical_record(&electronic_medical_record_file.id);

        Ok(electronic_medical_record_file)
    }

    fn electronic_medical_record_count_by_owner(owner_id: &T::AccountId) -> u64 {
        let electronic_medical_record_count =
            ElectronicMedicalRecordCountByOwner::<T>::get(owner_id).unwrap_or(0);
        return electronic_medical_record_count;
    }

    fn electronic_medical_record_file_count_by_electronic_medical_record_id(electronic_medical_record_id: &T::Hash) -> u64 {
        let electronic_medical_record_file_count =
        ElectronicMedicalRecordFileCountByElectronicMedicalRecordId::<T>::get(electronic_medical_record_id).unwrap_or(0);
        return electronic_medical_record_file_count;
    }

    fn electronic_medical_record_by_id(
        electronic_medical_record_id: &Self::ElectronicMedicalRecordId,
    ) -> Option<Self::ElectronicMedicalRecord> {
        match ElectronicMedicalRecordById::<T>::get(electronic_medical_record_id) {
            None => None,
            Some(electronic_medical_record) => Some(electronic_medical_record),
        }
    }

    fn electronic_medical_record_file_by_id(
        electronic_medical_record_file_id: &Self::ElectronicMedicalRecordFileId,
    ) -> Option<Self::ElectronicMedicalRecordFile> {
        match ElectronicMedicalRecordFileById::<T>::get(electronic_medical_record_file_id) {
            None => None,
            Some(electronic_medical_record_file) => Some(electronic_medical_record_file),
        }
    }
}

/// Pallet Methods
impl<T: Config> Pallet<T> {
    // Add electronic_medical_record by owner
    pub fn add_electronic_medical_record_by_owner(owner_id: &T::AccountId, electronic_medical_record_id: &T::Hash) {
        let mut electronic_medical_record =
            ElectronicMedicalRecordByOwner::<T>::get(owner_id).unwrap_or(vec![]);

        electronic_medical_record.push(*electronic_medical_record_id);
        ElectronicMedicalRecordByOwner::<T>::insert(
            owner_id,
            &electronic_medical_record,
        )
    }

    // Subtract electronic_medical_record by owner
    pub fn sub_electronic_medical_record_by_owner(owner_id: &T::AccountId, electronic_medical_record_id: &T::Hash) {
        let mut electronic_medical_record =
            ElectronicMedicalRecordByOwner::<T>::get(owner_id).unwrap_or(vec![]);
        electronic_medical_record.retain(|&x| x != *electronic_medical_record_id);
        ElectronicMedicalRecordByOwner::<T>::insert(
            owner_id,
            electronic_medical_record,
        );
    }



    // Add electronic_medical_record count
    pub fn add_electronic_medical_record_count() {
        let electronic_medical_record_count =
            <ElectronicMedicalRecordCount<T>>::get().unwrap_or(0);
        <ElectronicMedicalRecordCount<T>>::put(
            electronic_medical_record_count.wrapping_add(1),
        );
    }

    // Add electronic_medical_record count by owner
    pub fn add_electronic_medical_record_count_by_owner(owner_id: &T::AccountId) {
        let electronic_medical_record_count =
            ElectronicMedicalRecordCountByOwner::<T>::get(owner_id).unwrap_or(0);
        ElectronicMedicalRecordCountByOwner::<T>::insert(
            owner_id,
            electronic_medical_record_count.wrapping_add(1),
        )
    }

    // Subtract electronic_medical_record count
    pub fn sub_electronic_medical_record_count() {
        let electronic_medical_record_count =
            <ElectronicMedicalRecordCount<T>>::get().unwrap_or(1);
        ElectronicMedicalRecordCount::<T>::put(electronic_medical_record_count - 1);
    }

    // Subtract electronic_medical_record count by owner
    pub fn sub_electronic_medical_record_count_by_owner(owner_id: &T::AccountId) {
        let electronic_medical_record_count =
            ElectronicMedicalRecordCountByOwner::<T>::get(owner_id).unwrap_or(1);
        ElectronicMedicalRecordCountByOwner::<T>::insert(
            owner_id,
            electronic_medical_record_count - 1,
        );
    }



    // Add electronic_medical_record_file count
    pub fn add_electronic_medical_record_file_count() {
        let electronic_medical_record_file_count =
            <ElectronicMedicalRecordFileCount<T>>::get().unwrap_or(0);
        <ElectronicMedicalRecordFileCount<T>>::put(
            electronic_medical_record_file_count.wrapping_add(1),
        );
    }

    // Add electronic_medical_record_file count by electronic_medical_record
    pub fn add_electronic_medical_record_file_count_by_electronic_medical_record(electronic_medical_record_id: &T::Hash) {
        let electronic_medical_record_file_count =
            ElectronicMedicalRecordFileCountByElectronicMedicalRecordId::<T>::get(electronic_medical_record_id).unwrap_or(0);
        ElectronicMedicalRecordFileCountByElectronicMedicalRecordId::<T>::insert(
            electronic_medical_record_id,
            electronic_medical_record_file_count.wrapping_add(1),
        )
    }

    // Subtract electronic_medical_record_file count
    pub fn sub_electronic_medical_record_file_count() {
        let electronic_medical_record_file_count =
            <ElectronicMedicalRecordFileCount<T>>::get().unwrap_or(1);
        ElectronicMedicalRecordFileCount::<T>::put(electronic_medical_record_file_count - 1);
    }

    // Subtract electronic_medical_record_file count by electronic_medical_record
    pub fn sub_electronic_medical_record_file_count_by_electronic_medical_record(electronic_medical_record_id: &T::Hash) {
        let electronic_medical_record_file_count =
            ElectronicMedicalRecordFileCountByElectronicMedicalRecordId::<T>::get(electronic_medical_record_id).unwrap_or(1);
        ElectronicMedicalRecordFileCountByElectronicMedicalRecordId::<T>::insert(
            electronic_medical_record_id,
            electronic_medical_record_file_count - 1,
        );
    }
}

/// ElectronicMedicalRecordFilesProvider Trait Implementation
impl<T: Config> ElectronicMedicalRecordFilesProvider<T> for Pallet<T> {
    type Error = Error<T>;
    type Moment = MomentOf<T>;
    type ElectronicMedicalRecordFile = ElectronicMedicalRecordFileOf<T>;

    fn electronic_medical_record_file_by_id(
        electronic_medical_record_file_id: &T::Hash,
    ) -> Option<ElectronicMedicalRecordFileOf<T>> {
        <Self as ElectronicMedicalRecordInterface<T>>::electronic_medical_record_file_by_id(
            electronic_medical_record_file_id,
        )
    }

    fn remove_electronic_medical_record_file(
        owner_id: &T::AccountId,
        electronic_medical_record_file_id: &T::Hash,
    ) -> Result<Self::ElectronicMedicalRecordFile, Self::Error> {
        <Self as ElectronicMedicalRecordInterface<T>>::remove_electronic_medical_record_file(
            owner_id,
            electronic_medical_record_file_id,
        )
    }
}

impl<T, AccountId, Hash> ElectronicMedicalRecordFileOwnerInfo<T>
    for ElectronicMedicalRecord<AccountId, Hash>
where
    Hash: PartialEq + Eq,
    T: frame_system::Config<Hash = Hash>,
{
    fn get_electronic_medical_record_id(&self) -> &T::Hash {
        &self.get_id()
    }
}

impl<T: Config> ElectronicMedicalRecordFileByElectronicMedicalRecord<T> for Pallet<T> {
    type ElectronicMedicalRecord = ElectronicMedicalRecord<T::AccountId, T::Hash>;

    fn associate(electronic_medical_record_id: &T::Hash, electronic_medical_record_file_id: &T::Hash) -> () {
        <ElectronicMedicalRecordById<T>>::mutate(electronic_medical_record_id, |electronic_medical_record| {
            match electronic_medical_record {
                None => (), // If electronic_medical_record does not exist, do nothing
                Some(electronic_medical_record) => {
                    electronic_medical_record.add_file(*electronic_medical_record_file_id);
                }
            }
        });
    }

    fn disassociate(electronic_medical_record_id: &T::Hash, electronic_medical_record_file_id: &T::Hash) -> () {
        ElectronicMedicalRecordById::<T>::mutate(electronic_medical_record_id, |electronic_medical_record| {
            match electronic_medical_record {
                None => (),
                Some(electronic_medical_record) => {
                    electronic_medical_record.remove_file(*electronic_medical_record_file_id);
                }
            }
        });
    }
}
