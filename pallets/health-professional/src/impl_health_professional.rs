use crate::*;
use frame_support::{
	sp_runtime::traits::{SaturatedConversion, Saturating, Zero},
	traits::ExistenceRequirement,
};
use primitives_availability_status::AvailabilityStatus;
use primitives_stake_status::{StakeStatus, StakeStatusTrait};
use primitives_verification_status::VerificationStatus;
use traits_health_professional::HealthProfessionalCountT;

impl<T: Config> HealthProfessionalInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type Balance = BalanceOf<T>;
	type Moment = MomentOf<T>;
	type HealthProfessional = HealthProfessionalOf<T>;
	type HealthProfessionalInfo = HealthProfessionalInfoOf<T>;

	fn create_health_professional(
		account_id: &T::AccountId,
		health_professional_info: &Self::HealthProfessionalInfo,
	) -> Result<Self::HealthProfessional, Self::Error> {
		if HealthProfessionals::<T>::contains_key(account_id) {
			return Err(Error::<T>::AlreadyRegistered)
		}

		let health_professional = HealthProfessional::new(account_id, health_professional_info);

		HealthProfessionals::<T>::insert(account_id, &health_professional);

		Self::add_health_professional_count(1);

		Ok(health_professional)
	}

	fn update_health_professional_info(
		account_id: &<T as frame_system::Config>::AccountId,
		health_professional_info: &Self::HealthProfessionalInfo,
	) -> Result<Self::HealthProfessionalInfo, Self::Error> {
		HealthProfessionals::<T>::mutate(account_id, |result| match result {
			None => Err(Error::<T>::NotFound),
			Some(health_professional) => {
				health_professional.update_info(health_professional_info);
				Ok(health_professional_info.clone())
			},
		})
	}

	fn update_health_professional_verification_status(
		verifier_key: &T::AccountId,
		account_id: &T::AccountId,
		status: &VerificationStatus,
	) -> Result<VerificationStatus, Self::Error> {
		Self::can_verified(verifier_key)?;

		HealthProfessionals::<T>::mutate(account_id, |result| match result {
			None => Err(Error::<T>::NotFound),
			Some(health_professional) => {
				health_professional.update_verification_status(status);
				Ok(status.clone())
			},
		})
	}

	fn update_health_professional_availability_status(
		account_id: &T::AccountId,
		status: &AvailabilityStatus,
	) -> Result<AvailabilityStatus, Self::Error> {
		HealthProfessionals::<T>::mutate(account_id, |result| match result {
			None => Err(Error::<T>::NotFound),
			Some(health_professional) => {
				health_professional.update_availability_status(status);
				Ok(status.clone())
			},
		})
	}

	fn delete_health_professional(account_id: &T::AccountId) -> Result<(), Self::Error> {
		let health_professional =
			HealthProfessionals::<T>::get(account_id).ok_or(Error::<T>::NotFound)?;

		let qualification_ids = health_professional.qualifications();

		T::HealthProfessionalQualifications::delete_qualifications(account_id, qualification_ids);

		HealthProfessionals::<T>::remove(account_id);

		Self::substract_health_professional_count(1);

		Ok(())
	}

	fn stake_health_professional(account_id: &T::AccountId) -> Result<Self::Balance, Self::Error> {
		let staking_balance = Self::staking_balance(account_id)?;
		let staking_account_id = Self::staking_account_id(account_id);

		HealthProfessionals::<T>::mutate(account_id, |result| match result {
			None => Err(Error::<T>::NotFound),
			Some(health_professional) => {
				if health_professional.stake_status().is_staked() {
					return Err(Error::<T>::AlreadyStaked)
				}

				if health_professional.stake_status().is_waiting_for_unstaked() {
					return Err(Error::<T>::CannotStaked)
				}

				Self::do_transfer(
					account_id,
					&staking_account_id,
					staking_balance,
					ExistenceRequirement::KeepAlive,
				)?;

				health_professional.update_stake_status(StakeStatus::Staked, staking_balance);

				TotalStakedAmount::<T>::mutate(|value| {
					*value = value.saturating_add(staking_balance);
				});

				Ok(staking_balance)
			},
		})
	}

	fn unstake_health_professional(account_id: &T::AccountId) -> Result<Self::Moment, Self::Error> {
		HealthProfessionals::<T>::mutate(account_id, |result| match result {
			None => Err(Error::<T>::NotFound),
			Some(health_professional) => {
				if !health_professional.stake_status().is_staked() {
					return Err(Error::<T>::CannotUnstaked)
				}

				let now = pallet_timestamp::Pallet::<T>::get();
				let status = StakeStatus::WaitingForUnstaked;

				health_professional.update_stake_status(status, Zero::zero());
				health_professional.update_unstaked_at(Some(now));

				Ok(now)
			},
		})
	}

	fn retrieve_unstaked_amount(
		account_id: &T::AccountId,
	) -> Result<(Self::Balance, Self::Moment), Self::Error> {
		let staking_account_id = Self::staking_account_id(account_id);
		let unstake_time =
			UnstakeTime::<T>::get().ok_or(Error::<T>::CannotRetrieveUnstakedAmount)?;

		HealthProfessionals::<T>::mutate(account_id, |result| match result {
			None => Err(Error::<T>::NotFound),
			Some(health_professional) => {
				if !health_professional.stake_status().is_waiting_for_unstaked() {
					return Err(Error::<T>::CannotRetrieveUnstakedAmount)
				}

				let unstaked_at = health_professional
					.unstaked_at()
					.ok_or(Error::<T>::CannotRetrieveUnstakedAmount)?
					.saturated_into::<u128>();

				let staking_balance = *health_professional.stake_amount();
				let now = pallet_timestamp::Pallet::<T>::get();

				if now.saturated_into::<u128>().saturating_sub(unstaked_at) < unstake_time {
					return Err(Error::<T>::NotReadyToUnstaked)
				}

				Self::do_transfer(
					&staking_account_id,
					account_id,
					staking_balance,
					ExistenceRequirement::AllowDeath,
				)?;

				health_professional.update_stake_status(StakeStatus::Unstaked, Zero::zero());

				TotalStakedAmount::<T>::mutate(|value| {
					*value = value.saturating_sub(staking_balance);
				});

				Ok((staking_balance, now))
			},
		})
	}

	fn update_stake_amount(
		account_id: &T::AccountId,
		balance: &Self::Balance,
	) -> Result<(), Self::Error> {
		Self::can_verified(account_id)?;

		MinimumStakeAmount::<T>::put(balance);

		Ok(())
	}

	fn update_unstake_time(
		account_id: &<T as frame_system::Config>::AccountId,
		moment: u128,
	) -> Result<(), Self::Error> {
		Self::can_verified(account_id)?;

		UnstakeTime::<T>::put(moment);

		Ok(())
	}

	fn update_verifier_key(
		verifier_key: &T::AccountId,
		account_id: &T::AccountId,
	) -> Result<(), Self::Error> {
		Self::can_verified(verifier_key)?;

		HealthProfessionalVerifierKey::<T>::put(&account_id);

		Ok(())
	}
}
