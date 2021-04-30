#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
pub use pallet::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub mod interface;
pub use crate::interface::UserProfileInterface;
use frame_support::pallet_prelude::*;
/*
#[cfg(feature = "std")]
pub use serde::{self, Serialize, Deserialize, Serializer, Deserializer};
*/

/// An Ethereum address (i.e. 20 bytes, used to represent an Ethereum account).
///
/// This gets serialized to the 0x-prefixed hex representation.
#[derive(Clone, Copy, PartialEq, Eq, Encode, Decode, Default, RuntimeDebug)]
pub struct EthereumAddress([u8; 20]);

/*
#[cfg(feature = "std")]
impl Serialize for EthereumAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let hex: String = rustc_hex::ToHex::to_hex(&self.0[..]);
        serializer.serialize_str(&format!("0x{}", hex))
    }
}
#[cfg(feature = "std")]
impl<'de> Deserialize<'de> for EthereumAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let base_string = String::deserialize(deserializer)?;
        let offset = if base_string.starts_with("0x") { 2 } else { 0 };
        let s = &base_string[offset..];
        if s.len() != 40 {
                Err(serde::de::Error::custom("Bad length of Ethereum address (should be 42 including '0x')"))?;
        }
        let raw: Vec<u8> = rustc_hex::FromHex::from_hex(s)
                .map_err(|e| serde::de::Error::custom(format!("{:?}", e)))?;
        let mut r = Self::default();
        r.0.copy_from_slice(&raw);
        Ok(r)
    }
}
*/



#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResultWithPostInfo, pallet_prelude::*,
    };
    use frame_system::pallet_prelude::*;
    pub use sp_std::prelude::*;
    use crate::*;


    #[pallet::config]
    /// Configure the pallet by specifying the parameters and types on which it depends.
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    // ----- This is template code, every pallet needs this ---
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
    // --------------------------------------------------------

    
    // ---- Types ----------------------
    pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

    // ----- Storage ------------------
    #[pallet::storage]
    #[pallet::getter(fn usdt_address_by_account_id)]
    pub type UsdtAddresses<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, EthereumAddress>;
    // -----------------------------------


    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId", LabOf<T> = "Lab")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// User AccountId registered as lab
        /// parameters. [Lab, who]
        UsdtAddressSet(EthereumAddress, AccountIdOf<T>),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        Error,
    }


    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn set_usdt_address(origin: OriginFor<T>, usdt_address: EthereumAddress) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            <Self as UserProfileInterface<T>>::set_usdt_address_by_account_id(&who, &usdt_address);

            Self::deposit_event(Event::<T>::UsdtAddressSet(usdt_address, who));

            Ok(().into())
        }
    }
}

impl<T: Config> UserProfileInterface<T> for Pallet<T> {
    type UsdtAddress = EthereumAddress;

    fn get_usdt_address_by_account_id(account_id: &T::AccountId) -> Option<Self::UsdtAddress> {
        UsdtAddresses::<T>::get(account_id)
    }

    fn set_usdt_address_by_account_id(account_id: &T::AccountId, usdt_address: &Self::UsdtAddress) -> () {
        UsdtAddresses::<T>::insert(account_id, usdt_address)
    }
}
