use crate::*;

use frame_support::{
	dispatch::DispatchError, sp_runtime::traits::AccountIdConversion, traits::ExistenceRequirement,
	PalletId,
};

pub const PALLET_ID: PalletId = PalletId(*b"reqsrvc!");

impl<T: Config> Pallet<T> {
	/// Pallet Methods
	pub fn staking_account_id(request_id: ServiceIdOf<T>) -> AccountIdOf<T> {
		PALLET_ID.into_sub_account(request_id)
	}

	pub fn generate_request_id(
		requester_id: &T::AccountId,
		country: &[u8],
		region: &[u8],
		city: &[u8],
		service_category: &[u8],
	) -> T::Hash {
		let mut seed = requester_id.encode();
		let account_info = frame_system::Pallet::<T>::account(requester_id);

		seed.append(&mut account_info.nonce.encode());
		seed.append(&mut country.encode());
		seed.append(&mut region.encode());
		seed.append(&mut city.encode());
		seed.append(&mut service_category.encode());

		T::Hashing::hash(&seed)
	}

	pub fn do_transfer(
		sender: &T::AccountId,
		receiver: &T::AccountId,
		amount: BalanceOf<T>,
		existence: ExistenceRequirement,
	) -> Result<(), Error<T>> {
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
				DispatchError::Module(_) => Err(Error::<T>::Arithmetic),
			}
		}

		Ok(())
	}
}
