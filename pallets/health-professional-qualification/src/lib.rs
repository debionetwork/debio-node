#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod functions;
pub mod impl_health_professional_qualification;
pub mod interface;
pub mod types;
pub mod weights;

pub use types::*;

use frame_support::traits::StorageVersion;
use interface::HealthProfessionalQualificationInterface;
use traits_health_professional_qualifications::HealthProfessionalQualificationOwner;
use weights::WeightInfo;

/// The current storage version.
const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type HashOf<T> = <T as frame_system::Config>::Hash;
	pub type QualificationOf<T> = Qualification<HashOf<T>, AccountIdOf<T>>;

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type HealthProfessionalQualificationOwner: HealthProfessionalQualificationOwner<Self>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
	#[pallet::getter(fn health_professional_qualification_by_id)]
	pub type HealthProfessionalQualifications<T> =
		StorageMap<_, Blake2_128Concat, HashOf<T>, QualificationOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn health_professional_qualification_count)]
	pub type HealthProfessionalQualificationCount<T> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn health_professional_qualification_count_by_owner)]
	pub type HealthProfessionalQualificationCountByOwner<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, u64, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		HealthProfessionalQualificationCreated(AccountIdOf<T>, QualificationOf<T>),
		HealthProfessionalQualificationUpdated(AccountIdOf<T>, HashOf<T>),
		HealthProfessionalQualificationDeleted(AccountIdOf<T>, HashOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		NotRegistered,
		NotFound,
		Unauthorized,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::create())]
		pub fn create(
			origin: OriginFor<T>,
			experiences: Vec<Experience>,
			certifications: Vec<Certification>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as HealthProfessionalQualificationInterface<T>>::create_qualification(
				&who,
				&experiences,
				&certifications,
			) {
				Ok(qualification) => {
					Self::deposit_event(Event::HealthProfessionalQualificationCreated(
						who,
						qualification,
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::update())]
		pub fn update(
			origin: OriginFor<T>,
			qualification_id: HashOf<T>,
			experiences: Option<Vec<Experience>>,
			certifications: Option<Vec<Certification>>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as HealthProfessionalQualificationInterface<T>>::update_qualification(
				&who,
				&qualification_id,
				&experiences,
				&certifications,
			) {
				Ok(_) => {
					Self::deposit_event(Event::HealthProfessionalQualificationUpdated(
						who,
						qualification_id,
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::delete())]
		pub fn delete(
			origin: OriginFor<T>,
			qualification_id: HashOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as HealthProfessionalQualificationInterface<T>>::delete_qualification(
				&who,
				&qualification_id,
			) {
				Ok(_) => {
					Self::deposit_event(Event::HealthProfessionalQualificationDeleted(
						who,
						qualification_id,
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}
	}
}
