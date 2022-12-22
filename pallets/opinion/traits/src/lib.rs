#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;

pub trait OpinionCountT<T: Config> {
	fn add_opinion_count(value: u64);
	fn substract_opinion_count(value: u64);
	fn add_opinion_count_by_owner(account_id: &T::AccountId, value: u64);
	fn substract_opinion_count_by_owner(account_id: &T::AccountId, value: u64);
}
