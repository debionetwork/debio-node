#![cfg_attr(not(feature = "std"), no_std)]
mod mock;

#[allow(unused)]
use genetic_analyst_services::Pallet as GeneticAnalystServices;
use genetic_analyst_services::{Config as GeneticAnalystServicesConfig, GeneticAnalystServiceInfo};

#[allow(unused)]
use genetic_analysts::Pallet as GeneticAnalysts;
use genetic_analysts::{Config as GeneticAnalystsConfig, GeneticAnalystInfo};

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::sp_runtime::traits::Hash;
use frame_system::RawOrigin;
use sp_std::vec;

pub struct Pallet<T: Config>(GeneticAnalystServices<T>);

pub trait Config: GeneticAnalystServicesConfig + GeneticAnalystsConfig {}

use genetic_analyst_services::Call;

use primitives_duration::ExpectedDuration;
use primitives_price_and_currency::PriceByCurrency;

benchmarks! {
	create_genetic_analyst_service {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let genetic_analyst = GeneticAnalystInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
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

		let genetic_analyst_service_info = GeneticAnalystServiceInfo {
			name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
			prices_by_currency: vec![
				PriceByCurrency::default()
			],
			expected_duration: ExpectedDuration::default(),
			description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
			test_result_sample: "DeBio Genetic Analyst Service test_result_sample".as_bytes().to_vec(),
		};
	}: create_genetic_analyst_service(RawOrigin::Signed(caller), genetic_analyst_service_info)

	bulk_create_genetic_analyst_service {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let genetic_analyst = GeneticAnalystInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
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
		let _ = GeneticAnalysts::<T>::register_genetic_analyst(caller_origin, genetic_analyst);

		let twenty_services = vec![
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			},
			GeneticAnalystServiceInfo {
				name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
				prices_by_currency: vec![PriceByCurrency::default()],
				expected_duration: ExpectedDuration::default(),
				description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
				test_result_sample: "DeBio Genetic Analyst Service test_result_sample"
					.as_bytes()
					.to_vec(),
			}
		];
	}: bulk_create_genetic_analyst_service(RawOrigin::Signed(caller), twenty_services)

	update_genetic_analyst_service {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let genetic_analyst_info = GeneticAnalystInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
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
		let _add_genetic_analysts = GeneticAnalysts::<T>::register_genetic_analyst(caller_origin.clone(), genetic_analyst_info);

		let old_genetic_analyst_service_info = GeneticAnalystServiceInfo {
			name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
			prices_by_currency: vec![
				PriceByCurrency::default()
			],
			expected_duration: ExpectedDuration::default(),
			description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
			test_result_sample: "DeBio Genetic Analyst Service test_result_sample".as_bytes().to_vec(),
		};
		let _create_genetic_analyst_service = GeneticAnalystServices::<T>::create_genetic_analyst_service(caller_origin, old_genetic_analyst_service_info);

		let _genetic_analyst = GeneticAnalysts::<T>::genetic_analyst_by_account_id(caller.clone())
			.unwrap();

		let new_genetic_analyst_service_info = GeneticAnalystServiceInfo {
			name: "DeBio Genetic Analyst Service name 2".as_bytes().to_vec(),
			prices_by_currency: vec![
				PriceByCurrency::default()
			],
			expected_duration: ExpectedDuration::default(),
			description: "DeBio Genetic Analyst Service description 2".as_bytes().to_vec(),
			test_result_sample: "DeBio Genetic Analyst Service test_result_sample 2".as_bytes().to_vec(),
		};
	}: update_genetic_analyst_service(RawOrigin::Signed(caller), _genetic_analyst.services[0], new_genetic_analyst_service_info)

	delete_genetic_analyst_service {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let genetic_analyst_info = GeneticAnalystInfo {
			box_public_key: T::Hashing::hash("0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()),
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
		let _add_genetic_analysts = GeneticAnalysts::<T>::register_genetic_analyst(caller_origin.clone(), genetic_analyst_info);

		let old_genetic_analyst_service_info = GeneticAnalystServiceInfo {
			name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
			prices_by_currency: vec![
				PriceByCurrency::default()
			],
			expected_duration: ExpectedDuration::default(),
			description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
			test_result_sample: "DeBio Genetic Analyst Service test_result_sample".as_bytes().to_vec(),
		};
		let _create_genetic_analyst_service = GeneticAnalystServices::<T>::create_genetic_analyst_service(caller_origin, old_genetic_analyst_service_info);

		let _genetic_analyst = GeneticAnalysts::<T>::genetic_analyst_by_account_id(caller.clone())
			.unwrap();
	}: delete_genetic_analyst_service(RawOrigin::Signed(caller), _genetic_analyst.services[0])
}

impl_benchmark_test_suite! {Pallet, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
