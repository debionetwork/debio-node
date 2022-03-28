## Certifications Pallet
The Certifications pallet handles the logic for creating, updating, and deleting `Lab`'s certifications.

This pallet exposes the following extrinsic calls:
### Create Lab Certification
```rust
pub fn create_certification(
    origin: OriginFor<T>, 
    certification_info: CertificationInfoOf
) -> DispatchResultWithPostInfo
```
### Update Lab Certification
```rust
pub fn update_certification(
    origin: OriginFor<T>, 
    certification_id: HashOf<T>, 
    certification_info: CertificationInfoOf
) -> DispatchResultWithPostInfo
```
### Delete Lab Certification
```rust
pub fn delete_certification(
    origin: OriginFor<T>, 
    certification_id: T::Hash
) -> DispatchResultWithPostInfo
```