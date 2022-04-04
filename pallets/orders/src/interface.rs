use traits_services::types::ServiceFlow;

pub trait OrderInterface<T: frame_system::Config> {
	type Order;
	type Error;

	//fn generate_order_id(customer_id: &T::AccountId, service_id: &T::Hash) -> T::Hash;
	fn create_order(
		customer_id: &T::AccountId,
		service_id: &T::Hash,
		price_index: u32,
		customer_box_public_key: &T::Hash,
		order_flow: ServiceFlow,
	) -> Result<Self::Order, Self::Error>;
	fn cancel_order(
		customer_id: &T::AccountId,
		order_id: &T::Hash,
	) -> Result<Self::Order, Self::Error>;
	// set_order_paid Should only be called by Escrow API Server with the correct account_id
	fn set_order_paid(
		escrow_account_id: &T::AccountId,
		order_id: &T::Hash,
	) -> Result<Self::Order, Self::Error>;
	fn fulfill_order(
		seller_id: &T::AccountId,
		order_id: &T::Hash,
	) -> Result<Self::Order, Self::Error>;
	fn set_order_refunded(
		escrow_account_id: &T::AccountId,
		order_id: &T::Hash,
	) -> Result<Self::Order, Self::Error>;
	fn update_escrow_key(
		account_id: &T::AccountId,
		escrow_key: &T::AccountId,
	) -> Result<(), Self::Error>;
	fn is_pending_order_ids_by_seller_exist(account_id: &T::AccountId) -> bool;
}
