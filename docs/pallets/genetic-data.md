## Genetic Data Pallet
This pallet handles the logic of tracking and storing the `GeneticData` on the blockchain.

One account can have multiple `GeneticData`s.

The following extrinsic calls are exposed:
### Add a new Genetic Data to Account
```rust
pub fn add_genetic_data(
    origin: OriginFor<T>,
    title: Vec<u8>,
    description: Vec<u8>,
    report_link: Vec<u8>,
) -> DispatchResultWithPostInfo
```
### Update an Genetic Data from an Account
```rust
pub fn update_genetic_data(
    origin: OriginFor<T>,
    genetic_data_id: HashOf<T>,
    title: Vec<u8>,
    description: Vec<u8>,
    report_link: Vec<u8>,
) -> DispatchResultWithPostInfo
```
### Remove an Genetic Data from an Account
```rust
pub fn remove_genetic_data(
    origin: OriginFor<T>,
    genetic_data_id: HashOf<T>,
) -> DispatchResultWithPostInfo
```