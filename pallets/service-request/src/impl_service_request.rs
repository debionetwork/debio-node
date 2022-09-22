use super::*;

use frame_support::sp_runtime::traits::Zero;

/// Service Request Interface Implementation
impl<T: Config> SeviceRequestInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type Balance = BalanceOf<T>;
	type Request = RequestOf<T>;
	type Admin = AdminOf<T>;
	type ServiceOffer = ServiceOfferOf<T>;
	type ServiceInvoice = ServiceInvoiceOf<T>;
	type RequestId = RequestIdOf<T>;
	type RequesterId = RequesterIdOf<T>;
	type LabId = LabIdOf<T>;
	type Country = CountryOf;
	type Region = RegionOf;
	type City = CityOf;
	type ServiceCategory = ServiceCategoryOf;
	type ServiceId = ServiceIdOf<T>;
	type OrderId = OrderIdOf<T>;
	type DNASampleTrackingId = DNASampleTrackingIdOf;
	type ServicePrice = LabPriceOf<T>;

	// Stake with DBIO
	fn create_request(
		requester_id: Self::RequesterId,
		country: Self::Country,
		region: Self::Region,
		city: Self::City,
		service_category: Self::ServiceCategory,
		staking_amount: Self::Balance,
	) -> Result<Self::Request, Self::Error> {
		if staking_amount.is_zero() {
			return Err(Error::<T>::NotValidAmount)
		}

		let request_id =
			Self::generate_request_id(&requester_id, &country, &region, &city, &service_category);

		let now = T::TimeProvider::now().as_millis();

		Self::do_transfer(
			b"native",
			&requester_id,
			&Self::staking_account_id(request_id),
			staking_amount,
			true,
		)?;

		let request = Request::new(
			request_id,
			&requester_id,
			&country,
			&region,
			&city,
			&service_category,
			staking_amount,
			now,
		);

		RequestById::<T>::insert(&request_id, &request);
		RequestByAccountId::<T>::mutate(&requester_id, |request_ids| request_ids.push(request_id));
		StakingAccountIdByRequestId::<T>::insert(&request_id, Self::staking_account_id(request_id));
		ServiceCountRequest::<T>::mutate((country, region, city, service_category), |value| {
			*value = value.wrapping_add(1);
		});

		Ok(request)
	}

	// Unstake DBIO
	fn unstake(
		requester_id: Self::RequesterId,
		request_id: Self::RequestId,
	) -> Result<Self::Request, Self::Error> {
		let mut request = RequestById::<T>::get(request_id).ok_or(Error::<T>::RequestNotFound)?;

		if request.requester_address != requester_id {
			return Err(Error::<T>::Unauthorized)
		}

		if request.status != RequestStatus::Open {
			return Err(Error::<T>::RequestUnableToUnstake)
		}

		let now: u128 = T::TimeProvider::now().as_millis();

		request.status = RequestStatus::WaitingForUnstaked;
		request.unstaked_at = Some(now);

		RequestById::<T>::insert(request_id, &request);

		Ok(request)
	}

	// Unstake DBIO
	fn retrieve_unstaked_amount(
		admin: Self::Admin,
		request_id: Self::RequestId,
	) -> Result<Self::Request, Self::Error> {
		let _ = AdminKey::<T>::get()
			.filter(|account_id| account_id == &admin)
			.ok_or(Error::<T>::Unauthorized)?;

		let mut request = RequestById::<T>::get(request_id).ok_or(Error::<T>::RequestNotFound)?;

		if request.status != RequestStatus::WaitingForUnstaked {
			return Err(Error::<T>::RequestUnableToRetrieveUnstake)
		}

		let requester_id = request.requester_address.clone();

		let now: u128 = T::TimeProvider::now().as_millis();
		let unstaked_at: u128 = request.unstaked_at.unwrap();
		// TODO: move to constant runtime config
		let six_days: u128 = 3600_u128 * 144_u128 * 1000_u128;

		if (now - unstaked_at) == six_days {
			return Err(Error::<T>::RequestWaitingForUnstaked)
		}

		Self::do_transfer(
			b"native",
			&Self::staking_account_id(request_id),
			&requester_id,
			request.staking_amount,
			false,
		)?;

		Self::deposit_event(Event::StakingAmountRefunded(
			requester_id,
			request_id,
			request.staking_amount,
		));

		request.status = RequestStatus::Unstaked;

		RequestById::<T>::insert(request_id, &request);

		Ok(request)
	}

	// Claim request with AssetId
	fn claim_request(
		lab_id: Self::LabId,
		request_id: Self::RequestId,
		service_id: Self::ServiceId,
		service_price: Self::ServicePrice,
	) -> Result<(Self::Request, Self::ServiceOffer), Self::Error> {
		let mut request = RequestById::<T>::get(request_id).ok_or(Error::<T>::RequestNotFound)?;

		if request.status == RequestStatus::WaitingForUnstaked ||
			request.status == RequestStatus::Unstaked
		{
			return Err(Error::<T>::RequestAlreadyUnstaked)
		}

		if request.status == RequestStatus::Claimed {
			return Err(Error::<T>::RequestAlreadyClaimed)
		}

		if request.status != RequestStatus::Open {
			return Err(Error::<T>::RequestUnableToClaimed)
		}

		let lab_status =
			T::Labs::lab_verification_status(&lab_id).ok_or(Error::<T>::LabNotFound)?;

		let _ = Self::do_asset_exist(service_price.get_asset_id())?;

		let service_offer = ServiceOffer::new(request_id, &lab_id, service_id, &service_price);

		if lab_status.is_verified() {
			let now = T::TimeProvider::now().as_millis();

			request.status = RequestStatus::Claimed;
			request.lab_address = Some(lab_id);
			request.updated_at = Some(now);

			RequestById::<T>::insert(request_id, &request);
			ServiceOfferById::<T>::insert(request_id, &service_offer);
		} else {
			RequestsByLabId::<T>::try_mutate(lab_id, |value| {
				if value.iter().any(|x| x == &request_id) {
					return Err(Error::<T>::RequestAlreadyInList)
				}

				value.push(request_id);

				Ok(())
			})?;
		}

		Ok((request, service_offer))
	}

	// Process request with assetId
	fn process_request(
		requester_id: Self::RequesterId,
		lab_id: Self::LabId,
		request_id: Self::RequestId,
		order_id: Self::OrderId,
		dna_sample_tracking_id: Self::DNASampleTrackingId,
	) -> Result<Self::ServiceInvoice, Self::Error> {
		let mut request = RequestById::<T>::get(request_id).ok_or(Error::<T>::RequestNotFound)?;

		if requester_id != request.requester_address {
			return Err(Error::<T>::Unauthorized)
		}

		if request.status == RequestStatus::WaitingForUnstaked ||
			request.status == RequestStatus::Unstaked
		{
			return Err(Error::<T>::RequestAlreadyUnstaked)
		}

		let lab_address = request.get_lab_address().as_ref().ok_or(Error::<T>::LabNotFound)?;

		if lab_address != &lab_id {
			return Err(Error::<T>::LabNotFound)
		}

		if request.status != RequestStatus::Claimed {
			return Err(Error::<T>::RequestUnableToProccess)
		}

		let service_offer =
			ServiceOfferById::<T>::get(request_id).ok_or(Error::<T>::ServiceOfferNotFound)?;

		let service_price = service_offer.get_service_price();
		let asset_id = service_price.get_asset_id();
		let total_price = service_price.total_price();

		Self::do_transfer(
			asset_id,
			&requester_id,
			&Self::staking_account_id(request_id),
			total_price,
			true,
		)?;

		Self::deposit_event(Event::StakingAmountIncreased(
			requester_id.clone(),
			request_id,
			total_price,
		));

		let service_invoice = ServiceInvoice::new(
			request_id,
			order_id,
			service_offer.service_id,
			requester_id,
			lab_id,
			dna_sample_tracking_id,
			service_price,
		);

		let now = T::TimeProvider::now().as_millis();

		request.status = RequestStatus::Processed;
		request.updated_at = Some(now);

		RequestById::<T>::insert(request_id, request);
		ServiceInvoiceById::<T>::insert(request_id, &service_invoice);
		ServiceInvoiceByOrderId::<T>::insert(order_id, &service_invoice);

		Ok(service_invoice)
	}

	// Finalize request with assetId
	fn finalize_request(
		admin: Self::Admin,
		request_id: Self::RequestId,
		test_result_success: bool,
	) -> Result<Self::ServiceInvoice, Self::Error> {
		let _ = AdminKey::<T>::get()
			.filter(|account_id| account_id == &admin)
			.ok_or(Error::<T>::Unauthorized)?;

		let mut request = RequestById::<T>::get(request_id).ok_or(Error::<T>::RequestNotFound)?;
		let service_invoice =
			ServiceInvoiceById::<T>::get(request_id).ok_or(Error::<T>::ServiceInvoiceNotFound)?;

		if request.status != RequestStatus::Processed {
			return Err(Error::<T>::RequestUnableToFinalize)
		}

		let requester_id = service_invoice.customer_address.clone();
		let lab_id = service_invoice.seller_address.clone();

		let service_price = service_invoice.get_service_price();
		let asset_id = service_price.get_asset_id();
		let mut pay_amount = service_price.total_price();

		if !test_result_success {
			pay_amount = service_price.get_qc_price();

			let testing_price = service_price.get_testing_price();

			// Transfer testing price back to customer
			Self::do_transfer(
				asset_id,
				&Self::staking_account_id(request_id),
				&requester_id,
				testing_price,
				false,
			)?;
		}

		// Transfer qc_price to lab_id
		Self::do_transfer(
			asset_id,
			&Self::staking_account_id(request_id),
			&lab_id,
			pay_amount,
			false,
		)?;

		let balance = CurrencyOf::<T>::free_balance(&Self::staking_account_id(request_id));

		if !balance.is_zero() {
			// Transfer DBIO to requester_id
			Self::do_transfer(
				b"native",
				&Self::staking_account_id(request_id),
				&requester_id,
				balance,
				false,
			)?;
		}

		let now = T::TimeProvider::now().as_millis();

		request.status = RequestStatus::Finalized;
		request.updated_at = Some(now);

		RequestById::<T>::insert(request_id, &request);

		// Removed from customer request list
		RequestByAccountId::<T>::mutate(&requester_id, |request_ids| {
			request_ids.retain(|&x| x != request_id);
		});

		// Update service count request
		ServiceCountRequest::<T>::mutate(
			(&request.country, &request.region, &request.city, &request.service_category),
			|value| *value = value.wrapping_sub(1),
		);

		Ok(service_invoice)
	}
}
