#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	codec::{Decode, Encode},
	RuntimeDebug,
};
use scale_info::TypeInfo;

// ProfileRoles Struct
#[derive(Encode, Decode, Clone, Copy, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct ProfileRoles
{
	pub is_customer: bool,
	pub is_lab: bool,
	pub is_doctor: bool,
	pub is_hospital: bool,
	pub is_genetic_analyst: bool,
}