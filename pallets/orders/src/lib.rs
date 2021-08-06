#![cfg_attr(not(feature = "std"), no_std)]

pub mod interface;
use interface::OrderInterface;

pub use pallet::*;
use frame_support::traits::{ Currency };
use frame_support::codec::{Encode, Decode};
use frame_support::pallet_prelude::*;
use sp_std::prelude::*;
use traits_services::{
    ServicesProvider,
    ServiceInfo,
    types::{ Price, CurrencyType }
};
use traits_genetic_testing::{GeneticTestingProvider, DnaSampleTracking};
use traits_order::{OrderEventEmitter, OrderStatusUpdater};


#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub enum OrderStatus {
    Unpaid,
    Paid,
    Success,
    Refunded,
    Cancelled,
    Failed,
}
impl Default for OrderStatus {
    fn default() -> Self { OrderStatus::Unpaid }
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct Order<Hash, AccountId, Balance, Moment> {
    pub id: Hash,
    pub service_id: Hash,
    pub customer_id: AccountId,
    pub customer_box_public_key: Hash,
    pub seller_id: AccountId,
    pub dna_sample_tracking_id: Vec<u8>,
    pub currency: CurrencyType,
    pub prices: Vec<Price<Balance>>,
    pub additional_prices: Vec<Price<Balance>>,
    pub status: OrderStatus,
    pub created_at: Moment,
    pub updated_at: Moment,
}
impl<Hash, AccountId, Balance, Moment> Order<Hash, AccountId, Balance, Moment> {
    pub fn new(
        id: Hash,
        service_id: Hash,
        customer_id: AccountId,
        customer_box_public_key: Hash,
        seller_id: AccountId,
        dna_sample_tracking_id: Vec<u8>,
        currency: CurrencyType,
        prices: Vec<Price<Balance>>,
        additional_prices: Vec<Price<Balance>>,
        created_at: Moment,
        updated_at: Moment,
    )
        -> Self
    {
        Self {
            id,
            service_id,
            customer_id,
            customer_box_public_key,
            seller_id,
            dna_sample_tracking_id, 
            currency,
            prices,
            additional_prices,
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
        type Services: ServicesProvider<Self, BalanceOf<Self>>;
        type GeneticTesting: GeneticTestingProvider<Self>;
        type Currency: Currency<<Self as frame_system::Config>::AccountId>;
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
    pub type HashOf<T> = <T as frame_system::Config>::Hash;
    pub type CurrencyOf<T> = <T as self::Config>::Currency;
    pub type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;
    pub type OrderOf<T> = Order<HashOf<T>, AccountIdOf<T>, BalanceOf<T>, MomentOf<T>>;
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
    #[pallet::getter(fn last_order_by_customer_id)]
    pub type LastOrderByCustomer<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, HashOf<T>>;

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
        /// Order Success
        /// parameters, [Order]
        OrderSuccess(OrderOf<T>),
        /// Order Refunded
        /// parameters, [Order]
        OrderRefunded(OrderOf<T>),
        /// Order Cancelled
        /// parameters, [Order]
        OrderCancelled(OrderOf<T>),
        /// Order Not Found
        /// parameters, []
        OrderNotFound,
        /// Order Failed
        /// parameters, [Order]
        OrderFailed(OrderOf<T>),
    }
      

    #[pallet::error]
    pub enum Error<T> {
        /// Service id does not exist
        ServiceDoesNotExist,
        /// Order does not exist
        OrderNotFound,
        /// Unauthorized to fulfill order - user is not the seller who owns the service
        UnauthorizedOrderFulfillment,
        /// Unauthorized to cancel order - user is not the customer who created the order
        UnauthorizedOrderCancellation,
        /// Can not fulfill order before Specimen is processed
        DnaSampleNotSuccessfullyProcessed,
        /// Refund not allowed, Order is not expired yet
        OrderNotYetExpired,
        /// Unauthorized Account
        Unauthorized,
        /// Error on creating DNA sample
        DnaSampleInitalizationError,
        /// Customer eth address not found
        CustomerEthAddressNotFound,
        /// Seller eth address not found
        SellerEthAddressNotFound,
        /// Service Price Index not found
        PriceIndexNotFound,
    }


    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn create_order(origin: OriginFor<T>, service_id: T::Hash, price_index: u32, customer_box_public_key: T::Hash) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as OrderInterface<T>>::create_order(&who, &service_id, price_index, &customer_box_public_key) {
                Ok(order) => {
                    Self::deposit_event(Event::<T>::OrderCreated(order.clone()));
                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn cancel_order(origin: OriginFor<T>, order_id: T::Hash) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as OrderInterface<T>>::cancel_order(&who, &order_id) {
                Ok(order) => {
                    Self::deposit_event(Event::<T>::OrderCancelled(order.clone()));
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
                    Self::deposit_event(Event::<T>::OrderSuccess(order.clone()));
                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }
        
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn set_order_refunded(origin: OriginFor<T>, order_id: T::Hash) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as OrderInterface<T>>::set_order_refunded(&who, &order_id) {
                Ok(order) => {
                    Self::deposit_event(Event::<T>::OrderRefunded(order.clone()));
                    Ok(().into())
                },
                Err(error) => Err(error)?
            }
        }
    }
}

impl<T: Config> OrderInterface<T> for Pallet<T> {
    type Order = OrderOf<T>;
    type Error = Error<T>;

    fn create_order(customer_id: &T::AccountId, service_id: &T::Hash, price_index: u32, customer_box_public_key: &T::Hash) -> Result<Self::Order, Self::Error> {
        let service = T::Services::service_by_id(service_id);
        if service.is_none() {
            return Err(Error::<T>::ServiceDoesNotExist);
        }
        let service = service.unwrap();
        let order_id = Self::generate_order_id(customer_id, service_id);
        let seller_id = service.get_owner_id();
        let prices_by_currency = service.get_prices_by_currency();

        if prices_by_currency.len() == 0 || prices_by_currency.len() - 1 < price_index.try_into().unwrap() {
            return Err(Error::<T>::PriceIndexNotFound);
        }

        let price_by_currency = &prices_by_currency[price_index as usize];

        let currency = &price_by_currency.currency;
        let prices = &price_by_currency.price_components;
        let additional_prices = &price_by_currency.additional_prices;

        let now = pallet_timestamp::Pallet::<T>::get();

        // Initialize DnaSample
        let dna_sample = T::GeneticTesting::register_dna_sample(seller_id, customer_id, &order_id);
        if dna_sample.is_err() {
            return Err(Error::<T>::DnaSampleInitalizationError);
        }
        let dna_sample = dna_sample.ok().unwrap();

        let order = Order::new(
            order_id.clone(),
            service_id.clone(),
            customer_id.clone(),
            customer_box_public_key.clone(),
            seller_id.clone(),
            dna_sample.get_tracking_id().clone(),
            currency.clone(),
            prices.clone(),
            additional_prices.clone(),
            now,
            now
        );
        Self::insert_order_to_storage(&order);

        Ok(order)
    }

    fn cancel_order(customer_id: &T::AccountId, order_id: &T::Hash) -> Result<Self::Order, Self::Error> {
        let order = Orders::<T>::get(order_id);
        if order.is_none() {
            return Err(Error::<T>::OrderNotFound);
        }
        let order = order.unwrap();

        if order.customer_id != customer_id.clone() {
            return Err(Error::<T>::UnauthorizedOrderCancellation);
        }

        // Delete dna sample associated with the order
        let _dna_sample = T::GeneticTesting::delete_dna_sample(&order.dna_sample_tracking_id);

        let order = Self::update_order_status(order_id, OrderStatus::Cancelled).unwrap();

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

        let dna_sample = T::GeneticTesting::dna_sample_by_tracking_id(&order.dna_sample_tracking_id);
        if dna_sample.unwrap().process_success() == false {
            return Err(Error::<T>::DnaSampleNotSuccessfullyProcessed);
        }

        let order = Self::update_order_status(order_id, OrderStatus::Success);

        Ok(order.unwrap())
    }

    fn set_order_refunded(escrow_account_id: &T::AccountId, order_id: &T::Hash) -> Result<Self::Order, Self::Error> {
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
use frame_support::sp_std::convert::{TryInto};

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
        -> Option<Order<T::Hash, T::AccountId, BalanceOf<T>, T::Moment>>
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
        Orders::<T>::insert(&order.id, order);
        LastOrderByCustomer::<T>::insert(&order.customer_id, &order.id);
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

    pub fn remove_order_id_from_orders_by_seller(seller_id: &T::AccountId, order_id: &T::Hash) -> () {
        let mut orders = OrdersBySeller::<T>::get(seller_id).unwrap_or(Vec::new());
        orders.retain(|o_id| o_id != order_id);
        OrdersBySeller::<T>::insert(seller_id, orders);
    }

    pub fn remove_order_id_from_orders_by_customer(customer_id: &T::AccountId, order_id: &T::Hash) -> () {
        let mut orders = OrdersByCustomer::<T>::get(customer_id).unwrap_or(Vec::new());
        orders.retain(|o_id| o_id != order_id);
        OrdersByCustomer::<T>::insert(customer_id, orders);
    }

    pub fn order_can_be_refunded(order: OrderOf<T>) -> bool {
        let dna_sample = T::GeneticTesting::dna_sample_by_tracking_id(&order.dna_sample_tracking_id).unwrap();
        if !dna_sample.is_rejected() {
            return false;
        }
        true
    }
}

impl<T: Config> OrderEventEmitter<T> for Pallet<T> {
    fn emit_event_order_failed(order_id: &HashOf<T>) -> () {
        match Self::order_by_id(order_id) {
            None => Self::deposit_event(Event::OrderNotFound),
            Some(order) => Self::deposit_event(Event::OrderFailed(order))
        }
    }
}

impl<T: Config> OrderStatusUpdater<T> for Pallet<T> {
    fn update_status_failed(order_id: &HashOf<T>) -> () {
        match Self::order_by_id(order_id) {
            None => Self::deposit_event(Event::OrderNotFound),
            Some(order) => {
                Self::update_order_status(&order.id, OrderStatus::Failed);
            },
        }
    }
}
