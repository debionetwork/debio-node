#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	codec::{Decode, Encode},
	RuntimeDebug,
};
use scale_info::TypeInfo;

// ProfileRoles Struct
#[derive(Encode, Decode, Clone, Copy, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct ProfileRoles {
	pub is_customer: bool,
	pub is_lab: bool,
	pub is_doctor: bool,
	pub is_hospital: bool,
	pub is_genetic_analyst: bool,
}

pub trait ProfileRolesTrait {
	fn set_is_customer(&mut self, role: bool);
	fn set_is_lab(&mut self, role: bool);
	fn set_is_doctor(&mut self, role: bool);
	fn set_is_hospital(&mut self, role: bool);
	fn set_is_genetic_analyst(&mut self, role: bool);
}

impl ProfileRolesTrait for ProfileRoles {
	fn set_is_customer(&mut self, role: bool) {
		self.is_customer = role;
	}
	fn set_is_lab(&mut self, role: bool) {
		self.is_lab = role;
	}
	fn set_is_doctor(&mut self, role: bool) {
		self.is_doctor = role;
	}
	fn set_is_hospital(&mut self, role: bool) {
		self.is_hospital = role;
	}
	fn set_is_genetic_analyst(&mut self, role: bool) {
		self.is_genetic_analyst = role;
	}
}
