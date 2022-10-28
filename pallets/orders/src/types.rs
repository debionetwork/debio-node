use crate::*;

use frame_support::{pallet_prelude::*, traits::Currency};
use primitives_price_and_currency::{CurrencyType, Price};
use scale_info::TypeInfo;
use sp_std::vec::Vec;
use traits_genetic_testing::DnaSampleTrackingId;
use traits_order::OrderInfo;
use traits_services::types::ServiceFlow;

pub type AssetId = u32;
pub type AssetBalance = u128;

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;
pub type HashOf<T> = <T as frame_system::Config>::Hash;
pub type CurrencyOf<T> = <T as self::Config>::Currency;
pub type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;
pub type OrderOf<T> = Order<HashOf<T>, AccountIdOf<T>, BalanceOf<T>, MomentOf<T>>;
pub type OrderIdsOf<T> = Vec<HashOf<T>>;
pub type AccountKeyTypeOf<T> = AccountKeyType<AccountIdOf<T>>;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum OrderStatus {
	Unpaid,
	Paid,
	Fulfilled,
	Refunded,
	Cancelled,
	Failed,
}
impl Default for OrderStatus {
	fn default() -> Self {
		OrderStatus::Unpaid
	}
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct Order<Hash, AccountId, Balance, Moment> {
	pub id: Hash,
	pub service_id: Hash,
	pub customer_id: AccountId,
	pub customer_box_public_key: Hash,
	pub seller_id: AccountId,
	pub dna_sample_tracking_id: DnaSampleTrackingId,
	pub asset_id: Option<u32>,
	pub currency: CurrencyType,
	pub prices: Vec<Price<Balance>>,
	pub additional_prices: Vec<Price<Balance>>,
	pub total_price: Balance,
	pub status: OrderStatus,
	pub order_flow: ServiceFlow,
	pub created_at: Moment,
	pub updated_at: Moment,
}
#[allow(clippy::too_many_arguments)]
impl<Hash, AccountId, Balance, Moment> Order<Hash, AccountId, Balance, Moment>
where
	AccountId: PartialEq + Eq,
{
	pub fn new(
		id: Hash,
		service_id: Hash,
		customer_id: AccountId,
		customer_box_public_key: Hash,
		seller_id: AccountId,
		dna_sample_tracking_id: DnaSampleTrackingId,
		asset_id: Option<u32>,
		currency: CurrencyType,
		order_flow: ServiceFlow,
		prices: Vec<Price<Balance>>,
		additional_prices: Vec<Price<Balance>>,
		total_price: Balance,
		created_at: Moment,
		updated_at: Moment,
	) -> Self {
		Self {
			id,
			service_id,
			customer_id,
			customer_box_public_key,
			seller_id,
			dna_sample_tracking_id,
			asset_id,
			currency,
			prices,
			additional_prices,
			status: OrderStatus::default(),
			order_flow,
			total_price,
			created_at,
			updated_at,
		}
	}

	pub fn get_id(&self) -> &Hash {
		&self.id
	}

	pub fn get_customer_id(&self) -> &AccountId {
		&self.customer_id
	}

	pub fn get_seller_id(&self) -> &AccountId {
		&self.seller_id
	}

	pub fn get_created_at(&self) -> &Moment {
		&self.created_at
	}

	pub fn get_service_id(&self) -> &Hash {
		&self.service_id
	}

	pub fn set_asset_id(mut self, asset_id: u32) -> Self {
		self.asset_id = Some(asset_id);
		self
	}

	pub fn is_authorized_customer(self, account_id: &AccountId) -> Option<Self> {
		if &self.customer_id == account_id {
			Some(self)
		} else {
			None
		}
	}

	pub fn is_authorized_seller(self, account_id: &AccountId) -> Option<Self> {
		if &self.seller_id == account_id {
			Some(self)
		} else {
			None
		}
	}

	pub fn can_cancelled(self) -> Option<Self> {
		match self.status {
			OrderStatus::Paid => Some(self),
			OrderStatus::Unpaid => Some(self),
			_ => None,
		}
	}

	pub fn can_paid(self) -> Option<Self> {
		if self.status == OrderStatus::Unpaid {
			Some(self)
		} else {
			None
		}
	}

	pub fn can_fulfilled(self) -> Option<Self> {
		if self.status == OrderStatus::Paid {
			Some(self)
		} else {
			None
		}
	}

	pub fn can_refunded(self) -> Option<Self> {
		if self.status == OrderStatus::Paid {
			Some(self)
		} else {
			None
		}
	}
}

impl<T, Hash, AccountId, Balance, Moment> OrderInfo<T> for Order<Hash, AccountId, Balance, Moment>
where
	T: frame_system::Config<AccountId = AccountId, Hash = Hash>,
	AccountId: PartialEq,
	Hash: PartialEq,
{
	fn is_order_paid(&self) -> bool {
		self.status == OrderStatus::Paid
	}

	fn is_order_unpaid(&self) -> bool {
		self.status == OrderStatus::Unpaid
	}

	fn is_order_fullfilled(&self) -> bool {
		self.status == OrderStatus::Fulfilled
	}

	fn is_order_refunded(&self) -> bool {
		self.status == OrderStatus::Refunded
	}

	fn is_order_failed(&self) -> bool {
		self.status == OrderStatus::Failed
	}

	fn is_order_to_lab(&self, account_id: &T::AccountId) -> bool {
		&self.seller_id == account_id
	}

	fn is_account_order(&self, account_id: &T::AccountId) -> bool {
		&self.customer_id == account_id
	}

	fn is_order_from_service(&self, service_id: &T::Hash) -> bool {
		&self.service_id == service_id
	}
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum AccountKeyType<AccountId> {
	TreasuryKey(AccountId),
	EscrowKey(AccountId),
}
