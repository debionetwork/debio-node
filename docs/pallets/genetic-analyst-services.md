## Genetic Analyst Services Pallet
This handles the logic for managing a `GeneticAnalyst`'s services. A user can only create a `Service` if his/her account is registered as a `GeneticAnalyst`.

This pallet exposes the following extrinsic calls:
### Create Genetic Analyst Service
```rust
pub fn create_genetic_analyst_service(
    origin: OriginFor<T>,
    genetic_analyst_service_info: GeneticAnalystServiceInfoOf<T>,
) -> DispatchResultWithPostInfo
```
### Bulk Create Multiple Genetic Analyst Services
```rust
pub fn bulk_create_genetic_analyst_service(
    origin: OriginFor<T>,
    genetic_analyst_service_infos: Vec<GeneticAnalystServiceInfoOf<T>>,
) -> DispatchResultWithPostInfo
```
### Update Genetic Analyst Service
```rust
pub fn update_genetic_analyst_service(
    origin: OriginFor<T>,
    genetic_analyst_service_id: HashOf<T>,
    genetic_analyst_service_info: GeneticAnalystServiceInfoOf<T>,
) -> DispatchResultWithPostInfo
```
### Delete Genetic Analyst Service
```rust
pub fn delete_genetic_analyst_service(
    origin: OriginFor<T>,
    genetic_analyst_service_id: T::Hash,
) -> DispatchResultWithPostInfo
```