#![cfg_attr(not(feature = "std"), no_std)]
mod mock;

#[allow(unused)]
use genetic_analyst_qualifications::Pallet as GeneticAnalystQualifications;
use genetic_analyst_qualifications::{
	Config as GeneticAnalystQualificationsConfig, GeneticAnalystCertification,
	GeneticAnalystExperience, GeneticAnalystQualificationInfo,
};

#[allow(unused)]
use genetic_analysts::Pallet as GeneticAnalysts;
use genetic_analysts::{Config as GeneticAnalystsConfig, GeneticAnalystInfo};

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

pub struct Pallet<T: Config>(GeneticAnalystQualifications<T>);

pub trait Config: GeneticAnalystQualificationsConfig + GeneticAnalystsConfig {}

use genetic_analyst_qualifications::Call;
use sp_std::vec;

benchmarks! {
	create_qualification {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));

		let genetic_analyst = GeneticAnalystInfo {
			box_public_key: Keccak256::hash(
				"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
			),
			first_name: "First Name".as_bytes().to_vec(),
			last_name: "Last Name".as_bytes().to_vec(),
			gender: "Gender".as_bytes().to_vec(),
			date_of_birth: <T as pallet_timestamp::pallet::Config>::Moment::default(),
			email: "Email".as_bytes().to_vec(),
			phone_number: "+6893026516".as_bytes().to_vec(),
			specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
			profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
			profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
		};
		let _add_genetic_analysts = GeneticAnalysts::<T>::register_genetic_analyst(caller_origin, genetic_analyst);

		let qualification = GeneticAnalystQualificationInfo {
			experience: vec![
				GeneticAnalystExperience {
					title: "DeBio title".as_bytes().to_vec(),
				}
			],
			certification: Some(
				vec![
					GeneticAnalystCertification {
						title: "DeBio title".as_bytes().to_vec(),
						issuer: "DeBio issuer".as_bytes().to_vec(),
						month: "DeBio month".as_bytes().to_vec(),
						year: "DeBio year".as_bytes().to_vec(),
						description: "DeBio description".as_bytes().to_vec(),
						supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
					}
				]
			),
		};
	}: create_qualification(RawOrigin::Signed(caller), qualification)

	update_qualification {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));

		let genetic_analyst = GeneticAnalystInfo {
			box_public_key: Keccak256::hash(
				"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
			),
			first_name: "First Name".as_bytes().to_vec(),
			last_name: "Last Name".as_bytes().to_vec(),
			gender: "Gender".as_bytes().to_vec(),
			date_of_birth: <T as pallet_timestamp::pallet::Config>::Moment::default(),
			email: "Email".as_bytes().to_vec(),
			phone_number: "+6893026516".as_bytes().to_vec(),
			specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
			profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
			profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
		};
		let _add_genetic_analysts = GeneticAnalysts::<T>::register_genetic_analyst(caller_origin.clone(), genetic_analyst);

		let old_qualification = GeneticAnalystQualificationInfo {
			experience: vec![
				GeneticAnalystExperience {
					title: "DeBio title".as_bytes().to_vec(),
				}
			],
			certification: Some(
				vec![
					GeneticAnalystCertification {
						title: "DeBio title".as_bytes().to_vec(),
						issuer: "DeBio issuer".as_bytes().to_vec(),
						month: "DeBio month".as_bytes().to_vec(),
						year: "DeBio year".as_bytes().to_vec(),
						description: "DeBio description".as_bytes().to_vec(),
						supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
					}
				]
			),
		};
		let _create_qualification = GeneticAnalystQualifications::<T>::create_qualification(caller_origin, old_qualification);
		let _genetic_analyst = GeneticAnalysts::<T>::genetic_analyst_by_account_id(caller.clone())
			.unwrap();

		let new_qualification = GeneticAnalystQualificationInfo {
			experience: vec![
				GeneticAnalystExperience {
					title: "DeBio title 2".as_bytes().to_vec(),
				}
			],
			certification: Some(
				vec![
					GeneticAnalystCertification {
						title: "DeBio title 2".as_bytes().to_vec(),
						issuer: "DeBio issuer 2".as_bytes().to_vec(),
						month: "DeBio month 2".as_bytes().to_vec(),
						year: "DeBio year 2".as_bytes().to_vec(),
						description: "DeBio description 2".as_bytes().to_vec(),
						supporting_document: Some("DeBio Profile Image uwu 2".as_bytes().to_vec()),
					}
				]
			),
		};
	}: update_qualification(RawOrigin::Signed(caller), _genetic_analyst.qualifications[0], new_qualification)

	delete_qualification {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));

		let genetic_analyst = GeneticAnalystInfo {
			box_public_key: Keccak256::hash(
				"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes(),
			),
			first_name: "First Name".as_bytes().to_vec(),
			last_name: "Last Name".as_bytes().to_vec(),
			gender: "Gender".as_bytes().to_vec(),
			date_of_birth: <T as pallet_timestamp::pallet::Config>::Moment::default(),
			email: "Email".as_bytes().to_vec(),
			phone_number: "+6893026516".as_bytes().to_vec(),
			specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
			profile_link: "DeBio Genetic Analyst profile_link".as_bytes().to_vec(),
			profile_image: Some("DeBio Genetic Analyst profile_image".as_bytes().to_vec()),
		};
		let _add_genetic_analysts = GeneticAnalysts::<T>::register_genetic_analyst(caller_origin.clone(), genetic_analyst);

		let old_qualification = GeneticAnalystQualificationInfo {
			experience: vec![
				GeneticAnalystExperience {
					title: "DeBio title".as_bytes().to_vec(),
				}
			],
			certification: Some(
				vec![
					GeneticAnalystCertification {
						title: "DeBio title".as_bytes().to_vec(),
						issuer: "DeBio issuer".as_bytes().to_vec(),
						month: "DeBio month".as_bytes().to_vec(),
						year: "DeBio year".as_bytes().to_vec(),
						description: "DeBio description".as_bytes().to_vec(),
						supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
					}
				]
			),
		};
		let _create_qualification = GeneticAnalystQualifications::<T>::create_qualification(caller_origin, old_qualification);
		let _genetic_analyst = GeneticAnalysts::<T>::genetic_analyst_by_account_id(caller.clone())
			.unwrap();
	}: delete_qualification(RawOrigin::Signed(caller), _genetic_analyst.qualifications[0])
}

impl_benchmark_test_suite! {Pallet, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
