use frame_support::codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use sp_std::prelude::*;
use scale_info::TypeInfo;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum DurationType {
    WorkingDays,
    Hours,
    Days,
}
impl Default for DurationType {
    fn default() -> Self {
        DurationType::WorkingDays
    }
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct ExpectedDuration {
    pub duration: i8,
    pub duration_type: DurationType,
}