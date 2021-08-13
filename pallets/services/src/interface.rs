// use sp_std::prelude::*;
use sp_std::vec::Vec;

pub trait ServiceInterface<T: frame_system::Config> {
    type Error;
    type ServiceId;
    type Service;
    type ServiceInfo;
    type Hash;

    fn generate_service_id(owner_id: &T::AccountId, service_count: u64) -> Self::ServiceId;

    fn create_service(owner_id: &T::AccountId, service: &Self::ServiceInfo) -> Result<Self::Service, Self::Error>;
    fn update_service(owner_id: &T::AccountId, service_id: &Self::ServiceId, service: &Self::ServiceInfo) -> Result<Self::Service, Self::Error>;
    fn delete_service(owner_id: &T::AccountId, service_id: &Self::ServiceId) -> Result<Self::Service, Self::Error>;
    fn request_service_staking(owner_id: &T::AccountId, lab_id: Option<&T::AccountId>, service_category: Vec<u8>, amount_staked: u128, tx_hash: &Self::Hash, request_hash: &Self::Hash, country: Vec<u8>, city: Vec<u8>) -> Result<Self::Service, Self::Error>;

    fn services_count_by_owner(owner_id: &T::AccountId) -> u64;
    fn service_by_id(service_id: &Self::ServiceId) -> Option<Self::Service>;
}
