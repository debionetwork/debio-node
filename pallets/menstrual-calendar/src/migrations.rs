use crate::*;
use frame_support::{
	pallet_prelude::{Decode, Encode},
	traits::Get,
	weights::Weight,
};
use sp_std::vec::Vec;

pub fn migrate<T: Config>() -> Weight {
	let mut weight: Weight = Weight::zero();
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
			pub struct OldMenstrualCalendar<AccountId, Hash, Moment> {
				pub id: Hash,
				pub address_id: AccountId,
				pub average_cycle: u8,
				pub cycle_log: Vec<Hash>,
				pub created_at: Moment,
				pub updated_at: Moment,
			}

			pub type OldMenstrualCalendarOf<T> =
				OldMenstrualCalendar<AccountIdOf<T>, HashOf<T>, MomentOf<T>>;

			MenstrualCalendarById::<T>::translate(|_key, old: OldMenstrualCalendarOf<T>| {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

				Some(MenstrualCalendar {
					id: old.id,
					address_id: old.address_id,
					average_cycle: old.average_cycle,
					created_at: old.created_at,
					updated_at: old.updated_at,
				})
			});

			#[derive(Encode, Decode)]
			pub struct OldSymptom {
				name: Vec<u8>,
			}

			#[derive(Encode, Decode)]
			pub struct OldMenstrualCycleLog<Hash, Moment> {
				pub id: Hash,
				pub menstrual_calendar_id: Hash,
				pub date: Moment,
				pub menstruation: bool,
				pub symptoms: Vec<OldSymptom>,
				pub created_at: Moment,
				pub updated_at: Moment,
			}

			pub type OldMenstrualCycleLogOf<T> = OldMenstrualCycleLog<HashOf<T>, MomentOf<T>>;

			MenstrualCycleLogById::<T>::translate(|_key, old: OldMenstrualCycleLogOf<T>| {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

				let new_symptoms: Vec<Symptom> =
					old.symptoms.into_iter().map(|symptom| Symptom::from(&symptom.name)).collect();

				Some(MenstrualCycleLog {
					id: old.id,
					menstrual_calendar_id: old.menstrual_calendar_id,
					date: old.date,
					menstruation: old.menstruation,
					symptoms: new_symptoms,
					created_at: old.created_at,
					updated_at: old.updated_at,
				})
			});

			weight
		}
	}
}
