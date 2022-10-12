use crate::*;
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_std::vec::Vec;
use traits_menstrual_calendar::{
	MenstrualCalendar as MenstrualCalendarT, MenstrualCycleLog as MenstrualCycleLogT,
};

// Symptom
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct Symptom {
	pub name: Vec<u8>,
}

// MenstrualCycleLog
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct MenstrualCycleLog<Hash, Moment> {
	pub id: Hash,
	pub menstrual_calendar_id: Hash,
	pub date: Moment,
	pub menstruation: bool,
	pub symptoms: Vec<Symptom>,
	pub created_at: Moment,
	pub updated_at: Moment,
}

impl<Hash, Moment: Default> MenstrualCycleLog<Hash, Moment> {
	pub fn new(
		id: Hash,
		menstrual_calendar_id: Hash,
		date: Moment,
		menstruation: bool,
		symptoms: Vec<Symptom>,
		created_at: Moment,
	) -> Self {
		Self {
			id,
			menstrual_calendar_id,
			date,
			menstruation,
			symptoms,
			created_at,
			updated_at: Moment::default(),
		}
	}

	pub fn get_id(&self) -> &Hash {
		&self.id
	}

	pub fn get_menstrual_calendar_id(&self) -> &Hash {
		&self.menstrual_calendar_id
	}
}

impl<T, Hash, Moment: Default> MenstrualCycleLogT<T> for MenstrualCycleLog<Hash, Moment>
where
	T: frame_system::Config<Hash = Hash>,
{
	fn get_id(&self) -> &Hash {
		self.get_id()
	}
	fn get_menstrual_calendar_id(&self) -> &Hash {
		self.get_menstrual_calendar_id()
	}
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct MenstrualCalendar<AccountId, Hash, Moment> {
	pub id: Hash,
	pub address_id: AccountId,
	pub average_cycle: u8,
	pub cycle_log: Vec<Hash>,
	pub created_at: Moment,
	pub updated_at: Moment,
}

impl<AccountId, Hash, Moment: Default> MenstrualCalendar<AccountId, Hash, Moment> {
	pub fn new(id: Hash, address_id: AccountId, average_cycle: u8, created_at: Moment) -> Self {
		Self {
			id,
			address_id,
			average_cycle,
			cycle_log: Vec::new(),
			created_at,
			updated_at: Moment::default(),
		}
	}

	pub fn get_id(&self) -> &Hash {
		&self.id
	}

	pub fn get_address_id(&self) -> &AccountId {
		&self.address_id
	}
}

impl<T, AccountId, Hash, Moment: Default> MenstrualCalendarT<T>
	for MenstrualCalendar<AccountId, Hash, Moment>
where
	T: frame_system::Config<AccountId = AccountId, Hash = Hash>,
{
	fn get_id(&self) -> &Hash {
		self.get_id()
	}
	fn get_address_id(&self) -> &AccountId {
		self.get_address_id()
	}
}
