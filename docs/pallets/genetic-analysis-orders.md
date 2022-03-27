## Genetic Analysis Orders Pallet
The Genetic Analysis Orders pallet controls the genetic analysis order flow. Customers who want to request an analysis from a genetic analyst will create an RPC request to order from this pallet.

The pallet also contains an escrow wallet in the form of a Pallet Id used to hold funds before sending them to the genetic analysts.

This pallet exposes the following extrinsic calls:
### Create Genetic Analysis Order
```rust
pub fn create_genetic_analysis_order(
    origin: OriginFor<T>,
    genetic_data_id: T::Hash,
    service_id: T::Hash,
    price_index: u32,
    customer_box_public_key: T::Hash,
    genetic_link: Vec<u8>,
) -> DispatchResultWithPostInfo
```
### Cancel Genetic Analysis Order
```rust
pub fn cancel_genetic_analysis_order(
    origin: OriginFor<T>,
    genetic_analysis_order_id: T::Hash,
) -> DispatchResultWithPostInfo
```
### Fulfill Submitted Genetic Analysis Order
```rust
pub fn fulfill_genetic_analysis_order(
    origin: OriginFor<T>,
    genetic_analysis_order_id: T::Hash,
) -> DispatchResultWithPostInfo
```
### Set Genetic Analysis Order to Paid (Admin Only)
```rust
pub fn set_genetic_analysis_order_paid(
    origin: OriginFor<T>,
    genetic_analysis_order_id: T::Hash,
) -> DispatchResultWithPostInfo
```
### Set Genetic Analysis Order to Refunded (Admin Only)
```rust
pub fn set_genetic_analysis_order_refunded(
    origin: OriginFor<T>,
    genetic_analysis_order_id: T::Hash,
) -> DispatchResultWithPostInfo
```
### Update Genetic Analysis Order Pallet Escrow Key/Admin Key (Admin Only)
```rust
pub fn update_escrow_key(
    origin: OriginFor<T>,
    account_id: T::AccountId,
) -> DispatchResultWithPostInfo
```
### Update Genetic Analysis Order Pallet Escrow Key/Admin Key (Sudo Only)
```rust
pub fn sudo_update_escrow_key(
    origin: OriginFor<T>,
    account_id: T::AccountId,
) -> DispatchResultWithPostInfo
```

