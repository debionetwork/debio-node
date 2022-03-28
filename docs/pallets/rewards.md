## Rewards Pallet
The rewards pallet is only used to send rewards using the designated accounts from the rewards escrow in the pallet.

### Send Rewards using Authorized Account (Admin Only)
```rust
pub fn reward_funds(
    origin: OriginFor<T>,
    to_reward: T::AccountId,
    reward: BalanceOf<T>,
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