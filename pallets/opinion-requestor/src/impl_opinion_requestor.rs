use crate::*;
use sp_std::vec::Vec;
use traits_genetic_data::GeneticDataProvider;
use traits_opinion_requestor::OpinionRequestorCountT;

impl<T: Config> OpinionRequestorInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type OpinionRequestor = OpinionRequestorOf<T>;
	type RequestorInfo = RequestorInfoOf<T>;

	fn request_opinion(
		account_id: &T::AccountId,
		info: Self::RequestorInfo,
	) -> Result<Self::OpinionRequestor, Self::Error> {
		let total_requestor = OpinionRequestorCount::<T>::get();

		let id = Self::generate_opinion_requestor_id(account_id, total_requestor);
		let genetic_data_ids = info.genetic_data_ids();
		let valid_ids = T::GeneticData::valid_genetic_data_ids(account_id, genetic_data_ids);

		let mut requestor_info = info;

		requestor_info.update_opinion_ids(&Vec::new());
		requestor_info.update_genetic_data_ids(&valid_ids);

		let now = pallet_timestamp::Pallet::<T>::get();
		let requestor = OpinionRequestor::new(&id, account_id, &requestor_info, now);

		OpinionRequestors::<T>::insert(id, &requestor);
		OpinionRequestorByOwner::<T>::mutate(account_id, |result: &mut Vec<HashOf<T>>| {
			result.push(id);
		});

		Self::add_opinion_requestor_count(1);
		Self::add_opinion_requestor_count_by_owner(account_id, 1);

		Ok(requestor)
	}

	fn update_requestor_info(
		requestor_id: &T::Hash,
		account_id: &T::AccountId,
		info: Self::RequestorInfo,
	) -> Result<Self::RequestorInfo, Self::Error> {
		let now = pallet_timestamp::Pallet::<T>::get();

		let mut requestor = OpinionRequestors::<T>::get(requestor_id)
			.ok_or(Error::<T>::NotFound)?
			.is_authorized_owner(account_id)
			.ok_or(Error::<T>::Unauthorized)?;

		let genetic_data_ids = info.genetic_data_ids();
		let valid_ids = T::GeneticData::valid_genetic_data_ids(account_id, genetic_data_ids);

		let opinion_ids = requestor.info().opinion_ids();
		let mut requestor_info = info;

		requestor_info.update_genetic_data_ids(&valid_ids);
		requestor_info.update_opinion_ids(opinion_ids);

		requestor.update_info(requestor_info.clone(), now);

		OpinionRequestors::<T>::insert(requestor_id, &requestor);

		Ok(requestor_info)
	}
}
