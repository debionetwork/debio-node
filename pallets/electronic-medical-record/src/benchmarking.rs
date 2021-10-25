use super::*;

#[allow(unused)]
use crate::Pallet as ElectronicMedicalRecord;
use crate::ElectronicMedicalRecordInfo;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, vec, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::vec::Vec;

use frame_support::sp_runtime::traits::Hash;
use frame_support::sp_std::convert::TryInto;

benchmarks! {
	add_electronic_medical_record {
		let caller: T::AccountId = whitelisted_caller();
	}: add_electronic_medical_record(
        RawOrigin::Signed(caller)
    )

	remove_electronic_medical_record {
		let caller: T::AccountId = whitelisted_caller();

		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let _add_electronic_medical_record = ElectronicMedicalRecord::<T>::add_electronic_medical_record(caller_origin.clone());
	}: remove_electronic_medical_record(
        RawOrigin::Signed(caller)
    )
    
	add_electronic_medical_record_info {
		let caller: T::AccountId = whitelisted_caller();

		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let _add_electronic_medical_record = ElectronicMedicalRecord::<T>::add_electronic_medical_record(caller_origin.clone());
	}: add_electronic_medical_record_info(
        RawOrigin::Signed(caller),
        "DeBio EMR".as_bytes().to_vec(),
        "DeBio EMR Category".as_bytes().to_vec(),
        "DeBio EMR Document Title".as_bytes().to_vec(),
        "DeBio EMR Description".as_bytes().to_vec(),
        "DeBio EMR Link".as_bytes().to_vec()
    )
    
	remove_electronic_medical_record_info {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));

		let _add_electronic_medical_record = ElectronicMedicalRecord::<T>::add_electronic_medical_record(caller_origin.clone());

		let _add_electronic_medical_record_info = ElectronicMedicalRecord::<T>::add_electronic_medical_record_info(
            caller_origin.clone(),
            "DeBio EMR".as_bytes().to_vec(),
            "DeBio EMR Category".as_bytes().to_vec(),
            "DeBio EMR Document Title".as_bytes().to_vec(),
            "DeBio EMR Description".as_bytes().to_vec(),
            "DeBio EMR Link".as_bytes().to_vec(),
        );

        let _emr_info = ElectronicMedicalRecord::<T>::electronic_medical_record_by_owner_id(caller.clone())
            .unwrap();
	}: remove_electronic_medical_record_info(
        RawOrigin::Signed(caller), 
        _emr_info.info[0]
    )
}

impl_benchmark_test_suite! {ElectronicMedicalRecord, crate::mock::ExternalityBuilder::build(), crate::mock::Test}