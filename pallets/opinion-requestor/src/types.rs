use frame_support::{
	pallet_prelude::{Decode, Encode},
	RuntimeDebug,
};
use scale_info::TypeInfo;
use sp_std::vec::Vec;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct OpinionRequestor<AccountId, Hash, Moment>
where
	AccountId: Clone + PartialEq + Eq,
	Hash: PartialEq + Eq + Clone,
	Moment: Copy,
{
	id: Hash,
	account_id: AccountId,
	info: RequestorInfo<Hash>,
	created_at: Moment,
	updated_at: Moment,
}
impl<AccountId, Hash, Moment> OpinionRequestor<AccountId, Hash, Moment>
where
	AccountId: Clone + PartialEq + Eq,
	Hash: PartialEq + Eq + Clone,
	Moment: Copy,
{
	pub fn new(
		id: &Hash,
		account_id: &AccountId,
		info: &RequestorInfo<Hash>,
		created_at: Moment,
	) -> Self {
		Self {
			id: id.clone(),
			account_id: account_id.clone(),
			info: info.clone(),
			created_at,
			updated_at: created_at,
		}
	}

	// GET Method
	pub fn is_authorized_owner(self, account_id: &AccountId) -> Option<Self> {
		if &self.account_id != account_id {
			return None
		}

		Some(self)
	}

	pub fn account_id(&self) -> &AccountId {
		&self.account_id
	}

	pub fn info(&self) -> &RequestorInfo<Hash> {
		&self.info
	}

	pub fn opinion_ids(&self) -> &[Hash] {
		&self.info.opinion_ids
	}

	// SET Method
	pub fn add_opinion(&mut self, opinion_id: Hash) {
		self.info.add_opinion(opinion_id);
	}

	pub fn remove_opinion(&mut self, opinion_id: Hash) {
		self.info.remove_opinion(opinion_id);
	}

	pub fn update_info(&mut self, info: RequestorInfo<Hash>, date: Moment) {
		self.info = info;
		self.updated_at = date;
	}
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct RequestorInfo<Hash>
where
	Hash: PartialEq + Eq + Clone,
{
	category: Vec<u8>,
	description: Vec<u8>,
	genetic_data_ids: Vec<Hash>,
	opinion_ids: Vec<Hash>,
	myriad_url: Vec<u8>,
}
impl<Hash> RequestorInfo<Hash>
where
	Hash: PartialEq + Eq + Clone,
{
	pub fn new(
		category: &[u8],
		description: &[u8],
		genetic_data_ids: &[Hash],
		opinion_ids: &[Hash],
		myriad_url: &[u8],
	) -> Self {
		Self {
			category: category.to_vec(),
			description: description.to_vec(),
			genetic_data_ids: genetic_data_ids.to_vec(),
			opinion_ids: opinion_ids.to_vec(),
			myriad_url: myriad_url.to_vec(),
		}
	}

	// GET Method
	pub fn opinion_ids(&self) -> &[Hash] {
		&self.opinion_ids
	}

	pub fn genetic_data_ids(&self) -> &[Hash] {
		&self.genetic_data_ids
	}

	// SET Method
	pub fn add_opinion(&mut self, opinion_id: Hash) {
		self.opinion_ids.push(opinion_id);
	}

	pub fn remove_opinion(&mut self, opinion_id: Hash) {
		let position = &self.opinion_ids.iter().position(|x| *x == opinion_id);

		if let Some(index) = position {
			self.opinion_ids.remove(*index);
		}
	}

	pub fn update_opinion_ids(&mut self, opinion_ids: &[Hash]) {
		self.opinion_ids = opinion_ids.to_vec();
	}

	pub fn update_genetic_data_ids(&mut self, genetic_data_ids: &[Hash]) {
		self.genetic_data_ids = genetic_data_ids.to_vec();
	}
}
