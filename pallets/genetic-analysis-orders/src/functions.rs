use crate::*;

use frame_support::{
	dispatch::DispatchError,
	sp_runtime::traits::SaturatedConversion,
	traits::{fungibles, ExistenceRequirement},
};
use primitives_price_and_currency::CurrencyType;
use scale_info::prelude::string::String;

impl<T: Config> Pallet<T> {
	pub fn generate_genetic_analysis_order_id(
		customer_id: &T::AccountId,
		service_id: &T::Hash,
	) -> T::Hash {
		let mut customer_id_bytes = customer_id.encode();
		let mut service_id_bytes = service_id.encode();
		let account_info = frame_system::Pallet::<T>::account(customer_id);
		let mut nonce_bytes = account_info.nonce.encode();

		customer_id_bytes.append(&mut service_id_bytes);
		customer_id_bytes.append(&mut nonce_bytes);

		let seed = &customer_id_bytes;
		T::Hashing::hash(seed)
	}

	pub fn update_genetic_analysis_order_status(
		genetic_analysis_order_id: &T::Hash,
		status: GeneticAnalysisOrderStatus,
	) -> Result<GeneticAnalysisOrderOf<T>, Error<T>> {
		let genetic_analysis_order = GeneticAnalysisOrders::<T>::mutate(
			genetic_analysis_order_id,
			|genetic_analysis_order| match genetic_analysis_order {
				None => None,
				Some(genetic_analysis_order) => {
					genetic_analysis_order.status = status;
					genetic_analysis_order.updated_at = pallet_timestamp::Pallet::<T>::get();
					Some(genetic_analysis_order.clone())
				},
			},
		)
		.ok_or(Error::<T>::GeneticAnalysisOrderNotFound)?;

		Ok(genetic_analysis_order)
	}

	pub fn insert_genetic_analysis_order_to_storage(
		genetic_analysis_order: &GeneticAnalysisOrderOf<T>,
	) {
		GeneticAnalysisOrders::<T>::insert(genetic_analysis_order.id, genetic_analysis_order);
		LastGeneticAnalysisOrderByCustomer::<T>::insert(
			&genetic_analysis_order.customer_id,
			genetic_analysis_order.id,
		);
		Self::insert_genetic_analysis_order_id_into_genetic_analysis_orders_by_seller(
			genetic_analysis_order,
		);
		Self::insert_genetic_analysis_order_id_into_pending_genetic_analysis_orders_by_seller(
			genetic_analysis_order,
		);
		Self::insert_genetic_analysis_order_id_into_genetic_analysis_orders_by_customer(
			genetic_analysis_order,
		);
	}

	pub fn insert_genetic_analysis_order_id_into_genetic_analysis_orders_by_seller(
		genetic_analysis_order: &GeneticAnalysisOrderOf<T>,
	) {
		match GeneticAnalysisOrdersBySeller::<T>::get(&genetic_analysis_order.seller_id) {
			None => {
				GeneticAnalysisOrdersBySeller::<T>::insert(
					&genetic_analysis_order.seller_id,
					vec![genetic_analysis_order.id],
				);
			},
			Some(mut genetic_analysis_orders) => {
				genetic_analysis_orders.push(genetic_analysis_order.id);
				GeneticAnalysisOrdersBySeller::<T>::insert(
					&genetic_analysis_order.seller_id,
					genetic_analysis_orders,
				);
			},
		}
	}

	pub fn insert_genetic_analysis_order_id_into_genetic_analysis_orders_by_customer(
		genetic_analysis_order: &GeneticAnalysisOrderOf<T>,
	) {
		match GeneticAnalysisOrdersByCustomer::<T>::get(&genetic_analysis_order.customer_id) {
			None => {
				GeneticAnalysisOrdersByCustomer::<T>::insert(
					&genetic_analysis_order.customer_id,
					vec![genetic_analysis_order.id],
				);
			},
			Some(mut genetic_analysis_orders) => {
				genetic_analysis_orders.push(genetic_analysis_order.id);
				GeneticAnalysisOrdersByCustomer::<T>::insert(
					&genetic_analysis_order.customer_id,
					genetic_analysis_orders,
				);
			},
		}
	}

	pub fn insert_genetic_analysis_order_id_into_pending_genetic_analysis_orders_by_seller(
		genetic_analysis_order: &GeneticAnalysisOrderOf<T>,
	) {
		match PendingGeneticAnalysisOrdersBySeller::<T>::get(&genetic_analysis_order.seller_id) {
			None => {
				PendingGeneticAnalysisOrdersBySeller::<T>::insert(
					&genetic_analysis_order.seller_id,
					vec![genetic_analysis_order.id],
				);
			},
			Some(mut genetic_analysis_orders) => {
				genetic_analysis_orders.push(genetic_analysis_order.id);
				PendingGeneticAnalysisOrdersBySeller::<T>::insert(
					&genetic_analysis_order.seller_id,
					genetic_analysis_orders,
				);
			},
		}
	}

