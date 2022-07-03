#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
pub use pallet::*;
pub use weights::WeightInfo;

pub mod interface;
pub use crate::interface::RegistrationInterface;
// use frame_support::pallet_prelude::*;
pub use codec::EncodeLike;
pub use scale_info::TypeInfo;
use traits_registration::RegistrationProvider;

/// An Ethereum address (i.e. 20 bytes, used to represent an Ethereum account).
///
/// This gets serialized to the 0x-prefixed hex representation.
// #[derive(Clone, Copy, PartialEq, Eq, Encode, Decode, Default, RuntimeDebug)]
// pub struct EthereumAddress([u8; 20]);

#[frame_support::pallet]
pub mod pallet {
	use crate::*;
	pub use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	pub use frame_system::pallet_prelude::*;
	pub use sp_std::prelude::*;

	#[pallet::config]
	/// Configure the pallet by specifying the parameters and types on which it depends.
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type EthereumAddress: Clone
			+ Copy
			+ PartialEq
			+ Eq
			+ Encode
			+ EncodeLike
			+ Decode
			+ Default
			+ TypeInfo
			+ sp_std::fmt::Debug;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub admin_key: T::AccountId,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { admin_key: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			AdminKey::<T>::put(&self.admin_key);
		}
	}

	// ----- This is template code, every pallet needs this ---
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
	// --------------------------------------------------------

	// ---- Types ----------------------
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

	// ----- Storage ------------------
	#[pallet::storage]
	#[pallet::getter(fn admin_key)]
	pub type AdminKey<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn eth_address_by_account_id)]
	pub type GetAccountByAccountId<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// User AccountId registered as lab
		/// parameters. [Lab, who]
		RegisteredAccountId(AccountIdOf<T>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		Unauthorized,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::register_account())]
		pub fn register_account(
			origin: OriginFor<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			<Self as RegistrationInterface<T>>::register_account(
				&who,
			);

			Self::deposit_event(Event::<T>::RegisteredAccountId(who));

			Ok(().into())
		}
	}
}

impl<T: Config> RegistrationInterface<T, AccountIdOf<T>> for Pallet<T> {
	type Error = Error<T>;

	fn register_account(
		account_id: &T::AccountId,
	) {
		GetAccountByAccountId::<T>::insert(account_id);
	}

	fn get_account_by_id(account_id: &T::AccountId) -> Option<AccountIdOf<T>> {
		GetAccountByAccountId::<T>::get(account_id)
	}
}

impl<T: Config> RegistrationProvider<T, AccountIdOf<T>> for Pallet<T> {
	fn register_account(account_id: &T::AccountId) -> Option<AccountIdOf<T>> {
		<Self as RegistrationInterface<T, AccountIdOf<T>>>::register_account(
			account_id,
		)
	}
}
