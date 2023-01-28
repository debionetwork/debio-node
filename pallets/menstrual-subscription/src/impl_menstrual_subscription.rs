use crate::*;

use primitives_duration::MenstrualSubscriptionDuration;
use primitives_menstrual_status::{MenstrualSubscriptionStatus, PaymentStatus};
use primitives_price_and_currency::CurrencyType;

/// MenstrualSubscription Interface Implementation
impl<T: Config> MenstrualSubscriptionInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type Balance = BalanceOf<T>;
	type MenstrualSubscription = MenstrualSubscriptionOf<T>;
	type MenstrualSubscriptionPrice = MenstrualSubscriptionPriceOf<T>;

	fn add_menstrual_subscription(
		address_id: &T::AccountId,
		duration: &MenstrualSubscriptionDuration,
		currency: &CurrencyType,
	) -> Result<Self::MenstrualSubscription, Self::Error> {
		let _ = Self::do_inqueue_exist(address_id)?;
		let _ = MenstrualSubscriptionPrices::<T>::get(duration, currency)
			.ok_or(Error::<T>::MenstrualSubscriptionPriceNotExist)?;

		let owner_menstrual_subscription_count =
			MenstrualSubscriptionCountByOwner::<T>::get(address_id).unwrap_or(0);

		let menstrual_subscription_id = Self::generate_menstrual_subscription_id(
			address_id,
			owner_menstrual_subscription_count,
		);

		let now = pallet_timestamp::Pallet::<T>::get();

		let menstrual_subscription = MenstrualSubscription::new(
			menstrual_subscription_id,
			address_id.clone(),
			duration.clone(),
			currency.clone(),
			now,
		);

		// Store to MenstrualSubscriptionById storage
		MenstrualSubscriptionById::<T>::insert(menstrual_subscription_id, &menstrual_subscription);

		Self::add_menstrual_subscription_by_owner(address_id, &menstrual_subscription_id);
		Self::add_menstrual_subscription_count();
		Self::add_menstrual_subscription_count_by_owner(address_id);

		Ok(menstrual_subscription)
	}

	fn change_menstrual_subscription_status(
		menstrual_subscription_id: &T::Hash,
		status: &MenstrualSubscriptionStatus,
	) -> Result<Self::MenstrualSubscription, Self::Error> {
		let mut menstrual_subscription =
			MenstrualSubscriptionById::<T>::get(menstrual_subscription_id)
				.ok_or(Error::<T>::MenstrualSubscriptionDoesNotExist)?;

		let address = &menstrual_subscription.address_id;

		if menstrual_subscription.payment_status != PaymentStatus::Paid {
			return Err(Error::<T>::MenstrualSubscriptionNotPaid)
		}

		if status == &menstrual_subscription.status {
			return Ok(menstrual_subscription)
		}

		if menstrual_subscription.status == MenstrualSubscriptionStatus::Active {
			ActiveSubscriptionByOwner::<T>::remove(address);
		}

		if status == &MenstrualSubscriptionStatus::Active {
			ActiveSubscriptionByOwner::<T>::insert(address, menstrual_subscription_id);
		}

		let now = pallet_timestamp::Pallet::<T>::get();

		menstrual_subscription.status = status.clone();
		menstrual_subscription.updated_at = now;

		// Store to MenstrualSubscriptionById storage
		MenstrualSubscriptionById::<T>::insert(menstrual_subscription_id, &menstrual_subscription);

		Ok(menstrual_subscription)
	}

	fn set_menstrual_subscription_paid(
		address_id: &T::AccountId,
		menstrual_subscription_id: &T::Hash,
	) -> Result<Self::MenstrualSubscription, Self::Error> {
		let mut menstrual_subscription =
			MenstrualSubscriptionById::<T>::get(menstrual_subscription_id)
				.ok_or(Error::<T>::MenstrualSubscriptionDoesNotExist)?;

		if &menstrual_subscription.address_id != address_id {
			return Err(Error::<T>::NotMenstrualSubscriptionOwner)
		}

		if menstrual_subscription.payment_status == PaymentStatus::Paid {
			return Err(Error::<T>::MenstrualSubscriptionAlreadyPaid)
		}

		let subscription_duration = &menstrual_subscription.duration;
		let currency = &menstrual_subscription.currency;
		let subscription_price =
			MenstrualSubscriptionPrices::<T>::get(subscription_duration, currency)
				.ok_or(Error::<T>::MenstrualSubscriptionPriceNotExist)?;

		let amount = subscription_price.amount;

		if currency != &CurrencyType::DBIO {
			return Err(Error::<T>::Token)
		}

		Self::do_burn(address_id, amount)?;

		if Self::active_subscription_by_owner(address_id).is_none() {
			menstrual_subscription.status = MenstrualSubscriptionStatus::Active;
			ActiveSubscriptionByOwner::<T>::insert(address_id, menstrual_subscription_id);
		}

		let now = pallet_timestamp::Pallet::<T>::get();

		menstrual_subscription.payment_status = PaymentStatus::Paid;
		menstrual_subscription.updated_at = now;

		// Store to MenstrualSubscriptionById storage
		MenstrualSubscriptionById::<T>::insert(menstrual_subscription_id, &menstrual_subscription);

		Ok(menstrual_subscription)
	}

	fn set_menstrual_subscription_price(
		duration: &MenstrualSubscriptionDuration,
		currency: &CurrencyType,
		price: Self::Balance,
		asset_id: Option<AssetId>,
	) -> Result<Self::MenstrualSubscriptionPrice, Self::Error> {
		let asset_id = Self::do_validate_asset_id(currency, asset_id)?;
		let menstrual_subscription_price =
			MenstrualSubscriptionPrice::new(duration, currency, asset_id, price);

		MenstrualSubscriptionPrices::<T>::insert(duration, currency, &menstrual_subscription_price);

		Ok(menstrual_subscription_price)
	}
}
