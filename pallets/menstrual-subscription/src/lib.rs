#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod functions;
pub mod impl_menstrual_subscription;
pub mod interface;
pub mod migrations;
pub mod types;
pub mod weights;

pub use interface::MenstrualSubscriptionInterface;
pub use types::*;
pub use weights::WeightInfo;

pub use frame_support::traits::StorageVersion;

/// The current storage version
const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
		traits::{tokens::fungibles, Currency},
	};
	use frame_system::pallet_prelude::*;
	use primitives_duration::MenstrualSubscriptionDuration;
	use primitives_menstrual_status::MenstrualSubscriptionStatus;
	use primitives_price_and_currency::CurrencyType;
	use sp_std::vec::Vec;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<<Self as frame_system::Config>::AccountId>;
		type Assets: fungibles::Transfer<
				<Self as frame_system::Config>::AccountId,
				AssetId = AssetId,
				Balance = AssetBalance,
			> + fungibles::InspectMetadata<<Self as frame_system::Config>::AccountId>;
		type MenstrualSubscriptionWeightInfo: WeightInfo;
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub admin_key: Option<T::AccountId>,
		pub treasury_key: Option<T::AccountId>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { admin_key: None, treasury_key: None }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			if let Some(ref admin_key) = self.admin_key {
				AdminKey::<T>::put(admin_key);
			}
			if let Some(ref treasury_key) = self.treasury_key {
				TreasuryKey::<T>::put(treasury_key);
			}
		}
	}

	// ----- This is template code, every pallet needs this ---
	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_runtime_upgrade() -> Weight {
			migrations::migrate::<T>()
		}
	}
	// --------------------------------------------------------

	// ----- Types -------
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type HashOf<T> = <T as frame_system::Config>::Hash;
	pub type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;
	pub type MenstrualSubscriptionOf<T> =
		MenstrualSubscription<AccountIdOf<T>, HashOf<T>, MomentOf<T>>;
	pub type MenstrualSubscriptionPriceOf<T> = MenstrualSubscriptionPrice<BalanceOf<T>>;
	pub type MenstrualSubscriptionIdOf<T> = HashOf<T>;
	pub type CurrencyOf<T> = <T as self::Config>::Currency;
	pub type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;
	pub type AccountKeyTypeOf<T> = AccountKeyType<AccountIdOf<T>>;

	// ------- Storage -------------
	#[pallet::storage]
	#[pallet::getter(fn admin_key)]
	pub type AdminKey<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn treasury_key)]
	pub type TreasuryKey<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn menstrual_subscription_by_address_id)]
	pub type MenstrualSubscriptionByOwner<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Vec<MenstrualSubscriptionIdOf<T>>>;

	#[pallet::storage]
	#[pallet::getter(fn active_subscription_by_owner)]
	pub type ActiveSubscriptionByOwner<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, HashOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn menstrual_subscription_by_id)]
	pub type MenstrualSubscriptionById<T> =
		StorageMap<_, Blake2_128Concat, MenstrualSubscriptionIdOf<T>, MenstrualSubscriptionOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn menstrual_subscription_count_by_owner)]
	pub type MenstrualSubscriptionCountByOwner<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, u64>;

	#[pallet::storage]
	#[pallet::getter(fn menstrual_subscription_count)]
	pub type MenstrualSubscriptionCount<T> = StorageValue<_, u64>;

	#[pallet::storage]
	#[pallet::getter(fn menstrual_subscription_prices)]
	pub type MenstrualSubscriptionPrices<T> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		MenstrualSubscriptionDuration,
		Blake2_128Concat,
		CurrencyType,
		MenstrualSubscriptionPriceOf<T>,
	>;

	// -----------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters, [MenstrualSubscription, who]
		MenstrualSubscriptionAdded(MenstrualSubscriptionOf<T>, AccountIdOf<T>),
		//// MenstrualSubscription updated
		/// parameters, [MenstrualSubscription]
		MenstrualSubscriptionUpdated(MenstrualSubscriptionOf<T>),
		//// MenstrualSubscription paid
		/// parameters, [MenstrualSubscription, who]
		MenstrualSubscriptionPaid(MenstrualSubscriptionOf<T>, AccountIdOf<T>),
		/// Update menstrual subscription admin key successful
		/// parameters. [who]
		UpdateMenstrualSubscriptionKeySuccessful(AccountKeyTypeOf<T>),
		TotalSupplyDecreased(BalanceOf<T>),
		MenstrualSubscriptionPriceAdded(MenstrualSubscriptionPriceOf<T>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// User not allowed to create menstrual_subscription
		NotAllowedToCreate,
		/// User is not the owner of a menstrual_subscription
		NotMenstrualSubscriptionOwner,
		/// Ordering a menstrual_subscription that does not exist
		MenstrualSubscriptionDoesNotExist,
		// Unauthorized access of an Admin key
		Unauthorized,
		MenstrualSubscriptionPriceNotExist,
		MenstrualSubscriptionAlreadyPaid,
		MenstrualSubscriptionNotPaid,
		AssetIdNotFound,
		Module,
		Other,
		BadOrigin,
		CannotLookup,
		ConsumerRemaining,
		TooManyConsumers,
		NoProviders,
		Token,
		Arithmetic,
		InsufficientBalance,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::MenstrualSubscriptionWeightInfo::add_menstrual_subscription())]
		pub fn add_menstrual_subscription(
			origin: OriginFor<T>,
			duration: MenstrualSubscriptionDuration,
			currency: CurrencyType,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as MenstrualSubscriptionInterface<T>>::add_menstrual_subscription(
				&who, &duration, &currency,
			) {
				Ok(menstrual_subscription) => {
					Self::deposit_event(Event::MenstrualSubscriptionAdded(
						menstrual_subscription,
						who,
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::MenstrualSubscriptionWeightInfo::change_menstrual_subscription_status())]
		pub fn change_menstrual_subscription_status(
			origin: OriginFor<T>,
			menstrual_subscription_id: HashOf<T>,
			status: MenstrualSubscriptionStatus,
		) -> DispatchResultWithPostInfo {
			let admin = ensure_signed(origin)?;
			let admin_key = AdminKey::<T>::get().filter(|account_id| account_id == &admin);

			ensure!(admin_key.is_some(), Error::<T>::Unauthorized);

			match <Self as MenstrualSubscriptionInterface<T>>::change_menstrual_subscription_status(
				&menstrual_subscription_id,
				&status,
			) {
				Ok(menstrual_subscription) => {
					Self::deposit_event(Event::MenstrualSubscriptionUpdated(
						menstrual_subscription,
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::MenstrualSubscriptionWeightInfo::set_menstrual_subscription_paid())]
		pub fn set_menstrual_subscription_paid(
			origin: OriginFor<T>,
			menstrual_subscription_id: HashOf<T>,
		) -> DispatchResultWithPostInfo {
			let account_id = ensure_signed(origin)?;

			match <Self as MenstrualSubscriptionInterface<T>>::set_menstrual_subscription_paid(
				&account_id,
				&menstrual_subscription_id,
			) {
				Ok(menstrual_subscription) => {
					Self::deposit_event(Event::MenstrualSubscriptionPaid(
						menstrual_subscription,
						account_id,
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::MenstrualSubscriptionWeightInfo::set_menstrual_subscription_price())]
		pub fn set_menstrual_subscription_price(
			origin: OriginFor<T>,
			duration: MenstrualSubscriptionDuration,
			currency: CurrencyType,
			price: BalanceOf<T>,
			asset_id: Option<AssetId>,
		) -> DispatchResultWithPostInfo {
			let admin = ensure_signed(origin)?;
			let admin_key = AdminKey::<T>::get().filter(|account_id| account_id == &admin);

			ensure!(admin_key.is_some(), Error::<T>::Unauthorized);

			match <Self as MenstrualSubscriptionInterface<T>>::set_menstrual_subscription_price(
				&duration, &currency, price, asset_id,
			) {
				Ok(menstrual_subscription_price) => {
					Self::deposit_event(Event::MenstrualSubscriptionPriceAdded(
						menstrual_subscription_price,
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(0)]
		pub fn update_key(
			origin: OriginFor<T>,
			account_key_type: AccountKeyTypeOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match account_key_type.clone() {
				AccountKeyType::TreasuryKey(account_id) => {
					let result = TreasuryKey::<T>::get().filter(|account_id| account_id == &who);
					ensure!(result.is_some(), Error::<T>::Unauthorized);
					TreasuryKey::<T>::put(&account_id);
				},
				AccountKeyType::AdminKey(account_id) => {
					let result = AdminKey::<T>::get().filter(|account_id| account_id == &who);
					ensure!(result.is_some(), Error::<T>::Unauthorized);
					AdminKey::<T>::put(&account_id);
				},
			};

			Self::deposit_event(Event::UpdateMenstrualSubscriptionKeySuccessful(account_key_type));

			Ok(Pays::No.into())
		}

		#[pallet::weight(0)]
		pub fn sudo_update_key(
			origin: OriginFor<T>,
			account_key_type: AccountKeyTypeOf<T>,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			match account_key_type.clone() {
				AccountKeyType::TreasuryKey(account_id) => TreasuryKey::<T>::put(&account_id),
				AccountKeyType::AdminKey(account_id) => AdminKey::<T>::put(&account_id),
			};

			Self::deposit_event(Event::UpdateMenstrualSubscriptionKeySuccessful(account_key_type));

			Ok(Pays::No.into())
		}
	}
}
