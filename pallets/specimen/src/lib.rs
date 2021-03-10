#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_module, decl_storage, decl_event, decl_error,
    dispatch, debug,
    traits::{
        Get, // Randomness, Currency, ExistenceRequirement,
    }, 
};
use frame_system::ensure_signed;
use frame_support::codec::{Encode, Decode};
use frame_support::sp_runtime::{
    RuntimeDebug,
};
use frame_support::sp_std::prelude::*;
use frame_support::ensure;
// use frame_support::sp_std::convert::{TryInto, TryFrom};

pub trait Trait: frame_system::Trait + pallet_timestamp::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    // type RandomnessSource: Randomness<Self::Hash>;
}

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
    order_id: Hash,
    service_id: Hash,
    owner_id: AccountId,
    lab_id: AccountId,
    status: SpecimenStatus,
    created_at: Moment,
    updated_at: Moment,
    result_file: Option<Vec<u8>>, // IPFS Link
    result_report: Option<Vec<u8>>, // IPFS Link
    comments: Option<Vec<u8>>, // FIXME: Limit the length of comments
}

type SpecimenOf<T> = Specimen<
    <T as frame_system::Trait>::Hash,
    <T as frame_system::Trait>::AccountId,
    <T as pallet_timestamp::Trait>::Moment
>;

decl_storage! {
    trait Store for Module<T: Trait> as SpecimenStorage {
        pub Specimens get(fn specimen_by_order_id):
            map hasher(blake2_128_concat) T::Hash => Option<SpecimenOf<T>>;
    }
}

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// Specimen not found
        SpecimenNotFound,
        /// Origin not authorized to operate on specimen
        NotAuthorized,
    }
}

decl_event! {
    pub enum Event<T> where
        Hash = <T as frame_system::Trait>::Hash,
        AccountId = <T as frame_system::Trait>::AccountId,
        Moment = <T as pallet_timestamp::Trait>::Moment,
    {
        /// Specimen Received
        /// parameters [Specimen]
        SpecimenReceived(Specimen<Hash, AccountId, Moment>),
        /// Specimen Rejected
        /// parameters [Specimen]
        SpecimenRejected(Specimen<Hash, AccountId, Moment>),
        /// Specimen Processed
        /// parameters [Specimen]
        SpecimenProcessed(Specimen<Hash, AccountId, Moment>),
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;


        /**
         * Receive Specimen
         * */
        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn receive(origin, order_id: T::Hash) -> dispatch::DispatchResult {
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

            Self::deposit_event(RawEvent::SpecimenReceived(specimen.clone()));
            Ok(())
        }


        /**
         * Reject Specimen
         * */
        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn reject(origin, order_id: T::Hash, rejection_reason: Vec<u8>) -> dispatch::DispatchResult {
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

            Self::deposit_event(RawEvent::SpecimenRejected(specimen.clone()));
            Ok(())
        }


        /**
         * Process Specimen
         * */
        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn process(
            origin,
            order_id: T::Hash,
            result_file: Vec<u8>,
            result_report: Vec<u8>
        ) 
            -> dispatch::DispatchResult
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

            Self::deposit_event(RawEvent::SpecimenProcessed(specimen.clone()));
            Ok(())
        }
    }
}


impl<T: Trait> Module<T> {
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
