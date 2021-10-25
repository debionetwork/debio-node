use super::*;

#[allow(unused)]
use crate::Pallet as ElectronicMedicalRecord;
use crate::ElectronicMedicalRecordFile;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, vec, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	add_electronic_medical_record {
		let caller: T::AccountId = whitelisted_caller();
	}: add_electronic_medical_record(
        RawOrigin::Signed(caller),
        "DeBio EMR".as_bytes().to_vec(),
        "DeBio EMR Category".as_bytes().to_vec()
    )

	remove_electronic_medical_record {
		let caller: T::AccountId = whitelisted_caller();

		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let _add_electronic_medical_record = ElectronicMedicalRecord::<T>::add_electronic_medical_record(
            caller_origin.clone(),
            "DeBio EMR".as_bytes().to_vec(),
            "DeBio EMR Category".as_bytes().to_vec()
        );
	}: remove_electronic_medical_record(
        RawOrigin::Signed(caller)
    )
    
	add_electronic_medical_record_file {
		let caller: T::AccountId = whitelisted_caller();

		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let _add_electronic_medical_record = ElectronicMedicalRecord::<T>::add_electronic_medical_record(
            caller_origin.clone(),
            "DeBio EMR".as_bytes().to_vec(),
            "DeBio EMR Category".as_bytes().to_vec()
        );
	}: add_electronic_medical_record_file(
        RawOrigin::Signed(caller),
        "DeBio EMR Document Title".as_bytes().to_vec(),
        "DeBio EMR Description".as_bytes().to_vec(),
        "DeBio EMR Link".as_bytes().to_vec()
    )
    
	remove_electronic_medical_record_file {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));

		let _add_electronic_medical_record = ElectronicMedicalRecord::<T>::add_electronic_medical_record(
            caller_origin.clone(),
            "DeBio EMR".as_bytes().to_vec(),
            "DeBio EMR Category".as_bytes().to_vec(),
        );

		let _add_electronic_medical_record_file = ElectronicMedicalRecord::<T>::add_electronic_medical_record_file(
            caller_origin.clone(),
            "DeBio EMR Document Title".as_bytes().to_vec(),
            "DeBio EMR Description".as_bytes().to_vec(),
            "DeBio EMR Link".as_bytes().to_vec(),
        );

        let _emr_info = ElectronicMedicalRecord::<T>::electronic_medical_record_by_owner_id(caller.clone())
            .unwrap();
	}: remove_electronic_medical_record_file(
        RawOrigin::Signed(caller), 
        _emr_info.files[0]
    )
}

impl_benchmark_test_suite! {ElectronicMedicalRecord, crate::mock::ExternalityBuilder::build(), crate::mock::Test}