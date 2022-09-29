use crate::*;

use frame_support::{pallet_prelude::*, sp_runtime::traits::Hash};

/// Pallet Methods
impl<T: Config> Pallet<T> {
	pub fn generate_service_id(owner_id: &T::AccountId, service_count: u64) -> T::Hash {
		let mut account_id_bytes = owner_id.encode();
		let mut service_count_bytes = service_count.encode();
		account_id_bytes.append(&mut service_count_bytes);

		let seed = &account_id_bytes;
		T::Hashing::hash(seed)
	}

	// Services Count Addition and Substraction Helpers
	// Add services count
	pub fn add_services_count() {
		let services_count = <ServicesCount<T>>::get().unwrap_or(0);
		<ServicesCount<T>>::put(services_count.wrapping_add(1));
	}
	// Add services count by owner
	pub fn add_services_count_by_owner(owner_id: &T::AccountId) {
		let services_count = ServicesCountByOwner::<T>::get(owner_id).unwrap_or(0);
		ServicesCountByOwner::<T>::insert(owner_id, services_count.wrapping_add(1))
	}

	// Subtract services count
	pub fn sub_services_count() {
		let services_count = <ServicesCount<T>>::get().unwrap_or(1);
		ServicesCount::<T>::put(services_count - 1);
	}
	// Subtract services count by owner
	pub fn sub_services_count_by_owner(owner_id: &T::AccountId) {
		let services_count = ServicesCountByOwner::<T>::get(owner_id).unwrap_or(1);
		ServicesCountByOwner::<T>::insert(owner_id, services_count - 1);
	}
}
