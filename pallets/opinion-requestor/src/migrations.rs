use crate::*;
use frame_support::{
	pallet_prelude::{Decode, Encode},
	traits::Get,
	weights::Weight,
};
use sp_std::vec::Vec;

pub fn migrate<T: Config>() -> Weight {
	let mut weight: Weight = 0;
	let mut version = StorageVersion::get::<Pallet<T>>();

	if version < 1 {
		weight = weight.saturating_add(version::v1::migrate::<T>());
		version = StorageVersion::new(1);
	}

	version.put::<Pallet<T>>();
	weight
}

mod version {
	use super::*;

	pub mod v1 {
		use super::*;

		pub fn migrate<T: Config>() -> Weight {
			let mut weight = T::DbWeight::get().writes(1);

			#[derive(Encode, Decode)]
			pub struct OldRequestorInfo<Hash> {
				category: Vec<u8>,
				description: Vec<u8>,
				electronical_medical_record_ids: Vec<Hash>,
				opinion_ids: Vec<Hash>,
				myriad_url: Vec<u8>,
			}

			#[derive(Encode, Decode)]
			pub struct OldOpinionRequestor<AccountId, Hash, Moment> {
				id: Hash,
				account_id: AccountId,
				info: OldRequestorInfo<Hash>,
				created_at: Moment,
				updated_at: Moment,
			}

			pub type OldOpinionRequestorOf<T> =
				OldOpinionRequestor<AccountIdOf<T>, HashOf<T>, MomentOf<T>>;

			OpinionRequestors::<T>::translate(|_key, old: OldOpinionRequestorOf<T>| {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

				let old_requestor_info = old.info;
				let new_requester_info = RequestorInfo::new(
					&old_requestor_info.category,
					&old_requestor_info.description,
					&old_requestor_info.electronical_medical_record_ids,
					&old_requestor_info.opinion_ids,
					&old_requestor_info.myriad_url,
				);

				let mut new_opinion_requestor = OpinionRequestor::new(
					&old.id,
					&old.account_id,
					&new_requester_info,
					old.created_at,
				);

				new_opinion_requestor.update_info(new_requester_info, old.updated_at);

				Some(new_opinion_requestor)
			});

			weight
		}
	}
}
