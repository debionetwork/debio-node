#![cfg_attr(not(feature = "std"), no_std)]
mod mock;

#[allow(unused)]
use labs::Pallet as Labs;
use labs::{Call, Config as LabsConfig, LabInfo, LabVerifierKey};

pub struct Pallet<T: Config>(Labs<T>);

pub trait Config: LabsConfig {}

use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

use frame_support::{
	sp_runtime::{traits::Hash, SaturatedConversion},
	traits::Currency,
};
use primitives_area_code::{CityCode, CountryCode, RegionCode};
use primitives_verification_status::VerificationStatus;

benchmarks! {
	register_lab {
		let lab = LabInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			name: "DeBio Lab".as_bytes().to_vec(),
			email: "DeBio Email".as_bytes().to_vec(),
			country: CountryCode::from_vec("DC".as_bytes().to_vec()),
			region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
			city: CityCode::from_vec("City".as_bytes().to_vec()),
			address: "DeBio Address".as_bytes().to_vec(),
			phone_number: "+6281394653625".as_bytes().to_vec(),
			website: "DeBio Website".as_bytes().to_vec(),
			latitude: Some("DeBio Latitude".as_bytes().to_vec()),
			longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
			profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};
		let caller: T::AccountId = whitelisted_caller();
	}: register_lab(
		RawOrigin::Signed(caller),
		lab
	)

	update_lab {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(caller.clone()));

		let old_lab = LabInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			name: "DeBio Lab".as_bytes().to_vec(),
			email: "DeBio Email".as_bytes().to_vec(),
			country: CountryCode::from_vec("DC".as_bytes().to_vec()),
			region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
			city: CityCode::from_vec("City".as_bytes().to_vec()),
			address: "DeBio Address".as_bytes().to_vec(),
			phone_number: "+6281394653625".as_bytes().to_vec(),
			website: "DeBio Website".as_bytes().to_vec(),
			latitude: Some("DeBio Latitude".as_bytes().to_vec()),
			longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
			profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};
		let _add_labs = Labs::<T>::register_lab(caller_origin, old_lab);

		let new_lab = LabInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			name: "DeBio Lab 2".as_bytes().to_vec(),
			email: "DeBio Email 2".as_bytes().to_vec(),
			country: CountryCode::from_vec("C2".as_bytes().to_vec()),
			region: RegionCode::from_vec("DBI2".as_bytes().to_vec()),
			city: CityCode::from_vec("C2C2".as_bytes().to_vec()),
			address: "DeBio Address 2".as_bytes().to_vec(),
			phone_number: "+6281394653625".as_bytes().to_vec(),
			website: "DeBio Website".as_bytes().to_vec(),
			latitude: Some("DeBio Latitude 2".as_bytes().to_vec()),
			longitude: Some("DeBio Longtitude 2".as_bytes().to_vec()),
			profile_image: Some("DeBio Profile Image owo".as_bytes().to_vec()),
		};
	}: update_lab(
		RawOrigin::Signed(caller),
		new_lab
	)

	update_lab_verification_status {
		let caller: T::AccountId = LabVerifierKey::<T>::get().unwrap();
		let caller_origin = <T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(caller.clone()));

		let old_lab = LabInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			name: "DeBio Lab".as_bytes().to_vec(),
			email: "DeBio Email".as_bytes().to_vec(),
			country: CountryCode::from_vec("DC".as_bytes().to_vec()),
			region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
			city: CityCode::from_vec("City".as_bytes().to_vec()),
			address: "DeBio Address".as_bytes().to_vec(),
			phone_number: "+6281394653625".as_bytes().to_vec(),
			website: "DeBio Website".as_bytes().to_vec(),
			latitude: Some("DeBio Latitude".as_bytes().to_vec()),
			longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
			profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};
		let _add_labs = Labs::<T>::register_lab(caller_origin, old_lab);
	}: update_lab_verification_status(
		RawOrigin::Signed(caller),
		caller.clone(),
		VerificationStatus::default()
	)

	deregister_lab {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(caller.clone()));

		let lab = LabInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			name: "DeBio Lab".as_bytes().to_vec(),
			email: "DeBio Email".as_bytes().to_vec(),
			country: CountryCode::from_vec("DC".as_bytes().to_vec()),
			region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
			city: CityCode::from_vec("City".as_bytes().to_vec()),
			address: "DeBio Address".as_bytes().to_vec(),
			phone_number: "+6281394653625".as_bytes().to_vec(),
			website: "DeBio Website".as_bytes().to_vec(),
			latitude: Some("DeBio Latitude".as_bytes().to_vec()),
			longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
			profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};

		let _add_labs = Labs::<T>::register_lab(caller_origin, lab);
	}: deregister_lab(
		RawOrigin::Signed(caller)
	)

	stake_lab {
		let caller: T::AccountId = LabVerifierKey::<T>::get().unwrap();
		let caller_origin = <T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(caller.clone()));

		let _ = <T as labs::Config>::Currency::deposit_creating(&caller, 60000000000000000000000u128.saturated_into());

		let old_lab = LabInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			name: "DeBio Lab".as_bytes().to_vec(),
			email: "DeBio Email".as_bytes().to_vec(),
			country: CountryCode::from_vec("DC".as_bytes().to_vec()),
			region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
			city: CityCode::from_vec("City".as_bytes().to_vec()),
			address: "DeBio Address".as_bytes().to_vec(),
			phone_number: "+6281394653625".as_bytes().to_vec(),
			website: "DeBio Website".as_bytes().to_vec(),
			latitude: Some("DeBio Latitude".as_bytes().to_vec()),
			longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
			profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};
		let _ = Labs::<T>::register_lab(caller_origin, old_lab);
	}: stake_lab(
		RawOrigin::Signed(caller)
	)

	unstake_lab {
		let caller: T::AccountId = LabVerifierKey::<T>::get().unwrap();
		let caller_origin = <T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(caller.clone()));

		let _ = <T as labs::Config>::Currency::deposit_creating(&caller, 60000000000000000000000u128.saturated_into());

		let old_lab = LabInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			name: "DeBio Lab".as_bytes().to_vec(),
			email: "DeBio Email".as_bytes().to_vec(),
			country: CountryCode::from_vec("DC".as_bytes().to_vec()),
			region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
			city: CityCode::from_vec("City".as_bytes().to_vec()),
			address: "DeBio Address".as_bytes().to_vec(),
			phone_number: "+6281394653625".as_bytes().to_vec(),
			website: "DeBio Website".as_bytes().to_vec(),
			latitude: Some("DeBio Latitude".as_bytes().to_vec()),
			longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
			profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};
		let _ = Labs::<T>::register_lab(caller_origin.clone(), old_lab);

		let _ = Labs::<T>::stake_lab(caller_origin);
	}: unstake_lab(
		RawOrigin::Signed(caller)
	)

	retrieve_unstake_amount {
		let caller_admin: T::AccountId = LabVerifierKey::<T>::get().unwrap();
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(caller.clone()));

		let _ = <T as labs::Config>::Currency::deposit_creating(&caller, 60000000000000000000000u128.saturated_into());

		let old_lab = LabInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
			name: "DeBio Lab".as_bytes().to_vec(),
			email: "DeBio Email".as_bytes().to_vec(),
			country: CountryCode::from_vec("DC".as_bytes().to_vec()),
			region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
			city: CityCode::from_vec("City".as_bytes().to_vec()),
			address: "DeBio Address".as_bytes().to_vec(),
			phone_number: "+6281394653625".as_bytes().to_vec(),
			website: "DeBio Website".as_bytes().to_vec(),
			latitude: Some("DeBio Latitude".as_bytes().to_vec()),
			longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
			profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};
		let _ = Labs::<T>::register_lab(caller_origin.clone(), old_lab);

		let _ = Labs::<T>::stake_lab(caller_origin.clone());

		let _ = Labs::<T>::unstake_lab(caller_origin);
	}: retrieve_unstake_amount(
		RawOrigin::Signed(caller_admin),
		caller
	)

	update_minimum_stake_amount {
		let caller: T::AccountId = LabVerifierKey::<T>::get().unwrap();
	}: update_minimum_stake_amount(
		RawOrigin::Signed(caller),
		60000000000000000000000u128.saturated_into()
	)

	update_unstake_time {
		let caller: T::AccountId = LabVerifierKey::<T>::get().unwrap();
	}: update_unstake_time(
		RawOrigin::Signed(caller),
		0u64.saturated_into()
	)

	update_admin_key {
		let caller: T::AccountId = LabVerifierKey::<T>::get().unwrap();
		let caller2: T::AccountId = whitelisted_caller();
	}: update_admin_key(
		RawOrigin::Signed(caller),
		caller2
	)
}
