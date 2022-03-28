## Genetic Testing Pallet
This pallet handles the logic of tracking `DnaSample` and storing `DnaTestResult` on the blockchain.

`DnaSample`s are sent by customers to a `Lab` to be processed. The result is then submitted to DeBio blockchain as `DnaTestResult`.

Users can also submit `DnaTestResult` that are processed off chain.

The following extrinsic calls are exposed:
### Reject DNA Sample
```rust
pub fn reject_dna_sample(
    origin: OriginFor<T>,
    tracking_id: DnaSampleTrackingId,
    rejected_title: Vec<u8>,
    rejected_description: Vec<u8>,
) -> DispatchResultWithPostInfo
```
### Process DNA Sample
```rust
pub fn process_dna_sample(
    origin: OriginFor<T>,
    tracking_id: DnaSampleTrackingId,
    status: DnaSampleStatus,
) -> DispatchResultWithPostInfo
```
### Submit Test Result
```rust
pub fn submit_test_result(
    origin: OriginFor<T>,
    tracking_id: DnaSampleTrackingId,
    submission: DnaTestResultSubmission,
) -> DispatchResultWithPostInfo
```
### Submit Independent Test Result
```rust
pub fn submit_independent_test_result(
    origin: OriginFor<T>,
    submission: DnaTestResultSubmission,
) -> DispatchResultWithPostInfo
```
### Submit Data Bounty Details
```rust
pub fn submit_data_bounty_details(
    origin: OriginFor<T>,
    data_hash: T::Hash,
    order_id: T::Hash,
) -> DispatchResultWithPostInfo
```