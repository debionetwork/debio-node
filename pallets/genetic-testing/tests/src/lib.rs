mod mock;

#[cfg(test)]
mod tests {
	use crate::mock::*;
	
	use frame_support::{
		assert_noop, assert_ok,
		sp_runtime::traits::{Hash, Keccak256},
	};
	
	use labs::LabInfo;
	use services::ServiceInfo;
	use orders::{EscrowKey, Order, OrderStatus};
	use genetic_testing::{Error, DnaSampleStatus, DnaTestResultSubmission};
	
	use primitives_area_code::{CityCode, CountryCode, RegionCode};
	use traits_services::types::{CurrencyType, ExpectedDuration, PriceByCurrency, ServiceFlow};
	
	#[test]
	fn reject_dna_sample_works() {
		<ExternalityBuilder>::default().existential_deposit(1).build().execute_with(|| {
			assert_ok!(Labs::register_lab(
				Origin::signed(1),
				LabInfo {
					box_public_key: Keccak256::hash(
						"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()
					),
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
				}
			));
	
			assert_ok!(UserProfile::set_eth_address(Origin::signed(1), EthereumAddress([b'X'; 20])));
	
			assert_ok!(Services::create_service(
				Origin::signed(1),
				ServiceInfo {
					name: "DeBio service name".as_bytes().to_vec(),
					prices_by_currency: vec![
						PriceByCurrency::default()
					],
					expected_duration: ExpectedDuration::default(),
					category: "DeBio service category".as_bytes().to_vec(),
					description: "DeBio service description".as_bytes().to_vec(),
					dna_collection_process : "DeBio service dna_collection_process".as_bytes().to_vec(),
					test_result_sample: "DeBio service test_result_sample".as_bytes().to_vec(),
					long_description: Some("DeBio service long_description".as_bytes().to_vec()),
					image: Some("DeBio service image".as_bytes().to_vec()),
				},
				ServiceFlow::default()
			));

			let _lab = Labs::lab_by_account_id(1).unwrap();
	
			assert_ok!(Orders::create_order(
				Origin::signed(2),
				_lab.services[0],
				0,
				Keccak256::hash("0xhJ7TRe456FADD2726A132ABJK5RCc9E6fC5869F4".as_bytes()),
				ServiceFlow::StakingRequestService
			));
	
			let _dna_sample = GeneticTesting::dna_samples_by_lab_id(1).unwrap();
	
			assert_ok!(GeneticTesting::reject_dna_sample(
				Origin::signed(1),
				_dna_sample[0].clone(),
				"Reject DNA Title".as_bytes().to_vec(),
				"Reject DNA Description".as_bytes().to_vec()
			));
		})
	}
}