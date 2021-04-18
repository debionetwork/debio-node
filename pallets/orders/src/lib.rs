#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use frame_support::codec::{Encode, Decode};
use frame_support::pallet_prelude::*;
use sp_std::prelude::*;
use traits_services::{ServicesProvider};

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub enum OrderStatus {
    Unpaid,
    Paid,
    Fulfilled,
    Refunded,
}
impl Default for OrderStatus {
    fn default() -> Self { OrderStatus::Unpaid }
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct Order<Hash, AccountId, Moment> {
    pub id: Hash,
    pub service_id: Hash,
    pub customer_id: AccountId,
    pub seller_id: AccountId,
    pub dna_sample_tracking_id: Option<Vec<u8>>, // TODO: get from Specimen pallet
    pub status: OrderStatus,
    pub created_at: Moment,
    pub updated_at: Moment,
}
impl<Hash, AccountId, Moment> Order<Hash, AccountId, Moment> {
    pub fn new(
        id: Hash,
        service_id: Hash,
        customer_id: AccountId,
        seller_id: AccountId,
        created_at: Moment,
        updated_at: Moment,
    )
        -> Self
    {
        Self {
            id,
            service_id,
            customer_id,
            seller_id,
            dna_sample_tracking_id: None, 
            status: OrderStatus::default(),
            created_at,
            updated_at,
        }
    }

    pub fn get_id(&self) -> &Hash {
        &self.id
    }

    pub fn get_created_at(&self) -> &Moment {
        &self.created_at
    }

