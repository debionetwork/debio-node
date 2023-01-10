use frame_support::{
	pallet_prelude::{Decode, Encode},
	sp_runtime::traits::Saturating,
	RuntimeDebug,
};
use scale_info::TypeInfo;
use sp_std::vec::Vec;

use primitives_availability_status::AvailabilityStatus;
use primitives_stake_status::StakeStatus;
use primitives_verification_status::VerificationStatus;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct HealthProfessionalInfo<Hash, Moment>
where
	Hash: PartialEq + Eq + Clone,
	Moment: Clone,
{
	pub box_public_key: Hash,
	pub first_name: Vec<u8>,
	pub last_name: Vec<u8>,
	pub myriad_username: Vec<u8>,
	pub gender: Vec<u8>,
	pub date_of_birth: Moment,
	pub email: Vec<u8>,
	pub phone_number: Vec<u8>,
	pub role: Vec<u8>,
	pub category: Vec<u8>,
	pub profile_link: Option<Vec<u8>>,
	pub profile_image: Option<Vec<u8>>,
	pub anonymous: bool,
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct HealthProfessional<AccountId, Hash, Moment, Balance>
where
	Hash: PartialEq + Eq + Clone,
	Moment: Clone,
{
	account_id: AccountId,
	qualifications: Vec<Hash>,
	info: HealthProfessionalInfo<Hash, Moment>,
	stake_amount: Balance,
	stake_status: StakeStatus,
	verification_status: VerificationStatus,
	availability_status: AvailabilityStatus,
	unstaked_at: Option<Moment>,
}
impl<AccountId, Hash, Moment, Balance> HealthProfessional<AccountId, Hash, Moment, Balance>
where
	AccountId: Clone + PartialEq + Eq,
	Hash: PartialEq + Eq + Clone,
	Moment: Default + Clone,
	Balance: Default + Saturating,
{
	pub fn new(account_id: &AccountId, info: &HealthProfessionalInfo<Hash, Moment>) -> Self {
		Self {
			account_id: account_id.clone(),
			qualifications: Vec::<Hash>::new(),
			info: info.clone(),
			stake_amount: Balance::default(),
			stake_status: StakeStatus::default(),
			verification_status: VerificationStatus::default(),
			availability_status: AvailabilityStatus::default(),
			unstaked_at: None,
		}
	}

	pub fn info(&self) -> &HealthProfessionalInfo<Hash, Moment> {
		&self.info
	}

	pub fn qualifications(&self) -> &[Hash] {
		&self.qualifications
	}

	pub fn stake_status(&self) -> &StakeStatus {
		&self.stake_status
	}

	pub fn stake_amount(&self) -> &Balance {
		&self.stake_amount
	}

	pub fn unstaked_at(&self) -> &Option<Moment> {
		&self.unstaked_at
	}

	pub fn update_info(&mut self, info: &HealthProfessionalInfo<Hash, Moment>) {
		self.info = info.clone();
	}

	pub fn update_unstaked_at(&mut self, moment: Option<Moment>) {
		self.unstaked_at = moment;
	}

	pub fn update_stake_status(&mut self, status: StakeStatus, amount: Balance) {
		self.stake_status = status.clone();
		if status != StakeStatus::WaitingForUnstaked {
			self.stake_amount = amount;
		}
		if status == StakeStatus::Unstaked {
			self.unstaked_at = None;
		}
	}

	pub fn update_verification_status(&mut self, status: &VerificationStatus) {
		self.verification_status = status.clone();
	}

	pub fn update_availability_status(&mut self, status: &AvailabilityStatus) {
		self.availability_status = status.clone();
	}

	pub fn add_qualification(&mut self, qualification_id: Hash) {
		self.qualifications.push(qualification_id);
	}

	pub fn remove_qualification(&mut self, qualification_id: Hash) {
		let position = &self.qualifications.iter().position(|x| *x == qualification_id);

		if let Some(index) = position {
			self.qualifications.remove(*index);
		}
	}
}
