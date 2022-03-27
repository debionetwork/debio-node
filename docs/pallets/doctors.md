## Doctors Pallet
The Doctors pallet handles the logic for registration, deregistration, and updating information of `Doctor` accounts.

This pallet exposes the following extrinsic calls:
### Register Doctor
```rust
pub fn register_doctor(
    origin: OriginFor<T>, 
    doctor_info: DoctorInfo
) -> DispatchResultWithPostInfo
```
### Update Doctor
```rust
pub fn update_doctor(
    origin: OriginFor<T>, 
    doctor_info: DoctorInfo
) -> DispatchResultWithPostInfo
```
### Deregister Doctor
```rust
pub fn deregister_doctor(
    origin: OriginFor<T>
) -> DispatchResultWithPostInfo
```
