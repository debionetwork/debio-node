# Description of DeBio Pallets
DeBio blockchain runtime uses the following custom pallets to handle its business logic

## Labs Pallet
The Labs pallet handles the logic for registration, deregistration, and updating information of `Lab` accounts.

This pallet exposes the following extrinsic calls:
### Register Lab
```rust
pub fn register_lab(origin: OriginFor<T>, lab_info: LabInfo) -> DispatchResultWithPostInfo
```
### Deregister Lab
```rust
pub fn deregister_lab(origin: OriginFor<T>) -> DispatchResultWithPostInfo
```
### Update Lab
```rust
pub fn update_lab(origin: OriginFor<T>, lab_info: LabInfo) -> DispatchResultWithPostInfo
```
### Update Lab Verification Status
```rust
pub fn update_lab_verification_status(origin: OriginFor<T>, account_id: T::AccountId, lab_verification_status: LabVerificationStatus) -> DispatchResultWithPostInfo
```

## Certifications Pallet
The Certifications pallet handles the logic for creating, updating, and deleting `Lab`'s certifications.

This pallet exposes the following extrinsic calls:
### Create Lab Certification
```rust
pub fn create_certification(origin: OriginFor<T>, certification_info: CertificationInfoOf) -> DispatchResultWithPostInfo
```
### Update Lab Certification
```rust
pub fn update_certification(origin: OriginFor<T>, certification_id: HashOf<T>, certification_info: CertificationInfoOf) -> DispatchResultWithPostInfo
```
### Delete Lab Certification
```rust
pub fn delete_certification(origin: OriginFor<T>, certification_id: T::Hash) -> DispatchResultWithPostInfo
```

## Services Pallet
This handles the logic for managing a `Lab`'s services. A user can only create a `Service` if his/her account is registered as a `Lab` and has set their Ethereum address using the `set_eth_address` extrinsic in the `UserProfile` pallet.
The extrinsic calls exposed are:
### Create Service
```rust
pub fn create_service(origin: OriginFor<T>, service_info: ServiceInfoOf<T>, service_flow: ServiceFlow) -> DispatchResultWithPostInfo
```
### Update Service
```rust
pub fn update_service(origin: OriginFor<T>, service_id: HashOf<T>, service_info: ServiceInfoOf<T>) -> DispatchResultWithPostInfo
```
### Delete Service
```rust
pub fn delete_service(origin: OriginFor<T>, service_id: T::Hash) -> DispatchResultWithPostInfo
```

## Orders Pallet
This pallet handles the business logic of orders. An `Order` is associated with a `Service` and a `DnaSample`.

An `Order` is fulfilled if the associated `DnaSample` is successfully processed.
An `Order` payment can be refunded if the physical `DnaSample` is rejected upon receipt, it has not been processed for 7 days, or if the sample processing has failed.

Currently DeBio uses the ethereum network for payments in DAI. We use an escrow bridge that listens for payments to a smart contract deployed on the ethereum network and triggers an extrinsic call in DeBio that updates the status of the `Order` to paid.

Before a user can create an order, he/she is required to set their Ethereum address using the `set_eth_address` extrinsic in the `UserProfile` pallet.

This pallet exposes the following extrinsics:
### Create Order
```rust
pub fn create_order(origin: OriginFor<T>, service_id: T::Hash, price_index: u32, customer_box_public_key: T::Hash, order_flow: ServiceFlow) -> DispatchResultWithPostInfo
```
### Set Order Paid
```rust
pub fn set_order_paid(origin: OriginFor<T>, order_id: T::Hash) -> DispatchResultWithPostInfo
```
### Cancel Order
```rust
pub fn cancel_order(origin: OriginFor<T>, order_id: T::Hash) -> DispatchResultWithPostInfo
```
### Fulfill Order
```rust
pub fn fulfill_order(origin: OriginFor<T>, order_id: T::Hash) -> DispatchResultWithPostInfo
```
### Set Order is Refunded
```rust
pub fn set_order_refunded(origin: OriginFor<T>, order_id: T::Hash) -> DispatchResultWithPostInfo
```

