pub trait SeviceRequestInterface<T: frame_system::Config> {
    type Error;
	type Balance;
	type Request;
	type ServiceOffer;
	type ServiceInvoice;
	type RequestId;
	type RequesterId;
	type LabId;
	type Country;
	type Region;
	type City;
	type ServiceCategory;
	type ServiceId;
	type OrderId;
	type DNASampleTrackingId;

    fn generate_service_request_id(
        requester_id: Self::RequesterId,
		country: Self::Country,
		region: Self::Region,
		city: Self::City,
		service_category: Self::ServiceCategory,
    ) -> Self::RequestId;

    fn create_request(
		requester_id: Self::RequesterId,
		country: Self::Country,
		region: Self::Region,
		city: Self::City,
		service_category: Self::ServiceCategory,
		staking_amount: Self::Balance,
    ) -> Result<Self::Request, Self::Error>;

	fn unstake(
		requester_id: Self::RequesterId,
		request_id: Self::RequestId,
    ) -> Result<Self::Request, Self::Error>;

	fn retrieve_unstaked_amount(
		requester_id: Self::RequesterId,
		request_id: Self::RequestId,
    ) -> Result<Self::Request, Self::Error>;

	fn claim_request(
		lab_id: Self::LabId,
		service_id: Self::ServiceId,
		testing_price: Self::Balance,
		qc_price: Self::Balance,
    ) -> Result<Self::ServiceOffer, Self::Error>;

	fn process_request(
		requester_id: Self::RequesterId,
		lab_id: Self::LabId,
		request_id: Self::RequestId,
		order_id: Self::OrderId,
		dna_sample_tracking_id: Self::DNASampleTrackingId,
    ) -> Result<Self::ServiceInvoice, Self::Error>;
}
