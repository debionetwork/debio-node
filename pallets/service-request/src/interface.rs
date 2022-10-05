use sp_std::vec::Vec;

pub trait SeviceRequestInterface<T: frame_system::Config> {
	type Error;
	type Request;
	type Balance;

	fn create_request(
		requester_id: &T::AccountId,
		country: Vec<u8>,
		region: Vec<u8>,
		city: Vec<u8>,
		service_category: Vec<u8>,
		staking_amount: Self::Balance,
	) -> Result<Self::Request, Self::Error>;

	fn unstake(requester_id: &T::AccountId, request_id: &T::Hash) -> Result<(), Self::Error>;

	fn retrieve_unstaked_amount(
		requester_id: &T::AccountId,
		request_id: &T::Hash,
	) -> Result<Self::Balance, Self::Error>;

	fn claim_request(
		lab_id: &T::AccountId,
		request_id: &T::Hash,
		service_id: &T::Hash,
	) -> Result<bool, Self::Error>;

	fn process_request(
		requester_id: &T::AccountId,
		request_id: &T::Hash,
		order_id: &T::Hash,
	) -> Result<(), Self::Error>;

	fn finalize_request(lab_id: &T::AccountId, request_id: &T::Hash) -> Result<(), Self::Error>;
}
