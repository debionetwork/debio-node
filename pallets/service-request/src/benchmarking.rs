use super::*;

#[allow(unused)]
use crate::{
	Pallet as ServiceRequest,
	AdminKey,
	Config as ServiceRequestConfig,
	RequestByAccountId,
};
use labs::{LabInfo, LabVerifierKey, LabVerificationStatus};
use frame_benchmarking::{benchmarks, whitelisted_caller, impl_benchmark_test_suite, account};
use frame_support::sp_runtime::{SaturatedConversion, traits::Hash};
use frame_system::RawOrigin;
use labs::Pallet as PalletLab;

use primitives_area_code::{CountryCode, RegionCode, CityCode};

const SEED: u32 = 0;

benchmarks! {
	create_request {
		let caller: T::AccountId = whitelisted_caller();
		let _ = <T as pallet::Config>::Currency::deposit_creating(&caller, 1000000000000000000000u128.saturated_into());

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
		let _add_labs = PalletLab::<T>::register_lab(caller_origin.clone(), lab);

		// verified lab
		let admin_key: T::AccountId = LabVerifierKey::<T>::get();
		let admin_key_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(admin_key.clone()));
		let _verified_labs = PalletLab::<T>::update_lab_verification_status(admin_key_origin, caller.clone(), LabVerificationStatus::Verified);

		// Create request
		let customer: T::AccountId = account("recepient", 0, SEED);
		let customer_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(customer.clone()));
		let _ = <T as pallet::Config>::Currency::deposit_creating(&customer, 1000000000000000000000u128.saturated_into());

		let _new_request = ServiceRequest::<T>::create_request(
			customer_origin,
			"Indonesia".as_bytes().to_vec(),
			"West Java".as_bytes().to_vec(),
			"Bogor".as_bytes().to_vec(),
			"Vaksin".as_bytes().to_vec(),
			10000000000000000000u128.saturated_into()
		);

		let request_ids = RequestByAccountId::<T>::get(customer.clone());

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

		let _ = <T as pallet::Config>::Currency::deposit_creating(&caller, 1000000000000000000000u128.saturated_into());

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
		let _add_lab = PalletLab::<T>::register_lab(lab_origin.clone(), lab);

		// verified lab
		let admin_key: T::AccountId = LabVerifierKey::<T>::get();
		let admin_key_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(admin_key.clone()));
		let _verified_labs = PalletLab::<T>::update_lab_verification_status(admin_key_origin, lab_id.clone(), LabVerificationStatus::Verified);

		let _claim_request = ServiceRequest::<T>::claim_request(
			lab_origin.clone(),
			request_id.clone(),
			service_id.clone(),
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
		let _add_lab = PalletLab::<T>::register_lab(lab_id_origin.clone(), lab);

		// verified lab
		let admin_key: T::AccountId = LabVerifierKey::<T>::get();
		let admin_key_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(admin_key.clone()));
		let _verified_labs = PalletLab::<T>::update_lab_verification_status(admin_key_origin, lab_id.clone(), LabVerificationStatus::Verified);

		let _ = <T as pallet::Config>::Currency::deposit_creating(&customer_id, 1000000000000000000000u128.saturated_into());

		// Create request
		let _new_request = ServiceRequest::<T>::create_request(
			customer_id_origin.clone(),
			"Indonesia".as_bytes().to_vec(),
			"West Java".as_bytes().to_vec(),
			"Bogor".as_bytes().to_vec(),
			"Vaksin".as_bytes().to_vec(),
			10000000000000000000u128.saturated_into()
		);

		let request_ids = RequestByAccountId::<T>::get(customer_id.clone());
		let request_id = request_ids[0];

		// claim request
		let service_id = T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC576143".as_bytes());
		let testing_price = 1000000000000000000u128.saturated_into();
		let qc_price = 1000000000000000000u128.saturated_into();
		let _claim_request = ServiceRequest::<T>::claim_request(
			lab_id_origin.clone(),
			request_id.clone(),
			service_id.clone(),
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
			request_id.clone(),
			order_id,
			dna_sample_tracking_id,
			additional_staking_amount
		);
	}: finalize_request(
		RawOrigin::Signed(caller),
		request_id,
		true
	)
}

impl_benchmark_test_suite!(
	ServiceRequest,
	crate::mock::ExternalityBuilder::build(),
	crate::mock::Test,
);
