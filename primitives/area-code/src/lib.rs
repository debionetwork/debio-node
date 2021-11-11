#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::prelude::*;

use frame_support::{
	codec::{Encode, Decode},
	scale_info::TypeInfo,
    sp_std::convert::TryInto,
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

// RegionCode -> YYYY
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct RegionCode([u8; 4]); 

impl RegionCode {
    pub fn from_vec(_vec_id: Vec<u8>) -> Self {
        let _array = _vec_id.try_into()
            .unwrap_or_else(|_vec_id: Vec<u8>| panic!("Expected a Vec of length {} but it was {}", 4, _vec_id.len()));
        
        Self(_array)
    }
}

impl AreaCode for RegionCode {
    fn to_vec(self) -> Vec<u8> {
        self.0.iter().map(|c| *c as u8).collect::<Vec<u8>>()
    }
}

// CityCode -> ZZZZ
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct CityCode([u8; 4]); // city_code -> ZZZZ

impl CityCode {
    pub fn from_vec(_vec_id: Vec<u8>) -> Self {
        let _array = _vec_id.try_into()
            .unwrap_or_else(|_vec_id: Vec<u8>| panic!("Expected a Vec of length {} but it was {}", 4, _vec_id.len()));
        
        Self(_array)
    }
}

impl AreaCode for CityCode {
    fn to_vec(self) -> Vec<u8> {
        self.0.iter().map(|c| *c as u8).collect::<Vec<u8>>()
    }
}

// CountryCode-RegionCode -> XX-YYYY
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct CountryRegionCode([u8; 7]); 

impl CountryRegionCode {
    pub fn from_vec(_vec_id: Vec<u8>) -> Self {
        let _array = _vec_id.try_into()
            .unwrap_or_else(|_vec_id: Vec<u8>| panic!("Expected a Vec of length {} but it was {}", 7, _vec_id.len()));
        
        Self(_array)
    }
}

impl AreaCode for CountryRegionCode {
    fn to_vec(self) -> Vec<u8> {
        self.0.iter().map(|c| *c as u8).collect::<Vec<u8>>()
    }
}