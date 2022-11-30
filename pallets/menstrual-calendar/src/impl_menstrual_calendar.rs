use crate::*;

/// MenstrualCalendar Interface Implementation
impl<T: Config> MenstrualCalendarInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type MenstrualCycleLog = MenstrualCycleLogOf<T>;
	type MenstrualCalendar = MenstrualCalendarOf<T>;
	type MenstrualInfo = MenstrualInfoOf<T>;
	type Date = MomentOf<T>;

	fn add_menstrual_calendar(
		address_id: &T::AccountId,
		average_cycle: u8,
	) -> Result<Self::MenstrualCalendar, Self::Error> {
		let now = pallet_timestamp::Pallet::<T>::get();
		let total_count = MenstrualCalendarCountByOwner::<T>::get(address_id).unwrap_or(0);

		if total_count > 0 {
			return Err(Error::<T>::MenstrualCalendarAlreadyExist)
		}

		let id = Self::generate_id(address_id, total_count, None);
		let menstrual_calendar = MenstrualCalendar::new(id, address_id.clone(), average_cycle, now);

		// Store to MenstrualCalendarById storage
		MenstrualCalendarById::<T>::insert(&id, &menstrual_calendar);

		Self::add_menstrual_calendar_by_owner(address_id, &id);
		Self::add_menstrual_calendar_count();
		Self::add_menstrual_calendar_count_by_owner(address_id);

		Ok(menstrual_calendar)
	}

	fn update_menstrual_calendar(
		address_id: &T::AccountId,
		menstrual_calendar_id: &T::Hash,
		average_cycle: u8,
	) -> Result<Self::MenstrualCalendar, Self::Error> {
		let mut menstrual_calendar = MenstrualCalendarById::<T>::get(menstrual_calendar_id)
			.ok_or(Error::<T>::MenstrualCalendarDoesNotExist)?;

		if &menstrual_calendar.address_id != address_id {
			return Err(Error::<T>::NotMenstrualCalendarOwner)
		}

		let now = pallet_timestamp::Pallet::<T>::get();

		menstrual_calendar.average_cycle = average_cycle;
		menstrual_calendar.updated_at = now;

		// Store to MenstrualCalendarById storage
		MenstrualCalendarById::<T>::insert(menstrual_calendar_id, &menstrual_calendar);
		MenstrualCalendarByOwner::<T>::insert(address_id, vec![*menstrual_calendar_id]);

		Ok(menstrual_calendar)
	}

	fn add_menstrual_cycle_log(
		address_id: &T::AccountId,
		menstrual_calendar_id: &T::Hash,
		menstrual_infos: &[Self::MenstrualInfo],
	) -> Result<Vec<Self::MenstrualCycleLog>, Self::Error> {
		let menstrual_calendar = MenstrualCalendarById::<T>::get(menstrual_calendar_id)
			.ok_or(Error::<T>::MenstrualCalendarDoesNotExist)?;

		if &menstrual_calendar.address_id != address_id {
			return Err(Error::<T>::NotMenstrualCalendarOwner)
		}

		// Store to MenstrualCycleLogById storage
		let mut menstrual_cycle_logs: Vec<MenstrualCycleLogOf<T>> = Vec::new();

		for menstrual_info in menstrual_infos.iter() {
			let owner_menstrual_cycle_log_count =
				MenstrualCycleLogCountByOwner::<T>::get(menstrual_calendar_id).unwrap_or(0);

			let menstrual_cycle_log_id = Self::generate_id(
				address_id,
				owner_menstrual_cycle_log_count,
				Some(*menstrual_calendar_id),
			);

			let now = pallet_timestamp::Pallet::<T>::get();
			let date = &menstrual_info.date;
			let symptoms = &menstrual_info.symptoms;
			let menstruation = menstrual_info.menstruation;
			let menstrual_cycle_log = MenstrualCycleLog::new(
				menstrual_cycle_log_id,
				*menstrual_calendar_id,
				*date,
				menstruation,
				symptoms.to_vec(),
				now,
			);

			MenstrualCycleLogById::<T>::insert(menstrual_cycle_log_id, &menstrual_cycle_log);

			Self::add_menstrual_cycle_log_by_owner(menstrual_calendar_id, &menstrual_cycle_log_id);
			Self::add_menstrual_cycle_log_count();
			Self::add_menstrual_cycle_log_count_by_owner(menstrual_calendar_id);

			menstrual_cycle_logs.push(menstrual_cycle_log);
		}

		Ok(menstrual_cycle_logs)
	}

	fn update_menstrual_cycle_log(
		address_id: &T::AccountId,
		menstrual_cycle_logs: &[Self::MenstrualCycleLog],
	) -> Result<Vec<Self::MenstrualCycleLog>, Self::Error> {
		let now = pallet_timestamp::Pallet::<T>::get();
		let mut updated_menstrual_cycle_logs: Vec<MenstrualCycleLogOf<T>> = Vec::new();

		for menstrual_cycle_log in menstrual_cycle_logs.iter() {
			let menstrual_calendar_id = &menstrual_cycle_log.menstrual_calendar_id;
			let menstrual_cycle_log_id = &menstrual_cycle_log.id;
			let date = &menstrual_cycle_log.date;
			let symptoms = &menstrual_cycle_log.symptoms;
			let menstruation = menstrual_cycle_log.menstruation;

			let menstrual_calendar = MenstrualCalendarById::<T>::get(menstrual_calendar_id);
			let new_menstrual_cycle_log = MenstrualCycleLogById::<T>::get(menstrual_cycle_log_id);

			if menstrual_calendar.is_none() || new_menstrual_cycle_log.is_none() {
				continue
			}

			let menstrual_calendar = menstrual_calendar.unwrap();
			let mut new_menstrual_cycle_log = new_menstrual_cycle_log.unwrap();

			if &menstrual_calendar.address_id != address_id {
				continue
			}

			if &new_menstrual_cycle_log.menstrual_calendar_id != menstrual_calendar_id {
				continue
			}

			new_menstrual_cycle_log.date = *date;
			new_menstrual_cycle_log.menstruation = menstruation;
			new_menstrual_cycle_log.symptoms = symptoms.to_vec();
			new_menstrual_cycle_log.updated_at = now;

			MenstrualCycleLogById::<T>::insert(menstrual_cycle_log_id, &new_menstrual_cycle_log);

			updated_menstrual_cycle_logs.push(new_menstrual_cycle_log);
		}

		Ok(updated_menstrual_cycle_logs)
	}

	fn remove_menstrual_cycle_log(
		address_id: &T::AccountId,
		menstrual_calendar_id: &T::Hash,
		menstrual_cycle_log_id: &T::Hash,
	) -> Result<(), Self::Error> {
		let menstrual_calendar = MenstrualCalendarById::<T>::get(menstrual_calendar_id)
			.ok_or(Error::<T>::MenstrualCalendarDoesNotExist)?;

		if &menstrual_calendar.address_id != address_id {
			return Err(Error::<T>::NotMenstrualCalendarOwner)
		}

		let menstrual_cycle_log = MenstrualCycleLogById::<T>::get(menstrual_cycle_log_id)
			.ok_or(Error::<T>::MenstrualCycleLogDoesNotExist)?;

		if &menstrual_cycle_log.menstrual_calendar_id != menstrual_calendar_id {
			return Err(Error::<T>::NotMenstrualCycleLogOwner)
		}

		// Remove menstrual_cycle_log from storage
		MenstrualCycleLogById::<T>::remove(menstrual_cycle_log_id);

		Self::sub_menstrual_cycle_log_by_owner(menstrual_calendar_id, menstrual_cycle_log_id);
		Self::sub_menstrual_cycle_log_count();
		Self::sub_menstrual_cycle_log_count_by_owner(menstrual_calendar_id);

		Ok(())
	}
}

/// MenstrualCalendarProvider Trait Implementation
impl<T: Config> MenstrualCalendarProvider<T> for Pallet<T> {
	type Error = Error<T>;
	type MenstrualCalendar = MenstrualCalendarOf<T>;

	fn menstrual_calendar_by_id(id: &T::Hash) -> Option<MenstrualCalendarOf<T>> {
		MenstrualCalendarById::<T>::get(id)
	}
}
