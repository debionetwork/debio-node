use super::*;

#[allow(unused)]
use crate::Pallet as GeneticData;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	add_genetic_data {
		let caller: T::AccountId = whitelisted_caller();
	}: add_genetic_data(
        RawOrigin::Signed(caller),
        "DeBio Genetic Data".as_bytes().to_vec(),
        "DeBio Genetic Data Document Description".as_bytes().to_vec(),
        "DeBio Genetic Data Link".as_bytes().to_vec()
    )

	update_genetic_data {
		let caller: T::AccountId = whitelisted_caller();

		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let _add_genetic_data = GeneticData::<T>::add_genetic_data(
            caller_origin.clone(),
            "DeBio Genetic Data".as_bytes().to_vec(),
            "DeBio Genetic Data Document Description".as_bytes().to_vec(),
            "DeBio Genetic Data Link".as_bytes().to_vec()
        );

        let _emr_ids = GeneticData::<T>::genetic_data_by_owner_id(
            caller.clone()
        ).unwrap();
	}: update_genetic_data(
        RawOrigin::Signed(caller),
        _emr_ids[0],
        "DeBio Genetic Data 2".as_bytes().to_vec(),
        "DeBio Genetic Data Document Description 2".as_bytes().to_vec(),
        "DeBio Genetic Data Link 2".as_bytes().to_vec()
    )

	remove_genetic_data {
		let caller: T::AccountId = whitelisted_caller();

		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let _add_genetic_data = GeneticData::<T>::add_genetic_data(
            caller_origin.clone(),
            "DeBio Genetic Data".as_bytes().to_vec(),
            "DeBio Genetic Data Document Description".as_bytes().to_vec(),
            "DeBio Genetic Data Link".as_bytes().to_vec()
        );

        let _emr_ids = GeneticData::<T>::genetic_data_by_owner_id(
            caller.clone()
        ).unwrap();
	}: remove_genetic_data(
        RawOrigin::Signed(caller),
        _emr_ids[0]
    )
}

impl_benchmark_test_suite! {GeneticData, crate::mock::ExternalityBuilder::build(), crate::mock::Test}