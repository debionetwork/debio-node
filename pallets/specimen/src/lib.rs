#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResultWithPostInfo, pallet_prelude::*,
    };
    use frame_system::pallet_prelude::*;
    use sp_std::prelude::*;
    use frame_support::codec::{Encode, Decode};

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        // type RandomnessSource: Randomness<Self::Hash>;
    }

    // ----- This is template code, every pallet needs this ---
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
    // --------------------------------------------------------

    #[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
    pub enum SpecimenStatus {
        Sending,
        Received,
        Rejected,
        Processed,
    }
    impl Default for SpecimenStatus {
        fn default() -> Self { SpecimenStatus::Sending }
    }

    #[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
    pub struct Specimen<Hash, AccountId, Moment> {
        pub order_id: Hash,
        pub service_id: Hash,
        pub owner_id: AccountId,
        pub lab_id: AccountId,
        pub status: SpecimenStatus,
        pub created_at: Moment,
        pub updated_at: Moment,
        pub result_file: Option<Vec<u8>>, // IPFS Link
        pub result_report: Option<Vec<u8>>, // IPFS Link
        pub comments: Option<Vec<u8>>, // FIXME: Limit the length of comments
    } 

    // Types ----
    pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    pub type HashOf<T> = <T as frame_system::Config>::Hash;
    pub type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;
    pub type SpecimenOf<T> = Specimen<HashOf<T>, AccountIdOf<T>, MomentOf<T>>;

    // ------ Storage --------------------------
    #[pallet::storage]
    #[pallet::getter(fn specimen_by_order_id)]
    pub type Specimens<T> = StorageMap<_, Blake2_128Concat, HashOf<T>, SpecimenOf<T>>;


    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Specimen Received
        /// parameters [Specimen]
        SpecimenReceived(SpecimenOf<T>),
        /// Specimen Rejected
        /// parameters [Specimen]
        SpecimenRejected(SpecimenOf<T>),
        /// Specimen Processed
        /// parameters [Specimen]
        SpecimenProcessed(SpecimenOf<T>),
    }
    
    #[pallet::error]
    pub enum Error<T> {
        /// Specimen not found
        SpecimenNotFound,
        /// Origin not authorized to operate on specimen
        NotAuthorized,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /**
         * Receive Specimen
         * */
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn receive(origin: OriginFor<T>, order_id: T::Hash) -> DispatchResultWithPostInfo {
            let receiver_id = ensure_signed(origin)?;

            let specimen = Specimens::<T>::get(&order_id);
            if specimen == None {
                return Err(Error::<T>::SpecimenNotFound)?;
            }
            let mut specimen = specimen.unwrap();

            // Only specimen's lab (recorded at specimen creation in order creation) can receive
            ensure!(specimen.lab_id == receiver_id, "Not Authorized");
            /*
            if specimen.lab_id != receiver_id {
                return Err(Error::<T>::NotAuthorized)?;
            }
            */
            
            // TODO: Should escrow expiry be increased after receiving specimen??
            specimen.status = SpecimenStatus::Received;
            specimen.updated_at = pallet_timestamp::Module::<T>::get();
            Specimens::<T>::insert(&order_id, &specimen);

            Self::deposit_event(Event::SpecimenReceived(specimen.clone()));
            Ok(().into())
        }


        /**
         * Reject Specimen
         * */
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn reject(origin: OriginFor<T>, order_id: T::Hash, rejection_reason: Vec<u8>) -> DispatchResultWithPostInfo {
            let receiver_id = ensure_signed(origin)?;

            let specimen = Specimens::<T>::get(&order_id);
            if specimen == None {
                return Err(Error::<T>::SpecimenNotFound)?;
            }
            let mut specimen = specimen.unwrap();

            // Only specimen's lab (recorded at specimen creation in order creation) can receive
            if specimen.lab_id != receiver_id {
                return Err(Error::<T>::NotAuthorized)?;
            }

            specimen.status = SpecimenStatus::Rejected;
            specimen.comments = Some(rejection_reason);
            specimen.updated_at = pallet_timestamp::Module::<T>::get();
            Specimens::<T>::insert(&order_id, &specimen);

            Self::deposit_event(Event::SpecimenRejected(specimen.clone()));
            Ok(().into())
        }


        /**
         * Process Specimen
         * */
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn process(
            origin: OriginFor<T>,
            order_id: T::Hash,
            result_file: Vec<u8>,
            result_report: Vec<u8>
        ) 
            -> DispatchResultWithPostInfo
        {
            let receiver_id = ensure_signed(origin)?;

            let specimen = Specimens::<T>::get(&order_id);
            if specimen == None {
                return Err(Error::<T>::SpecimenNotFound)?;
            }
            let mut specimen = specimen.unwrap();

            // Only specimen's lab (recorded at specimen creation in order creation) can receive
            if specimen.lab_id != receiver_id {
                return Err(Error::<T>::NotAuthorized)?;
            }

            specimen.result_file = Some(result_file);
            specimen.result_report = Some(result_report);
            specimen.status = SpecimenStatus::Processed;
            specimen.updated_at = pallet_timestamp::Module::<T>::get();
            Specimens::<T>::insert(&order_id, &specimen);

            Self::deposit_event(Event::SpecimenProcessed(specimen.clone()));
            Ok(().into())
        }
    }
}

use frame_support::debug;
use crate::pallet::{
    Specimen, SpecimenOf
};

impl<T: Config> Pallet<T> {
    /**
     * Create Specimen
     * */
    pub fn create_specimen(
        order_id: &T::Hash,
        service_id: &T::Hash,
        owner_id: &T::AccountId,
        lab_id: &T::AccountId,
        order_created_at: &T::Moment
    )
        -> SpecimenOf<T>
    {
        let specimen = Specimen {
            order_id: order_id.clone(),
            service_id: service_id.clone(),
            owner_id: owner_id.clone(),
            lab_id: lab_id.clone(),
            status: SpecimenStatus::Sending,
            created_at: order_created_at.clone(),
            updated_at: order_created_at.clone(),
            result_file: None,
            result_report: None,
            comments: None
        };
        debug::info!("*** ---- Creating Specimen ---- ***: {:?}", &specimen);
        Specimens::<T>::insert(order_id, &specimen);

        specimen.clone()
    }

    /**
     * Is Status ?
     * */
    pub fn is_status(order_id: &T::Hash, status: SpecimenStatus) -> bool {
        let specimen = Specimens::<T>::get(order_id);
        match specimen {
            None => false,
            Some(specimen) => {
                return specimen.status == status;
            }
        }
    }
}

