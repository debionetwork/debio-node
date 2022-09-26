use crate::*;

impl<T: Config> OrderInterface<T> for Pallet<T> {
	type Order = OrderOf<T>;
	type Error = Error<T>;

	fn create_order(
		customer_id: &T::AccountId,
		service_id: &T::Hash,
		price_index: u32,
		customer_box_public_key: &T::Hash,
		order_flow: ServiceFlow,
	) -> Result<Self::Order, Self::Error> {
		let service =
			T::Services::service_by_id(service_id).ok_or(Error::<T>::ServiceDoesNotExist)?;

		let order_id = Self::generate_order_id(customer_id, service_id);
		let seller_id = service.get_owner_id();
		let prices_by_currency = service.get_prices_by_currency();

		if prices_by_currency.is_empty() ||
			prices_by_currency.len() - 1 < price_index.try_into().unwrap()
		{
			return Err(Error::<T>::PriceIndexNotFound)
		}

		let price_by_currency = &prices_by_currency[price_index as usize];

		let currency = &price_by_currency.currency;
		let prices = &price_by_currency.price_components;
		let additional_prices = &price_by_currency.additional_prices;

		let now = pallet_timestamp::Pallet::<T>::get();

		// Initialize DnaSample
		let dna_sample = T::GeneticTesting::register_dna_sample(seller_id, customer_id, &order_id)
			.map_err(|_| Error::<T>::DnaSampleInitalizationError)?;

		let order = Order::new(
			order_id,
			*service_id,
			customer_id.clone(),
			*customer_box_public_key,
			seller_id.clone(),
			dna_sample.get_tracking_id().clone(),
			currency.clone(),
			order_flow,
			prices.clone(),
			additional_prices.clone(),
			now,
			now,
		);

		Self::insert_order_to_storage(&order);

		Ok(order)
	}

	fn cancel_order(
		customer_id: &T::AccountId,
		order_id: &T::Hash,
	) -> Result<Self::Order, Self::Error> {
		let order = Orders::<T>::get(order_id).ok_or(Error::<T>::OrderNotFound)?;

		if order.get_customer_id() != customer_id {
			return Err(Error::<T>::UnauthorizedOrderCancellation)
		}

		let dna_sample_result =
			T::GeneticTesting::dna_sample_by_tracking_id(&order.dna_sample_tracking_id);

		if let Some(dna_sample) = dna_sample_result {
			if !dna_sample.is_registered() {
				return Err(Error::<T>::OngoingOrderCannotBeCancelled)
			}
		}

		// Delete dna sample associated with the order
		let _ = T::GeneticTesting::delete_dna_sample(&order.dna_sample_tracking_id);

		let order = Self::update_order_status(order_id, OrderStatus::Cancelled)
			.ok_or(Error::<T>::OrderNotFound)?;

		Ok(order)
	}

	fn set_order_paid(
		escrow_account_id: &T::AccountId,
		order_id: &T::Hash,
	) -> Result<Self::Order, Self::Error> {
		let _ = EscrowKey::<T>::get()
			.filter(|account_id| account_id == escrow_account_id)
			.ok_or(Error::<T>::Unauthorized)?;

		let order = Self::update_order_status(order_id, OrderStatus::Paid)
			.ok_or(Error::<T>::OrderNotFound)?;

		Ok(order)
	}

	fn fulfill_order(
		seller_id: &T::AccountId,
		order_id: &T::Hash,
	) -> Result<Self::Order, Self::Error> {
		let order = Orders::<T>::get(order_id).ok_or(Error::<T>::OrderNotFound)?;

		// Only the seller can fulfill the order
		if order.get_seller_id() != seller_id {
			return Err(Error::<T>::UnauthorizedOrderFulfillment)
		}

		let dna_sample_result =
			T::GeneticTesting::dna_sample_by_tracking_id(&order.dna_sample_tracking_id);

		if let Some(dna_sample) = dna_sample_result {
			if !dna_sample.process_success() {
				return Err(Error::<T>::DnaSampleNotSuccessfullyProcessed)
			}
		}

		let order = Self::update_order_status(order_id, OrderStatus::Fulfilled)
			.ok_or(Error::<T>::OrderNotFound)?;

		Ok(order)
	}

	fn set_order_refunded(
		escrow_account_id: &T::AccountId,
		order_id: &T::Hash,
	) -> Result<Self::Order, Self::Error> {
		let _ = EscrowKey::<T>::get()
			.filter(|account_id| account_id == escrow_account_id)
			.ok_or(Error::<T>::Unauthorized)?;

		let order = Orders::<T>::get(order_id).ok_or(Error::<T>::OrderNotFound)?;

		if !Self::order_can_be_refunded(order) {
			return Err(Error::<T>::OrderNotYetExpired)
		}

		let order = Self::update_order_status(order_id, OrderStatus::Refunded)
			.ok_or(Error::<T>::OrderNotFound)?;

		Ok(order)
	}

	fn update_escrow_key(
		account_id: &T::AccountId,
		escrow_key: &T::AccountId,
	) -> Result<(), Self::Error> {
		let _ = EscrowKey::<T>::get()
			.filter(|e| e == account_id)
			.ok_or(Error::<T>::Unauthorized)?;

		EscrowKey::<T>::put(escrow_key);

		Ok(())
	}
}
