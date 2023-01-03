use crate::*;
use frame_support::{
	sp_runtime::traits::{AccountIdConversion, Saturating},
	traits::{Currency, ExistenceRequirement},
	PalletId,
};
use traits_health_professional::HealthProfessionalCountT;
use traits_health_professional_qualifications::HealthProfessionalQualificationOwner;

pub const PALLET_ID: PalletId = PalletId(*b"hlthpro!");

impl<T: Config> Pallet<T> {
	pub fn staking_account_id(account_id: &T::AccountId) -> T::AccountId {
		PALLET_ID.into_sub_account(account_id)
	}

	pub fn staking_balance(account_id: &T::AccountId) -> Result<BalanceOf<T>, Error<T>> {
		let balance = T::Currency::free_balance(account_id);
		let minimum_balance = T::Currency::minimum_balance();
		let minimum_stake_amount =
			MinimumStakeAmount::<T>::get().ok_or(Error::<T>::InsufficientBalance)?;
		let tranferable_amount = minimum_stake_amount.saturating_add(minimum_balance);

		if tranferable_amount > balance {
			return Err(Error::<T>::InsufficientBalance)
		}

		Ok(minimum_stake_amount)
	}

	pub fn do_transfer(
		sender: &T::AccountId,
		receiver: &T::AccountId,
		amount: BalanceOf<T>,
		existence: ExistenceRequirement,
	) -> Result<(), Error<T>> {
		let result = CurrencyOf::<T>::transfer(sender, receiver, amount, existence);

		if result.is_err() {
			return Err(Error::<T>::BadOrigin)
		}

		Ok(())
	}

	pub fn can_verified(account_id: &T::AccountId) -> Result<(), Error<T>> {
		let _ = HealthProfessionalVerifierKey::<T>::get()
			.filter(|verifier_key| verifier_key == account_id)
			.ok_or(Error::<T>::Unauthorized)?;

		Ok(())
	}
}

impl<T: Config> HealthProfessionalCountT<T> for Pallet<T> {
	fn add_health_professional_count(value: u64) {
		HealthProfessionalCount::<T>::mutate(|count| {
			*count = count.saturating_add(value);
		});
	}

	fn substract_health_professional_count(value: u64) {
		HealthProfessionalCount::<T>::mutate(|count| {
			*count = count.saturating_sub(value);
		});
	}
}

impl<T: Config> HealthProfessionalQualificationOwner<T> for Pallet<T> {
	type Owner = HealthProfessionalOf<T>;

	fn get_owner(account_id: &T::AccountId) -> Option<Self::Owner> {
		HealthProfessionals::<T>::get(account_id)
	}

	fn can_create_qualification(account_id: &T::AccountId) -> bool {
		HealthProfessionals::<T>::contains_key(account_id)
	}

	fn associate(owner_id: &T::AccountId, qualification_id: &T::Hash) {
		HealthProfessionals::<T>::mutate(owner_id, |result| match result {
			None => (),
			Some(health_professional) => {
				health_professional.add_qualification(*qualification_id);
			},
		});
	}

	fn disassociate(owner_id: &T::AccountId, qualification_id: &T::Hash) {
		HealthProfessionals::<T>::mutate(owner_id, |result| match result {
			None => (),
			Some(health_professional) => {
				health_professional.remove_qualification(*qualification_id);
			},
		});
	}
}
