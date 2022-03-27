
## Hospital Certifications Pallet
The Hospital Certifications pallet handles the logic for creating, updating, and deleting `Hospital`'s certifications.

This pallet exposes the following extrinsic calls:
### Create Hospital Certification
```rust
pub fn create_certification(
    origin: OriginFor<T>,
    certification_info: HospitalCertificationInfoOf,
) -> DispatchResultWithPostInfo
```
### Update Hospital Certification
```rust
pub fn update_certification(
    origin: OriginFor<T>,
    certification_id: HashOf<T>,
    certification_info: HospitalCertificationInfoOf,
) -> DispatchResultWithPostInfo
```
### Delete Hospital Certification
```rust
pub fn delete_certification(
    origin: OriginFor<T>,
    certification_id: T::Hash,
) -> DispatchResultWithPostInfo
```