//use sp_std::prelude::*;

pub trait CertificationInterface<T: frame_system::Config> {
	type Error;
	type CertificationId;
	type Certification;
	type CertificationInfo;

	fn generate_certification_id(
		owner_id: &T::AccountId,
		certification_count: u64,
	) -> Self::CertificationId;

	fn create_certification(
		owner_id: &T::AccountId,
		certification: &Self::CertificationInfo,
	) -> Result<Self::Certification, Self::Error>;
	fn update_certification(
		owner_id: &T::AccountId,
		certification_id: &Self::CertificationId,
		certification: &Self::CertificationInfo,
	) -> Result<Self::Certification, Self::Error>;
	fn delete_certification(
		owner_id: &T::AccountId,
		certification_id: &Self::CertificationId,
	) -> Result<Self::Certification, Self::Error>;

	fn certification_count_by_owner(owner_id: &T::AccountId) -> u64;
	fn certification_by_id(certification_id: &Self::CertificationId)
		-> Option<Self::Certification>;
}
