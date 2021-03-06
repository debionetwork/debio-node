#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Trait;

pub trait EscrowController<T: Trait> {
    // Callback to the escrow's controller
    fn on_escrow_paid(controller_id: &T::Hash) -> ();
}
