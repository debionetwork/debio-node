#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
pub use pallet::*;
pub use scale_info::TypeInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use traits_electronic_medical_record::{
    ElectronicMedicalRecordFile as ElectronicMedicalRecordFileT,
    ElectronicMedicalRecordFilesProvider,
    ElectronicMedicalRecordFileByElectronicMedicalRecord
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

    pub fn add_file(&mut self, emr_files_id: Hash) {
        self.files.push(emr_files_id);
    }

    pub fn remove_file(&mut self, emr_files_id: Hash) {
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

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct ElectronicMedicalRecordFileSubmission
{
    pub title: Vec<u8>,
    pub description: Vec<u8>,
    pub record_link: Vec<u8>,
}

#[frame_support::pallet]
pub mod pallet {
    use crate::weights::WeightInfo;
    use crate::interface::ElectronicMedicalRecordInterface;
    use crate::{
        ElectronicMedicalRecord, 
        ElectronicMedicalRecordFile, 
        ElectronicMedicalRecordFileSubmission, 
        ElectronicMedicalRecordFileByElectronicMedicalRecord
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
    pub type ElectronicMedicalRecordFileSubmissionOf = ElectronicMedicalRecordFileSubmission;

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
    #[pallet::getter(fn electronic_medical_record_count)]
    pub type ElectronicMedicalRecordCount<T> = StorageValue<_, u64>;
    //                                _,  Hasher         ,  Key     ,  Value
    // -----------------------------

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters, [ElectronicMedicalRecord, who]
        ElectronicMedicalRecordAdded(ElectronicMedicalRecordOf<T>, AccountIdOf<T>),
        //// ElectronicMedicalRecord updated
        /// parameters, [ElectronicMedicalRecord, who]
        ElectronicMedicalRecordUpdated(ElectronicMedicalRecordOf<T>, AccountIdOf<T>),
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
        #[pallet::weight(T::ElectronicMedicalRecordWeightInfo::add_electronic_medical_record())]
        pub fn add_electronic_medical_record(
            origin: OriginFor<T>,
            title: Vec<u8>,
            category: Vec<u8>,
            files: Vec<ElectronicMedicalRecordFileSubmissionOf>
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as ElectronicMedicalRecordInterface<T>>::add_electronic_medical_record(&who, &title, &category, &files)
            {
                Ok(electronic_medical_record) => {
                    Self::deposit_event(Event::ElectronicMedicalRecordAdded(
                        electronic_medical_record,
                        who.clone(),
                    ));
                    Ok(().into())
                }
                Err(error) => Err(error.into()),
            }
        }
        #[pallet::weight(T::ElectronicMedicalRecordWeightInfo::update_electronic_medical_record())]
        pub fn update_electronic_medical_record(
            origin: OriginFor<T>,
            electronic_medical_record_id: HashOf<T>,
            title: Vec<u8>,
            category: Vec<u8>,
            files: Vec<ElectronicMedicalRecordFileSubmissionOf>
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as ElectronicMedicalRecordInterface<T>>::update_electronic_medical_record(
                &who, 
                &electronic_medical_record_id,
                &title, 
                &category, 
                &files
            ) {
                Ok(electronic_medical_record) => {
                    Self::deposit_event(Event::ElectronicMedicalRecordUpdated(
                        electronic_medical_record,
                        who.clone(),
                    ));
                    Ok(().into())
                }
                Err(error) => Err(error.into()),
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
                &electronic_medical_record_id
            ) {
                Ok(electronic_medical_record) => {
                    Self::deposit_event(Event::ElectronicMedicalRecordRemoved(
                        electronic_medical_record,
                        who.clone(),
                    ));
                    Ok(().into())
                }
                Err(error) => Err(error.into()),
            }
        }
    }
}

use core::convert::TryInto;
use frame_support::sp_runtime::traits::Hash;
use traits_electronic_medical_record::ElectronicMedicalRecordFileOwnerInfo;

/// ElectronicMedicalRecord Interface Implementation
impl<T: Config> ElectronicMedicalRecordInterface<T> for Pallet<T> {
    type Error = Error<T>;
    type ElectronicMedicalRecordId = T::Hash;
    type ElectronicMedicalRecord = ElectronicMedicalRecordOf<T>;

    type ElectronicMedicalRecordFileId = T::Hash;
    type ElectronicMedicalRecordFile = ElectronicMedicalRecordFileOf<T>;
    type ElectronicMedicalRecordFileSubmission = ElectronicMedicalRecordFileSubmissionOf;

    fn generate_electronic_medical_record_id(
        owner_id: &T::AccountId,
        electronic_medical_record_count: u64,
    ) -> Self::ElectronicMedicalRecordId {
        let mut account_id_bytes = owner_id.encode();
        let mut electronic_medical_record_count_bytes =
        electronic_medical_record_count.encode();
        account_id_bytes.append(&mut electronic_medical_record_count_bytes);

        let seed = &account_id_bytes;
        T::Hashing::hash(seed)
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
        T::Hashing::hash(seed)
    }

    fn add_electronic_medical_record(
        owner_id: &T::AccountId,
        title: &[u8],
        category: &[u8],
        files: &[Self::ElectronicMedicalRecordFileSubmission]
    ) -> Result<Self::ElectronicMedicalRecord, Self::Error> {
        let owner_electronic_medical_record_count = <Self as ElectronicMedicalRecordInterface<T>>::electronic_medical_record_count_by_owner(owner_id);
        let electronic_medical_record_id = Self::generate_electronic_medical_record_id(
            owner_id,
            owner_electronic_medical_record_count,
        );

        let electronic_medical_record = ElectronicMedicalRecord::new(
            electronic_medical_record_id,
            owner_id.clone(),
            title.to_vec(),
            category.to_vec()
        );

        // Store to ElectronicMedicalRecordById storage
        ElectronicMedicalRecordById::<T>::insert(electronic_medical_record_id, &electronic_medical_record);

        Self::add_electronic_medical_record_by_owner(
            owner_id,
            &electronic_medical_record_id
        );
        Self::add_electronic_medical_record_count();
        Self::add_electronic_medical_record_count_by_owner(owner_id);

        for emr_files in files {
            let electronic_medical_record = ElectronicMedicalRecordById::<T>::get(electronic_medical_record_id).unwrap();
            let electronic_medical_record_file_count = electronic_medical_record.files.len();
            let electronic_medical_record_file_id = Self::generate_electronic_medical_record_file_id(
                &electronic_medical_record_id,
                electronic_medical_record_file_count.try_into().unwrap(),
            );
    
            let electronic_medical_record_file = ElectronicMedicalRecordFile::new(
                electronic_medical_record_file_id,
                electronic_medical_record_id,
                emr_files.title.clone(),
                emr_files.description.clone(),
                emr_files.record_link.clone(),
                pallet_timestamp::Pallet::<T>::get(),
            );
    
            // Store to ElectronicMedicalRecordFiles storage
            ElectronicMedicalRecordFileById::<T>::insert(
                &electronic_medical_record_file_id,
                &electronic_medical_record_file,
            );
    
            // Associate created electronic_medical_record_file to the electronic_medical_record
            T::ElectronicMedicalRecord::associate(&electronic_medical_record_id, &electronic_medical_record_file_id);
        }

        Ok(electronic_medical_record)
    }

    fn update_electronic_medical_record(
        owner_id: &T::AccountId,
        electronic_medical_record_id: &T::Hash,
        title: &[u8],
        category: &[u8],
        files: &[Self::ElectronicMedicalRecordFileSubmission]
    ) -> Result<Self::ElectronicMedicalRecord, Self::Error> {
        let _ =  match <Self as ElectronicMedicalRecordInterface<T>>::remove_electronic_medical_record(
            owner_id,
            electronic_medical_record_id,
        ) {
            Ok(res) => res,
            Err(error) => return Err(error),
        };

        <Self as ElectronicMedicalRecordInterface<T>>::add_electronic_medical_record(
            owner_id,
            title,
            category,
            files,
        )
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

        // Get electronic_medical_record from storage
        let electronic_medical_record =
            ElectronicMedicalRecordById::<T>::get(electronic_medical_record_id).unwrap();

        for emr_file_id in &electronic_medical_record.files {
            // disassociate electronic_medical_record_file reference from the electronic_medical_record
            T::ElectronicMedicalRecord::disassociate(
                electronic_medical_record_id,
                emr_file_id,
            );
                
            // Remove electronic_medical_record_file from storage
            ElectronicMedicalRecordFileById::<T>::take(emr_file_id).unwrap();
        }

        // Remove electronic_medical_record from storage
        ElectronicMedicalRecordById::<T>::take(electronic_medical_record_id).unwrap();

        Self::sub_electronic_medical_record_by_owner(
            electronic_medical_record.get_owner_id(),
            electronic_medical_record_id
        );
        Self::sub_electronic_medical_record_count();
        Self::sub_electronic_medical_record_count_by_owner(electronic_medical_record.get_owner_id());

        Ok(electronic_medical_record)
    }

    fn electronic_medical_record_by_owner_id(
        owner_id: &T::AccountId,
    ) -> Option<Vec<T::Hash>> {
        ElectronicMedicalRecordByOwner::<T>::get(owner_id).map(|electronic_medical_record_vec| electronic_medical_record_vec)
    }

    fn electronic_medical_record_count_by_owner(owner_id: &T::AccountId) -> u64 {
        ElectronicMedicalRecordCountByOwner::<T>::get(owner_id).unwrap_or(0)
    }

    fn electronic_medical_record_by_id(
        electronic_medical_record_id: &Self::ElectronicMedicalRecordId,
    ) -> Option<Self::ElectronicMedicalRecord> {
        ElectronicMedicalRecordById::<T>::get(electronic_medical_record_id)
    }

    fn electronic_medical_record_file_by_id(
        electronic_medical_record_file_id: &Self::ElectronicMedicalRecordFileId,
    ) -> Option<Self::ElectronicMedicalRecordFile> {
        ElectronicMedicalRecordFileById::<T>::get(electronic_medical_record_file_id)
    }
}

/// Pallet Methods
impl<T: Config> Pallet<T> {
    // Add electronic_medical_record by owner
    pub fn add_electronic_medical_record_by_owner(owner_id: &T::AccountId, electronic_medical_record_id: &T::Hash) {
        let mut electronic_medical_record =
            ElectronicMedicalRecordByOwner::<T>::get(owner_id).unwrap_or_default();

        electronic_medical_record.push(*electronic_medical_record_id);
        ElectronicMedicalRecordByOwner::<T>::insert(
            owner_id,
            &electronic_medical_record,
        )
    }

    // Subtract electronic_medical_record by owner
    pub fn sub_electronic_medical_record_by_owner(owner_id: &T::AccountId, electronic_medical_record_id: &T::Hash) {
        let mut electronic_medical_record =
            ElectronicMedicalRecordByOwner::<T>::get(owner_id).unwrap_or_default();
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
}

/// ElectronicMedicalRecordFilesProvider Trait Implementation
impl<T: Config> ElectronicMedicalRecordFilesProvider<T> for Pallet<T> {
    type ElectronicMedicalRecordFile = ElectronicMedicalRecordFileOf<T>;

    fn electronic_medical_record_file_by_id(
        electronic_medical_record_file_id: &T::Hash,
    ) -> Option<ElectronicMedicalRecordFileOf<T>> {
        <Self as ElectronicMedicalRecordInterface<T>>::electronic_medical_record_file_by_id(
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
        self.get_id()
    }
}

impl<T: Config> ElectronicMedicalRecordFileByElectronicMedicalRecord<T> for Pallet<T> {
    type ElectronicMedicalRecord = ElectronicMedicalRecord<T::AccountId, T::Hash>;

    fn associate(electronic_medical_record_id: &T::Hash, electronic_medical_record_file_id: &T::Hash) {
        <ElectronicMedicalRecordById<T>>::mutate(electronic_medical_record_id, |electronic_medical_record| {
            match electronic_medical_record {
                None => (), // If electronic_medical_record does not exist, do nothing
                Some(electronic_medical_record) => {
                    electronic_medical_record.add_file(*electronic_medical_record_file_id);
                }
            }
        });
    }

    fn disassociate(electronic_medical_record_id: &T::Hash, electronic_medical_record_file_id: &T::Hash) {
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