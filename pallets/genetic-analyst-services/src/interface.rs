pub trait GeneticAnalystServiceInterface<T: frame_system::Config> {
    type Error;
    type GeneticAnalystServiceId;
    type GeneticAnalystService;
    type GeneticAnalystServiceInfo;

    fn generate_genetic_analyst_service_id(owner_id: &T::AccountId, genetic_analyst_service_count: u64) -> Self::GeneticAnalystServiceId;

    fn create_genetic_analyst_service(
        owner_id: &T::AccountId,
        genetic_analyst_service: &Self::GeneticAnalystServiceInfo
    ) -> Result<Self::GeneticAnalystService, Self::Error>;
    fn update_genetic_analyst_service(
        owner_id: &T::AccountId,
        genetic_analyst_service_id: &Self::GeneticAnalystServiceId,
        genetic_analyst_service: &Self::GeneticAnalystServiceInfo,
    ) -> Result<Self::GeneticAnalystService, Self::Error>;
    fn delete_genetic_analyst_service(
        owner_id: &T::AccountId,
        genetic_analyst_service_id: &Self::GeneticAnalystServiceId,
    ) -> Result<Self::GeneticAnalystService, Self::Error>;

    fn genetic_analyst_services_count_by_owner(owner_id: &T::AccountId) -> u64;
    fn genetic_analyst_service_by_id(genetic_analyst_service_id: &Self::GeneticAnalystServiceId) -> Option<Self::GeneticAnalystService>;
}
