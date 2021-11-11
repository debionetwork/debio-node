#![cfg_attr(not(feature = "std"), no_std)]

pub mod interface;
use interface::RewardInterface;

use frame_support::pallet_prelude::*;
use frame_support::{
    traits::{Currency, ExistenceRequirement::KeepAlive, OnUnbalanced, ReservableCurrency},
};

pub use pallet::*;
pub use scale_info::TypeInfo;

#[cfg(test)]
mod mock;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use sp_std::prelude::*;

#[frame_support::pallet]
pub mod pallet {
    use crate::*;
    use frame_support::{dispatch::DispatchResultWithPostInfo};
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config + Sized {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Currency type for this pallet.
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
    }
    // -----------------------------------------

    // ---- Types --------------------------------------------
    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    // -------------------------------------------------------

    // ------ Storage --------------------------
    #[pallet::storage]
    #[pallet::getter(fn admin_key)]
    pub type RewarderKey<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;
    // -----------------------------------------

    // ----- Genesis Configs ------------------
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub rewarder_key: T::AccountId,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                rewarder_key: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            RewarderKey::<T>::put(&self.rewarder_key);
        }
    }
    // ----------------------------------------

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        RewardFunds(T::AccountId, BalanceOf<T>, T::BlockNumber),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Unauthorized Account
        Unauthorized,
        /// Account doesn't exist
        NotExist,
    }

    // ----- This is template code, every pallet needs this ---
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
    // --------------------------------------------------------

    // Pallet run from this pallet::call
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(20_000 + T::DbWeight::get().writes(1))]
        pub fn reward_funds(
            origin: OriginFor<T>,
            to_reward: T::AccountId,
            reward: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            match <Self as RewardInterface<T>>::reward_funds(&who, &to_reward, reward) {
                Ok(_) => {
                    let now = <frame_system::Pallet<T>>::block_number();
                    Self::deposit_event(Event::<T>::RewardFunds(to_reward, reward, now));
                    Ok(().into())
                }
                Err(error) => Err(error)?,
            }
        }
    }
}

impl<T: Config> RewardInterface<T> for Pallet<T> {
    type Error = Error<T>;
    type Balance = BalanceOf<T>;

    fn reward_funds(
        rewarder_account_id: &T::AccountId,
        to_reward: &T::AccountId,
        reward: Self::Balance,
    ) -> Result<(), Self::Error> {
        if rewarder_account_id.clone() != RewarderKey::<T>::get() {
            return Err(Error::<T>::Unauthorized);
        }

        let _ = T::Currency::transfer(rewarder_account_id, to_reward, reward, KeepAlive);

        Ok(().into())
    }
}
