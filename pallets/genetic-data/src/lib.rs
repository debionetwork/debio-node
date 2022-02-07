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

pub mod weights;
pub mod interface;
pub use interface::GeneticDataInterface;
use sp_std::prelude::*;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct GeneticData<AccountId, Hash>
where
    Hash: PartialEq + Eq,
{
    pub id: Hash,
    pub owner_id: AccountId,
    pub title: Vec<u8>,
    pub description: Vec<u8>,
    pub report_link: Vec<u8>,
}

impl<AccountId, Hash> GeneticData<AccountId, Hash>
where
    Hash: PartialEq + Eq,
{
    pub fn new(
        id: Hash,
        owner_id: AccountId,
        title: Vec<u8>,
        description: Vec<u8>,
        report_link: Vec<u8>,
    ) -> Self {
        Self {
            id,
            owner_id,
            title,
            description,
            report_link,
        }
    }

    pub fn get_id(&self) -> &Hash {
        &self.id
    }

    pub fn get_owner_id(&self) -> &AccountId {
        &self.owner_id
    }
}

#[frame_support::pallet]
pub mod pallet {
    use crate::weights::WeightInfo;
    use crate::interface::GeneticDataInterface;
    use crate::GeneticData;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    pub use sp_std::prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type GeneticDataWeightInfo: WeightInfo;
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
    pub type GeneticDataOf<T> = GeneticData<AccountIdOf<T>, HashOf<T>>;
    pub type GeneticDataIdOf<T> = HashOf<T>;

    // ------- Storage -------------
    #[pallet::storage]
    #[pallet::getter(fn genetic_data_by_owner_id)]
    pub type GeneticDataByOwner<T> =
        StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Vec<GeneticDataIdOf<T>>>;

    #[pallet::storage]
    #[pallet::getter(fn genetic_data_by_id)]
    pub type GeneticDataById<T> =
        StorageMap<_, Blake2_128Concat, GeneticDataIdOf<T>, GeneticDataOf<T>>;

    #[pallet::storage]
    #[pallet::getter(fn genetic_data_count_by_owner)]
    pub type GeneticDataCountByOwner<T> =
        StorageMap<_, Blake2_128Concat, AccountIdOf<T>, u64>;

    #[pallet::storage]
    #[pallet::getter(fn genetic_data_count)]
    pub type GeneticDataCount<T> = StorageValue<_, u64>;
    //                                _,  Hasher         ,  Key     ,  Value
    // -----------------------------

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters, [GeneticData, who]
        GeneticDataAdded(GeneticDataOf<T>, AccountIdOf<T>),
        //// GeneticData updated
        /// parameters, [GeneticData, who]
        GeneticDataUpdated(GeneticDataOf<T>, AccountIdOf<T>),
        //// GeneticData deleted
        /// parameters, [GeneticData, who]
        GeneticDataRemoved(GeneticDataOf<T>, AccountIdOf<T>),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// User not allowed to create genetic_data
        NotAllowedToCreate,
        /// User is not the owner of a genetic_data
        NotGeneticDataOwner,
        /// Ordering a genetic_data that does not exist
        GeneticDataDoesNotExist,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(T::GeneticDataWeightInfo::add_genetic_data())]
        pub fn add_genetic_data(
            origin: OriginFor<T>,
            title: Vec<u8>,
            description: Vec<u8>,
            report_link: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as GeneticDataInterface<T>>::add_genetic_data(
                &who, 
                &title, 
                &description, 
                &report_link
            ) {
                Ok(genetic_data) => {
                    Self::deposit_event(Event::GeneticDataAdded(
                        genetic_data,
                        who.clone(),
                    ));
                    Ok(().into())
                }
                Err(error) => Err(error.into()),
            }
        }
        #[pallet::weight(T::GeneticDataWeightInfo::update_genetic_data())]
        pub fn update_genetic_data(
            origin: OriginFor<T>,
            genetic_data_id: HashOf<T>,
            title: Vec<u8>,
            description: Vec<u8>,
            report_link: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as GeneticDataInterface<T>>::update_genetic_data(
                &who, 
                &genetic_data_id,
                &title, 
                &description, 
                &report_link
            ) {
                Ok(genetic_data) => {
                    Self::deposit_event(Event::GeneticDataUpdated(
                        genetic_data,
                        who.clone(),
                    ));
                    Ok(().into())
                }
                Err(error) => Err(error.into()),
            }
        }

        #[pallet::weight(T::GeneticDataWeightInfo::remove_genetic_data())]
        pub fn remove_genetic_data(
            origin: OriginFor<T>,
            genetic_data_id: HashOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            
            match <Self as GeneticDataInterface<T>>::remove_genetic_data(
                &who,
                &genetic_data_id
            ) {
                Ok(genetic_data) => {
                    Self::deposit_event(Event::GeneticDataRemoved(
                        genetic_data,
                        who.clone(),
                    ));
                    Ok(().into())
                }
                Err(error) => Err(error.into()),
            }
        }
    }
}

use frame_support::sp_runtime::traits::Hash;

/// GeneticData Interface Implementation
impl<T: Config> GeneticDataInterface<T> for Pallet<T> {
    type Error = Error<T>;
    type GeneticDataId = T::Hash;
    type GeneticData = GeneticDataOf<T>;

    fn generate_genetic_data_id(
        owner_id: &T::AccountId,
        genetic_data_count: u64,
    ) -> Self::GeneticDataId {
        let mut account_id_bytes = owner_id.encode();
        let mut genetic_data_count_bytes =
        genetic_data_count.encode();
        account_id_bytes.append(&mut genetic_data_count_bytes);

        let seed = &account_id_bytes;
        T::Hashing::hash(seed)
    }

