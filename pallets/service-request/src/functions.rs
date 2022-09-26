use crate::*;

use frame_support::{
	dispatch::DispatchError,
	sp_runtime::traits::{AccountIdConversion, SaturatedConversion},
	traits::{fungibles, ExistenceRequirement},
	PalletId,
};
use scale_info::prelude::string::String;

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

	pub fn do_asset_exist(asset_id: &[u8]) -> Result<(), Error<T>> {
		let asset_id = Self::asset_id(asset_id)?;
		let name = <T::Assets as fungibles::InspectMetadata<T::AccountId>>::name(&asset_id);
		let symbol = <T::Assets as fungibles::InspectMetadata<T::AccountId>>::symbol(&asset_id);

		if name.is_empty() {
			return Err(Error::<T>::AssetNotExists)
		}

		if symbol.is_empty() {
			return Err(Error::<T>::AssetNotExists)
		}

		Ok(())
	}

	pub fn do_transfer(
		asset_id: &[u8],
		sender: &T::AccountId,
		receiver: &T::AccountId,
		amount: BalanceOf<T>,
		keep_alive: bool,
	) -> Result<(), Error<T>> {
		if asset_id == b"native" {
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
					DispatchError::Module(_) => Err(Error::<T>::Arithmetic),
				}
			}
		} else {
			let asset_id = Self::asset_id(asset_id)?;
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
					DispatchError::Module(_) => Err(Error::<T>::Arithmetic),
				}
			}
		}

		Ok(())
	}

	pub fn asset_id(asset_id: &[u8]) -> Result<u32, Error<T>> {
		let str_num = String::from_utf8(asset_id.to_vec()).map_err(|_| Error::<T>::WrongFormat)?;

		str_num.parse::<u32>().map_err(|_| Error::<T>::WrongFormat)
	}
}
