## Hospitals Pallet
The Hospitals pallet handles the logic for registration, deregistration, and updating information of `Hospital` accounts.

This pallet exposes the following extrinsic calls:
### Register Hospital
```rust
pub fn register_hospital(
    origin: OriginFor<T>,
    hospital_info: HospitalInfo,
) -> DispatchResultWithPostInfo
```
### Deregister Hospital
```rust
pub fn deregister_hospital(
    origin: OriginFor<T>
) -> DispatchResultWithPostInfo
```
### Update Hospital
```rust
pub fn update_hospital(
    origin: OriginFor<T>,
    hospital_info: HospitalInfo,
) -> DispatchResultWithPostInfo
```