#![cfg_attr(not(feature = "std"), no_std)]
mod mock;

#[allow(unused)]
use genetic_analyst_services::Pallet as GeneticAnalystServices;
use genetic_analyst_services::{
	Config as GeneticAnalystServicesConfig,
	GeneticAnalystServiceInfo
};

#[allow(unused)]
use genetic_analysts::Pallet as GeneticAnalysts;
use genetic_analysts::{
	Config as GeneticAnalystsConfig,
	GeneticAnalystInfo,
	StakeStatus,
};

#[allow(unused)]
use user_profile::Pallet as UserProfile;
use user_profile::Config as UserProfileConfig;

#[allow(unused)]
use genetic_analysis_orders::Pallet as GeneticAnalysisOrders;
use genetic_analysis_orders::{
	Config as GeneticAnalysisOrdersConfig
};

#[allow(unused)]
use genetic_analysis::Pallet as GeneticAnalysis;
use genetic_analysis::{
	GeneticAnalysisStatus,
	Config as GeneticAnalysisConfig
};

pub trait Config:
	GeneticAnalystServicesConfig
	+ GeneticAnalystsConfig
	+ UserProfileConfig
	+ GeneticAnalysisOrdersConfig
	+ GeneticAnalysisConfig
{}

pub struct Pallet<T: Config>(GeneticAnalysis<T>);

use sp_core::Decode;
use genetic_analysis::Call;
use frame_system::RawOrigin;
use frame_support::sp_runtime::traits::Hash;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, vec};

use primitives_duration::ExpectedDuration;
use primitives_price_and_currency::PriceByCurrency;

