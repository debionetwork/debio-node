## Doctor Certifications Pallet
The Doctor Certifications pallet handles the logic for creating, updating, and deleting `Doctor`'s certifications.

This pallet exposes the following extrinsic calls:
### Create Doctor Certification
```rust
pub fn create_certification(
    origin: OriginFor<T>, 
    certification_info: DoctorCertificationInfoOf
) -> DispatchResultWithPostInfo
```
### Update Doctor Certification
```rust
pub fn update_certification(
    origin: OriginFor<T>, 
    certification_id: HashOf<T>, 
    certification_info: DoctorCertificationInfoOf
) -> DispatchResultWithPostInfo
```
### Delete Doctor Certification
```rust
pub fn delete_certification(
    origin: OriginFor<T>, 
    certification_id: T::Hash
) -> DispatchResultWithPostInfo
```