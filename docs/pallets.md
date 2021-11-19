# Description of DeBio Pallets
DeBio blockchain runtime uses the following custom pallets to handle its business logic

## Labs Pallet
The Labs pallet handles the logic for registration, deregistration, and updating information of `Lab` accounts. This pallet exposes the following extrinsic calls:
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
This handles the logic for managing a `Lab`'s services. A user can only create a `Service` if his/her account is registered as a `Lab` and has set their Eth address using the `set_eth_address` extrinsic in the User Profile pallet.
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

Currently DeBio uses the ethereum network for payments in USDT. We use an escrow bridge that listens for payments to a smart contract deployed on the ethereum network and triggers an extrinsic call in DeBio that updates the status of the `Order` to paid. 

Before a user can create an order, he/she is required to set an eth address in the User Profile Pallet

This pallet exposes the following extrinsics:
### Create Order
```rust
pub fn create_order(origin: OriginFor<T>, service_id: T::Hash) -> DispatchResultWithPostInfo
```
### Set Order Paid
```rust
pub fn set_order_paid(origin: OriginFor<T>, order_id: T::Hash) -> DispatchResultWithPostInfo
```
### Fulfill Order
```rust
pub fn fulfill_order(origin: OriginFor<T>, order_id: T::Hash) -> DispatchResultWithPostInfo
```
### Refund Order
```rust
pub fn refund_order(origin: OriginFor<T>, order_id: T::Hash) -> DispatchResultWithPostInfo
```

## Genetic Testing Pallet
This pallet handles the logic of tracking `DnaSample` and storing `DnaTestResult` on the blockchain.
`DnaSample`s are sent by customers to a `Lab` to be processed. The result is then submitted to DeBio blockchain as `DnaTestResult`.
Users can also submit `DnaTestResult` that are processed off chain.

The following extrinsic calls are exposed:
### Receive DNA Sample
```rust
pub fn receive_dna_sample(origin: OriginFor<T>, tracking_id: Vec<u8>) -> DispatchResultWithPostInfo
```
### Reject DNA Sample
```rust
pub fn reject_dna_sample(origin: OriginFor<T>, tracking_id: Vec<u8>) -> DispatchResultWithPostInfo
```
### Process DNA Sample 
```rust
pub fn process_dna_sample(origin: OriginFor<T>, tracking_id: Vec<u8>) -> DispatchResultWithPostInfo
```
### Submit Test Result
```rust
pub fn submit_test_result(origin: OriginFor<T>, tracking_id: Vec<u8>, is_success: bool, submission: DnaTestResultSubmission) -> DispatchResultWithPostInfo
```
### Submit Independent Test Result
```rust
pub fn submit_independent_test_result(origin: OriginFor<T>, submission: DnaTestResultSubmission) -> DispatchResultWithPostInfo
```

## User Profile Pallet
Currently this pallet only stores a user's Ethereum address that is used to make and receive payments

### Set USDT Address
```rust
pub fn set_eth_address(origin: OriginFor<T>, eth_address: EthereumAddress) -> DispatchResultWithPostInfo
```
