#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use frame_support::traits::{ Randomness };
use frame_support::codec::{Encode, Decode};


#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResultWithPostInfo, pallet_prelude::*,
        sp_std::convert::{TryInto, TryFrom},
    };
    use frame_system::pallet_prelude::*;
    use sp_std::prelude::*;

    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + pallet_timestamp::Config
        + services::Config
        + escrow::Config
        + specimen::Config
    {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type RandomnessSource: crate::Randomness<Self::Hash>;
    }


    // ----- This is template code, every pallet needs this ---
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
    // --------------------------------------------------------

    

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
        pub lab_id: AccountId,
        pub escrow_id: AccountId,
        pub created_at: Moment,
        pub updated_at: Moment,
        pub status: OrderStatus,
    }
    impl<Hash, AccountId, Moment> Order<Hash, AccountId, Moment> {
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

    // ---- Types --------------------------------------------
    type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;
    type HashOf<T> = <T as frame_system::Config>::Hash;
    type OrderOf<T> = Order<HashOf<T>, AccountIdOf<T>, MomentOf<T>>;
    type OrderIdsOf<T> = Vec<HashOf<T>>;
    // -------------------------------------------------------

    // ------ Storage --------------------------
    #[pallet::storage]
    #[pallet::getter(fn order_by_id)]
    pub type Orders<T> = StorageMap<_, Blake2_128Concat, HashOf<T>, OrderOf<T>>;

    #[pallet::storage]
    #[pallet::getter(fn orders_by_costumer_id)]
    pub type CustomerOrders<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, OrderIdsOf<T>>;

    #[pallet::storage]
    #[pallet::getter(fn orders_by_lab_id)]
    pub type LabOrders<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, OrderIdsOf<T>>;
    // -----------------------------------------


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
        /// Lab id does not exist
        LabDoesNotExist,
        /// Service id does not exist
        ServiceDoesNotExist,
        /// Order does not exist
        OrderNotFound,
        /// Escrow not found
        EscrowNotFound,
        /// Unauthorized to fulfill order - user is not the lab who owns the service
        UnauthorizedOrderFulfillment,
        /// Can not fulfill order before Specimen is processed
        SpecimenNotProcessed,
        /// Refund not allowed, Order is not expired yet
        OrderNotYetExpired,
    }


    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn create_order(origin: OriginFor<T>, service_id: T::Hash) -> DispatchResultWithPostInfo {
            let customer_id = ensure_signed(origin)?;
            let service = services::Pallet::<T>::service_by_id(service_id);
            match service {
                None => Err(Error::<T>::ServiceDoesNotExist)?,
                Some(service) => {
                    let order_id = Self::generate_hash(&customer_id);
                    let service_id = service.get_id();
                    let lab_id = service.get_owner_id();
                    let created_at = pallet_timestamp::Pallet::<T>::get();


                    // Create escrow
                    let escrow_account_id = escrow::Pallet::<T>::create_escrow(
                        &order_id,
                        &customer_id, // buyer id
                        &lab_id, // seller id
                        service.get_price(), // amount to pay
                        &created_at,
                    );

                    // Create specimen
                    let _specimen = specimen::Pallet::<T>::create_specimen(
                        &order_id,
                        &service_id,
                        &customer_id, // specimen owner id
                        &lab_id,
                        &created_at,
                    );

                    // Create order
                    let order = Order {
                        id: order_id,
                        customer_id: customer_id.clone(),
                        service_id: *service_id,
                        lab_id: lab_id.clone(),
                        escrow_id: escrow_account_id.clone(),
                        created_at: created_at,
                        updated_at: created_at,
                        status: OrderStatus::Unpaid,
                    };
                    Orders::<T>::insert(&order_id, &order);

                    // Store order id reference for Customers
                    let orders = CustomerOrders::<T>::get(&customer_id);
                    if orders == None {
                        let mut orders = Vec::<T::Hash>::new();
                        orders.push(order_id);
                        CustomerOrders::<T>::insert(&customer_id, orders);
                    } else {
                        let mut orders = orders.unwrap();
                        orders.push(order_id);
                        CustomerOrders::<T>::insert(&customer_id, orders);
                    }
                    let orders = CustomerOrders::<T>::get(&customer_id);
                    //debug::info!("** ---- CustomerOrders ---- **: {:?}", orders);


                    // Store order id reference for Labs
                    let orders = LabOrders::<T>::get(&lab_id);
                    if orders == None {
                        let mut orders = Vec::<T::Hash>::new();
                        orders.push(order_id);
                        LabOrders::<T>::insert(&lab_id, orders);
                    } else {
                        let mut orders = orders.unwrap();
                        orders.push(order_id);
                        LabOrders::<T>::insert(&lab_id, orders);
                    }
                    let orders = LabOrders::<T>::get(&customer_id);
                    //debug::info!("** ---- LabOrders ---- **: {:?}", orders);

                    Self::deposit_event(Event::OrderCreated(order));
                    Ok(().into())
                }
            }
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn pay_order(origin: OriginFor<T>, order_id: T::Hash) -> DispatchResultWithPostInfo {
            let customer_id = ensure_signed(origin)?;
            let order = Orders::<T>::get(&order_id);
            
            if order == None {
                return Err(Error::<T>::OrderNotFound)?;
            }

            // Pay to escrow
            let _escrow = escrow::Pallet::<T>::deposit(&order_id, &customer_id);

            // Set order status to paid
            let order = Self::update_order_status(&order_id, OrderStatus::Paid);
            let order = order.unwrap();

            Self::deposit_event(Event::OrderPaid(order.clone()));
            Ok(().into())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn fulfill_order(origin: OriginFor<T>, order_id: T::Hash) -> DispatchResultWithPostInfo {
            let user_id = ensure_signed(origin)?;
            let order = Orders::<T>::get(&order_id);
            if order == None {
                return Err(Error::<T>::OrderNotFound)?;
            }
            let order = order.unwrap();

            // Only the lab who owns the service in the order can fulfill
            let is_lab = user_id == order.lab_id;
            if !is_lab {
                return Err(Error::<T>::UnauthorizedOrderFulfillment)?;
            }

            // Specimen has to be processed before order is fulfilled
            let is_specimen_processed = specimen::Pallet::<T>::is_status(
                &order_id,
                specimen::SpecimenStatus::Processed
            );
            if !is_specimen_processed {
                return Err(Error::<T>::SpecimenNotProcessed)?;
            }

            let order = Self::update_order_status(&order.id, OrderStatus::Fulfilled);
            let order = order.unwrap();

            // Release funds to lab
            escrow::Pallet::<T>::release(&order.id);

            Self::deposit_event(Event::OrderFulfilled(order.clone()));
            Ok(().into())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn refund_order(origin: OriginFor<T>, order_id: T::Hash) -> DispatchResultWithPostInfo {
            let _user_id = ensure_signed(origin)?;
            let order = Orders::<T>::get(&order_id);
            if order == None {
                return Err(Error::<T>::OrderNotFound)?;
            }
            let order = order.unwrap();

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

            // Refund back to customer
            escrow::Pallet::<T>::refund(&order_id);

            let order = Self::update_order_status(&order_id, OrderStatus::Refunded);
            let order = order.unwrap();

            Self::deposit_event(Event::OrderRefunded(order.clone()));
            Ok(().into())
        }
    }
}

impl<T: Config> Pallet<T> {
    // TODO: Maybe extract this fn as a separate module (this is used by pallet services also)
    fn generate_hash(account_id: &T::AccountId) -> T::Hash {
        let account_info = frame_system::Pallet::<T>::account(account_id);
        let hash = <T as Config>::RandomnessSource::random(&account_info.nonce.encode());
        // let hash = <T as Config>::Hashing::hash(random_result);
        return hash;
    }

    fn update_order_status(order_id: &T::Hash, status: pallet::OrderStatus)
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
}

/*
// TODO: Is it possible to trigger this from escrow pallet
// when the escrow account is paid by straight transfer not
// dispatchable calls??
impl<T: Trait> EscrowController<T> for Module<T> {
    fn on_escrow_paid(controller_id: &T::Hash) -> () {
        let order_id = controller_id;
        Orders::<T>::mutate(order_id, |order| {
            match order {
                None => (),
                Some(order) => {
                    order.status = OrderStatus::Paid;
                }
            }
        })
    }
}
*/
