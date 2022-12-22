use crate::*;
use frame_support::{codec::Encode, sp_runtime::traits::Hash, traits::fungibles};
use primitives_price_and_currency::CurrencyType;
use scale_info::prelude::string::String;
use sp_std::vec::Vec;
use traits_opinion::OpinionCountT;

impl<T: Config> Pallet<T> {
	pub fn generate_opinion_id(account_id: &T::AccountId, total_opinion: u64) -> T::Hash {
		let mut account_bytes = account_id.encode();
		let mut total_opinion_bytes = total_opinion.encode();
		let mut nonce = frame_system::Pallet::<T>::account(account_id).nonce.encode();

		account_bytes.append(&mut total_opinion_bytes);
		account_bytes.append(&mut nonce);

		let seed = &account_bytes;
		T::Hashing::hash(seed)
	}

	pub fn do_validate_asset_id(
		currency: &CurrencyType,
		asset_id: Option<u32>,
	) -> Result<Option<u32>, Error<T>> {
		if currency == &CurrencyType::DBIO {
			return Ok(None)
		}

		let asset_id = asset_id.ok_or(Error::<T>::NotFound)?;
		let symbol = <T::Assets as fungibles::InspectMetadata<T::AccountId>>::symbol(&asset_id);
		let str_symbol = String::from_utf8(symbol).map_err(|_| Error::<T>::NotFound)?;

		if currency.as_string().to_lowercase() != str_symbol.to_lowercase() {
			return Err(Error::<T>::NotFound)
		}

		Ok(Some(asset_id))
	}

	pub fn is_admin(account_id: &T::AccountId) -> Result<(), Error<T>> {
		let _ = OpinionAdminKey::<T>::get()
			.filter(|admin_key| admin_key == account_id)
			.ok_or(Error::<T>::Unauthorized)?;

		Ok(())
	}

	pub fn add_opinion_id(account_id: &T::AccountId, opinion_id: &T::Hash) {
		OpinionByOwner::<T>::mutate(account_id, |opinions: &mut Vec<T::Hash>| {
			opinions.push(*opinion_id);
		});
	}

	pub fn remove_opinion_id(account_id: &T::AccountId, opinion_id: &T::Hash) {
		OpinionByOwner::<T>::mutate(account_id, |opinions: &mut Vec<T::Hash>| {
			let position = opinions.iter().position(|x| x == opinion_id);

			if let Some(index) = position {
				opinions.remove(index);
			}
		});
	}
}

impl<T: Config> OpinionCountT<T> for Pallet<T> {
	fn add_opinion_count(value: u64) {
		OpinionCount::<T>::mutate(|count| {
			*count = count.saturating_add(value);
		});
	}

	fn substract_opinion_count(value: u64) {
		OpinionCount::<T>::mutate(|count| {
			*count = count.saturating_sub(value);
		});
	}

	fn add_opinion_count_by_owner(account_id: &T::AccountId, value: u64) {
		OpinionCountByOwner::<T>::mutate(account_id, |count| {
			*count = count.saturating_add(value);
		});
	}

	fn substract_opinion_count_by_owner(
		account_id: &<T as frame_system::Config>::AccountId,
		value: u64,
	) {
		OpinionCountByOwner::<T>::mutate(account_id, |count| {
			*count = count.saturating_sub(value);
		});
	}
}
