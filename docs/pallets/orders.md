
## Orders Pallet
This pallet handles the business logic of orders. An `Order` is associated with a `Service` and a `DnaSample`.

An `Order` is fulfilled if the associated `DnaSample` is successfully processed.
An `Order` payment can be refunded if the physical `DnaSample` is rejected upon receipt, it has not been processed for 7 days, or if the sample processing has failed.

Currently DeBio uses the ethereum network for payments in DAI. We use an escrow bridge that listens for payments to a smart contract deployed on the ethereum network and triggers an extrinsic call in DeBio that updates the status of the `Order` to paid.

Before a user can create an order, he/she is required to set their Ethereum address using the `set_eth_address` extrinsic in the `UserProfile` pallet.

This pallet exposes the following extrinsics:
### Create Order
```rust
pub fn create_order(
    origin: OriginFor<T>,
    service_id: T::Hash,
    price_index: u32,
    customer_box_public_key: T::Hash,
    order_flow: ServiceFlow,
) -> DispatchResultWithPostInfo
```
### Cancel Order
```rust
pub fn cancel_order(
    origin: OriginFor<T>, 
    order_id: T::Hash
) -> DispatchResultWithPostInfo
```
### Fulfill Order
```rust
pub fn fulfill_order(
    origin: OriginFor<T>,
    order_id: T::Hash,
) -> DispatchResultWithPostInfo
```
### Set Order Paid (Admin Only)
```rust
pub fn set_order_paid(
    origin: OriginFor<T>,
    order_id: T::Hash,
) -> DispatchResultWithPostInfo
```
### Set Order is Refunded (Admin Only)
```rust
pub fn set_order_refunded(
    origin: OriginFor<T>,
    order_id: T::Hash,
) -> DispatchResultWithPostInfo
```
### Update Escrow Key for Admin Transactions (Admin Only)
```rust
pub fn update_escrow_key(
    origin: OriginFor<T>,
    account_id: T::AccountId,
) -> DispatchResultWithPostInfo
```
### Update Escrow Key for Admin Transactions (Sudo Only)
```rust
pub fn sudo_update_escrow_key(
    origin: OriginFor<T>,
    account_id: T::AccountId,
) -> DispatchResultWithPostInfo
```