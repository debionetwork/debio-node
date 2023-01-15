use crate::*;
use frame_support::{
	pallet_prelude::{Decode, Encode},
	traits::Get,
	weights::Weight,
};
use sp_std::vec::Vec;

use primitives_availability_status::AvailabilityStatus;
use primitives_stake_status::StakeStatus;
use primitives_verification_status::VerificationStatus;

pub fn migrate<T: Config>() -> Weight {
	let mut weight: Weight = 0;
	let mut version = StorageVersion::get::<Pallet<T>>();

	if version < 1 {
		weight = weight.saturating_add(version::v1::migrate::<T>());
		version = StorageVersion::new(1);
	}

	version.put::<Pallet<T>>();
	weight
}

mod version {
	use super::*;

	pub mod v1 {
		use super::*;

		pub fn migrate<T: Config>() -> Weight {
			let mut weight = T::DbWeight::get().writes(1);

			#[derive(Encode, Decode)]
			pub struct OldHealthProfessionalInfo<Hash, Moment> {
				box_public_key: Hash,
				first_name: Vec<u8>,
				last_name: Vec<u8>,
				myriad_username: Vec<u8>,
				gender: Vec<u8>,
				date_of_birth: Moment,
				email: Vec<u8>,
				phone_number: Vec<u8>,
				role: Vec<u8>,
				category: Vec<u8>,
				profile_link: Vec<u8>,
				profile_image: Option<Vec<u8>>,
				anonymous: bool,
			}

			#[derive(Encode, Decode)]
			pub struct OldHealthProfessional<AccountId, Hash, Moment, Balance> {
				account_id: AccountId,
				qualifications: Vec<Hash>,
				info: OldHealthProfessionalInfo<Hash, Moment>,
				stake_amount: Balance,
				stake_status: StakeStatus,
				verification_status: VerificationStatus,
				availability_status: AvailabilityStatus,
				unstaked_at: Option<Moment>,
			}

			pub type OldHealthProfessionalOf<T> =
				OldHealthProfessional<AccountIdOf<T>, HashOf<T>, MomentOf<T>, BalanceOf<T>>;

			HealthProfessionals::<T>::translate(|_key, old: OldHealthProfessionalOf<T>| {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

				let old_health_professional_info = old.info;
				let new_health_professional_info = HealthProfessionalInfo {
					box_public_key: old_health_professional_info.box_public_key,
					first_name: old_health_professional_info.first_name,
					last_name: old_health_professional_info.last_name,
					myriad_username: old_health_professional_info.myriad_username,
					gender: old_health_professional_info.gender,
					date_of_birth: old_health_professional_info.date_of_birth,
					email: old_health_professional_info.email,
					phone_number: old_health_professional_info.phone_number,
					role: old_health_professional_info.role,
					category: old_health_professional_info.category,
					profile_link: Some(old_health_professional_info.profile_link),
					profile_image: old_health_professional_info.profile_image,
					anonymous: old_health_professional_info.anonymous,
				};

				let mut new_health_professional =
					HealthProfessional::new(&old.account_id, &new_health_professional_info);

				new_health_professional.update_verification_status(&old.verification_status);
				new_health_professional.update_availability_status(&old.availability_status);
				new_health_professional.update_unstaked_at(old.unstaked_at);
				new_health_professional.update_stake_amount(old.stake_amount);
				new_health_professional.update_stake_status(old.stake_status, old.stake_amount);
				new_health_professional.update_qualifications(old.qualifications);

				Some(new_health_professional)
			});

			weight
		}
	}
}