## Genetic Testing Pallet
This pallet handles the logic of tracking `DnaSample` and storing `DnaTestResult` on the blockchain.

`DnaSample`s are sent by customers to a `Lab` to be processed. The result is then submitted to DeBio blockchain as `DnaTestResult`.

Users can also submit `DnaTestResult` that are processed off chain.

The following extrinsic calls are exposed:
### Reject DNA Sample
```rust
pub fn reject_dna_sample(origin: OriginFor<T>, tracking_id: Vec<u8>, rejected_title: Vec<u8>, rejected_description: Vec<u8>) -> DispatchResultWithPostInfo
```
### Process DNA Sample
```rust
pub fn process_dna_sample(origin: OriginFor<T>, tracking_id: Vec<u8>, status: DnaSampleStatus) -> DispatchResultWithPostInfo
```
### Submit Test Result
```rust
pub fn submit_test_result(origin: OriginFor<T>, tracking_id: Vec<u8>, submission: DnaTestResultSubmission) -> DispatchResultWithPostInfo
```
### Submit Independent Test Result
```rust
pub fn submit_independent_test_result(origin: OriginFor<T>, submission: DnaTestResultSubmission) -> DispatchResultWithPostInfo
```
### Submit Data Bounty Details
```rust
pub fn submit_data_bounty_details(origin: OriginFor<T>, data_hash: T::Hash, order_id: T::Hash) -> DispatchResultWithPostInfo
```

## Doctors Pallet
The Doctors pallet handles the logic for registration, deregistration, and updating information of `Doctor` accounts.

This pallet exposes the following extrinsic calls:
### Register Doctor
```rust
pub fn register_doctor(origin: OriginFor<T>, doctor_info: DoctorInfo) -> DispatchResultWithPostInfo
```
### Deregister Doctor
```rust
pub fn deregister_doctor(origin: OriginFor<T>) -> DispatchResultWithPostInfo
```
### Update Doctor
```rust
pub fn update_doctor(origin: OriginFor<T>, doctor_info: DoctorInfo) -> DispatchResultWithPostInfo
```

## Doctor Certifications Pallet
The Doctor Certifications pallet handles the logic for creating, updating, and deleting `Doctor`'s certifications.

This pallet exposes the following extrinsic calls:
### Create Doctor Certification
```rust
pub fn create_certification(origin: OriginFor<T>, certification_info: DoctorCertificationInfoOf) -> DispatchResultWithPostInfo
```
### Update Doctor Certification
```rust
pub fn update_certification(origin: OriginFor<T>, certification_id: HashOf<T>, certification_info: DoctorCertificationInfoOf) -> DispatchResultWithPostInfo
```
### Delete Doctor Certification
```rust
pub fn delete_certification(origin: OriginFor<T>, certification_id: T::Hash) -> DispatchResultWithPostInfo
```

## Hospitals Pallet
The Hospitals pallet handles the logic for registration, deregistration, and updating information of `Hospital` accounts.

This pallet exposes the following extrinsic calls:
### Register Hospital
```rust
pub fn register_hospital(origin: OriginFor<T>, hospital_info: HospitalInfo) -> DispatchResultWithPostInfo
```
### Deregister Hospital
```rust
pub fn deregister_hospital(origin: OriginFor<T>) -> DispatchResultWithPostInfo
```
### Update Hospital
```rust
pub fn update_hospital(origin: OriginFor<T>, hospital_info: HospitalInfo) -> DispatchResultWithPostInfo
```

## Hospital Certifications Pallet
The Hospital Certifications pallet handles the logic for creating, updating, and deleting `Hospital`'s certifications.

