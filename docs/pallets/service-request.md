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
### Unstake Request
```rust
pub fn unstake(
    origin: OriginFor<T>, 
    request_id: HashOf<T>
) -> DispatchResultWithPostInfo
```
### Retrieve Unstaked Amount
```rust
pub fn retrieve_unstaked_amount(
    origin: OriginFor<T>,
    request_id: HashOf<T>,
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
### Update Administrator (Admin Only)
```rust
pub fn update_admin_key(
    origin: OriginFor<T>,
    account_id: T::AccountId,
) -> DispatchResultWithPostInfo
```