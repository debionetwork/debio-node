/// Interface for Customer Pallet
/// Defines the functionalities of Customer Pallet
pub trait CustomerInterface<T: frame_system::Config> {
	type Error;
	type CustomerInfo;
	type Customer;

	/// Get customer by associated account_id
	fn customer_by_account_id(account_id: &T::AccountId) -> Option<Self::Customer>;
	/// Store A customer with its information
	fn create_customer(
		account_id: &T::AccountId,
		customer_info: &Self::CustomerInfo,
	) -> Result<Self::Customer, Self::Error>;
	/// Update a Customer information
	fn update_customer(
		account_id: &T::AccountId,
		customer_info: &Self::CustomerInfo,
	) -> Result<Self::Customer, Self::Error>;
	/// Delete Customer
	fn delete_customer(account_id: &T::AccountId) -> Result<Self::Customer, Self::Error>;
}
