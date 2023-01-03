#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;

pub trait OpinionRequestorCountT<T: Config> {
	fn add_opinion_requestor_count(value: u64);
	fn substract_opinion_requestor_count(value: u64);
	fn add_opinion_requestor_count_by_owner(account_id: &T::AccountId, value: u64);
	fn substract_opinion_requestor_count_by_owner(account_id: &T::AccountId, value: u64);
}

pub trait OpinionRequestorProvider<T: Config> {
	fn can_give_opinion(requestor_id: &T::Hash) -> bool;
	fn associate(requestor_id: &T::Hash, opinion_id: &T::Hash);
	fn disassociate(requestor_id: &T::Hash, opinion_id: &T::Hash);
}
