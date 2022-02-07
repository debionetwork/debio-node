#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	codec::{Decode, Encode},
	RuntimeDebug,
};
use scale_info::TypeInfo;
use sp_std::{convert::TryInto, vec::Vec};

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct TrackingId([u8; 21]);
impl TrackingId {
	pub fn from_vec(_vec_id: Vec<u8>) -> Self {
		let _array = _vec_id.try_into().unwrap_or_else(|_vec_id: Vec<u8>| {
			panic!("Expected a Vec of length {} but it was {}", 21, _vec_id.len())
		});

		Self(_array)
	}
}

pub mod tracking_id_generator {
	use crate::*;

	pub const SAFE: [char; 36] = [
		'0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
		// 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
		// 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
		'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
		'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
	];

	pub fn generate(seed: Vec<u8>) -> TrackingId {
		let alphabet = &SAFE;
		let size = 21;
		let mask = alphabet.len().next_power_of_two() - 1;

		// Assert that the masking does not truncate the alphabet. (See #9)
		debug_assert!(alphabet.len() <= mask + 1);

		let mut id = Vec::new();

		loop {
			for &byte in &seed {
				let byte = byte as usize & mask;

				if alphabet.len() > byte {
					id.push(alphabet[byte]);

					if id.len() == size {
						let _vec_id = id.iter().map(|c| *c as u8).collect::<Vec<_>>();

						let _tracking_id = TrackingId::from_vec(_vec_id);
						return _tracking_id
					}
				}
			}
		}
	}
}
