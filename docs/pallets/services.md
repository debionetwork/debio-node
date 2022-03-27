## Services Pallet
This handles the logic for managing a `Lab`'s services. A user can only create a `Service` if his/her account is registered as a `Lab`.

The extrinsic calls exposed are:
### Create Service
```rust
pub fn create_service(
    origin: OriginFor<T>,
    service_info: ServiceInfoOf<T>,
    service_flow: ServiceFlow,
) -> DispatchResultWithPostInfo
```
### Update Service
```rust
pub fn update_service(
    origin: OriginFor<T>,
    service_id: HashOf<T>,
    service_info: ServiceInfoOf<T>,
) -> DispatchResultWithPostInfo
```
### Delete Service
```rust
pub fn delete_service(
    origin: OriginFor<T>,
    service_id: T::Hash,
) -> DispatchResultWithPostInfo
```