benchmarks! {
	reject_genetic_analysis {
		let caller: T::AccountId = T::AccountId::decode(&mut "18c79faa6203d8b8349b19cc72cc6bfd008c243ea998435847abf6618756ca0b".as_bytes()).unwrap_or_default();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		
		let genetic_analyst = GeneticAnalystInfo {
			first_name: "First Name".as_bytes().to_vec(),
			last_name: "Last Name".as_bytes().to_vec(),
			gender: "Gender".as_bytes().to_vec(),
			date_of_birth: <T as pallet_timestamp::pallet::Config>::Moment::default(),
			email: "Email".as_bytes().to_vec(),
			phone_number: "+6893026516".as_bytes().to_vec(),
			specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
			stake_amount: 100,
			stake_status: StakeStatus::default(),
		};
		let _add_genetic_analysts = GeneticAnalysts::<T>::register_genetic_analyst(caller_origin.clone(), genetic_analyst);

        let eth_address = <T as UserProfileConfig>::EthereumAddress::default();
		let _set_eth_address = UserProfile::<T>::set_eth_address(caller_origin.clone(), eth_address);

		let genetic_analyst_service_info = GeneticAnalystServiceInfo {
			name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
			prices_by_currency: vec![
				PriceByCurrency::default()
			],
			expected_duration: ExpectedDuration::default(),
			description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
			test_result_sample: "DeBio Genetic Analyst Service test_result_sample".as_bytes().to_vec(),
		};
		let _create_genetic_analyst_service = GeneticAnalystServices::<T>::create_genetic_analyst_service(caller_origin.clone(), genetic_analyst_service_info);
		
		let _genetic_analyst = GeneticAnalysts::<T>::genetic_analyst_by_account_id(caller.clone())
			.unwrap();

		let _create_genetic_analysis_order = GeneticAnalysisOrders::<T>::create_genetic_analysis_order(
			caller_origin.clone(), 
			_genetic_analyst.services[0],
			0,
			T::Hashing::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
		);

		let _genetic_analysis_order_id_list = GeneticAnalysisOrders::<T>::genetic_analysis_orders_by_genetic_analyst_id(caller.clone())
			.unwrap();
		let _genetic_analysis_order = GeneticAnalysisOrders::<T>::genetic_analysis_order_by_id(_genetic_analysis_order_id_list[0])
			.unwrap();
	}: reject_genetic_analysis(
		RawOrigin::Signed(caller), 
		_genetic_analysis_order.genetic_analysis_tracking_id,
		"Rejected title".as_bytes().to_vec(),
		"Rejected description".as_bytes().to_vec()
	)

	process_genetic_analysis {
		let caller: T::AccountId = T::AccountId::decode(&mut "18c79faa6203d8b8349b19cc72cc6bfd008c243ea998435847abf6618756ca0b".as_bytes()).unwrap_or_default();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		
		let genetic_analyst = GeneticAnalystInfo {
			first_name: "First Name".as_bytes().to_vec(),
			last_name: "Last Name".as_bytes().to_vec(),
			gender: "Gender".as_bytes().to_vec(),
			date_of_birth: <T as pallet_timestamp::pallet::Config>::Moment::default(),
			email: "Email".as_bytes().to_vec(),
			phone_number: "+6893026516".as_bytes().to_vec(),
			specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
			stake_amount: 100,
			stake_status: StakeStatus::default(),
		};
		let _add_genetic_analysts = GeneticAnalysts::<T>::register_genetic_analyst(caller_origin.clone(), genetic_analyst);

        let eth_address = <T as UserProfileConfig>::EthereumAddress::default();
		let _set_eth_address = UserProfile::<T>::set_eth_address(caller_origin.clone(), eth_address);

		let genetic_analyst_service_info = GeneticAnalystServiceInfo {
			name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
			prices_by_currency: vec![
				PriceByCurrency::default()
			],
			expected_duration: ExpectedDuration::default(),
			description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
			test_result_sample: "DeBio Genetic Analyst Service test_result_sample".as_bytes().to_vec(),
		};
		let _create_genetic_analyst_service = GeneticAnalystServices::<T>::create_genetic_analyst_service(caller_origin.clone(), genetic_analyst_service_info);
		
		let _genetic_analyst = GeneticAnalysts::<T>::genetic_analyst_by_account_id(caller.clone())
			.unwrap();

		let _create_genetic_analysis_order = GeneticAnalysisOrders::<T>::create_genetic_analysis_order(
			caller_origin.clone(), 
			_genetic_analyst.services[0],
			0,
			T::Hashing::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
		);

		let _genetic_analysis_order_id_list = GeneticAnalysisOrders::<T>::genetic_analysis_orders_by_genetic_analyst_id(caller.clone())
			.unwrap();
		let _genetic_analysis_order = GeneticAnalysisOrders::<T>::genetic_analysis_order_by_id(_genetic_analysis_order_id_list[0])
			.unwrap();
	}: process_genetic_analysis(
		RawOrigin::Signed(caller), 
		_genetic_analysis_order.genetic_analysis_tracking_id,
		GeneticAnalysisStatus::default()
	)

	submit_genetic_analysis {
		let caller: T::AccountId = T::AccountId::decode(&mut "18c79faa6203d8b8349b19cc72cc6bfd008c243ea998435847abf6618756ca0b".as_bytes()).unwrap_or_default();
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		
		let genetic_analyst = GeneticAnalystInfo {
			first_name: "First Name".as_bytes().to_vec(),
			last_name: "Last Name".as_bytes().to_vec(),
			gender: "Gender".as_bytes().to_vec(),
			date_of_birth: <T as pallet_timestamp::pallet::Config>::Moment::default(),
			email: "Email".as_bytes().to_vec(),
			phone_number: "+6893026516".as_bytes().to_vec(),
			specialization: "DeBio Genetic Analyst".as_bytes().to_vec(),
			stake_amount: 100,
			stake_status: StakeStatus::default(),
		};
		let _add_genetic_analysts = GeneticAnalysts::<T>::register_genetic_analyst(caller_origin.clone(), genetic_analyst);

        let eth_address = <T as UserProfileConfig>::EthereumAddress::default();
		let _set_eth_address = UserProfile::<T>::set_eth_address(caller_origin.clone(), eth_address);

		let genetic_analyst_service_info = GeneticAnalystServiceInfo {
			name: "DeBio Genetic Analyst Service name".as_bytes().to_vec(),
			prices_by_currency: vec![
				PriceByCurrency::default()
			],
			expected_duration: ExpectedDuration::default(),
			description: "DeBio Genetic Analyst Service description".as_bytes().to_vec(),
			test_result_sample: "DeBio Genetic Analyst Service test_result_sample".as_bytes().to_vec(),
		};
		let _create_genetic_analyst_service = GeneticAnalystServices::<T>::create_genetic_analyst_service(caller_origin.clone(), genetic_analyst_service_info);
		
		let _genetic_analyst = GeneticAnalysts::<T>::genetic_analyst_by_account_id(caller.clone())
			.unwrap();

		let _create_genetic_analysis_order = GeneticAnalysisOrders::<T>::create_genetic_analysis_order(
			caller_origin.clone(), 
			_genetic_analyst.services[0],
			0,
			T::Hashing::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
		);

		let _genetic_analysis_order_id_list = GeneticAnalysisOrders::<T>::genetic_analysis_orders_by_genetic_analyst_id(caller.clone())
			.unwrap();
		let _genetic_analysis_order = GeneticAnalysisOrders::<T>::genetic_analysis_order_by_id(_genetic_analysis_order_id_list[0])
			.unwrap();
	}: submit_genetic_analysis(
		RawOrigin::Signed(caller), 
		_genetic_analysis_order.genetic_analysis_tracking_id,
		"Genetic Analysis report_link".as_bytes().to_vec(),
		Some("Genetic Analysis comments".as_bytes().to_vec())
	)
}

impl_benchmark_test_suite! {Pallet, crate::mock::ExternalityBuilder::build(), crate::mock::Test}