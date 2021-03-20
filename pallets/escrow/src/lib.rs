#![cfg_attr(not(feature = "std"), no_std)]

/*
 --------------------
 Pallet Escrow ------
 --------------------
*/

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResultWithPostInfo, pallet_prelude::*,
        traits::{ Currency, ExistenceRequirement, ReservableCurrency }, 
    };
    use frame_system::pallet_prelude::*;
    use sp_std::prelude::*;
    use frame_support::codec::{Encode, Decode};

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config + services::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: ReservableCurrency<<Self as frame_system::Config>::AccountId>;
    }

    // Mandatory ------------------------------------
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
    // ----------------------------------------------
    
    #[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
    pub struct Escrow<AccountId, Hash, Balance, Moment> {
        pub account_id: AccountId,
        pub order_id: Hash,
        pub buyer_id: AccountId,
        pub seller_id: AccountId,
        pub amount_to_pay: Balance,
        pub amount_paid: Balance,
        pub expires_at: Moment,
    }

    impl<AccountId, Hash, Balance, Moment> Escrow<AccountId, Hash, Balance, Moment> {
        pub fn get_account_id(&self) -> &AccountId {
            &self.account_id
        }

        pub fn set_amount_paid(&mut self, amount: Balance) -> () {
            self.amount_paid = amount;
        }
    }

    // Types ---------------
    pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    pub type HashOf<T> = <T as frame_system::Config>::Hash;
    pub type CurrencyOf<T> = <T as services::Config>::Currency;
    pub type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;
    pub type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;
    pub type EscrowOf<T> = Escrow<AccountIdOf<T>, HashOf<T>, BalanceOf<T>, MomentOf<T>>;
    // ---------------------

    #[pallet::storage]
    #[pallet::getter(fn escrow_by_order_id)]
    pub type Escrows<T> = StorageMap<_, Blake2_128Concat, HashOf<T>, EscrowOf<T>>;


    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Escrow Created
        /// Parameters [Escrow]
        EscrowCreated(Escrow<AccountIdOf<T>, HashOf<T>, BalanceOf<T>, MomentOf<T>>),
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {

    }
}

use frame_support::traits::{ Currency, ExistenceRequirement };
use frame_support::sp_std::convert::{TryInto, TryFrom};
use frame_support::sp_runtime::{
    ModuleId,
    traits::{ AccountIdConversion, Zero },
};

const PALLET_ID: ModuleId = ModuleId(*b"dbescrow"); // Has to be 8 characters
impl<T: Config> Pallet<T> {

    pub fn generate_escrow_account_id(order_id: &T::Hash) -> T::AccountId {
        PALLET_ID.into_sub_account(&order_id)
    }
    
    pub fn create_escrow(
        order_id: &T::Hash,
        buyer_id: &T::AccountId,
        seller_id: &T::AccountId,
        amount_to_pay: &BalanceOf<T>,
        order_created_at: &MomentOf<T>,
    )
        -> T::AccountId
    {
        // Calculate escrow expiry
        // FIXME: Move calculating escrow expires_at to Order pallet
        let order_created_at = order_created_at.clone();
        let order_created_at_ms = TryInto::<u64>::try_into(order_created_at).ok().unwrap();
        let seven_days_ms = u64::try_from(chrono::Duration::days(7).num_milliseconds()).ok().unwrap();
        let expires_at_ms = order_created_at_ms + seven_days_ms;
        let expires_at = TryInto::<MomentOf<T>>::try_into(expires_at_ms).ok().unwrap();

        // Generate escrow's account_id
        let escrow_account_id = Self::generate_escrow_account_id(order_id);

        let escrow = Escrow {
            account_id: escrow_account_id.clone(),
            order_id: order_id.clone(),
            buyer_id: buyer_id.clone(),
            seller_id: seller_id.clone(),
            amount_to_pay: amount_to_pay.clone(),
            amount_paid: Zero::zero(),
            expires_at: expires_at,
        };

        Escrows::<T>::insert(order_id, &escrow);

        escrow_account_id
    }

    pub fn deposit(order_id: &T::Hash, depositor_account_id: &T::AccountId)
        -> Option<EscrowOf<T>>
    {
        Escrows::<T>::mutate(order_id, | escrow | {
            match escrow {
                None => None,
                Some(escrow) => {
                    // TODO: Handle transfer error
                    let _result = <T as services::Config>::Currency::transfer(
                        depositor_account_id,
                        &escrow.account_id,
                        escrow.amount_to_pay,
                        ExistenceRequirement::KeepAlive
                    );
                    escrow.set_amount_paid(escrow.amount_to_pay);
                    Some(escrow.clone())
                }
            }
        })
    }

    pub fn release(order_id: &T::Hash) -> () {
        let escrow = Escrows::<T>::get(order_id).unwrap(); // FIXME: handle escrow not found
        // let escrow_account_info = frame_system::Module::<T>::account(&escrow.account_id);
        
        // TODO: Handle transfer error
        let _result = <T as services::Config>::Currency::transfer(
            &escrow.account_id,
            &escrow.seller_id,
            escrow.amount_paid,
            ExistenceRequirement::AllowDeath,
        );
        // TODO: remove escrow struct in storage
    }

    pub fn refund(order_id: &T::Hash) -> () {
        let escrow = Escrows::<T>::get(order_id).unwrap(); // FIXME handle escrow not found

        // TODO: Handle transfer error
        let _result = <T as services::Config>::Currency::transfer(
            &escrow.account_id,
            &escrow.buyer_id,
            escrow.amount_paid,
            ExistenceRequirement::AllowDeath,
        );
        // TODO: remove escrow struct in storage
    }
}
