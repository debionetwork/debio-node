#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;

pub trait HealthProfessionalCountT<T: Config> {
	fn add_health_professional_count(value: u64);
	fn substract_health_professional_count(value: u64);
}
