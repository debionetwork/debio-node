//use sp_std::prelude::*;

pub trait GeneticAnalystQualificationInterface<T: frame_system::Config> {
    type Error;
    type GeneticAnalystQualificationId;
    type GeneticAnalystQualification;
    type GeneticAnalystQualificationInfo;

    fn generate_qualification_id(
        owner_id: &T::AccountId,
        qualification_count: u64,
    ) -> Self::GeneticAnalystQualificationId;

    fn create_qualification(
        owner_id: &T::AccountId,
        qualification: &Self::GeneticAnalystQualificationInfo,
    ) -> Result<Self::GeneticAnalystQualification, Self::Error>;
    fn update_qualification(
        owner_id: &T::AccountId,
        qualification_id: &Self::GeneticAnalystQualificationId,
        qualification: &Self::GeneticAnalystQualificationInfo,
    ) -> Result<Self::GeneticAnalystQualification, Self::Error>;
    fn delete_qualification(
        owner_id: &T::AccountId,
        qualification_id: &Self::GeneticAnalystQualificationId,
    ) -> Result<Self::GeneticAnalystQualification, Self::Error>;

    fn qualification_count_by_owner(owner_id: &T::AccountId) -> u64;
    fn qualification_by_id(
        qualification_id: &Self::GeneticAnalystQualificationId,
    ) -> Option<Self::GeneticAnalystQualification>;
}
