#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;

pub trait OrderEventEmitter<T: Config> {
	fn emit_event_order_failed(order_id: &T::Hash);
}

pub trait OrderStatusUpdater<T: Config> {
	fn update_status_failed(order_id: &T::Hash);
	fn remove_order_id_from_pending_orders_by_seller(
		seller_id: &T::AccountId,
		genetic_analysis_order_id: &T::Hash,
	);
	fn is_pending_order_by_seller_exist(seller_id: &T::AccountId) -> bool;
}
