use super::*;

#[allow(unused)]
use crate::Pallet as Certifications;
use crate::CertificationInfo;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	create_certification {
		let s in 0 .. 100;
		let t = CertificationInfo {
			title: "DeBio certificate".as_bytes().to_vec(),
			issuer: "DeBio".as_bytes().to_vec(),
			month: "August".as_bytes().to_vec(),
			year: "2021".as_bytes().to_vec(),
			description: "This is my description".as_bytes().to_vec(),
			supporting_document: Some("This is my document".as_bytes().to_vec()),
		};
		let caller: T::AccountId = whitelisted_caller();
	}: create_certification(RawOrigin::Signed(caller), t)
}

impl_benchmark_test_suite! {Certifications, crate::mock::ExternalityBuilder::build(), crate::mock::Test}