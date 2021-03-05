#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_module, decl_storage, decl_event, decl_error,
    dispatch, debug,
    traits::{
        Get, Randomness, Currency, ExistenceRequirement,
    }, 
};
use frame_system::ensure_signed;
use frame_support::codec::{Encode, Decode};
use frame_support::sp_runtime::{RuntimeDebug, traits::Hash};
use frame_support::sp_std::prelude::*;

pub trait Trait: frame_system::Trait + debio_services::Trait + pallet_timestamp::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type RandomnessSource: Randomness<Self::Hash>;
    type Hashing: Hash<Output = Self::Hash>;
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct Order<Hash, AccountId, Moment> {
    id: Hash,
    service_id: Hash,
    created_at: Moment,
    customer_id: AccountId,
    lab_id: AccountId,
}

decl_storage! {
    trait Store for Module<T: Trait> as OrdersStorage {
        pub Orders get(fn order_by_id): map hasher(blake2_128_concat)
                T::Hash => Option<Order<T::Hash, T::AccountId, T::Moment>>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        Hash = <T as frame_system::Trait>::Hash,
        Moment = <T as pallet_timestamp::Trait>::Moment
    {
        /// Order created
        /// parameters, [Order, customer, lab]
        OrderCreated(Order<Hash, AccountId, Moment>, AccountId, AccountId),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// Lab id does not exist
        LabDoesNotExist,
        /// Service id does not exist
        ServiceDoesNotExist,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn create_order(origin, service_id: T::Hash) -> dispatch::DispatchResult {
            let customer_id = ensure_signed(origin)?;
            let service = debio_services::Module::<T>::service_by_id(service_id);
            match service {
                None => Err(Error::<T>::ServiceDoesNotExist)?,
                Some(service) => {
                    let order_id = Self::generate_hash(&customer_id);
                    let service_id = service.get_id();
                    let lab_id = service.get_lab_id();

                    let order = Order {
                        id: order_id,
                        customer_id: customer_id.clone(),
                        service_id: *service_id,
                        lab_id: lab_id.clone(),
                        created_at: pallet_timestamp::Module::<T>::get(),
                    };

                    Orders::<T>::insert(&order_id, &order);

                    Self::deposit_event(RawEvent::OrderCreated(order, customer_id, lab_id.clone()));
                    Ok(())
                }
            }

        }
    }
}

// TODO: Maybe extract this fn as a separate module (this is used by pallet services also)
impl<T: Trait> Module<T> {
    fn generate_hash(account_id: &T::AccountId) -> <T as frame_system::Trait>::Hash
    {
        let account_info = frame_system::Module::<T>::account(account_id);
        let hash = <T as Trait>::RandomnessSource::random(&account_info.nonce.encode());
        // let hash = <T as Trait>::Hashing::hash(random_result);
        return hash;
    }
}
