#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod functions;
pub mod impl_health_professional;
pub mod interface;
pub mod types;
pub mod weights;

pub use types::*;

use frame_support::traits::StorageVersion;
use interface::HealthProfessionalInterface;
use traits_health_professional_qualifications::HealthProfessionalQualificationProvider;
use weights::WeightInfo;

/// The current storage version.
const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use frame_support::{pallet_prelude::*, traits::Currency};
	use frame_system::pallet_prelude::*;
	use primitives_availability_status::AvailabilityStatus;
	use primitives_stake_status::StakeStatus;
	use primitives_verification_status::VerificationStatus;

	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type HashOf<T> = <T as frame_system::Config>::Hash;
	pub type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;
	pub type CurrencyOf<T> = <T as self::Config>::Currency;
	pub type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;
	pub type HealthProfessionalOf<T> =
		HealthProfessional<AccountIdOf<T>, HashOf<T>, MomentOf<T>, BalanceOf<T>>;
	pub type HealthProfessionalInfoOf<T> = HealthProfessionalInfo<HashOf<T>, MomentOf<T>>;

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<Self::AccountId>;
		type HealthProfessionalQualifications: HealthProfessionalQualificationProvider<Self>;
		type HealthProfessionalWeightInfo: WeightInfo;
	}

	#[pallet::storage]
	#[pallet::getter(fn health_professional_by_account_id)]
	pub type HealthProfessionals<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, HealthProfessionalOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn health_professional_count)]
	pub type HealthProfessionalCount<T> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn verifier_key)]
	pub type HealthProfessionalVerifierKey<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn unstake_time)]
	pub type UnstakeTime<T> = StorageValue<_, u128, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn minimum_stake_amount)]
	pub type MinimumStakeAmount<T> = StorageValue<_, BalanceOf<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn total_staked_amount)]
	pub type TotalStakedAmount<T> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		HealthProfessionalRegistered(AccountIdOf<T>, HealthProfessionalOf<T>),
		HealthProfessionalUnregistered(AccountIdOf<T>),
		HealthProfessionalInfoUpdated(AccountIdOf<T>, HealthProfessionalInfoOf<T>),
		HealthProfessionalVerificationStatusUpdated(AccountIdOf<T>, VerificationStatus),
		HealthProfessionalAvailabilityStatusUpdated(AccountIdOf<T>, AvailabilityStatus),
		HealthProfessionalStaked(AccountIdOf<T>, BalanceOf<T>),
		HealthProfessionalWaitingForUnstaked(AccountIdOf<T>, StakeStatus, MomentOf<T>),
		HealthProfessionalUnstaked(AccountIdOf<T>, BalanceOf<T>, StakeStatus, MomentOf<T>),
		VerifierKeyUpdated(AccountIdOf<T>),
		MinimumStakeAmountUpdated(BalanceOf<T>),
		UnstakeTimeUpdated(u128),
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub verifier_key: Option<T::AccountId>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { verifier_key: None }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			if let Some(ref verifier_key) = self.verifier_key {
				HealthProfessionalVerifierKey::<T>::put(verifier_key);
			}
		}
	}

	#[pallet::error]
	pub enum Error<T> {
		InsufficientBalance,
		AlreadyRegistered,
		AlreadyStaked,
		AlreadyUnstaked,
		CannotStaked,
		CannotUnstaked,
		CannotRetrieveUnstakedAmount,
		NotReadyToUnstaked,
		BadOrigin,
		NotFound,
		Unauthorized,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::HealthProfessionalWeightInfo::register())]
		pub fn register(
			origin: OriginFor<T>,
			info: HealthProfessionalInfoOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as HealthProfessionalInterface<T>>::create_health_professional(&who, &info)
			{
				Ok(health_professional) => {
					Self::deposit_event(Event::HealthProfessionalRegistered(
						who,
						health_professional,
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::HealthProfessionalWeightInfo::update_info())]
		pub fn update_info(
			origin: OriginFor<T>,
			info: HealthProfessionalInfoOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as HealthProfessionalInterface<T>>::update_health_professional_info(
				&who, &info,
			) {
				Ok(info) => {
					Self::deposit_event(Event::HealthProfessionalInfoUpdated(who, info));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::HealthProfessionalWeightInfo::update_availability_status())]
		pub fn update_availability_status(
			origin: OriginFor<T>,
			status: AvailabilityStatus,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as HealthProfessionalInterface<T>>::update_health_professional_availability_status(&who, &status) {
				Ok(_) => {
					Self::deposit_event(Event::HealthProfessionalAvailabilityStatusUpdated(who, status));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::HealthProfessionalWeightInfo::update_verification_status())]
		pub fn update_verification_status(
			origin: OriginFor<T>,
			account_id: AccountIdOf<T>,
			status: VerificationStatus,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as HealthProfessionalInterface<T>>::update_health_professional_verification_status(&who, &account_id, &status) {
				Ok(_) => {
					Self::deposit_event(Event::HealthProfessionalVerificationStatusUpdated(account_id, status));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::HealthProfessionalWeightInfo::deregister())]
		pub fn deregister(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as HealthProfessionalInterface<T>>::delete_health_professional(&who) {
				Ok(_) => {
					Self::deposit_event(Event::HealthProfessionalUnregistered(who));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::HealthProfessionalWeightInfo::stake())]
		pub fn stake(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as HealthProfessionalInterface<T>>::stake_health_professional(&who) {
				Ok(staking_balance) => {
					Self::deposit_event(Event::HealthProfessionalStaked(who, staking_balance));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::HealthProfessionalWeightInfo::unstake())]
		pub fn unstake(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as HealthProfessionalInterface<T>>::unstake_health_professional(&who) {
				Ok(waiting_at) => {
					Self::deposit_event(Event::HealthProfessionalWaitingForUnstaked(
						who,
						StakeStatus::WaitingForUnstaked,
						waiting_at,
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::HealthProfessionalWeightInfo::retrieve_unstaked_amount())]
		pub fn retrieve_unstaked_amount(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as HealthProfessionalInterface<T>>::retrieve_unstaked_amount(&who) {
				Ok((staking_balance, unstaked_at)) => {
					Self::deposit_event(Event::HealthProfessionalUnstaked(
						who,
						staking_balance,
						StakeStatus::Unstaked,
						unstaked_at,
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::HealthProfessionalWeightInfo::update_stake_amount())]
		pub fn update_stake_amount(
			origin: OriginFor<T>,
			amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as HealthProfessionalInterface<T>>::update_stake_amount(&who, &amount) {
				Ok(()) => {
					Self::deposit_event(Event::MinimumStakeAmountUpdated(amount));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::HealthProfessionalWeightInfo::update_unstake_time())]
		pub fn update_unstake_time(
			origin: OriginFor<T>,
			moment: u128,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as HealthProfessionalInterface<T>>::update_unstake_time(&who, moment) {
				Ok(()) => {
					Self::deposit_event(Event::UnstakeTimeUpdated(moment));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::HealthProfessionalWeightInfo::update_verifier_key())]
		pub fn update_verifier_key(
			origin: OriginFor<T>,
			account_id: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as HealthProfessionalInterface<T>>::update_verifier_key(&who, &account_id) {
				Ok(_) => {
					Self::deposit_event(Event::VerifierKeyUpdated(who));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(0)]
		pub fn sudo_update_verifier_key(
			origin: OriginFor<T>,
			account_id: T::AccountId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			HealthProfessionalVerifierKey::<T>::put(&account_id);

			Self::deposit_event(Event::VerifierKeyUpdated(account_id));

			Ok(Pays::No.into())
		}
	}
}
