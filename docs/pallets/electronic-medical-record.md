## Electronic Medical Record Pallet
This pallet handles the logic of tracking and storing the `ElectronicMedicalRecord` on the blockchain.

One account can have multiple `ElectronicMedicalRecord`s, and one `ElectronicMedicalRecord` contains multiple `ElectronicMedicalRecordFile` structs.

Each `ElectronicMedicalRecord` will be mapped to an account, and each `ElectronicMedicalRecordFile` will be mapped to an `ElectronicMedicalRecord`.

The following extrinsic calls are exposed:
### Add a new Electronic Medical Record to Account
```rust
pub fn add_electronic_medical_record(
	origin: OriginFor<T>,
	title: Vec<u8>,
	category: Vec<u8>,
	files: Vec<ElectronicMedicalRecordFileSubmissionOf>,
) -> DispatchResultWithPostInfo
```
### Remove an Electronic Medical Record from an Account
```rust
pub fn update_electronic_medical_record(
    origin: OriginFor<T>,
    electronic_medical_record_id: HashOf<T>,
    title: Vec<u8>,
    category: Vec<u8>,
    files: Vec<ElectronicMedicalRecordFileSubmissionOf>,
) -> DispatchResultWithPostInfo
```
### Remove an Electronic Medical Record from an Account
```rust
pub fn remove_electronic_medical_record(
    origin: OriginFor<T>,
    electronic_medical_record_id: HashOf<T>,
) -> DispatchResultWithPostInfo
```