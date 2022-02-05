#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	codec::{Encode, Decode},
	scale_info::TypeInfo,
    sp_std::convert::TryInto,
    sp_std::vec::Vec,
	sp_runtime::{RuntimeDebug},
};

pub trait AreaCode {
    fn to_vec(self) -> Vec<u8>;
}

// CountryCode -> XX
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct CountryCode([u8; 2]); 

impl CountryCode {
    pub fn from_vec(_vec_id: Vec<u8>) -> Self {
        let _array = _vec_id.try_into()
            .unwrap_or_else(|_vec_id: Vec<u8>| panic!("Expected a Vec of length {} but it was {}", 2, _vec_id.len()));
        
        Self(_array)
    }
}

impl AreaCode for CountryCode {
    fn to_vec(self) -> Vec<u8> {
        self.0.iter().map(|c| *c as u8).collect::<Vec<u8>>()
    }
}

// RegionCode -> YY
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct RegionCode(Vec<u8>); 

impl RegionCode {
    pub fn from_vec(_vec_id: Vec<u8>) -> Self {
        Self(_vec_id)
    }
}

impl AreaCode for RegionCode {
    fn to_vec(self) -> Vec<u8> {
        self.0.iter().map(|c| *c as u8).collect::<Vec<u8>>()
    }
}

// CityCode -> ZZZZ
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct CityCode(Vec<u8>); // city_code -> ZZZZ

impl CityCode {
    pub fn from_vec(_vec_id: Vec<u8>) -> Self {
        Self(_vec_id)
    }
}

impl AreaCode for CityCode {
    fn to_vec(self) -> Vec<u8> {
        self.0.iter().map(|c| *c as u8).collect::<Vec<u8>>()
    }
}

// CountryCode-RegionCode -> XX-YY
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct CountryRegionCode(Vec<u8>); 

impl CountryRegionCode {
    pub fn from_vec(_vec_id: Vec<u8>) -> Self {
        Self(_vec_id)
    }
    
    /// Concatenate CountryCode with RegionCode with a '-'
    pub fn build_country_region_code(country_code: &CountryCode, region_code: &RegionCode) -> Self {
        // container
        let mut country_region_code = Vec::new();
        let mut country_code = country_code.clone()
            .to_vec();
            
        // dash character as u8
        let mut dash = ['-'].iter()
            .map(|c| *c as u8)
            .collect::<Vec<u8>>();

        let mut region_code = region_code.clone()
            .to_vec();

        country_region_code.append(&mut country_code);
        country_region_code.append(&mut dash);
        country_region_code.append(&mut region_code);

        CountryRegionCode::from_vec(country_region_code)
    }
}

impl AreaCode for CountryRegionCode {
    fn to_vec(self) -> Vec<u8> {
        self.0.iter().map(|c| *c as u8).collect::<Vec<u8>>()
    }
}