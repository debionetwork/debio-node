## User Profile Pallet
Currently this pallet only stores a user's Ethereum address that is used to make and receive payments

### Set Ethereum Address
```rust
pub fn set_eth_address(
    origin: OriginFor<T>,
    eth_address: EthereumAddressOf<T>,
) -> DispatchResultWithPostInfo
```
### Admin Set Ethereum Address (Admin Only)
```rust
pub fn admin_set_eth_address(
    origin: OriginFor<T>,
    account_id: AccountIdOf<T>,
    eth_address: EthereumAddressOf<T>,
) -> DispatchResultWithPostInfo
```
### Update Administrator (Admin Only)
```rust
pub fn update_admin_key(
    origin: OriginFor<T>,
    account_id: T::AccountId,
) -> DispatchResultWithPostInfo
```
### Update Administrator (Sudo Only)
```rust
pub fn sudo_update_admin_key(
    origin: OriginFor<T>,
    account_id: T::AccountId,
) -> DispatchResultWithPostInfo
```