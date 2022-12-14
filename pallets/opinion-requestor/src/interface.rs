pub trait OpinionRequestorInterface<T: frame_system::Config> {
	type Error;
	type OpinionRequestor;
	type RequestorInfo;

	fn request_opinion(
		account_id: &T::AccountId,
		info: Self::RequestorInfo,
	) -> Result<Self::OpinionRequestor, Self::Error>;

	fn update_requestor_info(
		id: &T::Hash,
		account_id: &T::AccountId,
		info: Self::RequestorInfo,
	) -> Result<(), Self::Error>;
}
