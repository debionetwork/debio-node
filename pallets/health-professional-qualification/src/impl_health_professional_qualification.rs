use crate::*;
use sp_std::vec::Vec;
use traits_health_professional_qualifications::HealthProfessionalQualificationCountT;

impl<T: Config> HealthProfessionalQualificationInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type Experience = Experience;
	type Certification = Certification;
	type Qualification = QualificationOf<T>;

	fn create_qualification(
		owner: &T::AccountId,
		experiences: &[Self::Experience],
		certifications: &[Self::Certification],
	) -> Result<Self::Qualification, Self::Error> {
		if !T::HealthProfessionalQualificationOwner::can_create_qualification(owner) {
			return Err(Error::<T>::NotRegistered)
		};

		let qualification_count = HealthProfessionalQualificationCount::<T>::get();
		let qualification_id = Self::generate_qualification_id(owner, qualification_count);
		let qualification =
			Qualification::new(qualification_id, owner, experiences, certifications);

		HealthProfessionalQualifications::<T>::insert(qualification_id, &qualification);

		T::HealthProfessionalQualificationOwner::associate(owner, &qualification_id);

		Self::add_health_professional_qualification_count(1);
		Self::add_health_professional_qualification_count_by_owner(owner, 1);

		Ok(qualification)
	}

	fn update_qualification(
		owner: &T::AccountId,
		qualification_id: &T::Hash,
		experiences: &Option<Vec<Self::Experience>>,
		certifications: &Option<Vec<Self::Certification>>,
	) -> Result<Self::Qualification, Self::Error> {
		let mut qualification = HealthProfessionalQualifications::<T>::get(qualification_id)
			.ok_or(Error::<T>::NotFound)?
			.is_authorized_owner(owner)
			.ok_or(Error::<T>::Unauthorized)?;

		if let Some(experiences) = experiences.as_ref() {
			qualification.set_experiences(experiences);
		}

		if let Some(certifications) = certifications.as_ref() {
			qualification.set_certifications(certifications);
		}

		HealthProfessionalQualifications::<T>::insert(qualification_id, &qualification);

		Ok(qualification)
	}

	fn delete_qualification(
		owner: &T::AccountId,
		qualification_id: &T::Hash,
	) -> Result<(), Self::Error> {
		let _ = HealthProfessionalQualifications::<T>::get(qualification_id)
			.ok_or(Error::<T>::NotFound)?
			.is_authorized_owner(owner)
			.ok_or(Error::<T>::Unauthorized)?;

		HealthProfessionalQualifications::<T>::remove(qualification_id);

		T::HealthProfessionalQualificationOwner::disassociate(owner, qualification_id);

		Self::substract_health_professional_qualification_count(1);
		Self::substract_health_professional_qualification_count_by_owner(owner, 1);

		Ok(())
	}
}