    fn add_genetic_data(
        owner_id: &T::AccountId,
        title: &[u8],
        description: &[u8],
        report_link: &[u8],
    ) -> Result<Self::GeneticData, Self::Error> {
        let owner_genetic_data_count = <Self as GeneticDataInterface<T>>::genetic_data_count_by_owner(owner_id);
        let genetic_data_id = Self::generate_genetic_data_id(
            owner_id,
            owner_genetic_data_count,
        );

        let genetic_data = GeneticData::new(
            genetic_data_id,
            owner_id.clone(),
            title.to_vec(),
            description.to_vec(),
            report_link.to_vec(),
        );

        // Store to GeneticDataById storage
        GeneticDataById::<T>::insert(genetic_data_id, &genetic_data);

        Self::add_genetic_data_by_owner(
            owner_id,
            &genetic_data_id
        );
        Self::add_genetic_data_count();
        Self::add_genetic_data_count_by_owner(owner_id);

        Ok(genetic_data)
    }

    fn update_genetic_data(
        owner_id: &T::AccountId,
        genetic_data_id: &T::Hash,
        title: &[u8],
        description: &[u8],
        report_link: &[u8],
    ) -> Result<Self::GeneticData, Self::Error> {
        let _ =  match <Self as GeneticDataInterface<T>>::remove_genetic_data(
            owner_id,
            genetic_data_id,
        ) {
            Ok(res) => res,
            Err(error) => return Err(error),
        };

        <Self as GeneticDataInterface<T>>::add_genetic_data(
            owner_id,
            title,
            description,
            report_link,
        )
    }

    fn remove_genetic_data(
        owner_id: &T::AccountId,
        genetic_data_id: &T::Hash,
    ) -> Result<Self::GeneticData, Self::Error> {
        let genetic_data = GeneticDataById::<T>::get(genetic_data_id);
        if genetic_data == None {
            return Err(Error::<T>::GeneticDataDoesNotExist);
        }

        if genetic_data.unwrap().owner_id != owner_id.clone() {
            return Err(Error::<T>::NotGeneticDataOwner);
        }

        // Get genetic_data from storage
        let genetic_data =
            GeneticDataById::<T>::get(genetic_data_id).unwrap();

        // Remove genetic_data from storage
        GeneticDataById::<T>::take(genetic_data_id).unwrap();

        Self::sub_genetic_data_by_owner(
            genetic_data.get_owner_id(),
            genetic_data_id
        );
        Self::sub_genetic_data_count();
        Self::sub_genetic_data_count_by_owner(genetic_data.get_owner_id());

        Ok(genetic_data)
    }

    fn genetic_data_by_owner_id(
        owner_id: &T::AccountId,
    ) -> Option<Vec<T::Hash>> {
        GeneticDataByOwner::<T>::get(owner_id)
    }

    fn genetic_data_count_by_owner(owner_id: &T::AccountId) -> u64 {
        GeneticDataCountByOwner::<T>::get(owner_id).unwrap_or(0)
    }

    fn genetic_data_by_id(
        genetic_data_id: &Self::GeneticDataId,
    ) -> Option<Self::GeneticData> {
        GeneticDataById::<T>::get(genetic_data_id)
    }
}

/// Pallet Methods
impl<T: Config> Pallet<T> {
    // Add genetic_data by owner
    pub fn add_genetic_data_by_owner(owner_id: &T::AccountId, genetic_data_id: &T::Hash) {
        let mut genetic_data =
            GeneticDataByOwner::<T>::get(owner_id).unwrap_or_default();

        genetic_data.push(*genetic_data_id);
        GeneticDataByOwner::<T>::insert(
            owner_id,
            &genetic_data,
        )
    }

    // Subtract genetic_data by owner
    pub fn sub_genetic_data_by_owner(owner_id: &T::AccountId, genetic_data_id: &T::Hash) {
        let mut genetic_data =
            GeneticDataByOwner::<T>::get(owner_id).unwrap_or_default();
        genetic_data.retain(|&x| x != *genetic_data_id);
        GeneticDataByOwner::<T>::insert(
            owner_id,
            genetic_data,
        );
    }

    // Add genetic_data count
    pub fn add_genetic_data_count() {
        let genetic_data_count =
            <GeneticDataCount<T>>::get().unwrap_or(0);
        <GeneticDataCount<T>>::put(
            genetic_data_count.wrapping_add(1),
        );
    }

    // Add genetic_data count by owner
    pub fn add_genetic_data_count_by_owner(owner_id: &T::AccountId) {
        let genetic_data_count =
            GeneticDataCountByOwner::<T>::get(owner_id).unwrap_or(0);
        GeneticDataCountByOwner::<T>::insert(
            owner_id,
            genetic_data_count.wrapping_add(1),
        )
    }

    // Subtract genetic_data count
    pub fn sub_genetic_data_count() {
        let genetic_data_count =
            <GeneticDataCount<T>>::get().unwrap_or(1);
        GeneticDataCount::<T>::put(genetic_data_count - 1);
    }

    // Subtract genetic_data count by owner
    pub fn sub_genetic_data_count_by_owner(owner_id: &T::AccountId) {
        let genetic_data_count =
            GeneticDataCountByOwner::<T>::get(owner_id).unwrap_or(1);
        GeneticDataCountByOwner::<T>::insert(
            owner_id,
            genetic_data_count - 1,
        );
    }
}