This pallet exposes the following extrinsic calls:
### Create Hospital Certification
```rust
pub fn create_certification(origin: OriginFor<T>, certification_info: HospitalCertificationInfoOf) -> DispatchResultWithPostInfo
```
### Update Hospital Certification
```rust
pub fn update_certification(origin: OriginFor<T>, certification_id: HashOf<T>, certification_info: HospitalCertificationInfoOf) -> DispatchResultWithPostInfo
```
### Delete Hospital Certification
```rust
pub fn delete_certification(origin: OriginFor<T>, certification_id: T::Hash) -> DispatchResultWithPostInfo
```

## Electronic Medical Record Pallet
This pallet handles the logic of tracking and storing the `ElectronicMedicalRecord` on the blockchain.

One account can have multiple `ElectronicMedicalRecord`s, and one `ElectronicMedicalRecord` contains multiple `ElectronicMedicalRecordFile`s.

Each `ElectronicMedicalRecord` will be mapped to an account, and each `ElectronicMedicalRecordFile` will be mapped to an `ElectronicMedicalRecord`.

The following extrinsic calls are exposed:
### Add a new Electronic Medical Record to Account
```rust
pub fn add_electronic_medical_record(origin: OriginFor<T>, title: Vec<u8>, category: Vec<u8>) -> DispatchResultWithPostInfo
```
### Remove an Electronic Medical Record from an Account
```rust
pub fn remove_electronic_medical_record(origin: OriginFor<T>, electronic_medical_record_id: HashOf<T>) -> DispatchResultWithPostInfo
```
### Attach a new file to an existing Electronic Medical Record
```rust
pub fn add_electronic_medical_record_file(origin: OriginFor<T>, electronic_medical_record_id: HashOf<T>, mut title: Vec<u8>, mut description: Vec<u8>, mut record_link: Vec<u8>) -> DispatchResultWithPostInfo
```
### Remove file attached to an existing Electronic Medical Record
```rust
pub fn remove_electronic_medical_record_file(origin: OriginFor<T>, electronic_medical_record_file_id: HashOf<T>) -> DispatchResultWithPostInfo
```

## Service Request Pallet
The Service Request pallet handles the logic for creating, claiming, processing, and finalizing Request.

This pallet exposes the following extrinsic calls:

The following extrinsic calls are exposed:
### Create Request
```rust
pub fn create_request(
			origin: OriginFor<T>,
			country: CountryOf,
			region: RegionOf,
			city: CityOf,
			service_category: ServiceCategoryOf,
			staking_amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo
```

### Claim Request
```rust
pub fn claim_request(
  origin: OriginFor<T>,
  request_id: HashOf<T>,
  service_id: HashOf<T>,
  testing_price: BalanceOf<T>,
  qc_price: BalanceOf<T>,
) -> DispatchResultWithPostInfo
```

### Process Request
```rust
pub fn process_request(
  origin: OriginFor<T>,
  lab_id: LabIdOf<T>,
  request_id: HashOf<T>,
  order_id: HashOf<T>,
  dna_sample_tracking_id: DNASampleTrackingIdOf,
  additional_staking_amount: BalanceOf<T>,
) -> DispatchResultWithPostInfo
```

### Finalize Request
```rust
pub fn finalize_request(
  origin: OriginFor<T>,
  request_id: HashOf<T>,
  test_result_success: bool,
) -> DispatchResultWithPostInfo
```

## User Profile Pallet
Currently this pallet only stores a user's Ethereum address that is used to make and receive payments

### Set DAI Address
```rust
pub fn set_eth_address(origin: OriginFor<T>, eth_address: EthereumAddressOf<T>) -> DispatchResultWithPostInfo
```
### Set DAI Address using Sudo Account
```rust
pub fn sudo_set_eth_address(origin: OriginFor<T>, account_id: AccountIdOf<T>, eth_address: EthereumAddressOf<T>) -> DispatchResultWithPostInfo
```

## Rewards Pallet
The rewards pallet is only used to send rewards using the designated accounts from the rewards escrow in the pallet.

### Send Rewards using Authorized Account
```rust
pub fn reward_funds(origin: OriginFor<T>, to_reward: T::AccountId, reward: BalanceOf<T>) -> DispatchResultWithPostInfo
```
