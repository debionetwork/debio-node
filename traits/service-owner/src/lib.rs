#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Trait;

pub trait ServiceOwner<T: Trait> {
    fn associate(owner_id: &T::AccountId, service_id: &T::Hash) -> ();

    fn disassociate(owner_id: &T::AccountId, service_id: &T::Hash) -> ();

    fn is_owner(owner_id: &T::AccountId, service_id: &T::Hash) -> bool;

    fn can_create_service(owner_id: &T::AccountId) -> bool;
}
