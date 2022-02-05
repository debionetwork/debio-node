#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
use frame_support::RuntimeDebug;
use sp_std::vec::Vec;
use std::convert::TryInto;
use scale_info::TypeInfo;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct TrackingId([u8; 21]);
impl TrackingId {
    pub fn from_vec(_vec_id: Vec<u8>) -> Self {
        let _array = _vec_id.try_into()
            .unwrap_or_else(|_vec_id: Vec<u8>| panic!("Expected a Vec of length {} but it was {}", 21, _vec_id.len()));
        
        Self(_array)
    }
}