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
    amount_to_pay: Balance,
    amount_paid: Balance,
    expires_at: Moment,
}

impl<AccountId, Hash, Balance, Moment> Escrow<AccountId, Hash, Balance, Moment> {
    pub fn get_account_id(&self) -> &AccountId {
        &self.account_id
    } 
}

type AccountIdOf<T> = <T as frame_system::Trait>::AccountId;
type HashOf<T> = <T as frame_system::Trait>::Hash;
type BalanceOf<T> = <<T as services::Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;
type MomentOf<T> = <T as pallet_timestamp::Trait>::Moment;
type EscrowOf<T> = Escrow<AccountIdOf<T>, HashOf<T>, BalanceOf<T>, MomentOf<T>>;

decl_storage! {
    trait Store for Module<T: Trait> as EscrowStorage {
        Escrows get(fn escrow_by_order_id):
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
        order_created_at: &MomentOf<T>,
        amount_to_pay: &BalanceOf<T>,
    )
        -> T::AccountId
    {
        // Calculate escrow expiry
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
            amount_to_pay: amount_to_pay.clone(),
            amount_paid: Zero::zero(),
            expires_at: expires_at,
        };

        Escrows::<T>::insert(order_id, &escrow);

        escrow_account_id
    }
}
