#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	codec::{Decode, Encode},
	RuntimeDebug,
};
use scale_info::TypeInfo;
use sp_std::vec::Vec;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum CurrencyType {
	DBIO,
	USN,
	USDT,
	DAI,
	USDO,
	ETH,
}
impl Default for CurrencyType {
	fn default() -> Self {
		CurrencyType::DBIO
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