    pub fn get_service_id(&self) -> &Hash {
        &self.service_id
    }
}

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResultWithPostInfo, pallet_prelude::*,
    };
    use frame_system::pallet_prelude::*;
    use sp_std::prelude::*;
    use crate::*;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Services: ServicesProvider<Self>;
    }


    // ----- This is template code, every pallet needs this ---
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
    // --------------------------------------------------------

    // ---- Types --------------------------------------------
    type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    pub type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;
    type HashOf<T> = <T as frame_system::Config>::Hash;
    pub type OrderOf<T> = Order<HashOf<T>, AccountIdOf<T>, MomentOf<T>>;
    type OrderIdsOf<T> = Vec<HashOf<T>>;
    // -------------------------------------------------------

    // ------ Storage --------------------------
    #[pallet::storage]
    #[pallet::getter(fn order_by_id)]
    pub type Orders<T> = StorageMap<_, Blake2_128Concat, HashOf<T>, OrderOf<T>>;

    #[pallet::storage]
    #[pallet::getter(fn orders_by_costumer_id)]
    pub type OrdersByCustomer<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, OrderIdsOf<T>>;

    #[pallet::storage]
    #[pallet::getter(fn orders_by_lab_id)]
    pub type OrdersBySeller<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, OrderIdsOf<T>>;

    #[pallet::storage]
    #[pallet::getter(fn admin_key)]
    pub type EscrowKey<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;
    // -----------------------------------------


    // ----- Genesis Configs ------------------
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub escrow_key: T::AccountId,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                escrow_key: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            EscrowKey::<T>::put(&self.escrow_key);
        }
    }
    // ----------------------------------------

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Order created
        /// parameters, [Order]
        OrderCreated(OrderOf<T>),
        /// Order paid
        /// parameters, [Order]
        OrderPaid(OrderOf<T>),
        /// Order Fulfilled
        /// parameters, [Order]
        OrderFulfilled(OrderOf<T>),
        /// Order Refunded
        /// parameters, [Order]
        OrderRefunded(OrderOf<T>),
    }
      

    #[pallet::error]
    pub enum Error<T> {
        /// Service id does not exist
        ServiceDoesNotExist,
        /// Order does not exist
        OrderNotFound,
        /// Unauthorized to fulfill order - user is not the seller who owns the service
        UnauthorizedOrderFulfillment,
        /// Can not fulfill order before Specimen is processed
        SpecimenNotProcessed,
        /// Refund not allowed, Order is not expired yet
        OrderNotYetExpired,
        /// Unauthorized Account
        Unauthorized
    }


    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn create_order(origin: OriginFor<T>, service_id: T::Hash) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as OrderInterface<T>>::create_order(&who, &service_id) {
                Ok(order) => {
                    Self::deposit_event(Event::<T>::OrderCreated(order.clone()));
                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn set_order_paid(origin: OriginFor<T>, order_id: T::Hash) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as OrderInterface<T>>::set_order_paid(&who, &order_id) {
                Ok(order) => {
                    Self::deposit_event(Event::<T>::OrderPaid(order.clone()));
                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn fulfill_order(origin: OriginFor<T>, order_id: T::Hash) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as OrderInterface<T>>::fulfill_order(&who, &order_id) {
                Ok(order) => {
                    Self::deposit_event(Event::<T>::OrderFulfilled(order.clone()));
                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }
        
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn refund_order(origin: OriginFor<T>, order_id: T::Hash) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as OrderInterface<T>>::fulfill_order(&who, &order_id) {
                Ok(order) => {
                    Self::deposit_event(Event::<T>::OrderRefunded(order.clone()));
                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }
    }
}

pub mod interface;
use interface::OrderInterface;
use traits_services::ServiceInfo;

impl<T: Config> OrderInterface<T> for Pallet<T> {
    type Order = OrderOf<T>;
    type Error = Error<T>;

    fn create_order(customer_id: &T::AccountId, service_id: &T::Hash) -> Result<Self::Order, Self::Error> {
        let service = T::Services::service_by_id(service_id);
        if service.is_none() {
            return Err(Error::<T>::ServiceDoesNotExist);
        }
        let service = service.unwrap();
        let order_id = Self::generate_order_id(customer_id, service_id);
        let seller_id = service.get_owner_id();
        let now = pallet_timestamp::Pallet::<T>::get();

        let order = Order::new(
            order_id.clone(),
            service_id.clone(),
            customer_id.clone(),
            seller_id.clone(),
            now,
            now
        );
        Self::insert_order_to_storage(&order);

        Ok(order)
    }

    fn set_order_paid(escrow_account_id: &T::AccountId, order_id: &T::Hash) -> Result<Self::Order, Self::Error> {
        if escrow_account_id.clone() != EscrowKey::<T>::get() {
            return Err(Error::<T>::Unauthorized);
        }

        let order = Self::update_order_status(order_id, OrderStatus::Paid);
        if order.is_none() {
            return Err(Error::<T>::OrderNotFound);
        }

        Ok(order.unwrap())
    }

    fn fulfill_order(seller_id: &T::AccountId, order_id: &T::Hash) -> Result<Self::Order, Self::Error> {
        let order = Orders::<T>::get(order_id);
        if order.is_none() {
            return Err(Error::<T>::OrderNotFound);
        }
        let order = order.unwrap();

        // Only the seller can fulfill the order
        if order.seller_id != seller_id.clone() {
            return Err(Error::<T>::UnauthorizedOrderFulfillment);
        }

        // An order can only be fulfilled if the DnaSample is processed
        // TODO: Check if DnaSample is processed
        // - if GeneticTesting::get_sample(order.dna_sample_tracking_id).success() == false:
        //      return Err(Error::<T>::DnaSampleNotSuccessfullyProcessed)

        let order = Self::update_order_status(order_id, OrderStatus::Fulfilled);

        Ok(order.unwrap())
    }

    fn refund_order(escrow_account_id: &T::AccountId, order_id: &T::Hash) -> Result<Self::Order, Self::Error> {
        if escrow_account_id.clone() != EscrowKey::<T>::get() {
            return Err(Error::<T>::Unauthorized);
        }

        let order = Orders::<T>::get(order_id);
        if order.is_none() {
            return Err(Error::<T>::OrderNotFound);
        }

        let order_can_be_refunded = Self::order_can_be_refunded(order.unwrap());
        if !order_can_be_refunded {
            return Err(Error::<T>::OrderNotYetExpired);
        }

        let order = Self::update_order_status(order_id, OrderStatus::Refunded);
        Ok(order.unwrap())
    }
}

use frame_support::sp_runtime::traits::Hash;
use frame_support::sp_std::convert::{TryInto, TryFrom};

impl<T: Config> Pallet<T> {
    pub fn generate_order_id(customer_id: &T::AccountId, service_id: &T::Hash) -> T::Hash {
        let mut customer_id_bytes = customer_id.encode();
        let mut service_id_bytes = service_id.encode();
        let account_info = frame_system::Pallet::<T>::account(customer_id);
        let mut nonce_bytes = account_info.nonce.encode();

        customer_id_bytes.append(&mut service_id_bytes);
        customer_id_bytes.append(&mut nonce_bytes);

        let seed = &customer_id_bytes;
        T::Hashing::hash(seed)
    }


    pub fn update_order_status(order_id: &T::Hash, status: OrderStatus)
        -> Option<Order<T::Hash, T::AccountId, T::Moment>>
    {
        Orders::<T>::mutate(order_id, |order| {
            match order {
                None => None,
                Some(order) => {
                    order.status = status;
                    order.updated_at = pallet_timestamp::Pallet::<T>::get();
                    Some(order.clone())
                }
            }
        })
    }


    pub fn insert_order_to_storage(order: &OrderOf<T>) -> () {
        Orders::<T>::insert(order.id, order);
        Self::insert_order_id_into_orders_by_seller(order);
        Self::insert_order_id_into_orders_by_customer(order);
    }


    pub fn insert_order_id_into_orders_by_seller(order: &OrderOf<T>) -> () {
        match OrdersBySeller::<T>::get(&order.seller_id) {
            None => {
                let mut orders = Vec::new();
                orders.push(order.id.clone());
                OrdersBySeller::<T>::insert(&order.seller_id, orders);
            },
            Some(mut orders) => {
                orders.push(order.id.clone());
                OrdersBySeller::<T>::insert(&order.seller_id, orders);
            }
        }
    }


    pub fn insert_order_id_into_orders_by_customer(order: &OrderOf<T>) -> () {
        match OrdersByCustomer::<T>::get(&order.customer_id) {
            None => {
                let mut orders = Vec::new();
                orders.push(order.id.clone());
                OrdersByCustomer::<T>::insert(&order.customer_id, orders);
            },
            Some(mut orders) => {
                orders.push(order.id.clone());
                OrdersByCustomer::<T>::insert(&order.customer_id, orders);
            }
        }
    }


    pub fn order_can_be_refunded(order: OrderOf<T>) -> bool {
        // Check if order expired ------------------
        let now = pallet_timestamp::Pallet::<T>::get();
        let order_created_at = order.created_at.clone();
        // convert to u64
        let order_created_at_ms = TryInto::<u64>::try_into(order_created_at).ok().unwrap();
        // Add 7 days
        let seven_days_ms = u64::try_from(chrono::Duration::days(7).num_milliseconds()).ok().unwrap();
        let expires_at_ms = order_created_at_ms + seven_days_ms;
        // convert back to Moment
        let expires_at = TryInto::<MomentOf<T>>::try_into(expires_at_ms).ok().unwrap();

        /*
        // TODO:
        // Check if specimen rejected
        let is_specimen_rejected = specimen::Pallet::<T>::is_status(
            &order_id,
            specimen::SpecimenStatus::Rejected
        );

        // Can refund if order expired or specimen rejected
        let can_refund = now > expires_at || is_specimen_rejected;
        if !can_refund {
            return Err(Error::<T>::OrderNotYetExpired)?;
        }
        */

        let can_refund = now > expires_at;
        if !can_refund {
            return false;
        }
            
        true
    }
}

