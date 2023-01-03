use sp_std::vec::Vec;

pub trait HealthProfessionalQualificationInterface<T: frame_system::Config> {
	type Error;
	type Experience;
	type Certification;
	type Qualification;

	fn create_qualification(
		owner: &T::AccountId,
		experiences: &[Self::Experience],
		certifications: &[Self::Certification],
	) -> Result<Self::Qualification, Self::Error>;

	fn update_qualification(
		owner: &T::AccountId,
		qualification_id: &T::Hash,
		experiences: &Option<Vec<Self::Experience>>,
		certifications: &Option<Vec<Self::Certification>>,
	) -> Result<(), Self::Error>;

	fn delete_qualification(
		owner: &T::AccountId,
		qualification_id: &T::Hash,
	) -> Result<(), Self::Error>;
}
