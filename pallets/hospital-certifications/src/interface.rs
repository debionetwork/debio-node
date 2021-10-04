//use sp_std::prelude::*;

pub trait HospitalCertificationInterface<T: frame_system::Config> {
    type Error;
    type HospitalCertificationId;
    type HospitalCertification;
    type HospitalCertificationInfo;

    fn generate_certification_id(
        owner_id: &T::AccountId,
        certification_count: u64,
    ) -> Self::HospitalCertificationId;

    fn create_certification(
        owner_id: &T::AccountId,
        certification: &Self::HospitalCertificationInfo,
    ) -> Result<Self::HospitalCertification, Self::Error>;
    fn update_certification(
        owner_id: &T::AccountId,
        certification_id: &Self::HospitalCertificationId,
        certification: &Self::HospitalCertificationInfo,
    ) -> Result<Self::HospitalCertification, Self::Error>;
    fn delete_certification(
        owner_id: &T::AccountId,
        certification_id: &Self::HospitalCertificationId,
    ) -> Result<Self::HospitalCertification, Self::Error>;

    fn certification_count_by_owner(owner_id: &T::AccountId) -> u64;
    fn certification_by_id(
        certification_id: &Self::HospitalCertificationId,
    ) -> Option<Self::HospitalCertification>;
}
