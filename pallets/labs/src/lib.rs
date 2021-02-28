#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{
    decl_module, decl_storage, decl_event, decl_error,
    dispatch, traits::Get, traits::Randomness
};
use frame_system::ensure_signed;
use frame_support::codec::{Encode, Decode};
use frame_support::sp_runtime::{RuntimeDebug, traits::Hash};
use frame_support::sp_std::prelude::*;
use sp_core::H256;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
        type RandomnessSource: Randomness<H256>;
        type Hashing: Hash<Output = H256>;
}

/*
type Hash = sp_core::H256;
type ServiceId = Hash;
*/

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct Service<AccountId> {
    id: H256,
    lab_id: AccountId,
    name: Vec<u8>,
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct Lab<AccountId> {
    address: AccountId,
    name: Vec<u8>,
    services: Vec<H256>,
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as LabsStorage {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
                Labs get(fn lab_by_account_id):
                    map hasher(blake2_128_concat) T::AccountId
                        => Option<Lab<T::AccountId>>;

                Services get(fn service_by_uuid):
                    map hasher(blake2_128_concat) H256
                        => Option<Service<T::AccountId>>;

                Nonce get(fn nonce): u32;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where
            AccountId = <T as frame_system::Trait>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
                /// parameters. [Lab, who]
                LabRegistered(Lab<AccountId>, AccountId),
                /// parameters, [Service, who]
                ServiceCreated(Service<AccountId>, AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
                /// Account already has lab registered
                LabAlreadyRegistered,
                /// Lab identified by the AccountId does not exist
                LabDoesNotExist,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

                #[weight = 10_000 + T::DbWeight::get().writes(1)]
                pub fn register_lab(origin, lab_name: Vec<u8>) -> dispatch::DispatchResult {
                    let who = ensure_signed(origin)?;

                    let lab_exists = <Labs<T>>::contains_key(&who);
                    if lab_exists {
                        return Err(Error::<T>::LabAlreadyRegistered)?;
                    }

                    let services: Vec<H256> = Vec::new();
                    let lab = Lab {
                        address: who.clone(),
                        name: lab_name,
                        services: services,
                    };
                    <Labs<T>>::insert(&who, &lab);

                    Self::deposit_event(RawEvent::LabRegistered(lab, who.clone()));

                    Ok(())
                }

                #[weight = 10_000 + T::DbWeight::get().writes(1)]
                pub fn add_service(origin, service_name: Vec<u8>) -> dispatch::DispatchResult {
                    let who = ensure_signed(origin)?;
                    // Check if lab exists
                    let lab_exists = <Labs<T>>::contains_key(&who);
                    if !lab_exists {
                        return Err(Error::<T>::LabDoesNotExist)?;
                    }

                    // service_id is a random hash
                    let nonce = Self::encode_and_update_nonce();
                    let service_id = <T as Trait>::Hashing::hash(&nonce);
                    // create a service
                    let service = Service {
                        id: service_id,
                        lab_id: who.clone(),
                        name: service_name
                    };
                    // Insert service to storage map
                    <Services<T>>::insert(&service_id, &service);
                    // Add service id to lab
                    <Labs<T>>::mutate(&who, | lab | {
                        match lab {
                            None => (), // If lab does not exist, do nothing
                            Some(_lab) => {
                                _lab.services.push(service_id);
                            }
                        }
                    });

                    Self::deposit_event(RawEvent::ServiceCreated(service, who.clone()));

                    Ok(())
                }
	}
}

impl<T: Trait> Module<T> {
	/// Reads the nonce from storage, increments the stored nonce, and returns
	/// the encoded nonce to the caller.
	fn encode_and_update_nonce() -> Vec<u8> {
		let nonce = Nonce::get();
		Nonce::put(nonce.wrapping_add(1));
		nonce.encode()
	}
}
