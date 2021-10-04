#![cfg_attr(not(feature = "std"), no_std)]

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResultWithPostInfo,
        pallet_prelude::*,
        traits::{Currency, Imbalance, OnUnbalanced, ReservableCurrency},
    };
    use frame_system::pallet_prelude::*;

    // balance type using reservable currency type
    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    type PositiveImbalanceOf<T> = <<T as Config>::Currency as Currency<
        <T as frame_system::Config>::AccountId,
    >>::PositiveImbalance;
    type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
        <T as frame_system::Config>::AccountId,
    >>::NegativeImbalance;

    #[pallet::config]
    pub trait Config: frame_system::Config + Sized {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Currency type for this pallet.
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

        /// Handler for the unbalanced increment when rewarding (minting rewards)
        type Reward: OnUnbalanced<PositiveImbalanceOf<Self>>;

        /// Handler for the unbalanced decrement when slashing (burning collateral)
        type Slash: OnUnbalanced<NegativeImbalanceOf<Self>>;
    }

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        SlashFunds(T::AccountId, BalanceOf<T>, T::BlockNumber),
        RewardFunds(T::AccountId, BalanceOf<T>, T::BlockNumber),
        CreateAccountRewarded(T::AccountId, BalanceOf<T>, T::BlockNumber),
    }

    #[pallet::error]
    pub enum Error<T> {
        ServiceNotPurchased,
		NotAStaker,
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Pallet run from this pallet::call
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(20_000 + T::DbWeight::get().writes(1))]
        pub fn slash_funds(
            origin: OriginFor<T>,
            to_punish: T::AccountId,
            collateral: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;

            let imbalance = T::Currency::slash_reserved(&to_punish, collateral).0;
            T::Slash::on_unbalanced(imbalance);

            let now = <frame_system::Pallet<T>>::block_number();
            Self::deposit_event(Event::SlashFunds(to_punish, collateral, now));
            Ok(().into())
        }

        #[pallet::weight(20_000 + T::DbWeight::get().writes(1))]
        pub fn reward_funds(
            origin: OriginFor<T>,
            to_reward: T::AccountId,
            reward: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;

            let mut total_imbalance = <PositiveImbalanceOf<T>>::zero();

            let r = T::Currency::deposit_into_existing(&to_reward, reward).ok();
            total_imbalance.maybe_subsume(r);
            T::Reward::on_unbalanced(total_imbalance);

            let now = <frame_system::Pallet<T>>::block_number();
            Self::deposit_event(Event::RewardFunds(to_reward, reward, now));
            Ok(().into())
        }

		#[pallet::weight(20_000 + T::DbWeight::get().writes(1))]
        fn create_account_reward(
            origin: OriginFor<T>,
            to_reward: T::AccountId,
            create_account_reward: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;
            let mut total_imbalance = <PositiveImbalanceOf<T>>::zero();

            let r = T::Currency::deposit_into_existing(&to_reward, create_account_reward).ok();
            total_imbalance.maybe_subsume(r);
            T::Reward::on_unbalanced(total_imbalance);

            let now = <frame_system::Pallet<T>>::block_number();
            Self::deposit_event(Event::CreateAccountRewarded(
                to_reward,
                create_account_reward,
                now,
            ));
            Ok(().into())
        }

        #[pallet::weight(20_000 + T::DbWeight::get().writes(1))]
        fn service_purchased_reward(
            origin: OriginFor<T>,
            to_reward: T::AccountId,
            service_purchased_reward: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;
            let mut total_imbalance = <PositiveImbalanceOf<T>>::zero();

            let r = T::Currency::deposit_into_existing(&to_reward, service_purchased_reward).ok();
            total_imbalance.maybe_subsume(r);
            T::Reward::on_unbalanced(total_imbalance);

            let service_purchased = <frame_system::Pallet<T>>::block_number();

            Self::deposit_event(Event::CreateAccountRewarded(
                to_reward,
                service_purchased_reward,
                service_purchased,
            ));
            Ok(().into())
        }

		#[pallet::weight(20_000 + T::DbWeight::get().writes(1))]
        fn service_staking_reward(
            origin: OriginFor<T>,
			staker: T::AccountId,
            to_reward: T::AccountId,
            service_staking_reward: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;
			ensure!(staker == to_reward, Error::<T>::NotAStaker);
            let mut total_imbalance = <PositiveImbalanceOf<T>>::zero();

            let r = T::Currency::deposit_into_existing(&to_reward, service_staking_reward).ok();
            total_imbalance.maybe_subsume(r);
            T::Reward::on_unbalanced(total_imbalance);

            let service_staked = <frame_system::Pallet<T>>::block_number();

            Self::deposit_event(Event::CreateAccountRewarded(
                to_reward,
                service_staking_reward,
                service_staked,
            ));
            Ok(().into())
        }
    }
}
