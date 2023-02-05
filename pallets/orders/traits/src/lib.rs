#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;

pub trait OrderEventEmitter<T: Config> {
	fn emit_event_order_failed(order_id: &T::Hash);
}

pub trait OrderStatusUpdater<T: Config> {
	fn update_status_failed(order_id: &T::Hash) -> bool;
	fn remove_order_id_from_pending_orders_by_seller(
		seller_id: &T::AccountId,
		genetic_analysis_order_id: &T::Hash,
	);
	fn is_pending_order_by_seller_exist(seller_id: &T::AccountId) -> bool;
	fn is_order_paid(order_id: &T::Hash) -> bool;
}

pub trait OrderInfo<T: Config> {
	fn is_order_paid(&self) -> bool;
	fn is_order_unpaid(&self) -> bool;
	fn is_order_fullfilled(&self) -> bool;
	fn is_order_refunded(&self) -> bool;
	fn is_order_failed(&self) -> bool;
	fn is_order_to_lab(&self, account_id: &T::AccountId) -> bool;
	fn is_account_order(&self, account_id: &T::AccountId) -> bool;
	fn is_order_from_service(&self, service_id: &T::Hash) -> bool;
}

pub trait OrderProvider<T: Config> {
	type Orders: OrderInfo<T>;

	fn get_order_by_id(order_id: &T::Hash) -> Option<Self::Orders>;
}
