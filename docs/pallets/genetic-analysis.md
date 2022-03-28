## Genetic Analysis Pallet
The Genetic Analysis pallet handles the logic for processing, rejecting, and submitting genetic analysis results created by the Genetic Analysis Orders pallet.

This pallet exposes the following extrinsic calls:
### Process Genetic Analysis Sample
```rust
pub fn process_genetic_analysis(
    origin: OriginFor<T>,
    genetic_analysis_tracking_id: TrackingId,
    status: GeneticAnalysisStatus,
) -> DispatchResultWithPostInfo
```
### Reject Genetic Analysis Sample
```rust
pub fn reject_genetic_analysis(
    origin: OriginFor<T>,
    genetic_analysis_tracking_id: TrackingId,
    rejected_title: Vec<u8>,
    rejected_description: Vec<u8>,
) -> DispatchResultWithPostInfo
```
### Submit Genetic Analysis Sample Report
```rust
pub fn submit_genetic_analysis(
    origin: OriginFor<T>,
    genetic_analysis_tracking_id: TrackingId,
    report_link: Vec<u8>,
    comment: Option<Vec<u8>>,
) -> DispatchResultWithPostInfo
```
