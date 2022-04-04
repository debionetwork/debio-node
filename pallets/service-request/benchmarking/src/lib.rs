#![cfg_attr(not(feature = "std"), no_std)]
mod mock;

use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{
	sp_runtime::{traits::Hash, SaturatedConversion},
	traits::Currency,
};
use frame_system::RawOrigin;
use labs::{Config as LabsConfig, LabInfo, LabVerifierKey, Pallet as Labs};
#[allow(unused)]
use service_request::{
	AdminKey, Call, Config as ServiceRequestConfig, Pallet as ServiceRequest, RequestByAccountId,
};

use primitives_area_code::{CityCode, CountryCode, RegionCode};
use primitives_verification_status::VerificationStatus;

pub trait Config: ServiceRequestConfig + LabsConfig {}

pub struct Pallet<T: Config>(ServiceRequest<T>);

const SEED: u32 = 0;

benchmarks! {
	create_request {
		let caller: T::AccountId = whitelisted_caller();
		let _ = <T as service_request::Config>::Currency::deposit_creating(&caller, 1000000000000000000000u128.saturated_into());

		let country = "Indonesia".as_bytes().to_vec();
		let region = "West Java".as_bytes().to_vec();
		let city = "Bogor".as_bytes().to_vec();
		let service_category = "Vaksin".as_bytes().to_vec();
		let total_staked = 10000000000000000000u128.saturated_into();
	}: create_request(
		RawOrigin::Signed(caller),
		country,
		region,
		city,
		service_category,
		total_staked
	)

	unstake {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let _ = <T as service_request::Config>::Currency::deposit_creating(&caller, 1000000000000000000000u128.saturated_into());

		let _new_request = ServiceRequest::<T>::create_request(
			caller_origin,
			"Indonesia".as_bytes().to_vec(),
			"West Java".as_bytes().to_vec(),
			"Bogor".as_bytes().to_vec(),
			"Vaksin".as_bytes().to_vec(),
			10000000000000000000u128.saturated_into()
		);

		let request_ids = RequestByAccountId::<T>::get(caller.clone());
		let request_id = request_ids[0];


	}: unstake(
		RawOrigin::Signed(caller),
		request_ids[0]
	)

	retrieve_unstaked_amount {
		let caller: T::AccountId = AdminKey::<T>::get();

		let customer: T::AccountId = account("customer", 0, SEED);
		let customer_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(customer.clone()));

		let _ = <T as service_request::Config>::Currency::deposit_creating(&customer, 1000000000000000000000u128.saturated_into());

		let _new_request = ServiceRequest::<T>::create_request(
			customer_origin.clone(),
			"Indonesia".as_bytes().to_vec(),
			"West Java".as_bytes().to_vec(),
			"Bogor".as_bytes().to_vec(),
			"Vaksin".as_bytes().to_vec(),
			10000000000000000000u128.saturated_into()
		);

		let request_ids = RequestByAccountId::<T>::get(customer);
		let request_id = request_ids[0];

		let _request_unstake = ServiceRequest::<T>::unstake(customer_origin, request_id);
	}: retrieve_unstaked_amount (
		RawOrigin::Signed(caller),
		request_id
	)

	claim_request {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		// Set lab info
		let lab = LabInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			name: "DeBio Lab".as_bytes().to_vec(),
			email: "DeBio Email".as_bytes().to_vec(),
			country: CountryCode::from_vec("DC".as_bytes().to_vec()),
			region: RegionCode::from_vec("DB".as_bytes().to_vec()),
			city: CityCode::from_vec("CITY".as_bytes().to_vec()),
			address: "DeBio Address".as_bytes().to_vec(),
			phone_number: "+6281394653625".as_bytes().to_vec(),
			website: "DeBio Website".as_bytes().to_vec(),
			latitude: Some("DeBio Latitude".as_bytes().to_vec()),
			longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
			profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};

		// register lab
		let _add_labs = Labs::<T>::register_lab(caller_origin, lab);

		// verified lab
		let admin_key: T::AccountId = LabVerifierKey::<T>::get();
		let admin_key_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(admin_key));
		let _verified_labs = Labs::<T>::update_lab_verification_status(admin_key_origin, caller.clone(), VerificationStatus::Verified);

		// Create request
		let customer: T::AccountId = account("recepient", 0, SEED);
		let customer_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(customer.clone()));
		let _ = <T as service_request::Config>::Currency::deposit_creating(&customer, 1000000000000000000000u128.saturated_into());

		let _new_request = ServiceRequest::<T>::create_request(
			customer_origin,
			"Indonesia".as_bytes().to_vec(),
			"West Java".as_bytes().to_vec(),
			"Bogor".as_bytes().to_vec(),
			"Vaksin".as_bytes().to_vec(),
			10000000000000000000u128.saturated_into()
		);

		let request_ids = RequestByAccountId::<T>::get(customer);

		let request_id = request_ids[0];
		let service_id = T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC576143".as_bytes());
		let testing_price = 10000000000000000000u128.saturated_into();
		let qc_price = 10000000000000000000u128.saturated_into();
	}: claim_request(
		RawOrigin::Signed(caller),
		request_id,
		service_id,
		testing_price,
		qc_price
	)

	process_request {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

		let _ = <T as service_request::Config>::Currency::deposit_creating(&caller, 1000000000000000000000u128.saturated_into());

		let _new_request = ServiceRequest::<T>::create_request(
			caller_origin,
			"Indonesia".as_bytes().to_vec(),
			"West Java".as_bytes().to_vec(),
			"Bogor".as_bytes().to_vec(),
			"Vaksin".as_bytes().to_vec(),
			10000000000000000000u128.saturated_into()
		);

		let request_ids = RequestByAccountId::<T>::get(caller.clone());

		let request_id = request_ids[0];
		let service_id = T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC576143".as_bytes());
		let testing_price = 1000000000000000000u128.saturated_into();
		let qc_price = 1000000000000000000u128.saturated_into();

		let lab_id: T::AccountId = account("recepient", 0, SEED);
		let lab_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(lab_id.clone()));
		// Set lab info
		let lab = LabInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			name: "DeBio Lab".as_bytes().to_vec(),
			email: "DeBio Email".as_bytes().to_vec(),
			country: CountryCode::from_vec("DC".as_bytes().to_vec()),
			region: RegionCode::from_vec("DB".as_bytes().to_vec()),
			city: CityCode::from_vec("CITY".as_bytes().to_vec()),
			address: "DeBio Address".as_bytes().to_vec(),
			phone_number: "+6281394653625".as_bytes().to_vec(),
			website: "DeBio Website".as_bytes().to_vec(),
			latitude: Some("DeBio Latitude".as_bytes().to_vec()),
			longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
			profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};
		let _add_lab = Labs::<T>::register_lab(lab_origin.clone(), lab);

		// verified lab
		let admin_key: T::AccountId = LabVerifierKey::<T>::get();
		let admin_key_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(admin_key));
		let _verified_labs = Labs::<T>::update_lab_verification_status(admin_key_origin, lab_id.clone(), VerificationStatus::Verified);

		let _claim_request = ServiceRequest::<T>::claim_request(
			lab_origin,
			request_id,
			service_id,
			testing_price,
			qc_price
		);

		let order_id = T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC576143".as_bytes());
		let dna_sample_tracking_id = "DeBio Sample".as_bytes().to_vec();
		let additional_staking_amount = 0u128.saturated_into();

	}: process_request(
		RawOrigin::Signed(caller),
		lab_id,
		request_id,
		order_id,
		dna_sample_tracking_id,
		additional_staking_amount
	)

	finalize_request{
		let caller: T::AccountId = AdminKey::<T>::get();

		let lab_id: T::AccountId = account("lab", 0, SEED);
		let customer_id: T::AccountId = account("customer", 0, SEED);

		let lab_id_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(lab_id.clone()));
		let customer_id_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(customer_id.clone()));

		// Register lab
		let lab = LabInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			name: "DeBio Lab".as_bytes().to_vec(),
			email: "DeBio Email".as_bytes().to_vec(),
			country: CountryCode::from_vec("DC".as_bytes().to_vec()),
			region: RegionCode::from_vec("DB".as_bytes().to_vec()),
			city: CityCode::from_vec("CITY".as_bytes().to_vec()),
			address: "DeBio Address".as_bytes().to_vec(),
			phone_number: "+6281394653625".as_bytes().to_vec(),
			website: "DeBio Website".as_bytes().to_vec(),
			latitude: Some("DeBio Latitude".as_bytes().to_vec()),
			longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
			profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};
		let _add_lab = Labs::<T>::register_lab(lab_id_origin.clone(), lab);

		// verified lab
		let admin_key: T::AccountId = LabVerifierKey::<T>::get();
		let admin_key_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(admin_key));
		let _verified_labs = Labs::<T>::update_lab_verification_status(admin_key_origin, lab_id.clone(), VerificationStatus::Verified);

		let _ = <T as service_request::Config>::Currency::deposit_creating(&customer_id, 1000000000000000000000u128.saturated_into());

		// Create request
		let _new_request = ServiceRequest::<T>::create_request(
			customer_id_origin.clone(),
			"Indonesia".as_bytes().to_vec(),
			"West Java".as_bytes().to_vec(),
			"Bogor".as_bytes().to_vec(),
			"Vaksin".as_bytes().to_vec(),
			10000000000000000000u128.saturated_into()
		);

		let request_ids = RequestByAccountId::<T>::get(customer_id);
		let request_id = request_ids[0];

		// claim request
		let service_id = T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC576143".as_bytes());
		let testing_price = 1000000000000000000u128.saturated_into();
		let qc_price = 1000000000000000000u128.saturated_into();
		let _claim_request = ServiceRequest::<T>::claim_request(
			lab_id_origin,
			request_id,
			service_id,
			testing_price,
			qc_price
		);

		// process request
		let order_id = T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC576143".as_bytes());
		let dna_sample_tracking_id = "DeBio Sample".as_bytes().to_vec();
		let additional_staking_amount = 0u128.saturated_into();
		let _process_request = ServiceRequest::<T>::process_request(
			customer_id_origin,
			lab_id,
			request_id,
			order_id,
			dna_sample_tracking_id,
			additional_staking_amount
		);
	}: finalize_request(
		RawOrigin::Signed(caller),
		request_id,
		true
	)

	update_admin_key {
		let caller: T::AccountId = AdminKey::<T>::get();
		let caller2: T::AccountId = whitelisted_caller();
	}: update_admin_key(
		RawOrigin::Signed(caller),
		caller2
	)
}

impl_benchmark_test_suite!(
	ServiceRequest,
	crate::mock::ExternalityBuilder::build(),
	crate::mock::Test,
);
