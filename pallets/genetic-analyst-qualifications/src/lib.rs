#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	codec::{Decode, Encode},
	pallet_prelude::*,
};
pub use pallet::*;
pub use scale_info::TypeInfo;
use traits_genetic_analyst_qualifications::{
	GeneticAnalystQualificationInfo as GeneticAnalystQualificationInfoT,
	GeneticAnalystQualificationOwner, GeneticAnalystQualificationsProvider,
};

pub mod interface;
pub mod weights;
pub use interface::GeneticAnalystQualificationInterface;
use sp_std::prelude::*;

/// GeneticAnalystExperience struct
/// Information that is mutable by user
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct GeneticAnalystExperience {
	pub title: Vec<u8>,
}

/// GeneticAnalystCertification struct
/// Information that is mutable by user
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct GeneticAnalystCertification {
	pub title: Vec<u8>,
	pub issuer: Vec<u8>,
	pub month: Vec<u8>,
	pub year: Vec<u8>,
	pub description: Vec<u8>,
	pub supporting_document: Option<Vec<u8>>,
}

/// GeneticAnalystQualificationInfo struct
/// Information that is mutable by user
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct GeneticAnalystQualificationInfo {
	pub experience: Vec<GeneticAnalystExperience>,
	pub certification: Option<Vec<GeneticAnalystCertification>>,
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct GeneticAnalystQualification<AccountId, Hash> {
	pub id: Hash,
	pub owner_id: AccountId,
	pub info: GeneticAnalystQualificationInfo,
}
impl GeneticAnalystQualificationInfo {
	pub fn does_experience_exist(&self) -> bool {
		!self.experience.is_empty()
	}
}
impl<AccountId, Hash> GeneticAnalystQualification<AccountId, Hash> {
	pub fn new(id: Hash, owner_id: AccountId, info: GeneticAnalystQualificationInfo) -> Self {
		Self { id, owner_id, info }
	}

	pub fn get_id(&self) -> &Hash {
		&self.id
	}

	pub fn get_owner_id(&self) -> &AccountId {
		&self.owner_id
	}
}

impl<T, AccountId, Hash> GeneticAnalystQualificationInfoT<T>
	for GeneticAnalystQualification<AccountId, Hash>
where
	T: frame_system::Config<AccountId = AccountId, Hash = Hash>,
{
	fn get_id(&self) -> &Hash {
		self.get_id()
	}
	fn get_owner_id(&self) -> &AccountId {
		self.get_owner_id()
	}
}

#[frame_support::pallet]
pub mod pallet {
	use crate::{
		interface::GeneticAnalystQualificationInterface, weights::WeightInfo,
		GeneticAnalystQualification, GeneticAnalystQualificationInfo,
		GeneticAnalystQualificationOwner,
	};
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	pub use sp_std::prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type GeneticAnalystQualificationOwner: GeneticAnalystQualificationOwner<Self>;
		type WeightInfo: WeightInfo;
	}

	// ----- This is template code, every pallet needs this ---
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
	// --------------------------------------------------------

	// ----- Types -------
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type HashOf<T> = <T as frame_system::Config>::Hash;
	pub type GeneticAnalystQualificationOf<T> =
		GeneticAnalystQualification<AccountIdOf<T>, HashOf<T>>;
	pub type GeneticAnalystQualificationInfoOf = GeneticAnalystQualificationInfo;
	pub type GeneticAnalystQualificationIdOf<T> = HashOf<T>;

	// ------- Storage -------------
	#[pallet::storage]
	#[pallet::getter(fn qualification_by_id)]
	pub type GeneticAnalystQualifications<T> =
		StorageMap<_, Blake2_128Concat, HashOf<T>, GeneticAnalystQualificationOf<T>>;
	//                                _,  Hasher         ,  Key     ,  Value

	#[pallet::storage]
	#[pallet::getter(fn qualifications_count)]
	pub type GeneticAnalystQualificationsCount<T> = StorageValue<_, u64>;

	#[pallet::storage]
	#[pallet::getter(fn qualification_count_by_owner)]
	pub type GeneticAnalystQualificationsCountByOwner<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, u64>;
	// -----------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters, [GeneticAnalystQualification, who]
		GeneticAnalystQualificationCreated(GeneticAnalystQualificationOf<T>, AccountIdOf<T>),
		//// GeneticAnalystQualification updated
		/// parameters, [GeneticAnalystQualification, who]
		GeneticAnalystQualificationUpdated(GeneticAnalystQualificationOf<T>, AccountIdOf<T>),
		//// GeneticAnalystQualification deleted
		/// parameters, [GeneticAnalystQualification, who]
		GeneticAnalystQualificationDeleted(GeneticAnalystQualificationOf<T>, AccountIdOf<T>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// User not allowed to create qualification
		NotAllowedToCreate,
		/// User is not the owner of a qualification
		NotGeneticAnalystQualificationOwner,
		/// Ordering a qualification that does not exist
		GeneticAnalystQualificationDoesNotExist,
		/// Creating a qualification without experience
		GeneticAnalystExperienceCannotBeEmpty,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::create_qualification())]
		pub fn create_qualification(
			origin: OriginFor<T>,
			qualification_info: GeneticAnalystQualificationInfoOf,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as GeneticAnalystQualificationInterface<T>>::create_qualification(
				&who,
				&qualification_info,
			) {
				Ok(qualification) => {
					Self::deposit_event(Event::GeneticAnalystQualificationCreated(
						qualification,
						who.clone(),
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::update_qualification())]
		pub fn update_qualification(
			origin: OriginFor<T>,
			qualification_id: HashOf<T>,
			qualification_info: GeneticAnalystQualificationInfoOf,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			match <Self as GeneticAnalystQualificationInterface<T>>::update_qualification(
				&who,
				&qualification_id,
				&qualification_info,
			) {
				Ok(qualification) => {
					Self::deposit_event(Event::GeneticAnalystQualificationUpdated(
						qualification,
						who.clone(),
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::delete_qualification())]
		pub fn delete_qualification(
			origin: OriginFor<T>,
			qualification_id: T::Hash,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			match <Self as GeneticAnalystQualificationInterface<T>>::delete_qualification(
				&who,
				&qualification_id,
			) {
				Ok(qualification) => {
					Self::deposit_event(Event::GeneticAnalystQualificationDeleted(
						qualification,
						who.clone(),
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}
	}
}

use frame_support::sp_runtime::traits::Hash;
use traits_genetic_analyst_qualifications::GeneticAnalystQualificationOwnerInfo;

/// GeneticAnalystQualification Interface Implementation
impl<T: Config> GeneticAnalystQualificationInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type GeneticAnalystQualificationId = T::Hash;
	type GeneticAnalystQualification = GeneticAnalystQualificationOf<T>;
	type GeneticAnalystQualificationInfo = GeneticAnalystQualificationInfoOf;

	fn generate_qualification_id(
		owner_id: &T::AccountId,
		qualification_count: u64,
	) -> Self::GeneticAnalystQualificationId {
		let mut account_id_bytes = owner_id.encode();
		let mut qualification_count_bytes = qualification_count.encode();
		account_id_bytes.append(&mut qualification_count_bytes);

		let seed = &account_id_bytes;
		T::Hashing::hash(seed)
	}

	/// Create GeneticAnalystQualification
	/// Add reference to GeneticAnalystQualificationsByCountryCity storage
	/// Associate qualification reference to the owner (creator)
	/// Increment Counts
	fn create_qualification(
		owner_id: &T::AccountId,
		qualification_info: &Self::GeneticAnalystQualificationInfo,
	) -> Result<Self::GeneticAnalystQualification, Self::Error> {
		// Check if user can create_qualification
		let can_create_qualification =
			T::GeneticAnalystQualificationOwner::can_create_qualification(owner_id);
		if !can_create_qualification {
			return Err(Error::<T>::NotAllowedToCreate)
		}
		if !qualification_info.does_experience_exist() {
			return Err(Error::<T>::GeneticAnalystExperienceCannotBeEmpty)
		}

		let owner_qualification_count =
			<Self as GeneticAnalystQualificationInterface<T>>::qualification_count_by_owner(
				owner_id,
			);
		let qualification_id = Self::generate_qualification_id(owner_id, owner_qualification_count);

		let qualification = GeneticAnalystQualification::new(
			qualification_id,
			owner_id.clone(),
			qualification_info.clone(),
		);
		// Store to GeneticAnalystQualifications storage
		GeneticAnalystQualifications::<T>::insert(&qualification_id, &qualification);

		// Increment GeneticAnalystQualifications Count
		Self::add_qualifications_count();
		// Increment GeneticAnalystQualificationsCountByOwner
		Self::add_qualification_count_by_owner(&qualification.owner_id);

		// Associate created qualification to the owner
		T::GeneticAnalystQualificationOwner::associate(owner_id, &qualification_id);

		Ok(qualification)
	}

	/// Update GeneticAnalystQualification information
	fn update_qualification(
		owner_id: &T::AccountId,
		qualification_id: &Self::GeneticAnalystQualificationId,
		qualification_info: &Self::GeneticAnalystQualificationInfo,
	) -> Result<Self::GeneticAnalystQualification, Self::Error> {
		let qualification = GeneticAnalystQualifications::<T>::get(qualification_id);
		if qualification == None {
			return Err(Error::<T>::GeneticAnalystQualificationDoesNotExist)
		}
		let mut qualification = qualification.unwrap();

		if qualification.owner_id != owner_id.clone() {
			return Err(Error::<T>::NotGeneticAnalystQualificationOwner)
		}

		if !qualification_info.does_experience_exist() {
			return Err(Error::<T>::GeneticAnalystExperienceCannotBeEmpty)
		}

		qualification.info = qualification_info.clone();
		GeneticAnalystQualifications::<T>::insert(qualification_id, &qualification);

		Ok(qualification)
	}

	/// Delete GeneticAnalystQualification
	/// Delete from GeneticAnalystQualifications Storage
	/// Remove the qualification id reference in GeneticAnalystQualificationsByCountryCity storage
	/// Disassociate qualification id from the owner
	/// Decrement Counts
	fn delete_qualification(
		owner_id: &T::AccountId,
		qualification_id: &Self::GeneticAnalystQualificationId,
	) -> Result<Self::GeneticAnalystQualification, Self::Error> {
		let qualification = GeneticAnalystQualifications::<T>::get(qualification_id);
		if qualification == None {
			return Err(Error::<T>::GeneticAnalystQualificationDoesNotExist)
		}
		let qualification = qualification.unwrap();

		if qualification.owner_id != owner_id.clone() {
			return Err(Error::<T>::NotGeneticAnalystQualificationOwner)
		}
		// Remove qualification from storage
		let qualification = GeneticAnalystQualifications::<T>::take(qualification_id).unwrap();

		let owner = T::GeneticAnalystQualificationOwner::get_owner(owner_id).unwrap();
		// disassociate qualification reference from the owner
		T::GeneticAnalystQualificationOwner::disassociate(owner.get_owner_id(), &qualification.id);
		// Decrement counts
		Self::sub_qualifications_count();
		Self::sub_qualification_count_by_owner(owner.get_owner_id());

		Ok(qualification)
	}

	fn qualification_by_id(
		qualification_id: &Self::GeneticAnalystQualificationId,
	) -> Option<Self::GeneticAnalystQualification> {
		GeneticAnalystQualifications::<T>::get(qualification_id)
	}

	fn qualification_count_by_owner(owner_id: &T::AccountId) -> u64 {
		Self::qualification_count_by_owner(owner_id).unwrap_or(0)
	}
}

/// Pallet Methods
impl<T: Config> Pallet<T> {
	// GeneticAnalystQualifications Count Addition and Substraction Helpers
	// Add qualifications count
	pub fn add_qualifications_count() {
		let qualifications_count = <GeneticAnalystQualificationsCount<T>>::get().unwrap_or(0);
		<GeneticAnalystQualificationsCount<T>>::put(qualifications_count.wrapping_add(1));
	}
	// Add qualifications count by owner
	pub fn add_qualification_count_by_owner(owner_id: &T::AccountId) {
		let qualifications_count =
			GeneticAnalystQualificationsCountByOwner::<T>::get(owner_id).unwrap_or(0);
		GeneticAnalystQualificationsCountByOwner::<T>::insert(
			owner_id,
			qualifications_count.wrapping_add(1),
		)
	}

	// Subtract qualifications count
	pub fn sub_qualifications_count() {
		let qualifications_count = <GeneticAnalystQualificationsCount<T>>::get().unwrap_or(1);
		GeneticAnalystQualificationsCount::<T>::put(qualifications_count - 1);
	}
	// Subtract qualifications count by owner
	pub fn sub_qualification_count_by_owner(owner_id: &T::AccountId) {
		let qualifications_count =
			GeneticAnalystQualificationsCountByOwner::<T>::get(owner_id).unwrap_or(1);
		GeneticAnalystQualificationsCountByOwner::<T>::insert(owner_id, qualifications_count - 1);
	}
}

/// GeneticAnalystQualificationsProvider Trait Implementation
impl<T: Config> GeneticAnalystQualificationsProvider<T> for Pallet<T> {
	type Error = Error<T>;
	type GeneticAnalystQualification = GeneticAnalystQualificationOf<T>;

	fn qualification_by_id(id: &T::Hash) -> Option<GeneticAnalystQualificationOf<T>> {
		<Self as GeneticAnalystQualificationInterface<T>>::qualification_by_id(id)
	}

	fn delete_qualification(
		owner_id: &T::AccountId,
		id: &T::Hash,
	) -> Result<Self::GeneticAnalystQualification, Self::Error> {
		<Self as GeneticAnalystQualificationInterface<T>>::delete_qualification(owner_id, id)
	}
}
