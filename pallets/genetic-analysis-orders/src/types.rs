use crate::*;
use scale_info::TypeInfo;

// Asset ID and Balance types
pub type AssetId = u32;
pub type AssetBalance = u128;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum GeneticAnalysisOrderStatus {
	Unpaid,
	Paid,
	Fulfilled,
	Refunded,
	Cancelled,
	Failed,
}
impl Default for GeneticAnalysisOrderStatus {
	fn default() -> Self {
		GeneticAnalysisOrderStatus::Unpaid
	}
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct GeneticAnalysisOrder<Hash, AccountId, Balance, Moment> {
	pub id: Hash,
	pub service_id: Hash,
	pub customer_id: AccountId,
	pub customer_box_public_key: Hash,
	pub seller_id: AccountId,
	pub genetic_data_id: Hash,
	pub genetic_analysis_tracking_id: TrackingId,
	pub asset_id: Option<u32>,
	pub currency: CurrencyType,
	pub prices: Vec<Price<Balance>>,
	pub additional_prices: Vec<Price<Balance>>,
	pub total_price: Balance,
	pub status: GeneticAnalysisOrderStatus,
	pub created_at: Moment,
	pub updated_at: Moment,
	pub genetic_link: Vec<u8>,
}
#[allow(clippy::too_many_arguments)]
impl<Hash, AccountId, Balance, Moment: Default>
	GeneticAnalysisOrder<Hash, AccountId, Balance, Moment>
where
	AccountId: PartialEq + Eq,
{
	pub fn new(
		id: Hash,
		service_id: Hash,
		customer_id: AccountId,
		customer_box_public_key: Hash,
		seller_id: AccountId,
		genetic_data_id: Hash,
		genetic_analysis_tracking_id: TrackingId,
		genetic_link: Vec<u8>,
		asset_id: Option<u32>,
		currency: CurrencyType,
		prices: Vec<Price<Balance>>,
		additional_prices: Vec<Price<Balance>>,
		total_price: Balance,
		created_at: Moment,
	) -> Self {
		Self {
			id,
			service_id,
			customer_id,
			customer_box_public_key,
			seller_id,
			genetic_data_id,
			genetic_analysis_tracking_id,
			genetic_link,
			asset_id,
			currency,
			prices,
			additional_prices,
			status: GeneticAnalysisOrderStatus::default(),
			total_price,
			created_at,
			updated_at: Moment::default(),
		}
	}

	pub fn get_id(&self) -> &Hash {
		&self.id
	}

	pub fn get_created_at(&self) -> &Moment {
		&self.created_at
	}

	pub fn get_service_id(&self) -> &Hash {
		&self.service_id
	}

	pub fn is_authorized_customer(self, account_id: &AccountId) -> Option<Self> {
		if &self.customer_id == account_id {
			Some(self)
		} else {
			None
		}
	}

	pub fn can_cancelled(self) -> Option<Self> {
		match self.status {
			GeneticAnalysisOrderStatus::Paid => Some(self),
			GeneticAnalysisOrderStatus::Unpaid => Some(self),
			_ => None,
		}
	}

	pub fn can_paid(self) -> Option<Self> {
		if self.status == GeneticAnalysisOrderStatus::Unpaid {
			Some(self)
		} else {
			None
		}
	}

	pub fn can_fulfilled(self) -> Option<Self> {
		if self.status == GeneticAnalysisOrderStatus::Paid {
			Some(self)
		} else {
			None
		}
	}

	pub fn can_refunded(self) -> Option<Self> {
		if self.status == GeneticAnalysisOrderStatus::Paid ||
			self.status == GeneticAnalysisOrderStatus::Failed
		{
			Some(self)
		} else {
			None
		}
	}
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum AccountKeyType<AccountId> {
	TreasuryKey(AccountId),
	EscrowKey(AccountId),
}
