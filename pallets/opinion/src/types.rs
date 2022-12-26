use frame_support::{
	pallet_prelude::{Decode, Encode},
	RuntimeDebug,
};
use primitives_price_and_currency::CurrencyType;
use scale_info::TypeInfo;
use sp_std::vec::Vec;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct Opinion<AccountId, Hash, Moment>
where
	AccountId: Clone + PartialEq + Eq,
	Hash: PartialEq + Eq + Clone,
	Moment: Copy,
{
	id: Hash,
	requestor_id: Hash,
	professional_id: AccountId,
	info: OpinionInfo,
	status: Status,
	created_at: Moment,
}
impl<AccountId, Hash, Moment> Opinion<AccountId, Hash, Moment>
where
	AccountId: Clone + PartialEq + Eq,
	Hash: PartialEq + Eq + Clone,
	Moment: Copy,
{
	pub fn new(
		id: &Hash,
		requestor_id: &Hash,
		professional_id: &AccountId,
		info: &OpinionInfo,
		created_at: Moment,
	) -> Self {
		Self {
			id: id.clone(),
			requestor_id: requestor_id.clone(),
			professional_id: professional_id.clone(),
			info: info.clone(),
			status: Status::default(),
			created_at,
		}
	}

	pub fn is_authorized_owner(self, account_id: &AccountId) -> Option<Self> {
		if &self.professional_id != account_id {
			return None
		}

		Some(self)
	}

	pub fn requestor_id(&self) -> &Hash {
		&self.requestor_id
	}

	pub fn professional_id(&self) -> &AccountId {
		&self.professional_id
	}

	pub fn update_info(&mut self, info: &OpinionInfo) {
		self.info = info.clone();
	}

	pub fn update_asset_id(&mut self, asset_id: Option<u32>) {
		self.info.asset_id = asset_id;
	}
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct OpinionInfo {
	description: Vec<u8>,
	myriad_url: Vec<u8>,
	asset_id: Option<u32>,
	currency: CurrencyType,
	amount: u128,
}
impl OpinionInfo {
	pub fn new(
		description: Vec<u8>,
		myriad_url: Vec<u8>,
		asset_id: Option<u32>,
		currency: CurrencyType,
		amount: u128,
	) -> Self {
		Self { description, myriad_url, asset_id, currency, amount }
	}

	pub fn asset_id(&self) -> &Option<u32> {
		&self.asset_id
	}

	pub fn currency(&self) -> &CurrencyType {
		&self.currency
	}
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum Status {
	Unpaid,
	Paid,
}
impl Default for Status {
	fn default() -> Self {
		Status::Unpaid
	}
}
