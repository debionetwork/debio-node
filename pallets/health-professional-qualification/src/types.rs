use frame_support::{
	pallet_prelude::{Decode, Encode},
	RuntimeDebug,
};
use scale_info::TypeInfo;
use sp_std::vec::Vec;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct Experience {
	pub title: Vec<u8>,
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct Certification {
	pub title: Vec<u8>,
	pub issuer: Vec<u8>,
	pub month: Vec<u8>,
	pub year: Vec<u8>,
	pub description: Vec<u8>,
	pub supporting_document: Option<Vec<u8>>,
}

#[derive(Encode, Decode, Default, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct QualificationInfo {
	experiences: Vec<Experience>,
	certifications: Vec<Certification>,
}
impl QualificationInfo {
	pub fn new(experiences: &[Experience], certifications: &[Certification]) -> Self {
		Self { experiences: experiences.to_vec(), certifications: certifications.to_vec() }
	}
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct Qualification<Hash, AccountId>
where
	Hash: PartialEq + Eq,
	AccountId: Clone,
{
	id: Hash,
	owner: AccountId,
	info: QualificationInfo,
}
impl<Hash, AccountId> Qualification<Hash, AccountId>
where
	Hash: PartialEq + Eq,
	AccountId: Clone + Eq,
{
	pub fn new(
		id: Hash,
		owner: &AccountId,
		experiences: &[Experience],
		certifications: &[Certification],
	) -> Self {
		let info = QualificationInfo::new(experiences, certifications);

		Self { id, owner: owner.clone(), info }
	}

	pub fn is_authorized_owner(self, account_id: &AccountId) -> Option<Self> {
		if &self.owner != account_id {
			return None
		}

		Some(self)
	}

	pub fn set_experiences(&mut self, experiences: &[Experience]) {
		self.info.experiences = experiences.to_vec();
	}

	pub fn set_certifications(&mut self, certifications: &[Certification]) {
		self.info.certifications = certifications.to_vec();
	}
}
