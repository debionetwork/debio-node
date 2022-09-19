use super::*;

use frame_support::{sp_runtime::traits::Zero, traits::ExistenceRequirement};

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
			&requester_id,
			&Self::staking_account_id(request_id),
			staking_amount,
			ExistenceRequirement::KeepAlive,
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
			&Self::staking_account_id(request_id),
			&requester_id,
			request.staking_amount,
			ExistenceRequirement::AllowDeath,
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

	fn claim_request(
		lab_id: Self::LabId,
		request_id: Self::RequestId,
		service_id: Self::ServiceId,
		testing_price: Self::Balance,
		qc_price: Self::Balance,
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
		let service_offer =
			ServiceOffer::new(request_id, &lab_id, service_id, testing_price, qc_price);

		let now = T::TimeProvider::now().as_millis();

		if lab_status.is_verified() {
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

	fn process_request(
		requester_id: Self::RequesterId,
		lab_id: Self::LabId,
		request_id: Self::RequestId,
		order_id: Self::OrderId,
		dna_sample_tracking_id: Self::DNASampleTrackingId,
		additional_staking_amount: Self::Balance,
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
		let pay_amount = request.staking_amount;
		let testing_price = service_offer.testing_price;
		let qc_price = service_offer.qc_price;
		let total_price = testing_price + qc_price;

		let mut final_pay_amount = total_price;

		if pay_amount > total_price {
			let excess = pay_amount - total_price;

			Self::do_transfer(
				&Self::staking_account_id(request_id),
				&requester_id,
				excess,
				ExistenceRequirement::KeepAlive,
			)?;

			Self::deposit_event(Event::StakingAmountExcessRefunded(
				requester_id.clone(),
				request_id,
				excess,
			));
		} else {
			final_pay_amount = request.staking_amount + additional_staking_amount;

			if final_pay_amount != total_price {
				return Err(Error::<T>::NotValidAmount)
			}

			Self::do_transfer(
				&requester_id,
				&Self::staking_account_id(request_id),
				additional_staking_amount,
				ExistenceRequirement::KeepAlive,
			)?;

			Self::deposit_event(Event::StakingAmountIncreased(
				requester_id.clone(),
				request_id,
				additional_staking_amount,
			));
		}

		let service_invoice = ServiceInvoice::new(
			request_id,
			order_id,
			service_offer.service_id,
			requester_id,
			lab_id,
			dna_sample_tracking_id,
			testing_price,
			qc_price,
			final_pay_amount,
		);

		let now = T::TimeProvider::now().as_millis();

		request.status = RequestStatus::Processed;
		request.updated_at = Some(now);

		RequestById::<T>::insert(request_id, request);
		ServiceInvoiceById::<T>::insert(request_id, &service_invoice);
		ServiceInvoiceByOrderId::<T>::insert(order_id, &service_invoice);

		Ok(service_invoice)
	}

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

		let mut pay_amount = service_invoice.pay_amount;

		if !test_result_success {
			pay_amount = service_invoice.qc_price;

			let testing_price = service_invoice.testing_price;

			// Transfer testing price back to customer
			Self::do_transfer(
				&Self::staking_account_id(request_id),
				&requester_id,
				testing_price,
				ExistenceRequirement::AllowDeath,
			)?;
		}

		// Transfer to lab_id
		Self::do_transfer(
			&Self::staking_account_id(request_id),
			&lab_id,
			pay_amount,
			ExistenceRequirement::AllowDeath,
		)?;

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
