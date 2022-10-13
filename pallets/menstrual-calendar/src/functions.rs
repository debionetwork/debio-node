use crate::*;

use frame_support::{codec::Encode, sp_runtime::traits::Hash};

/// Pallet Methods
impl<T: Config> Pallet<T> {
	pub fn generate_id(
		address_id: &T::AccountId,
		menstrual_calendar_count: u64,
		additional: Option<T::Hash>,
	) -> T::Hash {
		let mut account_id_bytes = address_id.encode();
		let mut menstrual_calendar_count_bytes = menstrual_calendar_count.encode();

		let account_info = frame_system::Pallet::<T>::account(address_id);
		let mut nonce_bytes = account_info.nonce.encode();

		if let Some(additional_id) = additional {
			let mut additional_id_bytes = additional_id.encode();
			account_id_bytes.append(&mut additional_id_bytes);
		}

		account_id_bytes.append(&mut nonce_bytes);
		account_id_bytes.append(&mut menstrual_calendar_count_bytes);

		let seed = &account_id_bytes;
		T::Hashing::hash(seed)
	}

	// Add menstrual_calendar by owner
	pub fn add_menstrual_calendar_by_owner(
		address_id: &T::AccountId,
		menstrual_calendar_id: &T::Hash,
	) {
		let mut menstrual_calendar =
			MenstrualCalendarByOwner::<T>::get(address_id).unwrap_or_default();

		menstrual_calendar.push(*menstrual_calendar_id);
		MenstrualCalendarByOwner::<T>::insert(address_id, &menstrual_calendar)
	}

	// Subtract menstrual_calendar by owner
	pub fn sub_menstrual_calendar_by_owner(
		address_id: &T::AccountId,
		menstrual_calendar_id: &T::Hash,
	) {
		let mut menstrual_calendar =
			MenstrualCalendarByOwner::<T>::get(address_id).unwrap_or_default();
		menstrual_calendar.retain(|&x| x != *menstrual_calendar_id);
		MenstrualCalendarByOwner::<T>::insert(address_id, menstrual_calendar);
	}

	// Add menstrual_calendar count
	pub fn add_menstrual_calendar_count() {
		let menstrual_calendar_count = <MenstrualCalendarCount<T>>::get().unwrap_or(0);
		<MenstrualCalendarCount<T>>::put(menstrual_calendar_count.wrapping_add(1));
	}

	// Add menstrual_calendar count by owner
	pub fn add_menstrual_calendar_count_by_owner(address_id: &T::AccountId) {
		let menstrual_calendar_count =
			MenstrualCalendarCountByOwner::<T>::get(address_id).unwrap_or(0);
		MenstrualCalendarCountByOwner::<T>::insert(
			address_id,
			menstrual_calendar_count.wrapping_add(1),
		)
	}

	// Subtract menstrual_calendar count
	pub fn sub_menstrual_calendar_count() {
		let menstrual_calendar_count = <MenstrualCalendarCount<T>>::get().unwrap_or(1);
		MenstrualCalendarCount::<T>::put(menstrual_calendar_count - 1);
	}

	// Subtract menstrual_calendar count by owner
	pub fn sub_menstrual_calendar_count_by_owner(address_id: &T::AccountId) {
		let menstrual_calendar_count =
			MenstrualCalendarCountByOwner::<T>::get(address_id).unwrap_or(1);
		MenstrualCalendarCountByOwner::<T>::insert(address_id, menstrual_calendar_count - 1);
	}
	// Add menstrual_cycle_log by owner
	pub fn add_menstrual_cycle_log_by_owner(
		menstrual_calendar_id: &T::Hash,
		menstrual_cycle_log_id: &T::Hash,
	) {
		let mut menstrual_cycle_log =
			MenstrualCycleLogByOwner::<T>::get(menstrual_calendar_id).unwrap_or_default();

		menstrual_cycle_log.push(*menstrual_cycle_log_id);
		MenstrualCycleLogByOwner::<T>::insert(menstrual_calendar_id, &menstrual_cycle_log)
	}

	// Subtract menstrual_cycle_log by owner
	pub fn sub_menstrual_cycle_log_by_owner(
		menstrual_calendar_id: &T::Hash,
		menstrual_cycle_log_id: &T::Hash,
	) {
		let mut menstrual_cycle_log =
			MenstrualCycleLogByOwner::<T>::get(menstrual_calendar_id).unwrap_or_default();
		menstrual_cycle_log.retain(|&x| x != *menstrual_cycle_log_id);
		MenstrualCycleLogByOwner::<T>::insert(menstrual_calendar_id, menstrual_cycle_log);
	}

	// Add menstrual_cycle_log count
	pub fn add_menstrual_cycle_log_count() {
		let menstrual_cycle_log_count = <MenstrualCycleLogCount<T>>::get().unwrap_or(0);
		<MenstrualCycleLogCount<T>>::put(menstrual_cycle_log_count.wrapping_add(1));
	}

	// Add menstrual_cycle_log count by owner
	pub fn add_menstrual_cycle_log_count_by_owner(menstrual_calendar_id: &T::Hash) {
		let menstrual_cycle_log_count =
			MenstrualCycleLogCountByOwner::<T>::get(menstrual_calendar_id).unwrap_or(0);
		MenstrualCycleLogCountByOwner::<T>::insert(
			menstrual_calendar_id,
			menstrual_cycle_log_count.wrapping_add(1),
		)
	}

	// Subtract menstrual_cycle_log count
	pub fn sub_menstrual_cycle_log_count() {
		let menstrual_cycle_log_count = <MenstrualCycleLogCount<T>>::get().unwrap_or(1);
		MenstrualCycleLogCount::<T>::put(menstrual_cycle_log_count - 1);
	}

	// Subtract menstrual_cycle_log count by owner
	pub fn sub_menstrual_cycle_log_count_by_owner(menstrual_calendar_id: &T::Hash) {
		let menstrual_cycle_log_count =
			MenstrualCycleLogCountByOwner::<T>::get(menstrual_calendar_id).unwrap_or(1);
		MenstrualCycleLogCountByOwner::<T>::insert(
			menstrual_calendar_id,
			menstrual_cycle_log_count - 1,
		);
	}
}
