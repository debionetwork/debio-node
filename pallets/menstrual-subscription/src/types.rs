use frame_support::pallet_prelude::*;
use primitives_duration::MenstrualSubscriptionDuration;
use primitives_menstrual_status::{MenstrualSubscriptionStatus, PaymentStatus};
use primitives_price_and_currency::CurrencyType;
use scale_info::TypeInfo;
use traits_menstrual_subscription::MenstrualSubscription as MenstrualSubscriptionT;

pub type AssetId = u32;
pub type AssetBalance = u128;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct MenstrualSubscriptionPrice<Balance> {
	pub duration: MenstrualSubscriptionDuration,
	pub currency: CurrencyType,
	pub asset_id: Option<AssetId>,
	pub amount: Balance,
}
impl<Balance> MenstrualSubscriptionPrice<Balance> {
	pub fn new(
		duration: &MenstrualSubscriptionDuration,
		currency: &CurrencyType,
		asset_id: Option<AssetId>,
		amount: Balance,
	) -> Self {
		Self { duration: duration.clone(), currency: currency.clone(), asset_id, amount }
	}
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum AccountKeyType<AccountId> {
	TreasuryKey(AccountId),
	AdminKey(AccountId),
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct MenstrualSubscription<AccountId, Hash, Moment> {
	pub id: Hash,
	pub address_id: AccountId,
	pub duration: MenstrualSubscriptionDuration,
	pub currency: CurrencyType,
	pub payment_status: PaymentStatus,
	pub status: MenstrualSubscriptionStatus,
	pub created_at: Moment,
	pub updated_at: Moment,
}
impl<AccountId, Hash, Moment: Default> MenstrualSubscription<AccountId, Hash, Moment> {
	pub fn new(
		id: Hash,
		address_id: AccountId,
		duration: MenstrualSubscriptionDuration,
		currency: CurrencyType,
		created_at: Moment,
	) -> Self {
		Self {
			id,
			address_id,
			duration,
			currency,
			payment_status: PaymentStatus::default(),
			status: MenstrualSubscriptionStatus::InQueue,
			created_at,
			updated_at: Moment::default(),
		}
	}

	pub fn get_id(&self) -> &Hash {
		&self.id
	}

	pub fn get_address_id(&self) -> &AccountId {
		&self.address_id
	}
}

impl<T, AccountId, Hash, Moment: Default> MenstrualSubscriptionT<T>
	for MenstrualSubscription<AccountId, Hash, Moment>
where
	T: frame_system::Config<AccountId = AccountId, Hash = Hash>,
{
	fn get_id(&self) -> &Hash {
		self.get_id()
	}
	fn get_address_id(&self) -> &AccountId {
		self.get_address_id()
	}
}
