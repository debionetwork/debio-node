use crate::*;
use frame_support::{codec::Encode, sp_runtime::traits::Hash};
use traits_opinion_requestor::{OpinionRequestorCountT, OpinionRequestorProvider};

impl<T: Config> Pallet<T> {
	pub fn generate_opinion_requestor_id(
		account_id: &T::AccountId,
		total_requestor: u64,
	) -> T::Hash {
		let mut account_bytes = account_id.encode();
		let mut total_requestor_bytes = total_requestor.encode();
		let mut nonce = frame_system::Pallet::<T>::account(account_id).nonce.encode();

		account_bytes.append(&mut total_requestor_bytes);
		account_bytes.append(&mut nonce);

		let seed = &account_bytes;
		T::Hashing::hash(seed)
	}
}

impl<T: Config> OpinionRequestorCountT<T> for Pallet<T> {
	fn add_opinion_requestor_count(value: u64) {
		OpinionRequestorCount::<T>::mutate(|count| {
			*count = count.saturating_add(value);
		});
	}

	fn substract_opinion_requestor_count(value: u64) {
		OpinionRequestorCount::<T>::mutate(|count| {
			*count = count.saturating_sub(value);
		});
	}

	fn add_opinion_requestor_count_by_owner(account_id: &T::AccountId, value: u64) {
		OpinionRequestorCountByOwner::<T>::mutate(account_id, |count| {
			*count = count.saturating_add(value);
		});
	}

	fn substract_opinion_requestor_count_by_owner(
		account_id: &<T as frame_system::Config>::AccountId,
		value: u64,
	) {
		OpinionRequestorCountByOwner::<T>::mutate(account_id, |count| {
			*count = count.saturating_sub(value);
		});
	}
}

impl<T: Config> OpinionRequestorProvider<T> for Pallet<T> {
	fn can_give_opinion(requestor_id: &T::Hash) -> bool {
		OpinionRequestors::<T>::contains_key(requestor_id)
	}

	fn associate(requestor_id: &T::Hash, opinion_id: &T::Hash) {
		OpinionRequestors::<T>::mutate(
			requestor_id,
			|result: &mut Option<OpinionRequestorOf<T>>| match result {
				None => (),
				Some(opinion_requestor) => {
					opinion_requestor.add_opinion(*opinion_id);
				},
			},
		);
	}

	fn disassociate(requestor_id: &T::Hash, opinion_id: &T::Hash) {
		OpinionRequestors::<T>::mutate(
			requestor_id,
			|result: &mut Option<OpinionRequestorOf<T>>| match result {
				None => (),
				Some(requestor) => {
					requestor.remove_opinion(*opinion_id);
				},
			},
		);
	}
}
