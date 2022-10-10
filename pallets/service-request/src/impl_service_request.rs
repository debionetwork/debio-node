use super::*;

use frame_support::{sp_runtime::traits::Zero, traits::ExistenceRequirement};
use primitives_verification_status::VerificationStatusTrait;
use traits_order::OrderInfo;
use traits_services::ServiceInfo;

/// Service Request Interface Implementation
impl<T: Config> SeviceRequestInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type Request = RequestOf<T>;
	type Balance = BalanceOf<T>;

	// Stake with DBIO
	fn create_request(
		requester_id: &T::AccountId,
		country: Vec<u8>,
		region: Vec<u8>,
		city: Vec<u8>,
		service_category: Vec<u8>,
		staking_amount: Self::Balance,
	) -> Result<Self::Request, Self::Error> {
		if staking_amount.is_zero() {
			return Err(Error::<T>::NotValidAmount)
		}

		let request_id =
			Self::generate_request_id(requester_id, &country, &region, &city, &service_category);

		let now = T::TimeProvider::now().as_millis();

		Self::do_transfer(
			requester_id,
			&Self::staking_account_id(request_id),
			staking_amount,
			ExistenceRequirement::KeepAlive,
		)?;

		let request = Request::new(
			request_id,
			requester_id,
			&country,
			&region,
			&city,
			&service_category,
			staking_amount,
			now,
		);

		RequestById::<T>::insert(request_id, &request);
		RequestByAccountId::<T>::mutate(requester_id, |request_ids| request_ids.push(request_id));
		StakingAccountIdByRequestId::<T>::insert(request_id, Self::staking_account_id(request_id));
		ServiceCountRequest::<T>::mutate((country, region, city, service_category), |value| {
			*value = value.wrapping_add(1);
		});

		Ok(request)
	}

	// Unstake DBIO
	fn unstake(requester_id: &T::AccountId, request_id: &T::Hash) -> Result<(), Self::Error> {
		let mut request = RequestById::<T>::get(request_id).ok_or(Error::<T>::RequestNotFound)?;

		if &request.requester_address != requester_id {
			return Err(Error::<T>::Unauthorized)
		}

		if request.status != RequestStatus::Open {
			return Err(Error::<T>::RequestUnableToUnstake)
		}

		let now = T::TimeProvider::now().as_millis();

		request.status = RequestStatus::WaitingForUnstaked;
		request.unstaked_at = Some(now);

		RequestById::<T>::insert(request_id, &request);

		Ok(())
	}

	// Unstake DBIO
	fn retrieve_unstaked_amount(
		requester_id: &T::AccountId,
		request_id: &T::Hash,
	) -> Result<Self::Balance, Self::Error> {
		let mut request = RequestById::<T>::get(request_id).ok_or(Error::<T>::RequestNotFound)?;

		if requester_id != &request.requester_address {
			return Err(Error::<T>::Unauthorized)
		}

		if request.status != RequestStatus::WaitingForUnstaked {
			return Err(Error::<T>::RequestUnableToRetrieveUnstake)
		}

		let requester_id = request.requester_address.clone();

		let now = T::TimeProvider::now().as_millis();
		let unstaked_at = request.unstaked_at.unwrap();

		if (now - unstaked_at) < T::UnstakePeriode::get() as u128 {
			return Err(Error::<T>::RequestWaitingForUnstaked)
		}

		Self::do_transfer(
			&Self::staking_account_id(*request_id),
			&requester_id,
			request.staking_amount,
			ExistenceRequirement::AllowDeath,
		)?;

		request.status = RequestStatus::Unstaked;

		RequestById::<T>::insert(request_id, &request);

		Ok(request.staking_amount)
	}

	// Claim request by lab
	fn claim_request(
		lab_id: &T::AccountId,
		request_id: &T::Hash,
		service_id: &T::Hash,
	) -> Result<bool, Self::Error> {
		let mut request = RequestById::<T>::get(request_id).ok_or(Error::<T>::RequestNotFound)?;
		let mut is_claimed = true;
		match request.status {
			RequestStatus::WaitingForUnstaked => Err(Error::<T>::RequestAlreadyUnstaked),
			RequestStatus::Unstaked => Err(Error::<T>::RequestAlreadyUnstaked),
			RequestStatus::Claimed => Err(Error::<T>::RequestAlreadyClaimed),
			RequestStatus::Open => Ok(()),
			_ => Err(Error::<T>::RequestUnableToClaimed),
		}?;

		let lab_status = T::Labs::lab_verification_status(lab_id).ok_or(Error::<T>::LabNotFound)?;

		let service = T::Services::service_by_id(service_id).ok_or(Error::<T>::ServiceNotFound)?;

		if !service.is_service_owner(lab_id) {
			return Err(Error::<T>::Unauthorized)
		}

		if lab_status.is_verified() {
			let now = T::TimeProvider::now().as_millis();

			request.status = RequestStatus::Claimed;
			request.lab_address = Some(lab_id.clone());
			request.service_id = Some(*service_id);
			request.updated_at = Some(now);

			RequestById::<T>::insert(request_id, request);
		} else {
			RequestsByLabId::<T>::try_mutate(lab_id, |value| {
				if value.iter().any(|x| x == request_id) {
					return Err(Error::<T>::RequestAlreadyInList)
				}

				value.push(*request_id);

				Ok(())
			})?;

			is_claimed = false;
		}

		Ok(is_claimed)
	}

	// Process request by customer
	fn process_request(
		requester_id: &T::AccountId,
		request_id: &T::Hash,
		order_id: &T::Hash,
	) -> Result<(), Self::Error> {
		RequestById::<T>::try_mutate(request_id, |result| match result {
			Some(request) => {
				match request.status {
					RequestStatus::WaitingForUnstaked => Err(Error::<T>::RequestAlreadyUnstaked),
					RequestStatus::Unstaked => Err(Error::<T>::RequestAlreadyUnstaked),
					RequestStatus::Claimed => Ok(()),
					_ => Err(Error::<T>::RequestUnableToProccess),
				}?;

				if requester_id != &request.requester_address {
					return Err(Error::<T>::Unauthorized)
				}

				let order =
					T::Orders::get_order_by_id(order_id).ok_or(Error::<T>::OrderNotFound)?;

				if !order.is_order_unpaid() && !order.is_order_paid() {
					return Err(Error::<T>::RequestUnableToProccess)
				}

				if !order.is_order_to_lab(request.lab_address.as_ref().unwrap()) {
					return Err(Error::<T>::RequestUnableToProccess)
				}

				if !order.is_account_order(&request.requester_address) {
					return Err(Error::<T>::RequestUnableToProccess)
				}

				if !order.is_order_from_service(request.service_id.as_ref().unwrap()) {
					return Err(Error::<T>::RequestUnableToProccess)
				}

				let now = T::TimeProvider::now().as_millis();

				request.order_id = Some(*order_id);
				request.status = RequestStatus::Processed;
				request.updated_at = Some(now);

				Ok(())
			},
			None => Err(Error::<T>::RequestNotFound),
		})?;

		RequestByOrderId::<T>::insert(order_id, request_id);

		Ok(())
	}

	// Finalize request with assetId
	fn finalize_request(lab_id: &T::AccountId, request_id: &T::Hash) -> Result<(), Self::Error> {
		RequestById::<T>::try_mutate(request_id, |result| match result {
			Some(request) => {
				if request.status != RequestStatus::Processed {
					return Err(Error::<T>::RequestUnableToFinalize)
				}

				let _ = request
					.lab_address
					.as_ref()
					.filter(|account_id| account_id == &lab_id)
					.ok_or(Error::<T>::LabNotFound)?;

				let order_id = request.order_id.ok_or(Error::<T>::OrderNotFound)?;
				let order =
					T::Orders::get_order_by_id(&order_id).ok_or(Error::<T>::OrderNotFound)?;

				// Ketika order di cancelled ?
				if !order.is_order_fullfilled() &&
					!order.is_order_refunded() &&
					!order.is_order_failed()
				{
					return Err(Error::<T>::RequestUnableToFinalize)
				}

				if !order.is_order_to_lab(request.lab_address.as_ref().unwrap()) {
					return Err(Error::<T>::RequestUnableToFinalize)
				}

				if !order.is_account_order(&request.requester_address) {
					return Err(Error::<T>::RequestUnableToFinalize)
				}

				if !order.is_order_from_service(request.service_id.as_ref().unwrap()) {
					return Err(Error::<T>::RequestUnableToFinalize)
				}

				let balance = request.staking_amount;
				Self::do_transfer(
					&Self::staking_account_id(*request_id),
					&request.requester_address,
					balance,
					ExistenceRequirement::AllowDeath,
				)?;

				let now = T::TimeProvider::now().as_millis();

				request.status = RequestStatus::Finalized;
				request.updated_at = Some(now);

				// Removed from order request
				RequestByOrderId::<T>::remove(order_id);

				// Removed from customer request list
				RequestByAccountId::<T>::mutate(&request.requester_address, |request_ids| {
					request_ids.retain(|&x| x != *request_id);
				});

				// Update service count request
				ServiceCountRequest::<T>::mutate(
					(&request.country, &request.region, &request.city, &request.service_category),
					|value| *value = value.wrapping_sub(1),
				);

				Ok(())
			},
			None => Err(Error::<T>::RequestNotFound),
		})?;

		Ok(())
	}
}
