use sp_std::prelude::*;

pub trait ServiceInterface<T: frame_system::Config> {
    type Error;
    type ServiceId;
    type Service;
    type ServiceInfo;

    fn create_service(owner_id: &T::AccountId, service: &Self::ServiceInfo) -> Result<(), Self::Error>;
    fn update_service(service_id: &Self::ServiceId, service: &Self::ServiceInfo) -> Result<(), Self::Error>;
    fn delete_service(service_id: &Self::ServiceId) -> Result<(), Self::Error>;

    fn generate_service_id(owner_id: &T::AccountId, service_count: u64) -> Self::ServiceId;

    fn service_by_id(service_id: &Self::ServiceId) -> Option<Self::Service>;
    fn services_by_country_city(country: Vec<u8>, city: Vec<u8>) -> Option<Vec<Self::ServiceId>>;

    fn services_count_by_owner(owner_id: &T::AccountId) -> u64;
}
