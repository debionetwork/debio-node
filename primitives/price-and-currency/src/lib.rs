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

use scale_info::prelude::string::String;
use sp_std::borrow::ToOwned;

impl CurrencyType {
	pub fn to_asset_id(&self) -> String {
		match self {
			CurrencyType::DBIO => "dbio".to_owned(),
			CurrencyType::USN => "usn".to_owned(),
			CurrencyType::USDT => "usdt".to_owned(),
			CurrencyType::DAI => "dai".to_owned(),
			CurrencyType::USDO => "usdo".to_owned(),
			CurrencyType::ETH => "eth".to_owned(),
		}
	}

	pub fn as_string(&self) -> String {
		match self {
			CurrencyType::DBIO => "DBIO".to_owned(),
			CurrencyType::USN => "USN".to_owned(),
			CurrencyType::USDT => "USDT".to_owned(),
			CurrencyType::DAI => "DAI".to_owned(),
			CurrencyType::USDO => "USDO".to_owned(),
			CurrencyType::ETH => "ETH".to_owned(),
		}
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
