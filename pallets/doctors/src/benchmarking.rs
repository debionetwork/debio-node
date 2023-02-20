use super::*;

use crate::DoctorInfo;
#[allow(unused)]
use crate::Pallet as Doctors;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use primitives_area_code::{CityCode, CountryCode, RegionCode};

benchmarks! {
	register_doctor {
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
		let caller: T::AccountId = whitelisted_caller();
	}: register_doctor(
		RawOrigin::Signed(caller),
		doctor
	)

	update_doctor {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(caller.clone()));

		let old_doctor = DoctorInfo {
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
		let _add_doctors = Doctors::<T>::register_doctor(caller_origin.clone(), old_doctor);

		let new_doctor = DoctorInfo {
			name: "DeBio Doctor 2".as_bytes().to_vec(),
			email: "DeBio Email 2".as_bytes().to_vec(),
			country: CountryCode::from_vec("C2".as_bytes().to_vec()),
			region: RegionCode::from_vec("DBI2".as_bytes().to_vec()),
			city: CityCode::from_vec("C2C2".as_bytes().to_vec()),
			address: "DeBio Address 2".as_bytes().to_vec(),
			latitude: Some("DeBio Latitude 2".as_bytes().to_vec()),
			longitude: Some("DeBio Longtitude 2".as_bytes().to_vec()),
			profile_image: Some("DeBio Profile Image owo".as_bytes().to_vec()),
		};
	}: update_doctor(
		RawOrigin::Signed(caller),
		new_doctor
	)

	deregister_doctor {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = <T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(caller.clone()));

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
	}: deregister_doctor(
		RawOrigin::Signed(caller)
	)
}

impl_benchmark_test_suite! {Doctors, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
