#![cfg_attr(not(feature = "std"), no_std)]
mod mock;

use labs::LabInfo;
use certifications::{
	Error, 
	CertificationInfo
};

use frame_support::{
	assert_noop, assert_ok,
	sp_runtime::traits::{Hash, Keccak256},
};
use primitives_area_code::{CityCode, CountryCode, RegionCode};

#[test]
fn create_certification_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(Labs::register_lab(
			Origin::signed(1),
			LabInfo {
				box_public_key: Keccak256::hash(
					"0xDb9Af2d1f3ADD2726A132AA7A65Cc9E6fC5761C3".as_bytes()
				),
				name: "DeBio Lab".as_bytes().to_vec(),
				email: "DeBio Email".as_bytes().to_vec(),
				country: CountryCode::from_vec("DC".as_bytes().to_vec()),
				region: RegionCode::from_vec("DB".as_bytes().to_vec()),
				city: CityCode::from_vec("CITY".as_bytes().to_vec()),
				address: "DeBio Address".as_bytes().to_vec(),
				phone_number: "+6281394653625".as_bytes().to_vec(),
				website: "DeBio Website".as_bytes().to_vec(),
				latitude: Some("DeBio Latitude".as_bytes().to_vec()),
				longitude: Some("DeBio Longtitude".as_bytes().to_vec()),
				profile_image: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
			}
		));
		
		assert_ok!(Certifications::create_certification(
			Origin::signed(1),
			CertificationInfo {
				title: "DeBio title".as_bytes().to_vec(),
				issuer: "DeBio issuer".as_bytes().to_vec(),
				month: "DeBio month".as_bytes().to_vec(),
				year: "DeBio year".as_bytes().to_vec(),
				description: "DeBio description".as_bytes().to_vec(),
				supporting_document: Some("DeBio Profile Image uwu".as_bytes().to_vec()),
			}
		));
	})
}
