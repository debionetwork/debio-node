//use sp_std::prelude::*;

pub trait ServiceInterface<T: frame_system::Config> {
    type Error;
    type ServiceId;
    type Service;
    type ServiceInfo;

    fn generate_service_id(owner_id: &T::AccountId, service_count: u64) -> Self::ServiceId;

    fn create_service(owner_id: &T::AccountId, service: &Self::ServiceInfo) -> Result<Self::Service, Self::Error>;
    fn update_service(owner_id: &T::AccountId, service_id: &Self::ServiceId, service: &Self::ServiceInfo) -> Result<Self::Service, Self::Error>;
    fn delete_service(owner_id: &T::AccountId, service_id: &Self::ServiceId) -> Result<Self::Service, Self::Error>;

    fn services_count_by_owner(owner_id: &T::AccountId) -> u64;
    fn service_by_id(service_id: &Self::ServiceId) -> Option<Self::Service>;
}
