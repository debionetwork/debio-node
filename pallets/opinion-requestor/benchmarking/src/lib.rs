#![cfg_attr(not(feature = "std"), no_std)]
mod mock;

#[allow(unused)]
use opinion_requestor::{
	Config as OpinionRequestorConfig, Pallet as OpinionRequestor, RequestorInfo,
};

use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

use pallet_timestamp::Config as TimestampConfig;

pub struct Pallet<T: Config>(OpinionRequestor<T>);

pub trait Config: OpinionRequestorConfig + TimestampConfig {}

use opinion_requestor::Call;
use sp_std::vec::Vec;

benchmarks! {
	request_opinion {
		let caller: T::AccountId = whitelisted_caller();
		let info = RequestorInfo::new(
			b"category",
			b"description",
			&Vec::new(),
			&Vec::new(),
			b"myriad_url",
		);

	}: request_opinion(RawOrigin::Signed(caller), info)

	update_requestor_info {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));

		let info = RequestorInfo::new(
			b"category",
			b"description",
			&Vec::new(),
			&Vec::new(),
			b"myriad_url",
		);

		let _ = OpinionRequestor::<T>::request_opinion(caller_origin, info);

		let requestor_ids = OpinionRequestor::<T>::opinion_requestor_by_owner(caller.clone());
		let requestor_id = requestor_ids[0];
		let updated_info = RequestorInfo::new(
			b"new_category",
			b"description",
			&Vec::new(),
			&Vec::new(),
			b"myriad_url",
		);
	}: update_requestor_info(RawOrigin::Signed(caller), requestor_id, updated_info)
}
