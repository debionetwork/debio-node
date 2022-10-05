#![cfg_attr(not(feature = "std"), no_std)]
mod mock;

#[allow(unused)]
use doctor_certifications::Pallet as DoctorCertifications;
use doctor_certifications::{Config as DoctorCertificationsConfig, DoctorCertificationInfo};

#[allow(unused)]
use doctors::Pallet as Doctors;
use doctors::{Config as DoctorsConfig, DoctorInfo};

use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

pub trait Config: DoctorCertificationsConfig + DoctorsConfig {}

pub struct Pallet<T: Config>(DoctorCertifications<T>);

use doctor_certifications::Call;
use primitives_area_code::{CityCode, CountryCode, RegionCode};

benchmarks! {
	create_certification {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));

		let doctor = DoctorInfo {
			name: "DeBio Doctor".as_bytes().to_vec(),
			email: "DeBio Email".as_bytes().to_vec(),
			country: CountryCode::from_vec("DC".as_bytes().to_vec()),
			region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
			city: CityCode::from_vec("City".as_bytes().to_vec()),
			address: "DeBio Address".as_bytes().to_vec(),
			latitude: Some("DeBio Latitude".as_bytes().to_vec()),
			longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
			profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};
		let _add_doctors = Doctors::<T>::register_doctor(caller_origin, doctor);

		let certification = DoctorCertificationInfo {
			title: "DeBio certificate".as_bytes().to_vec(),
			issuer: "DeBio".as_bytes().to_vec(),
			month: "August".as_bytes().to_vec(),
			year: "2021".as_bytes().to_vec(),
			description: "This is my description".as_bytes().to_vec(),
			supporting_document: Some("This is my document".as_bytes().to_vec()),
		};
	}: create_certification(RawOrigin::Signed(caller), certification)

	update_certification {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));

		let doctor = DoctorInfo {
			name: "DeBio Doctor".as_bytes().to_vec(),
			email: "DeBio Email".as_bytes().to_vec(),
			country: CountryCode::from_vec("DC".as_bytes().to_vec()),
			region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
			city: CityCode::from_vec("City".as_bytes().to_vec()),
			address: "DeBio Address".as_bytes().to_vec(),
			latitude: Some("DeBio Latitude".as_bytes().to_vec()),
			longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
			profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};
		let _add_doctors = Doctors::<T>::register_doctor(caller_origin.clone(), doctor);

		let old_certification = DoctorCertificationInfo {
			title: "DeBio certificate".as_bytes().to_vec(),
			issuer: "DeBio".as_bytes().to_vec(),
			month: "August".as_bytes().to_vec(),
			year: "2021".as_bytes().to_vec(),
			description: "This is my description".as_bytes().to_vec(),
			supporting_document: Some("This is my document".as_bytes().to_vec()),
		};
		let _create_certification = DoctorCertifications::<T>::create_certification(caller_origin, old_certification);
		let _doctor = Doctors::<T>::doctor_by_account_id(caller.clone())
			.unwrap();

		let new_certification = DoctorCertificationInfo {
			title: "DeBio certificate 2".as_bytes().to_vec(),
			issuer: "DeBio 2".as_bytes().to_vec(),
			month: "September".as_bytes().to_vec(),
			year: "2022".as_bytes().to_vec(),
			description: "This is my description 2".as_bytes().to_vec(),
			supporting_document: Some("This is my document 2".as_bytes().to_vec()),
		};
	}: update_certification(RawOrigin::Signed(caller), _doctor.certifications[0], new_certification)

	delete_certification {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));

		let doctor = DoctorInfo {
			name: "DeBio Doctor".as_bytes().to_vec(),
			email: "DeBio Email".as_bytes().to_vec(),
			country: CountryCode::from_vec("DC".as_bytes().to_vec()),
			region: RegionCode::from_vec("DBIO".as_bytes().to_vec()),
			city: CityCode::from_vec("City".as_bytes().to_vec()),
			address: "DeBio Address".as_bytes().to_vec(),
			latitude: Some("DeBio Latitude".as_bytes().to_vec()),
			longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
			profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
		};
		let _add_doctors = Doctors::<T>::register_doctor(caller_origin.clone(), doctor);

		let old_certification = DoctorCertificationInfo {
			title: "DeBio certificate".as_bytes().to_vec(),
			issuer: "DeBio".as_bytes().to_vec(),
			month: "August".as_bytes().to_vec(),
			year: "2021".as_bytes().to_vec(),
			description: "This is my description".as_bytes().to_vec(),
			supporting_document: Some("This is my document".as_bytes().to_vec()),
		};
		let _create_certification = DoctorCertifications::<T>::create_certification(caller_origin, old_certification);
		let _doctor = Doctors::<T>::doctor_by_account_id(caller.clone())
			.unwrap();
	}: delete_certification(RawOrigin::Signed(caller), _doctor.certifications[0])
}
