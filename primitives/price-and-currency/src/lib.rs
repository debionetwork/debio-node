use frame_support::codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use sp_std::prelude::*;
use scale_info::TypeInfo;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum CurrencyType {
    DAI,
    ETH,
}
impl Default for CurrencyType {
    fn default() -> Self {
        CurrencyType::DAI
    }
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct Price<Balance> {
    pub component: Vec<u8>,
    pub value: Balance,
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct PriceByCurrency<Balance> {
    pub currency: CurrencyType,
    pub total_price: Balance,
    pub price_components: Vec<Price<Balance>>,
    pub additional_prices: Vec<Price<Balance>>,
}