#![cfg_attr(not(feature = "std"), no_std)]

pub mod interface;
use interface::RewardInterface;

use frame_support::{
	pallet_prelude::*,
	traits::{Currency, ExistenceRequirement, WithdrawReasons},
};

pub use pallet::*;
pub use scale_info::TypeInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use sp_std::prelude::*;

pub mod weights;
pub use weights::WeightInfo;

use frame_support::PalletId;
use sp_runtime::traits::AccountIdConversion;

#[frame_support::pallet]
pub mod pallet {
	use crate::*;
	use frame_support::dispatch::DispatchResultWithPostInfo;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + Sized {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// Currency type for this pallet.
		type PalletId: Get<PalletId>;
		type Currency: Currency<<Self as frame_system::Config>::AccountId>;
		type WeightInfo: WeightInfo;
	}
	// -----------------------------------------

	// ---- Types --------------------------------------------
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type CurrencyOf<T> = <T as self::Config>::Currency;
	pub type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;
	// -------------------------------------------------------

	// ------ Storage --------------------------
	#[pallet::storage]
	#[pallet::getter(fn admin_key)]
	pub type RewarderKey<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn pallet_id)]
	pub type PalletAccount<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn total_reward_amount)]
	pub type TotalRewardAmount<T> = StorageValue<_, BalanceOf<T>>;
	// -----------------------------------------

	// ----- Genesis Configs ------------------
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub rewarder_key: T::AccountId,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { rewarder_key: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			RewarderKey::<T>::put(&self.rewarder_key);
			PalletAccount::<T>::put(<Pallet<T>>::get_pallet_id());
			<Pallet<T>>::set_total_reward_amount();
		}
	}
	// ----------------------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		RewardFunds(T::AccountId, BalanceOf<T>, T::BlockNumber),
		UpdateRewardsAdminKeySuccessful(AccountIdOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Insufficient funds.
		InsufficientFunds,
		/// Amount overflow.
		AmountOverflow,
		/// Unauthorized Account
		Unauthorized,
		/// Account doesn't exist
		NotExist,
		BadSignature,
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
		#[pallet::weight(T::WeightInfo::reward_funds())]
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
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(0)]
		pub fn sudo_update_admin_key(
			origin: OriginFor<T>,
			account_id: T::AccountId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			RewarderKey::<T>::put(&account_id);

			Self::deposit_event(Event::UpdateRewardsAdminKeySuccessful(account_id));

			Ok(Pays::No.into())
		}

		#[pallet::weight(T::WeightInfo::update_admin_key())]
		pub fn update_admin_key(
			origin: OriginFor<T>,
			account_id: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as RewardInterface<T>>::update_admin_key(&who, &account_id) {
				Ok(_) => {
					Self::deposit_event(Event::UpdateRewardsAdminKeySuccessful(who.clone()));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}
	}
}

impl<T: Config> Pallet<T> {
	/// The injected pallet ID
	pub fn get_pallet_id() -> AccountIdOf<T> {
		T::PalletId::get().into_account()
	}

	/// The account ID that holds the funds
	pub fn account_id() -> AccountIdOf<T> {
		<PalletAccount<T>>::get()
	}

	/// Set current total reward amount
	pub fn set_total_reward_amount() {
		let balance = T::Currency::free_balance(&Self::account_id());
		TotalRewardAmount::<T>::put(balance);
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
		let pallet_id = Self::account_id();
		if rewarder_account_id.clone() != RewarderKey::<T>::get() {
			return Err(Error::<T>::Unauthorized)
		}

		match CurrencyOf::<T>::withdraw(
			&pallet_id,
			reward,
			WithdrawReasons::TRANSFER,
			ExistenceRequirement::KeepAlive,
		) {
			Ok(imb) => {
				CurrencyOf::<T>::resolve_creating(to_reward, imb);
				Self::set_total_reward_amount();
			},
			_ => return Err(Error::<T>::BadSignature),
		}

		Ok(())
	}

	fn update_admin_key(
		account_id: &T::AccountId,
		admin_key: &T::AccountId,
	) -> Result<(), Self::Error> {
		if account_id.clone() != RewarderKey::<T>::get() {
			return Err(Error::<T>::Unauthorized)
		}

		RewarderKey::<T>::put(admin_key);

		Ok(())
	}
}