	pub fn remove_genetic_analysis_order_id_from_pending_genetic_analysis_orders_by_seller(
		seller_id: &T::AccountId,
		genetic_analysis_order_id: &T::Hash,
	) {
		let mut genetic_analysis_orders =
			PendingGeneticAnalysisOrdersBySeller::<T>::get(seller_id).unwrap_or_default();
		genetic_analysis_orders.retain(|o_id| o_id != genetic_analysis_order_id);
		PendingGeneticAnalysisOrdersBySeller::<T>::insert(seller_id, genetic_analysis_orders);
	}

	pub fn genetic_analysis_order_can_be_refunded(tracking_id: &TrackingId) -> bool {
		match T::GeneticAnalysis::genetic_analysis_by_genetic_analysis_tracking_id(tracking_id) {
			Some(genetic_analysis) => genetic_analysis.is_rejected(),
			None => false,
		}
	}

	pub fn do_transfer(
		currency: &CurrencyType,
		sender: &T::AccountId,
		receiver: &T::AccountId,
		amount: BalanceOf<T>,
		keep_alive: bool,
		asset_id: Option<u32>,
	) -> Result<(), Error<T>> {
		if currency == &CurrencyType::DBIO {
			let existence = if keep_alive {
				ExistenceRequirement::KeepAlive
			} else {
				ExistenceRequirement::AllowDeath
			};

			let result = CurrencyOf::<T>::transfer(sender, receiver, amount, existence);

			if let Err(dispatch) = result {
				return match dispatch {
					DispatchError::Other(_) => Err(Error::<T>::Other),
					DispatchError::CannotLookup => Err(Error::<T>::CannotLookup),
					DispatchError::BadOrigin => Err(Error::<T>::BadOrigin),
					DispatchError::TooManyConsumers => Err(Error::<T>::TooManyConsumers),
					DispatchError::ConsumerRemaining => Err(Error::<T>::ConsumerRemaining),
					DispatchError::NoProviders => Err(Error::<T>::NoProviders),
					DispatchError::Token(_) => Err(Error::<T>::Token),
					DispatchError::Arithmetic(_) => Err(Error::<T>::Arithmetic),
					DispatchError::Module(_) => Err(Error::<T>::Other),
				}
			}
		} else {
			let asset_id = asset_id.ok_or(Error::<T>::AssetIdNotFound)?;
			let result = <T::Assets as fungibles::Transfer<T::AccountId>>::transfer(
				asset_id,
				sender,
				receiver,
				amount.saturated_into(),
				keep_alive,
			);

			if let Err(dispatch) = result {
				return match dispatch {
					DispatchError::Other(_) => Err(Error::<T>::Other),
					DispatchError::CannotLookup => Err(Error::<T>::CannotLookup),
					DispatchError::BadOrigin => Err(Error::<T>::BadOrigin),
					DispatchError::TooManyConsumers => Err(Error::<T>::TooManyConsumers),
					DispatchError::ConsumerRemaining => Err(Error::<T>::ConsumerRemaining),
					DispatchError::NoProviders => Err(Error::<T>::NoProviders),
					DispatchError::Token(_) => Err(Error::<T>::Token),
					DispatchError::Arithmetic(_) => Err(Error::<T>::Arithmetic),
					DispatchError::Module(_) => Err(Error::<T>::Module),
				}
			}
		}

		Ok(())
	}

	pub fn do_validate_asset_id(
		currency: &CurrencyType,
		asset_id: Option<u32>,
	) -> Result<Option<u32>, Error<T>> {
		if currency == &CurrencyType::DBIO {
			return Ok(None)
		}

		let asset_id = asset_id.ok_or(Error::<T>::AssetIdNotFound)?;
		let symbol = <T::Assets as fungibles::InspectMetadata<T::AccountId>>::symbol(&asset_id);
		let str_symbol = String::from_utf8(symbol).map_err(|_| Error::<T>::AssetIdNotFound)?;

		if currency.as_string().to_lowercase() != str_symbol.to_lowercase() {
			return Err(Error::<T>::AssetIdNotFound)
		}

		Ok(Some(asset_id))
	}

	/// The injected pallet ID
	pub fn get_pallet_id() -> AccountIdOf<T> {
		T::PalletId::get().into_account()
	}

	/// The account ID that holds the funds
	pub fn account_id() -> AccountIdOf<T> {
		<PalletAccount<T>>::get().unwrap()
	}

	/// Is the balance sufficient for payment
	pub fn is_balance_sufficient_for_payment(
		account_id: &AccountIdOf<T>,
		price: BalanceOf<T>,
	) -> bool {
		let balance = T::Currency::free_balance(account_id);
		balance >= price
	}

	/// Is the pallet balance sufficient for transfer
	pub fn is_pallet_balance_sufficient_for_transfer(price: BalanceOf<T>) -> bool {
		let balance = T::Currency::free_balance(&Self::account_id());
		balance >= price
	}

	/// Set current escrow amount
	pub fn set_escrow_amount() {
		TotalEscrowAmount::<T>::put(T::Currency::free_balance(&Self::account_id()));
	}

	// Get token identifier
	pub fn asset_id(currency_type: &CurrencyType) -> Result<u32, Error<T>> {
		currency_type
			.to_asset_id()
			.parse::<u32>()
			.map_err(|_| Error::<T>::WrongAssetIdFormat)
	}
}
