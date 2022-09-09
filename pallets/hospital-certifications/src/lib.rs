#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	codec::{Decode, Encode},
	pallet_prelude::*,
};
pub use pallet::*;
pub use scale_info::TypeInfo;
use traits_hospital_certifications::{
	HospitalCertificationInfo as HospitalCertificationInfoT, HospitalCertificationOwner,
	HospitalCertificationsProvider,
};

pub mod interface;
pub mod weights;
pub use interface::HospitalCertificationInterface;
use sp_std::prelude::*;

/// HospitalCertificationInfo struct
/// Information that is mutable by user
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct HospitalCertificationInfo {
	pub title: Vec<u8>,
	pub issuer: Vec<u8>,
	pub month: Vec<u8>,
	pub year: Vec<u8>,
	pub description: Vec<u8>,
	pub supporting_document: Option<Vec<u8>>,
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct HospitalCertification<AccountId, Hash> {
	pub id: Hash,
	pub owner_id: AccountId,
	pub info: HospitalCertificationInfo,
}
impl<AccountId, Hash> HospitalCertification<AccountId, Hash> {
	pub fn new(id: Hash, owner_id: AccountId, info: HospitalCertificationInfo) -> Self {
		Self { id, owner_id, info }
	}

	pub fn get_id(&self) -> &Hash {
		&self.id
	}

	pub fn get_owner_id(&self) -> &AccountId {
		&self.owner_id
	}
}

impl<T, AccountId, Hash> HospitalCertificationInfoT<T> for HospitalCertification<AccountId, Hash>
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
		interface::HospitalCertificationInterface, weights::WeightInfo, HospitalCertification,
		HospitalCertificationInfo, HospitalCertificationOwner,
	};
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	pub use sp_std::prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type HospitalCertificationOwner: HospitalCertificationOwner<Self>;
		type WeightInfo: WeightInfo;
	}

	// ----- This is template code, every pallet needs this ---
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
	// --------------------------------------------------------

	// ----- Types -------
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type HashOf<T> = <T as frame_system::Config>::Hash;
	pub type HospitalCertificationOf<T> = HospitalCertification<AccountIdOf<T>, HashOf<T>>;
	pub type HospitalCertificationInfoOf = HospitalCertificationInfo;
	pub type HospitalCertificationIdOf<T> = HashOf<T>;

	// ------- Storage -------------
	#[pallet::storage]
	#[pallet::getter(fn certification_by_id)]
	pub type HospitalCertifications<T> =
		StorageMap<_, Blake2_128Concat, HashOf<T>, HospitalCertificationOf<T>>;
	//                                _,  Hasher         ,  Key     ,  Value

	#[pallet::storage]
	#[pallet::getter(fn certifications_count)]
	pub type HospitalCertificationsCount<T> = StorageValue<_, u64>;

	#[pallet::storage]
	#[pallet::getter(fn certification_count_by_owner)]
	pub type HospitalCertificationsCountByOwner<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, u64>;
	// -----------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters, [HospitalCertification, who]
		HospitalCertificationCreated(HospitalCertificationOf<T>, AccountIdOf<T>),
		//// HospitalCertification updated
		/// parameters, [HospitalCertification, who]
		HospitalCertificationUpdated(HospitalCertificationOf<T>, AccountIdOf<T>),
		//// HospitalCertification deleted
		/// parameters, [HospitalCertification, who]
		HospitalCertificationDeleted(HospitalCertificationOf<T>, AccountIdOf<T>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// User not allowed to create certification
		NotAllowedToCreate,
		/// User is not the owner of a certification
		NotHospitalCertificationOwner,
		/// Ordering a certification that does not exist
		HospitalCertificationDoesNotExist,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::create_certification())]
		pub fn create_certification(
			origin: OriginFor<T>,
			certification_info: HospitalCertificationInfoOf,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as HospitalCertificationInterface<T>>::create_certification(
				&who,
				&certification_info,
			) {
				Ok(certification) => {
					Self::deposit_event(Event::HospitalCertificationCreated(
						certification,
						who.clone(),
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::update_certification())]
		pub fn update_certification(
			origin: OriginFor<T>,
			certification_id: HashOf<T>,
			certification_info: HospitalCertificationInfoOf,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			match <Self as HospitalCertificationInterface<T>>::update_certification(
				&who,
				&certification_id,
				&certification_info,
			) {
				Ok(certification) => {
					Self::deposit_event(Event::HospitalCertificationUpdated(
						certification,
						who.clone(),
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::delete_certification())]
		pub fn delete_certification(
			origin: OriginFor<T>,
			certification_id: T::Hash,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			match <Self as HospitalCertificationInterface<T>>::delete_certification(
				&who,
				&certification_id,
			) {
				Ok(certification) => {
					Self::deposit_event(Event::HospitalCertificationDeleted(
						certification,
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
use traits_hospital_certifications::HospitalCertificationOwnerInfo;

/// HospitalCertification Interface Implementation
impl<T: Config> HospitalCertificationInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type HospitalCertificationId = T::Hash;
	type HospitalCertification = HospitalCertificationOf<T>;
	type HospitalCertificationInfo = HospitalCertificationInfoOf;

	fn generate_certification_id(
		owner_id: &T::AccountId,
		certification_count: u64,
	) -> Self::HospitalCertificationId {
		let mut account_id_bytes = owner_id.encode();
		let mut certification_count_bytes = certification_count.encode();
		account_id_bytes.append(&mut certification_count_bytes);

		let seed = &account_id_bytes;
		T::Hashing::hash(seed)
	}

	/// Create HospitalCertification
	/// Add reference to HospitalCertificationsByCountryCity storage
	/// Associate certification reference to the owner (creator)
	/// Increment Counts
	fn create_certification(
		owner_id: &T::AccountId,
		certification_info: &Self::HospitalCertificationInfo,
	) -> Result<Self::HospitalCertification, Self::Error> {
		// Check if user can create_certification
		let can_create_certification =
			T::HospitalCertificationOwner::can_create_certification(owner_id);
		if !can_create_certification {
			return Err(Error::<T>::NotAllowedToCreate)
		}

		let owner_certification_count =
			<Self as HospitalCertificationInterface<T>>::certification_count_by_owner(owner_id);
		let certification_id = Self::generate_certification_id(owner_id, owner_certification_count);

		let certification = HospitalCertification::new(
			certification_id,
			owner_id.clone(),
			certification_info.clone(),
		);
		// Store to HospitalCertifications storage
		HospitalCertifications::<T>::insert(certification_id, &certification);

		// Increment HospitalCertifications Count
		Self::add_certifications_count();
		// Increment HospitalCertificationsCountByOwner
		Self::add_certification_count_by_owner(&certification.owner_id);

		// Associate created certification to the owner
		T::HospitalCertificationOwner::associate(owner_id, &certification_id);

		Ok(certification)
	}

	/// Update HospitalCertification information
	fn update_certification(
		owner_id: &T::AccountId,
		certification_id: &Self::HospitalCertificationId,
		certification_info: &Self::HospitalCertificationInfo,
	) -> Result<Self::HospitalCertification, Self::Error> {
		let certification = HospitalCertifications::<T>::get(certification_id);
		if certification.is_none() {
			return Err(Error::<T>::HospitalCertificationDoesNotExist)
		}
		let mut certification = certification.unwrap();

		if certification.owner_id != owner_id.clone() {
			return Err(Error::<T>::NotHospitalCertificationOwner)
		}

		certification.info = certification_info.clone();
		HospitalCertifications::<T>::insert(certification_id, &certification);

		Ok(certification)
	}

	/// Delete HospitalCertification
	/// Delete from HospitalCertifications Storage
	/// Remove the certification id reference in HospitalCertificationsByCountryCity storage
	/// Disassociate certification id from the owner
	/// Decrement Counts
	fn delete_certification(
		owner_id: &T::AccountId,
		certification_id: &Self::HospitalCertificationId,
	) -> Result<Self::HospitalCertification, Self::Error> {
		let certification = HospitalCertifications::<T>::get(certification_id);
		if certification.is_none() {
			return Err(Error::<T>::HospitalCertificationDoesNotExist)
		}
		let certification = certification.unwrap();

		if certification.owner_id != owner_id.clone() {
			return Err(Error::<T>::NotHospitalCertificationOwner)
		}
		// Remove certification from storage
		let certification = HospitalCertifications::<T>::take(certification_id).unwrap();

		let owner = T::HospitalCertificationOwner::get_owner(owner_id).unwrap();
		// disassociate certification reference from the owner
		T::HospitalCertificationOwner::disassociate(owner.get_owner_id(), &certification.id);
		// Decrement counts
		Self::sub_certifications_count();
		Self::sub_certification_count_by_owner(owner.get_owner_id());

		Ok(certification)
	}

	fn certification_by_id(
		certification_id: &Self::HospitalCertificationId,
	) -> Option<Self::HospitalCertification> {
		HospitalCertifications::<T>::get(certification_id)
	}

	fn certification_count_by_owner(owner_id: &T::AccountId) -> u64 {
		Self::certification_count_by_owner(owner_id).unwrap_or(0)
	}
}

/// Pallet Methods
impl<T: Config> Pallet<T> {
	// HospitalCertifications Count Addition and Substraction Helpers
	// Add certifications count
	pub fn add_certifications_count() {
		let certifications_count = <HospitalCertificationsCount<T>>::get().unwrap_or(0);
		<HospitalCertificationsCount<T>>::put(certifications_count.wrapping_add(1));
	}
	// Add certifications count by owner
	pub fn add_certification_count_by_owner(owner_id: &T::AccountId) {
		let certifications_count =
			HospitalCertificationsCountByOwner::<T>::get(owner_id).unwrap_or(0);
		HospitalCertificationsCountByOwner::<T>::insert(
			owner_id,
			certifications_count.wrapping_add(1),
		)
	}

	// Subtract certifications count
	pub fn sub_certifications_count() {
		let certifications_count = <HospitalCertificationsCount<T>>::get().unwrap_or(1);
		HospitalCertificationsCount::<T>::put(certifications_count - 1);
	}
	// Subtract certifications count by owner
	pub fn sub_certification_count_by_owner(owner_id: &T::AccountId) {
		let certifications_count =
			HospitalCertificationsCountByOwner::<T>::get(owner_id).unwrap_or(1);
		HospitalCertificationsCountByOwner::<T>::insert(owner_id, certifications_count - 1);
	}
}

/// HospitalCertificationsProvider Trait Implementation
impl<T: Config> HospitalCertificationsProvider<T> for Pallet<T> {
	type Error = Error<T>;
	type HospitalCertification = HospitalCertificationOf<T>;

	fn certification_by_id(id: &T::Hash) -> Option<HospitalCertificationOf<T>> {
		<Self as HospitalCertificationInterface<T>>::certification_by_id(id)
	}

	fn delete_certification(
		owner_id: &T::AccountId,
		id: &T::Hash,
	) -> Result<Self::HospitalCertification, Self::Error> {
		<Self as HospitalCertificationInterface<T>>::delete_certification(owner_id, id)
	}
}
