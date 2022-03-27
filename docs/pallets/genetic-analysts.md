## Genetic Analysts Pallet
The Genetic Analysts pallet handles the logic for registration, deregistration, and updating information of `GeneticAnalyst` accounts.

This pallet exposes the following extrinsic calls:
### Register Genetic Analyst
```rust
pub fn register_genetic_analyst(
    origin: OriginFor<T>,
    genetic_analyst_info: GeneticAnalystInfo<HashOf<T>, MomentOf<T>>,
) -> DispatchResultWithPostInfo
```
### Update Genetic Analyst
```rust
pub fn update_genetic_analyst(
    origin: OriginFor<T>,
    genetic_analyst_info: GeneticAnalystInfo<HashOf<T>, MomentOf<T>>,
) -> DispatchResultWithPostInfo
```
### Update Genetic Analyst Verification Status (Admin Only)
```rust
pub fn update_genetic_analyst_verification_status(
    origin: OriginFor<T>,
    account_id: T::AccountId,
    status: VerificationStatus,
) -> DispatchResultWithPostInfo
```
### Update Genetic Analyst Availability Status (Admin Only)
```rust
pub fn update_genetic_analyst_availability_status(
    origin: OriginFor<T>,
    status: AvailabilityStatus,
) -> DispatchResultWithPostInfo
```
### Deregister Genetic Analyst
```rust
pub fn deregister_genetic_analyst(
    origin: OriginFor<T>
) -> DispatchResultWithPostInfo
```
### Update Genetic Analysts Administrator (Admin Only)
```rust
pub fn update_admin_key(
    origin: OriginFor<T>,
    account_id: T::AccountId,
) -> DispatchResultWithPostInfo
```
### Update Genetic Analysts Administrator (Sudo Only)
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