use crate::*;

use frame_support::{pallet_prelude::*, traits::Currency};
use primitives_price_and_currency::{CurrencyType, Price};
use scale_info::TypeInfo;
use sp_std::vec::Vec;

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;
pub type HashOf<T> = <T as frame_system::Config>::Hash;
pub type CurrencyOf<T> = <T as self::Config>::Currency;
pub type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;
pub type OrderOf<T> = Order<HashOf<T>, AccountIdOf<T>, BalanceOf<T>, MomentOf<T>>;
pub type OrderIdsOf<T> = Vec<HashOf<T>>;

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
	pub currency: CurrencyType,
	pub prices: Vec<Price<Balance>>,
	pub additional_prices: Vec<Price<Balance>>,
	pub status: OrderStatus,
	pub order_flow: ServiceFlow,
	pub created_at: Moment,
	pub updated_at: Moment,
}
#[allow(clippy::too_many_arguments)]
impl<Hash, AccountId, Balance, Moment> Order<Hash, AccountId, Balance, Moment> {
	pub fn new(
		id: Hash,
		service_id: Hash,
		customer_id: AccountId,
		customer_box_public_key: Hash,
		seller_id: AccountId,
		dna_sample_tracking_id: DnaSampleTrackingId,
		currency: CurrencyType,
		order_flow: ServiceFlow,
		prices: Vec<Price<Balance>>,
		additional_prices: Vec<Price<Balance>>,
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
			currency,
			prices,
			additional_prices,
			status: OrderStatus::default(),
			order_flow,
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
}
