#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_module, decl_storage, decl_event, decl_error,
    dispatch, debug,
    traits::{
        Get, Currency, ExistenceRequirement, ReservableCurrency, 
    }, 
};
use frame_system::ensure_signed;
use frame_support::codec::{Encode, Decode};
use frame_support::sp_runtime::{
    RuntimeDebug, ModuleId,
    traits::{Hash, AccountIdConversion, Saturating, Zero},
};
use frame_support::sp_std::prelude::*;
use frame_support::sp_std::convert::{TryInto, TryFrom};
use escrow_controller::EscrowController;

pub trait Trait: frame_system::Trait + services::Trait + pallet_timestamp::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type Currency: ReservableCurrency<Self::AccountId>;
    type Controller: EscrowController<Self>;
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct Escrow<AccountId, Hash, Balance, Moment> {
    account_id: AccountId,
    order_id: Hash,
    buyer_id: AccountId,
    seller_id: AccountId,
    amount_to_pay: Balance,
    amount_paid: Balance,
    expires_at: Moment,
}

impl<AccountId, Hash, Balance, Moment> Escrow<AccountId, Hash, Balance, Moment> {
    pub fn get_account_id(&self) -> &AccountId {
        &self.account_id
    }

    pub fn set_amount_paid(&mut self, amount: Balance) -> () {
        self.amount_paid = amount;
    }
}

type AccountIdOf<T> = <T as frame_system::Trait>::AccountId;
type HashOf<T> = <T as frame_system::Trait>::Hash;
type BalanceOf<T> = <<T as services::Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;
type MomentOf<T> = <T as pallet_timestamp::Trait>::Moment;
type EscrowOf<T> = Escrow<AccountIdOf<T>, HashOf<T>, BalanceOf<T>, MomentOf<T>>;

decl_storage! {
    trait Store for Module<T: Trait> as EscrowStorage {
        pub Escrows get(fn escrow_by_order_id):
            map hasher(blake2_128_concat) T::Hash => Option<EscrowOf<T>>;
    }
}

decl_event! {
    pub enum Event<T> where
        AccountId = AccountIdOf<T>,
        Hash = HashOf<T>,
        Balance = BalanceOf<T>,
        Moment = MomentOf<T>,
    {
        /// Escrow Created
        /// Parameters [Escrow]
        EscrowCreated(Escrow<AccountId, Hash, Balance, Moment>),
    }
}


decl_error! {
    pub enum Error for Module<T: Trait> {

    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;
    }
}

const PALLET_ID: ModuleId = ModuleId(*b"dbescrow"); // Has to be 8 characters
impl<T: Trait> Module<T> {
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
                    <T as services::Trait>::Currency::transfer(
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
        <T as services::Trait>::Currency::transfer(
            &escrow.account_id,
            &escrow.seller_id,
            escrow.amount_paid,
            ExistenceRequirement::AllowDeath,
        );
        // TODO: remove escrow struct in storage
    }

    pub fn refund(order_id: &T::Hash) -> () {
        let escrow = Escrows::<T>::get(order_id).unwrap(); // FIXME handle escrow not found
        <T as services::Trait>::Currency::transfer(
            &escrow.account_id,
            &escrow.buyer_id,
            escrow.amount_paid,
            ExistenceRequirement::AllowDeath,
        );
        // TODO: remove escrow struct in storage
    }
}
