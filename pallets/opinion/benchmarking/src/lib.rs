#![cfg_attr(not(feature = "std"), no_std)]
mod mock;

#[allow(unused)]
use opinion_requestor::{
	Config as OpinionRequestorConfig, Pallet as OpinionRequestor, RequestorInfo,
};

#[allow(unused)]
use opinion::{Config as OpinionConfig, OpinionAdminKey, OpinionInfo, Pallet as Opinion};

use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use primitives_price_and_currency::CurrencyType;

use pallet_timestamp::Config as TimestampConfig;

pub struct Pallet<T: Config>(Opinion<T>);

pub trait Config: OpinionRequestorConfig + OpinionConfig + TimestampConfig {}

use opinion::Call;
use sp_std::vec::Vec;

const SEED: u32 = 0;

benchmarks! {
	create {
		let caller: T::AccountId = OpinionAdminKey::<T>::get().unwrap();
		let doctor: T::AccountId = account("doctor", 0, SEED);
		let customer: T::AccountId = whitelisted_caller();
		let customer_origin = T::Origin::from(RawOrigin::Signed(customer.clone()));

		let info = RequestorInfo::new(
			b"category",
			b"description",
			&Vec::new(),
			&Vec::new(),
			b"myriad_url",
		);

		let _ = OpinionRequestor::<T>::request_opinion(customer_origin, info);

		let requestor_ids = OpinionRequestor::<T>::opinion_requestor_by_owner(customer);
		let requestor_id = requestor_ids[0];

		let info = OpinionInfo::new(
			b"description".to_vec(),
			b"myriad_url".to_vec(),
			None,
			CurrencyType::DBIO,
			1000,
		);
	}: create(RawOrigin::Signed(caller), requestor_id, doctor, info)

	update {
		let caller: T::AccountId = OpinionAdminKey::<T>::get().unwrap();
		let doctor: T::AccountId = account("doctor", 0, SEED);
		let customer: T::AccountId = whitelisted_caller();

		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let customer_origin = T::Origin::from(RawOrigin::Signed(customer.clone()));

		let info = RequestorInfo::new(
			b"category",
			b"description",
			&Vec::new(),
			&Vec::new(),
			b"myriad_url",
		);

		let _ = OpinionRequestor::<T>::request_opinion(customer_origin, info);

		let requestor_ids = OpinionRequestor::<T>::opinion_requestor_by_owner(customer);
		let requestor_id = requestor_ids[0];

		let info = OpinionInfo::new(
			b"description".to_vec(),
			b"myriad_url".to_vec(),
			None,
			CurrencyType::DBIO,
			1000,
		);

		let _ = Opinion::<T>::create(caller_origin, requestor_id, doctor.clone(), info);

		let opinion_ids = Opinion::<T>::opinion_by_owner(doctor.clone());
		let opinion_id = opinion_ids[0];

		let updated_info = OpinionInfo::new(
			b"description".to_vec(),
			b"url".to_vec(),
			None,
			CurrencyType::DBIO,
			1000,
		);
	}: update(RawOrigin::Signed(caller), opinion_id, doctor, updated_info)

	delete {
		let caller: T::AccountId = OpinionAdminKey::<T>::get().unwrap();
		let doctor: T::AccountId = account("doctor", 0, SEED);
		let customer: T::AccountId = whitelisted_caller();

		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let customer_origin = T::Origin::from(RawOrigin::Signed(customer.clone()));

		let info = RequestorInfo::new(
			b"category",
			b"description",
			&Vec::new(),
			&Vec::new(),
			b"myriad_url",
		);

		let _ = OpinionRequestor::<T>::request_opinion(customer_origin, info);

		let requestor_ids = OpinionRequestor::<T>::opinion_requestor_by_owner(customer);
		let requestor_id = requestor_ids[0];

		let info = OpinionInfo::new(
			b"description".to_vec(),
			b"myriad_url".to_vec(),
			None,
			CurrencyType::DBIO,
			1000,
		);

		let _ = Opinion::<T>::create(caller_origin, requestor_id, doctor.clone(), info);

		let opinion_ids = Opinion::<T>::opinion_by_owner(doctor.clone());
		let opinion_id = opinion_ids[0];
	}: delete(RawOrigin::Signed(caller), doctor, opinion_id)

	update_admin_key {
		let caller: T::AccountId = OpinionAdminKey::<T>::get().unwrap();
		let new_admin: T::AccountId = whitelisted_caller();
	}: update_admin_key(RawOrigin::Signed(caller), new_admin)
}
