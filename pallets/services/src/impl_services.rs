use crate::*;

use traits_services::ServiceOwnerInfo;

/// Service Interface Implementation
impl<T: Config> ServiceInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type ServiceId = T::Hash;
	type Service = ServiceOf<T>;
	type ServiceInfo = ServiceInfoOf<T>;
	type ServiceFlow = ServiceFlow;

	/// Create Service
	/// Add reference to ServicesByCountryCity storage
	/// Associate service reference to the owner (creator)
	/// Increment Counts
	fn create_service(
		owner_id: &T::AccountId,
		service_info: &Self::ServiceInfo,
		service_flow: &Self::ServiceFlow,
	) -> Result<Self::Service, Self::Error> {
		// Check if user can create_service
		let can_create_service = T::ServiceOwner::can_create_service(owner_id);
		if !can_create_service {
			return Err(Error::<T>::NotAllowedToCreate)
		}

		let owner_service_count = <Self as ServiceInterface<T>>::services_count_by_owner(owner_id);
		let service_id = Self::generate_service_id(owner_id, owner_service_count);

		// Calculate total price
		let mut service_info_mut = service_info.clone();
		for (idx, price_by_currency) in service_info.prices_by_currency.iter().enumerate() {
			service_info_mut.prices_by_currency[idx].total_price -= price_by_currency.total_price;
			for price_component in price_by_currency.price_components.iter() {
				service_info_mut.prices_by_currency[idx].total_price += price_component.value;
			}

			for additional_price in price_by_currency.additional_prices.iter() {
				service_info_mut.prices_by_currency[idx].total_price += additional_price.value;
			}
		}

		let service =
			Service::new(service_id, owner_id.clone(), service_info_mut, service_flow.clone());
		// Store to Services storage
		Services::<T>::insert(service_id, &service);

		// Increment Services Count
		Self::add_services_count();
		// Increment ServicesCountByOwner
		Self::add_services_count_by_owner(&service.owner_id);

		// Associate created service to the owner
		T::ServiceOwner::associate(owner_id, &service_id);

		Ok(service)
	}

	/// Update Service information
	fn update_service(
		owner_id: &T::AccountId,
		service_id: &Self::ServiceId,
		service_info: &Self::ServiceInfo,
	) -> Result<Self::Service, Self::Error> {
		let service = Services::<T>::get(service_id);
		if service.is_none() {
			return Err(Error::<T>::ServiceDoesNotExist)
		}
		let mut service = service.unwrap();

		if service.owner_id != owner_id.clone() {
			return Err(Error::<T>::NotServiceOwner)
		}

		// Calculate total price
		let mut service_info_mut = service_info.clone();
		for (idx, price_by_currency) in service_info.prices_by_currency.iter().enumerate() {
			service_info_mut.prices_by_currency[idx].total_price -= price_by_currency.total_price;
			for price_component in price_by_currency.price_components.iter() {
				service_info_mut.prices_by_currency[idx].total_price += price_component.value;
			}

			for additional_price in price_by_currency.additional_prices.iter() {
				service_info_mut.prices_by_currency[idx].total_price += additional_price.value;
			}
		}

		service.info = service_info_mut;
		Services::<T>::insert(service_id, &service);

		Ok(service)
	}

	/// Delete Service
	/// Delete from Services Storage
	/// Remove the service id reference in ServicesByCountryCity storage
	/// Disassociate service id from the owner
	/// Decrement Counts
	fn delete_service(
		owner_id: &T::AccountId,
		service_id: &Self::ServiceId,
	) -> Result<Self::Service, Self::Error> {
		let service = Services::<T>::get(service_id);
		if service.is_none() {
			return Err(Error::<T>::ServiceDoesNotExist)
		}
		let service = service.unwrap();

		if service.owner_id != owner_id.clone() {
			return Err(Error::<T>::NotServiceOwner)
		}
		// Remove service from storage
		let service = Services::<T>::take(service_id).unwrap();

		let owner = T::ServiceOwner::get_owner(owner_id).unwrap();
		// disassociate service reference from the owner
		T::ServiceOwner::disassociate(owner.get_id(), &service.id);
		// Decrement counts
		Self::sub_services_count();
		Self::sub_services_count_by_owner(owner.get_id());

		Ok(service)
	}

	fn service_by_id(service_id: &Self::ServiceId) -> Option<Self::Service> {
		Services::<T>::get(service_id)
	}

	fn services_count_by_owner(owner_id: &T::AccountId) -> u64 {
		Self::services_count_by_owner(owner_id).unwrap_or(0)
	}
}

/// ServicesProvider Trait Implementation
impl<T: Config, Balance> ServicesProvider<T, Balance> for Pallet<T>
where
	ServiceOf<T>: traits_services::ServiceInfo<T, Balance>,
{
	type Error = Error<T>;
	type Service = ServiceOf<T>;

	fn service_by_id(id: &T::Hash) -> Option<ServiceOf<T>> {
		<Self as ServiceInterface<T>>::service_by_id(id)
	}

	fn delete_service(owner_id: &T::AccountId, id: &T::Hash) -> Result<Self::Service, Self::Error> {
		<Self as ServiceInterface<T>>::delete_service(owner_id, id)
	}
}
