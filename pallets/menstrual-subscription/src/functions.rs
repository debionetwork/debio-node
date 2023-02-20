use crate::*;
use frame_support::{
	codec::Encode,
	dispatch::DispatchError,
	sp_runtime::{
		traits::{CheckedSub, Hash},
		SaturatedConversion,
	},
	traits::{fungibles, Currency, ExistenceRequirement, WithdrawReasons},
};
use primitives_menstrual_status::MenstrualSubscriptionStatus;
use primitives_price_and_currency::CurrencyType;
use scale_info::prelude::string::String;
use traits_menstrual_subscription::MenstrualSubscriptionProvider;

/// Pallet Methods
impl<T: Config> Pallet<T> {
	pub fn generate_menstrual_subscription_id(
		address_id: &T::AccountId,
		menstrual_subscription_count: u64,
	) -> T::Hash {
		let account_info = frame_system::Pallet::<T>::account(address_id);

		let mut account_id_bytes = address_id.encode();
		let mut menstrual_subscription_count_bytes = menstrual_subscription_count.encode();
		let mut nonce_bytes = account_info.nonce.encode();

		account_id_bytes.append(&mut menstrual_subscription_count_bytes);
		account_id_bytes.append(&mut nonce_bytes);

		let seed = &account_id_bytes;
		T::Hashing::hash(seed)
	}

	pub fn do_inqueue_exist(address_id: &T::AccountId) -> Result<(), Error<T>> {
		let subscription_ids =
			MenstrualSubscriptionByOwner::<T>::get(address_id).unwrap_or_default();

		if subscription_ids.is_empty() {
			return Ok(())
		}

		for subscription_id in subscription_ids {
			if let Some(menstrual_subscription) =
				MenstrualSubscriptionById::<T>::get(subscription_id)
			{
				if menstrual_subscription.status != MenstrualSubscriptionStatus::InQueue {
					continue
				}

				return Err(Error::<T>::MenstrualSubscriptionAlreadyInQueue)
			}
		}

		Ok(())
	}

	// Add menstrual_subscription by owner
	pub fn add_menstrual_subscription_by_owner(
		address_id: &T::AccountId,
		menstrual_subscription_id: &T::Hash,
	) {
		let mut menstrual_subscription =
			MenstrualSubscriptionByOwner::<T>::get(address_id).unwrap_or_default();

		menstrual_subscription.push(*menstrual_subscription_id);
		MenstrualSubscriptionByOwner::<T>::insert(address_id, &menstrual_subscription)
	}

	// Subtract menstrual_subscription by owner
	pub fn sub_menstrual_subscription_by_owner(
		address_id: &T::AccountId,
		menstrual_subscription_id: &T::Hash,
	) {
		let mut menstrual_subscription =
			MenstrualSubscriptionByOwner::<T>::get(address_id).unwrap_or_default();
		menstrual_subscription.retain(|&x| x != *menstrual_subscription_id);
		MenstrualSubscriptionByOwner::<T>::insert(address_id, menstrual_subscription);
	}

	// Add menstrual_subscription count
	pub fn add_menstrual_subscription_count() {
		let menstrual_subscription_count = <MenstrualSubscriptionCount<T>>::get().unwrap_or(0);
		<MenstrualSubscriptionCount<T>>::put(menstrual_subscription_count.wrapping_add(1));
	}

	// Add menstrual_subscription count by owner
	pub fn add_menstrual_subscription_count_by_owner(address_id: &T::AccountId) {
		let menstrual_subscription_count =
			MenstrualSubscriptionCountByOwner::<T>::get(address_id).unwrap_or(0);
		MenstrualSubscriptionCountByOwner::<T>::insert(
			address_id,
			menstrual_subscription_count.wrapping_add(1),
		)
	}

	// Subtract menstrual_subscription count
	pub fn sub_menstrual_subscription_count() {
		let menstrual_subscription_count = <MenstrualSubscriptionCount<T>>::get().unwrap_or(1);
		MenstrualSubscriptionCount::<T>::put(menstrual_subscription_count - 1);
	}

	// Subtract menstrual_subscription count by owner
	pub fn sub_menstrual_subscription_count_by_owner(address_id: &T::AccountId) {
		let menstrual_subscription_count =
			MenstrualSubscriptionCountByOwner::<T>::get(address_id).unwrap_or(1);
		MenstrualSubscriptionCountByOwner::<T>::insert(
			address_id,
			menstrual_subscription_count - 1,
		);
	}

	pub fn do_validate_asset_id(
		currency: &CurrencyType,
		asset_id: Option<AssetId>,
	) -> Result<Option<AssetId>, Error<T>> {
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

	pub fn do_burn(who: &T::AccountId, amount: BalanceOf<T>) -> Result<(), Error<T>> {
		let _ = CurrencyOf::<T>::total_issuance()
			.checked_sub(&amount)
			.ok_or(Error::<T>::InsufficientBalance)?;

		let result = CurrencyOf::<T>::withdraw(
			who,
			amount,
			WithdrawReasons::TRANSFER,
			ExistenceRequirement::KeepAlive,
		);

		if result.is_err() {
			return Err(Error::<T>::InsufficientBalance)
		}

		CurrencyOf::<T>::burn(amount);
		Self::deposit_event(Event::TotalSupplyDecreased(amount));

		Ok(())
	}

	pub fn do_transfer(
		currency: &CurrencyType,
		sender: &T::AccountId,
		receiver: &T::AccountId,
		amount: BalanceOf<T>,
		asset_id: Option<u32>,
	) -> Result<(), Error<T>> {
		if currency == &CurrencyType::DBIO {
			let result = CurrencyOf::<T>::transfer(
				sender,
				receiver,
				amount,
				ExistenceRequirement::KeepAlive,
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
					DispatchError::Module(_) => Err(Error::<T>::Other),
					DispatchError::Transactional(_) => return Err(Error::<T>::Arithmetic),
				}
			}
		} else {
			let asset_id = asset_id.ok_or(Error::<T>::AssetIdNotFound)?;
			let result = <T::Assets as fungibles::Transfer<T::AccountId>>::transfer(
				asset_id,
				sender,
				receiver,
				amount.saturated_into(),
				true,
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
					DispatchError::Transactional(_) => return Err(Error::<T>::Arithmetic),
				}
			}
		}

		Ok(())
	}
}

/// MenstrualSubscriptionProvider Trait Implementation
impl<T: Config> MenstrualSubscriptionProvider<T> for Pallet<T> {
	type Error = Error<T>;
	type MenstrualSubscription = MenstrualSubscriptionOf<T>;

	fn menstrual_subscription_by_id(id: &T::Hash) -> Option<MenstrualSubscriptionOf<T>> {
		MenstrualSubscriptionById::<T>::get(id)
	}
}
