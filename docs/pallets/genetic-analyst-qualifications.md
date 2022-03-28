## Genetic Analyst Qualifications Pallet
The Genetic Analyst Qualifications pallet handles the logic for creating, updating, and deleting `GeneticAnalyst`'s qualifications.

This pallet exposes the following extrinsic calls:
### Create Genetic Analyst Qualification
```rust
pub fn create_qualification(
    origin: OriginFor<T>,
    qualification_info: GeneticAnalystQualificationInfoOf,
) -> DispatchResultWithPostInfo
```
### Bulk Create Multiple Genetic Analyst Qualifications
```rust
pub fn bulk_create_qualification(
    origin: OriginFor<T>,
    qualification_infos: Vec<GeneticAnalystQualificationInfoOf>,
) -> DispatchResultWithPostInfo
```
### Update Genetic Analyst Qualification
```rust
pub fn update_qualification(
    origin: OriginFor<T>,
    qualification_id: HashOf<T>,
    qualification_info: GeneticAnalystQualificationInfoOf,
) -> DispatchResultWithPostInfo
```
### Delete Genetic Analyst Qualification
```rust
pub fn delete_qualification(
    origin: OriginFor<T>,
    qualification_id: T::Hash,
) -> DispatchResultWithPostInfo
```