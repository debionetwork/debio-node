use crate::*;

use frame_support::sp_runtime::{traits::Zero, SaturatedConversion};

impl<T: Config> OrderInterface<T> for Pallet<T> {
	type Order = OrderOf<T>;
	type Error = Error<T>;

	fn create_order(
		customer_id: &T::AccountId,
		service_id: &T::Hash,
		price_index: u32,
		customer_box_public_key: &T::Hash,
		order_flow: ServiceFlow,
		asset_id: Option<u32>,
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
		let asset_id = Self::do_validate_asset_id(currency, asset_id)?;
		let total_price = &price_by_currency.total_price;
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
			asset_id,
			currency.clone(),
			order_flow,
			prices.clone(),
			additional_prices.clone(),
			*total_price,
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
		let order = Orders::<T>::get(order_id)
			.ok_or(Error::<T>::OrderNotFound)?
			.is_authorized_customer(customer_id)
			.ok_or(Error::<T>::UnauthorizedOrderCancellation)?
			.can_cancelled()
			.ok_or(Error::<T>::OrderCannotBeCancelled)?;

		let dna_sample_opt =
			T::GeneticTesting::dna_sample_by_tracking_id(&order.dna_sample_tracking_id);

		if let Some(dna_sample) = dna_sample_opt {
			if !dna_sample.is_registered() {
				return Err(Error::<T>::OngoingOrderCannotBeCancelled)
			}
		}

		let can_transfer = order.currency.can_transfer();
		let mut order_status = OrderStatus::Cancelled;

		if can_transfer && order.status == OrderStatus::Paid {
			// Transfer
			match Self::pallet_id() {
				Some(pallet_id) => {
					order_status = OrderStatus::Refunded;

					Self::do_transfer(
						&order.currency,
						&pallet_id,
						&order.customer_id,
						order.total_price,
						order.asset_id,
					)
				},
				None => Err(Error::<T>::PalletAccountNotFound),
			}?;
		}

		// Delete dna sample associated with the order
		let _ = T::GeneticTesting::delete_dna_sample(&order.dna_sample_tracking_id);
		Self::remove_order_id_from_pending_orders_by_seller(&order.seller_id, &order.id);
		let order =
			Self::update_order_status(order_id, order_status).ok_or(Error::<T>::OrderNotFound)?;

		Ok(order)
	}

	fn set_order_paid(
		account_id: &T::AccountId,
		order_id: &T::Hash,
	) -> Result<Self::Order, Self::Error> {
		let order = Orders::<T>::get(order_id).ok_or(Error::<T>::OrderNotFound)?;

		if order.currency.can_transfer() {
			if account_id != &order.customer_id {
				return Err(Error::<T>::Unauthorized)
			}
		} else {
			let _ = EscrowKey::<T>::get()
				.filter(|escrow_id| escrow_id == account_id)
				.ok_or(Error::<T>::Unauthorized)?;
		}

		let order = order.can_paid().ok_or(Error::<T>::OrderCannotBePaid)?;

		if order.currency.can_transfer() {
			match Self::pallet_id() {
				Some(pallet_id) => Self::do_transfer(
					&order.currency,
					&order.customer_id,
					&pallet_id,
					order.total_price,
					order.asset_id,
				),
				None => Err(Error::<T>::PalletAccountNotFound),
			}?;
		}

		let order = Self::update_order_status(order_id, OrderStatus::Paid)
			.ok_or(Error::<T>::OrderNotFound)?;

		Ok(order)
	}

	fn fulfill_order(
		seller_id: &T::AccountId,
		order_id: &T::Hash,
	) -> Result<Self::Order, Self::Error> {
		let order = Orders::<T>::get(order_id)
			.ok_or(Error::<T>::OrderNotFound)?
			.is_authorized_seller(seller_id)
			.ok_or(Error::<T>::UnauthorizedOrderFulfillment)?
			.can_fulfilled()
			.ok_or(Error::<T>::OrderCannotBeFulfilled)?;

		let dna_sample_opt =
			T::GeneticTesting::dna_sample_by_tracking_id(&order.dna_sample_tracking_id);

		if let Some(dna_sample) = dna_sample_opt {
			if !dna_sample.process_success() {
				return Err(Error::<T>::DnaSampleNotSuccessfullyProcessed)
			}
		}

		if order.currency.can_transfer() {
			let treasury_key = Self::treasury_key().ok_or(Error::<T>::PalletAccountNotFound)?;
			let pallet_id = Self::pallet_id().ok_or(Error::<T>::PalletAccountNotFound)?;
			let price_substracted_value: BalanceOf<T> = order.total_price / 20u128.saturated_into();
			let total_price_paid = order.total_price - price_substracted_value;

			// Transfer 5% to treasury
			Self::do_transfer(
				&order.currency,
				&pallet_id,
				&treasury_key,
				price_substracted_value,
				order.asset_id,
			)?;

			// Withhold 5%
			Self::do_transfer(
				&order.currency,
				&pallet_id,
				order.get_seller_id(),
				total_price_paid,
				order.asset_id,
			)?;
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

		let order = Orders::<T>::get(order_id)
			.ok_or(Error::<T>::OrderNotFound)?
			.can_refunded()
			.ok_or(Error::<T>::OrderCannotBeRefunded)?;

		if !Self::order_can_be_refunded(&order) {
			return Err(Error::<T>::OrderNotYetExpired)
		}

		if order.currency.can_transfer() {
			match Self::pallet_id() {
				Some(pallet_id) => {
					let mut testing_price = Zero::zero();
					let mut qc_price = Zero::zero();

					for price in order.prices.iter() {
						testing_price += price.value;
					}

					for price in order.additional_prices.iter() {
						qc_price += price.value;
					}

					Self::do_transfer(
						&order.currency,
						&pallet_id,
						order.get_seller_id(),
						qc_price,
						order.asset_id,
					)?;

					Self::do_transfer(
						&order.currency,
						&pallet_id,
						&order.customer_id,
						testing_price,
						order.asset_id,
					)?;

					Ok(())
				},
				None => Err(Error::<T>::PalletAccountNotFound),
			}?;
		}

		let order = Self::update_order_status(order_id, OrderStatus::Refunded)
			.ok_or(Error::<T>::OrderNotFound)?;

		Ok(order)
	}
}
