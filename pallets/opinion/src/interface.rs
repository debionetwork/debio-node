use crate::Status;

pub trait OpinionInterface<T: frame_system::Config> {
	type Error;
	type Opinion;
	type OpinionInfo;

	fn add_opinion(
		admin: &T::AccountId,
		requestor_id: &T::Hash,
		account_id: &T::AccountId,
		info: &Self::OpinionInfo,
	) -> Result<Self::Opinion, Self::Error>;

	fn update_opinion(
		account_id: &T::AccountId,
		opinion_id: &T::Hash,
		info: &Self::OpinionInfo,
	) -> Result<Self::Opinion, Self::Error>;

	fn remove_opinion(account_id: &T::AccountId, opinion_id: &T::Hash) -> Result<(), Self::Error>;

	fn update_status(
		account_id: &T::AccountId,
		opinion_id: &T::Hash,
		status: &Status,
	) -> Result<(), Self::Error>;

	fn update_admin_key(admin: &T::AccountId, account_id: &T::AccountId)
		-> Result<(), Self::Error>;
}
