use crate::*;

use frame_support::{
	pallet_prelude::*,
	sp_runtime::traits::{AccountIdConversion, Hash},
	PalletId,
};
use sp_std::vec;

pub const PALLET_ID: PalletId = PalletId(*b"orders!!");

impl<T: Config> Pallet<T> {
	pub fn staking_account_id(order_id: HashOf<T>) -> AccountIdOf<T> {
		PALLET_ID.into_sub_account(order_id)
	}

	pub fn generate_order_id(customer_id: &T::AccountId, service_id: &T::Hash) -> T::Hash {
		let mut customer_id_bytes = customer_id.encode();
		let mut service_id_bytes = service_id.encode();
		let account_info = frame_system::Pallet::<T>::account(customer_id);
		let mut nonce_bytes = account_info.nonce.encode();

		customer_id_bytes.append(&mut service_id_bytes);
		customer_id_bytes.append(&mut nonce_bytes);

		let seed = &customer_id_bytes;
		T::Hashing::hash(seed)
	}

	pub fn update_order_status(order_id: &T::Hash, status: OrderStatus) -> Option<OrderOf<T>> {
		Orders::<T>::mutate(order_id, |order| match order {
			None => None,
			Some(order) => {
				order.status = status;
				order.updated_at = pallet_timestamp::Pallet::<T>::get();
				Some(order.clone())
			},
		})
	}

	pub fn insert_order_to_storage(order: &OrderOf<T>) {
		Orders::<T>::insert(order.id, order);
		LastOrderByCustomer::<T>::insert(&order.customer_id, order.id);
		Self::insert_order_id_into_orders_by_seller(order);
		Self::insert_order_id_into_pending_orders_by_seller(order);
		Self::insert_order_id_into_orders_by_customer(order);
	}

	pub fn insert_order_id_into_orders_by_seller(order: &OrderOf<T>) {
		match OrdersBySeller::<T>::get(&order.seller_id) {
			None => {
				OrdersBySeller::<T>::insert(&order.seller_id, vec![order.id]);
			},
			Some(mut orders) => {
				orders.push(order.id);
				OrdersBySeller::<T>::insert(&order.seller_id, orders);
			},
		}
	}

	pub fn insert_order_id_into_orders_by_customer(order: &OrderOf<T>) {
		match OrdersByCustomer::<T>::get(&order.customer_id) {
			None => {
				OrdersByCustomer::<T>::insert(&order.customer_id, vec![order.id]);
			},
			Some(mut orders) => {
				orders.push(order.id);
				OrdersByCustomer::<T>::insert(&order.customer_id, orders);
			},
		}
	}

	pub fn insert_order_id_into_pending_orders_by_seller(order: &OrderOf<T>) {
		match PendingOrdersBySeller::<T>::get(&order.seller_id) {
			None => {
				PendingOrdersBySeller::<T>::insert(&order.seller_id, vec![order.id]);
			},
			Some(mut orders) => {
				orders.push(order.id);
				PendingOrdersBySeller::<T>::insert(&order.seller_id, orders);
			},
		}
	}

	pub fn remove_order_id_from_pending_orders_by_seller(
		seller_id: &T::AccountId,
		order_id: &T::Hash,
	) {
		let mut orders = PendingOrdersBySeller::<T>::get(seller_id).unwrap_or_default();
		orders.retain(|o_id| o_id != order_id);
		PendingOrdersBySeller::<T>::insert(seller_id, orders);
	}

	pub fn remove_order_id_from_orders_by_seller(seller_id: &T::AccountId, order_id: &T::Hash) {
		let mut orders = OrdersBySeller::<T>::get(seller_id).unwrap_or_default();
		orders.retain(|o_id| o_id != order_id);
		OrdersBySeller::<T>::insert(seller_id, orders);
	}

	pub fn remove_order_id_from_orders_by_customer(customer_id: &T::AccountId, order_id: &T::Hash) {
		let mut orders = OrdersByCustomer::<T>::get(customer_id).unwrap_or_default();
		orders.retain(|o_id| o_id != order_id);
		OrdersByCustomer::<T>::insert(customer_id, orders);
	}

	pub fn order_can_be_refunded(order: OrderOf<T>) -> bool {
		let dna_sample =
			T::GeneticTesting::dna_sample_by_tracking_id(&order.dna_sample_tracking_id).unwrap();
		if !dna_sample.is_rejected() {
			return false
		}
		true
	}

	fn is_pending_order_ids_by_seller_exist(account_id: &T::AccountId) -> bool {
		match PendingOrdersBySeller::<T>::get(account_id) {
			Some(_arr) => !_arr.is_empty(),
			None => false,
		}
	}
}

impl<T: Config> OrderEventEmitter<T> for Pallet<T> {
	fn emit_event_order_failed(order_id: &HashOf<T>) {
		match Self::order_by_id(order_id) {
			None => Self::deposit_event(Event::OrderNotFound),
			Some(order) => Self::deposit_event(Event::OrderFailed(order)),
		}
	}
}

impl<T: Config> OrderStatusUpdater<T> for Pallet<T> {
	fn update_status_failed(order_id: &HashOf<T>) {
		match Self::order_by_id(order_id) {
			None => Self::deposit_event(Event::OrderNotFound),
			Some(order) => {
				Self::update_order_status(&order.id, OrderStatus::Failed);
			},
		}
	}

	fn remove_order_id_from_pending_orders_by_seller(
		seller_id: &AccountIdOf<T>,
		order_id: &HashOf<T>,
	) {
		Self::remove_order_id_from_pending_orders_by_seller(seller_id, order_id);
	}

	fn is_pending_order_by_seller_exist(seller_id: &AccountIdOf<T>) -> bool {
		Self::is_pending_order_ids_by_seller_exist(seller_id)
	}
}
