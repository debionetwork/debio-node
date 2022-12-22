use crate::*;
use traits_opinion::OpinionCountT;
use traits_opinion_requestor::OpinionRequestorProvider;

impl<T: Config> OpinionInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type Opinion = OpinionOf<T>;
	type OpinionInfo = OpinionInfo;

	fn add_opinion(
		admin: &T::AccountId,
		requestor_id: &T::Hash,
		account_id: &T::AccountId,
		info: &OpinionInfo,
	) -> Result<Self::Opinion, Self::Error> {
		Self::is_admin(admin)?;

		if !T::OpinionRequestor::can_give_opinion(requestor_id) {
			return Err(Error::<T>::NotFound)
		}

		let total_opinion = Self::opinion_count();
		let now = pallet_timestamp::Pallet::<T>::get();
		let opinion_id = Self::generate_opinion_id(admin, total_opinion);

		let mut opinion = Opinion::new(&opinion_id, requestor_id, account_id, info, now);

		let asset_id = *info.asset_id();
		let currency = info.currency();
		let asset_id = Self::do_validate_asset_id(currency, asset_id)?;

		opinion.update_asset_id(asset_id);

		Opinions::<T>::insert(opinion_id, &opinion);

		T::OpinionRequestor::associate(requestor_id, &opinion_id);

		Self::add_opinion_count(1);
		Self::add_opinion_count_by_owner(account_id, 1);
		Self::add_opinion_id(account_id, &opinion_id);

		Ok(opinion)
	}

	fn update_opinion(
		admin: &T::AccountId,
		opinion_id: &T::Hash,
		account_id: &T::AccountId,
		info: &OpinionInfo,
	) -> Result<Self::Opinion, Self::Error> {
		Self::is_admin(admin)?;

		let mut opinion = Opinions::<T>::get(opinion_id)
			.ok_or(Error::<T>::NotFound)?
			.is_authorized_owner(account_id)
			.ok_or(Error::<T>::Unauthorized)?;

		let asset_id = *info.asset_id();
		let currency = info.currency();
		let asset_id = Self::do_validate_asset_id(currency, asset_id)?;

		opinion.update_info(info);
		opinion.update_asset_id(asset_id);

		Opinions::<T>::insert(opinion_id, &opinion);

		Ok(opinion)
	}

	fn remove_opinion(
		admin: &T::AccountId,
		opinion_id: &T::Hash,
		account_id: &T::AccountId,
	) -> Result<(), Self::Error> {
		Self::is_admin(admin)?;

		let opinion = Opinions::<T>::get(opinion_id)
			.ok_or(Error::<T>::NotFound)?
			.is_authorized_owner(account_id)
			.ok_or(Error::<T>::Unauthorized)?;

		let requestor_id = opinion.requestor_id();

		Opinions::<T>::remove(opinion_id);

		T::OpinionRequestor::disassociate(requestor_id, opinion_id);

		Self::substract_opinion_count(1);
		Self::substract_opinion_count_by_owner(account_id, 1);
		Self::remove_opinion_id(account_id, opinion_id);

		Ok(())
	}

	fn update_admin_key(
		admin: &T::AccountId,
		account_id: &T::AccountId,
	) -> Result<(), Self::Error> {
		Self::is_admin(admin)?;

		OpinionAdminKey::<T>::put(account_id);

		Ok(())
	}
}
