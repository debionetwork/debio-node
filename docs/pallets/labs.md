## Labs Pallet
The Labs pallet handles the logic for registration, deregistration, and updating information of `Lab` accounts.

This pallet exposes the following extrinsic calls:
### Register Lab
```rust
pub fn register_lab(
    origin: OriginFor<T>,
    lab_info: LabInfo<HashOf<T>>,
) -> DispatchResultWithPostInfo
```
### Update Lab
```rust
pub fn update_lab(
    origin: OriginFor<T>,
    lab_info: LabInfo<HashOf<T>>,
) -> DispatchResultWithPostInfo
```
### Update Lab Verification Status (Admin Only)
```rust
pub fn update_lab_verification_status(
    origin: OriginFor<T>,
    account_id: T::AccountId,
    lab_verification_status: LabVerificationStatus,
) -> DispatchResultWithPostInfo
```
### Deregister Lab
```rust
pub fn deregister_lab(
    origin: OriginFor<T>
) -> DispatchResultWithPostInfo
```
### Update Labs Administrator (Admin Only)
```rust
pub fn update_admin_key(
    origin: OriginFor<T>,
    account_id: T::AccountId,
) -> DispatchResultWithPostInfo
```
### Update Labs Administrator (Sudo Only)
```rust
pub fn sudo_update_admin_key(
    origin: OriginFor<T>,
    account_id: T::AccountId,
) -> DispatchResultWithPostInfo
```
### Stake Genetic Analysts
```rust
pub fn stake_genetic_analyst(
    origin: OriginFor<T>
) -> DispatchResultWithPostInfo
```
### Unstake Genetic Analysts
```rust
pub fn unstake_genetic_analyst(
    origin: OriginFor<T>
) -> DispatchResultWithPostInfo
```
### Retrieve Genetic Analysts Unstake Amount
```rust
pub fn retrieve_unstake_amount(
    origin: OriginFor<T>,
    account_id: T::AccountId,
) -> DispatchResultWithPostInfo
```
### Update Minimum Stake Amount (Admin Only)
```rust
pub fn update_minimum_stake_amount(
    origin: OriginFor<T>,
    amount: BalanceOf<T>,
) -> DispatchResultWithPostInfo
```
### Update Unstake Duration (Admin Only)
```rust
pub fn update_unstake_time(
    origin: OriginFor<T>,
    amount: MomentOf<T>,
) -> DispatchResultWithPostInfo
```