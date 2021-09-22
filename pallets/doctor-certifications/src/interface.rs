//use sp_std::prelude::*;

pub trait DoctorCertificationInterface<T: frame_system::Config> {
    type Error;
    type DoctorCertificationId;
    type DoctorCertification;
    type DoctorCertificationInfo;

    fn generate_certification_id(
        owner_id: &T::AccountId,
        certification_count: u64,
    ) -> Self::DoctorCertificationId;

    fn create_certification(
        owner_id: &T::AccountId,
        certification: &Self::DoctorCertificationInfo,
    ) -> Result<Self::DoctorCertification, Self::Error>;
    fn update_certification(
        owner_id: &T::AccountId,
        certification_id: &Self::DoctorCertificationId,
        certification: &Self::DoctorCertificationInfo,
    ) -> Result<Self::DoctorCertification, Self::Error>;
    fn delete_certification(
        owner_id: &T::AccountId,
        certification_id: &Self::DoctorCertificationId,
    ) -> Result<Self::DoctorCertification, Self::Error>;

    fn certification_count_by_owner(owner_id: &T::AccountId) -> u64;
    fn certification_by_id(
        certification_id: &Self::DoctorCertificationId,
    ) -> Option<Self::DoctorCertification>;
}
