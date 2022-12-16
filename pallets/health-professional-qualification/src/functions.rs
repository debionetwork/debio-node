use crate::*;
use frame_support::{codec::Encode, sp_runtime::traits::Hash};
use traits_health_professional_qualifications::{
	HealthProfessionalQualificationCountT, HealthProfessionalQualificationProvider,
};

impl<T: Config> Pallet<T> {
	pub fn generate_qualification_id(owner_id: &T::AccountId, qualification_count: u64) -> T::Hash {
		let mut account_id_bytes = owner_id.encode();
		let mut qualification_count_bytes = qualification_count.encode();
		let mut nonce = frame_system::Pallet::<T>::account(owner_id).nonce.encode();

		account_id_bytes.append(&mut qualification_count_bytes);
		account_id_bytes.append(&mut nonce);

		let seed = &account_id_bytes;
		T::Hashing::hash(seed)
	}
}

impl<T: Config> HealthProfessionalQualificationCountT<T> for Pallet<T> {
	fn add_health_professional_qualification_count(value: u64) {
		HealthProfessionalQualificationCount::<T>::mutate(|count| {
			*count = count.saturating_add(value);
		});
	}

	fn substract_health_professional_qualification_count(value: u64) {
		HealthProfessionalQualificationCount::<T>::mutate(|count| {
			*count = count.saturating_sub(value);
		});
	}

	fn add_health_professional_qualification_count_by_owner(account_id: &T::AccountId, value: u64) {
		HealthProfessionalQualificationCountByOwner::<T>::mutate(account_id, |count| {
			*count = count.saturating_add(value);
		});
	}

	fn substract_health_professional_qualification_count_by_owner(
		account_id: &<T as frame_system::Config>::AccountId,
		value: u64,
	) {
		HealthProfessionalQualificationCountByOwner::<T>::mutate(account_id, |count| {
			*count = count.saturating_sub(value);
		});
	}
}

impl<T: Config> HealthProfessionalQualificationProvider<T> for Pallet<T> {
	type Error = Error<T>;

	fn delete_qualifications(account_id: &T::AccountId, hash_ids: &[T::Hash]) {
		let mut count = 0;

		for hash_id in hash_ids.iter() {
			let result = HealthProfessionalQualifications::<T>::get(hash_id);

			if let Some(qualification) = result {
				if qualification.is_authorized_owner(account_id).is_some() {
					HealthProfessionalQualifications::<T>::remove(hash_id);

					count += 1;
				}
			}
		}

		if count > 0 {
			Self::substract_health_professional_qualification_count(count);
			Self::substract_health_professional_qualification_count_by_owner(account_id, count);
		}
	}
}
