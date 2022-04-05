#![cfg_attr(not(feature = "std"), no_std)]

pub mod interface;
pub mod migrations;
pub mod weights;

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
pub use pallet::*;
pub use scale_info::TypeInfo;
pub use weights::WeightInfo;

pub use crate::interface::GeneticAnalystInterface;
use frame_support::{
	pallet_prelude::*,
	sp_runtime::{traits::AccountIdConversion, RuntimeDebug},
	traits::Currency,
	PalletId,
};
use primitives_availability_status::{AvailabilityStatus, AvailabilityStatusTrait};
use primitives_stake_status::{StakeStatus, StakeStatusTrait};
use primitives_verification_status::{VerificationStatus, VerificationStatusTrait};
pub use traits_genetic_analysis_orders::{
	GeneticAnalysisOrderEventEmitter, GeneticAnalysisOrderStatusUpdater,
};
use traits_genetic_analyst_qualifications::GeneticAnalystQualificationOwnerInfo;
use traits_genetic_analyst_services::GeneticAnalystServiceOwnerInfo;
use traits_genetic_analysts::GeneticAnalystsProvider;

// GeneticAnalystInfo Struct
// Used as parameter of dispatchable calls
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct GeneticAnalystInfo<Hash, Moment>
where
	Hash: PartialEq + Eq,
{
	pub box_public_key: Hash,
	pub first_name: Vec<u8>,
	pub last_name: Vec<u8>,
	pub gender: Vec<u8>,
	pub date_of_birth: Moment,
	pub email: Vec<u8>,
	pub phone_number: Vec<u8>,
	pub specialization: Vec<u8>,
	pub profile_link: Vec<u8>,
	pub profile_image: Option<Vec<u8>>,
}

// GeneticAnalyst Struct
// the fields (excluding account_id and qualifications) come from GeneticAnalystInfo struct
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct GeneticAnalyst<AccountId, Hash, Moment, Balance>
where
	Hash: PartialEq + Eq,
{
	pub account_id: AccountId,
	pub services: Vec<Hash>,
	pub qualifications: Vec<Hash>,
	pub info: GeneticAnalystInfo<Hash, Moment>,
	pub stake_amount: Balance,
	pub stake_status: StakeStatus,
	pub verification_status: VerificationStatus,
	pub availability_status: AvailabilityStatus,
	pub unstake_at: Moment,
	pub retrieve_unstake_at: Moment,
}

impl<AccountId, Hash, Moment: Default, Balance: Default>
	GeneticAnalyst<AccountId, Hash, Moment, Balance>
where
	Hash: PartialEq + Eq,
{
	pub fn new(account_id: AccountId, info: GeneticAnalystInfo<Hash, Moment>) -> Self {
		Self {
			account_id,
			services: Vec::<Hash>::new(),
			qualifications: Vec::<Hash>::new(),
			info,
			stake_amount: Balance::default(),
			stake_status: StakeStatus::default(),
			verification_status: VerificationStatus::default(),
			availability_status: AvailabilityStatus::default(),
			unstake_at: Moment::default(),
			retrieve_unstake_at: Moment::default(),
		}
	}

	fn update_info(&mut self, info: GeneticAnalystInfo<Hash, Moment>) {
		self.info = info;
	}

	pub fn is_available(&self) -> bool {
		self.availability_status.is_available()
	}

	pub fn get_account_id(&self) -> &AccountId {
		&self.account_id
	}

	pub fn add_service(&mut self, service_id: Hash) {
		self.services.push(service_id);
	}

	pub fn remove_service(&mut self, service_id: Hash) {
		if let Some(pos) = &self.services.iter().position(|x| *x == service_id) {
			self.services.remove(*pos);
		}
	}

	pub fn add_qualification(&mut self, qualification_id: Hash) {
		self.qualifications.push(qualification_id);
	}

	pub fn remove_qualification(&mut self, qualification_id: Hash) {
		if let Some(pos) = &self.qualifications.iter().position(|x| *x == qualification_id) {
			self.qualifications.remove(*pos);
		}
	}
}

impl<T, AccountId, Hash, Moment: Default, Balance: Default> GeneticAnalystServiceOwnerInfo<T>
	for GeneticAnalyst<AccountId, Hash, Moment, Balance>
where
	Hash: PartialEq + Eq,
	T: frame_system::Config<AccountId = AccountId>,
{
	fn get_id(&self) -> &AccountId {
		self.get_account_id()
	}
}

impl<T, AccountId, Hash, Moment: Default, Balance: Default> GeneticAnalystQualificationOwnerInfo<T>
	for GeneticAnalyst<AccountId, Hash, Moment, Balance>
where
	Hash: PartialEq + Eq,
	T: frame_system::Config<AccountId = AccountId>,
{
	fn get_owner_id(&self) -> &AccountId {
		self.get_account_id()
	}
}

#[frame_support::pallet]
pub mod pallet {
	use crate::{interface::GeneticAnalystInterface, GeneticAnalyst, GeneticAnalystInfo, *};
	use codec::EncodeLike;
	use frame_support::{dispatch::DispatchResultWithPostInfo, traits::Currency};
	use frame_system::pallet_prelude::*;
	pub use sp_std::prelude::*;
	pub use traits_genetic_analyst_qualifications::{
		GeneticAnalystQualificationOwner, GeneticAnalystQualificationsProvider,
	};
	pub use traits_genetic_analyst_services::{
		GeneticAnalystServiceOwner, GeneticAnalystServicesProvider,
	};
	use traits_user_profile::UserProfileProvider;

	#[pallet::config]
	/// Configure the pallet by specifying the parameters and types on which it depends.
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<Self::AccountId>;
		type GeneticAnalystServices: GeneticAnalystServicesProvider<Self, BalanceOf<Self>>;
		type GeneticAnalystQualifications: GeneticAnalystQualificationsProvider<Self>;
		type GeneticAnalysisOrders: GeneticAnalysisOrderEventEmitter<Self>
			+ GeneticAnalysisOrderStatusUpdater<Self>;
		type EthereumAddress: Clone
			+ Copy
			+ PartialEq
			+ Eq
			+ Encode
			+ EncodeLike
			+ Decode
			+ Default
			+ TypeInfo
			+ sp_std::fmt::Debug;
		type UserProfile: UserProfileProvider<Self, Self::EthereumAddress>;
		type GeneticAnalystWeightInfo: WeightInfo;
		/// Currency type for this pallet.
		type PalletId: Get<PalletId>;
	}

	// ----- This is template code, every pallet needs this ---
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
	// --------------------------------------------------------

	// ---- Types ----------------------
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type HashOf<T> = <T as frame_system::Config>::Hash;
	pub type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;
	pub type CurrencyOf<T> = <T as self::Config>::Currency;
	pub type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;
	pub type GeneticAnalystOf<T> =
		GeneticAnalyst<AccountIdOf<T>, HashOf<T>, MomentOf<T>, BalanceOf<T>>;

	// ----- Storage ------------------
	/// Get GeneticAnalyst by account id
	/// AccountId => GeneticAnalyst
	#[pallet::storage]
	#[pallet::getter(fn genetic_analyst_by_account_id)]
	pub type GeneticAnalysts<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, GeneticAnalystOf<T>>;

	/// Get total genetic_analyst count
	/// u32
	#[pallet::storage]
	#[pallet::getter(fn genetic_analyst_count)]
	pub type GeneticAnalystCount<T> = StorageValue<_, u64>;

	#[pallet::storage]
	#[pallet::getter(fn admin_key)]
	pub type GeneticAnalystVerifierKey<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn pallet_id)]
	pub type PalletAccount<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn total_staked_amount)]
	pub type TotalStakedAmount<T> = StorageValue<_, BalanceOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn minimum_stake_amount)]
	pub type MinimumStakeAmount<T> = StorageValue<_, BalanceOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn unstake_time)]
	pub type UnstakeTime<T> = StorageValue<_, MomentOf<T>>;
	// -----------------------------------------

	// ----- Genesis Configs ------------------
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub genetic_analyst_verifier_key: T::AccountId,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { genetic_analyst_verifier_key: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			UnstakeTime::<T>::put(MomentOf::<T>::default());
			GeneticAnalystVerifierKey::<T>::put(&self.genetic_analyst_verifier_key);
			PalletAccount::<T>::put(<Pallet<T>>::get_pallet_id());
			<Pallet<T>>::set_minimum_stake_amount(50000000000000000000000u128.saturated_into());
			<Pallet<T>>::set_total_staked_amount();
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// User AccountId registered as genetic_analyst
		/// parameters. [GeneticAnalyst, who]
		GeneticAnalystRegistered(GeneticAnalystOf<T>, AccountIdOf<T>),
		/// GeneticAnalyst information updated
		/// parameters. [GeneticAnalyst, who]
		GeneticAnalystUpdated(GeneticAnalystOf<T>, AccountIdOf<T>),
		/// GeneticAnalyst deleted
		/// parameters. [GeneticAnalyst, who]
		GeneticAnalystDeleted(GeneticAnalystOf<T>, AccountIdOf<T>),
		/// GeneticAnalyst verification updated
		/// parameters. [GeneticAnalyst, who]
		GeneticAnalystUpdateVerificationStatus(GeneticAnalystOf<T>, AccountIdOf<T>),
		/// GeneticAnalyst availability updated
		/// parameters. [GeneticAnalyst, who]
		GeneticAnalystUpdateAvailabilityStatus(GeneticAnalystOf<T>, AccountIdOf<T>),
		/// GeneticAnalyst stake successful
		/// parameters. [GeneticAnalyst, who]
		GeneticAnalystStakeSuccessful(GeneticAnalystOf<T>, AccountIdOf<T>),
		/// GeneticAnalyst unstake successful
		/// parameters. [GeneticAnalyst, who]
		GeneticAnalystUnstakeSuccessful(GeneticAnalystOf<T>, AccountIdOf<T>),
		/// GeneticAnalyst retrive unstake amount
		/// parameters. [GeneticAnalyst, who]
		GeneticAnalystRetrieveUnstakeAmount(GeneticAnalystOf<T>, AccountIdOf<T>),
		/// Update GeneticAnalyst minimum stake successful
		/// parameters. [who]
		UpdateGeneticAnalystMinimumStakeSuccessful(AccountIdOf<T>),
		/// Update GeneticAnalyst unstake time successful
		/// parameters. [who]
		UpdateGeneticAnalystUnstakeTimeSuccessful(AccountIdOf<T>),
		/// Update GeneticAnalyst admin key
		/// parameters. [who]
		UpdateGeneticAnalystAdminKeySuccessful(AccountIdOf<T>),
		/// GeneticAnalyst verification failed
		/// parameters. [GeneticAnalyst, who]
		GeneticAnalystVerificationFailed(GeneticAnalystOf<T>, AccountIdOf<T>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Account already has genetic_analyst registered
		GeneticAnalystAlreadyRegistered,
		/// Account already has genetic_analyst staked
		GeneticAnalystAlreadyStaked,
		/// GeneticAnalyst identified by the AccountId does not exist
		GeneticAnalystDoesNotExist,
		/// GeneticAnalyst is not the owner of the qualification
		GeneticAnalystIsNotOwner,
		/// GeneticAnalyst verification failed
		GeneticAnalystVerificationFailed,
		/// Insufficient funds
		InsufficientFunds,
		/// Insufficient pallet funds
		InsufficientPalletFunds,
		/// Account has not staked
		GeneticAnalystIsNotStaked,
		/// Unauthorized access to extrinsic
		Unauthorized,
		// GeneticAnalyst has pending orders
		GeneticAnalystHasPendingOrders,
		// GeneticAnalyst not waiting for unstake
		GeneticAnalystIsNotWaitingForUnstake,
		// GeneticAnalyst cannot unstake now
		GeneticAnalystCannotUnstakeBeforeUnstakeTime,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::GeneticAnalystWeightInfo::register_genetic_analyst())]
		pub fn register_genetic_analyst(
			origin: OriginFor<T>,
			genetic_analyst_info: GeneticAnalystInfo<HashOf<T>, MomentOf<T>>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match Self::create_genetic_analyst(&who, &genetic_analyst_info) {
				Ok(genetic_analyst) => {
					Self::deposit_event(Event::GeneticAnalystRegistered(
						genetic_analyst,
						who.clone(),
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::GeneticAnalystWeightInfo::update_genetic_analyst())]
		pub fn update_genetic_analyst(
			origin: OriginFor<T>,
			genetic_analyst_info: GeneticAnalystInfo<HashOf<T>, MomentOf<T>>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as GeneticAnalystInterface<T>>::update_genetic_analyst(
				&who,
				&genetic_analyst_info,
			) {
				Ok(genetic_analyst) => {
					Self::deposit_event(Event::GeneticAnalystUpdated(genetic_analyst, who.clone()));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::GeneticAnalystWeightInfo::deregister_genetic_analyst())]
		pub fn deregister_genetic_analyst(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			// Check if user is a genetic_analyst
			let genetic_analyst = Self::genetic_analyst_by_account_id(&who);
			if genetic_analyst == None {
				return Err(Error::<T>::GeneticAnalystDoesNotExist.into())
			}

			match <Self as GeneticAnalystInterface<T>>::delete_genetic_analyst(&who) {
				Ok(genetic_analyst) => {
					Self::deposit_event(Event::GeneticAnalystDeleted(genetic_analyst, who.clone()));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::GeneticAnalystWeightInfo::update_genetic_analyst_verification_status())]
		pub fn update_genetic_analyst_verification_status(
			origin: OriginFor<T>,
			account_id: T::AccountId,
			status: VerificationStatus,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as GeneticAnalystInterface<T>>::update_genetic_analyst_verification_status(
				&who,
				&account_id,
				&status,
			) {
				Ok(genetic_analyst) => {
					Self::deposit_event(Event::GeneticAnalystUpdateVerificationStatus(
						genetic_analyst,
						who.clone(),
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::GeneticAnalystWeightInfo::update_genetic_analyst_availability_status())]
		pub fn update_genetic_analyst_availability_status(
			origin: OriginFor<T>,
			status: AvailabilityStatus,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as GeneticAnalystInterface<T>>::update_genetic_analyst_availability_status(
				&who, &status,
			) {
				Ok(genetic_analyst) => {
					Self::deposit_event(Event::GeneticAnalystUpdateAvailabilityStatus(
						genetic_analyst,
						who.clone(),
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::GeneticAnalystWeightInfo::stake_genetic_analyst())]
		pub fn stake_genetic_analyst(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as GeneticAnalystInterface<T>>::stake_genetic_analyst(&who) {
				Ok(genetic_analyst) => {
					Self::deposit_event(Event::GeneticAnalystStakeSuccessful(
						genetic_analyst,
						who.clone(),
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::GeneticAnalystWeightInfo::unstake_genetic_analyst())]
		pub fn unstake_genetic_analyst(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as GeneticAnalystInterface<T>>::unstake_genetic_analyst(&who) {
				Ok(genetic_analyst) => {
					Self::deposit_event(Event::GeneticAnalystUnstakeSuccessful(
						genetic_analyst,
						who.clone(),
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::GeneticAnalystWeightInfo::retrieve_unstake_amount())]
		pub fn retrieve_unstake_amount(
			origin: OriginFor<T>,
			account_id: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as GeneticAnalystInterface<T>>::retrieve_unstake_amount(&who, &account_id) {
				Ok(genetic_analyst) => {
					Self::deposit_event(Event::GeneticAnalystRetrieveUnstakeAmount(
						genetic_analyst,
						who.clone(),
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::GeneticAnalystWeightInfo::update_minimum_stake_amount())]
		pub fn update_minimum_stake_amount(
			origin: OriginFor<T>,
			amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as GeneticAnalystInterface<T>>::update_minimum_stake_amount(&who, amount) {
				Ok(_) => {
					Self::deposit_event(Event::UpdateGeneticAnalystMinimumStakeSuccessful(
						who.clone(),
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::GeneticAnalystWeightInfo::update_unstake_time())]
		pub fn update_unstake_time(
			origin: OriginFor<T>,
			amount: MomentOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as GeneticAnalystInterface<T>>::update_unstake_time(&who, amount) {
				Ok(_) => {
					Self::deposit_event(Event::UpdateGeneticAnalystUnstakeTimeSuccessful(
						who.clone(),
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::GeneticAnalystWeightInfo::update_admin_key())]
		pub fn update_admin_key(
			origin: OriginFor<T>,
			account_id: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as GeneticAnalystInterface<T>>::update_admin_key(&who, &account_id) {
				Ok(_) => {
					Self::deposit_event(Event::UpdateGeneticAnalystAdminKeySuccessful(who.clone()));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(0)]
		pub fn sudo_update_admin_key(
			origin: OriginFor<T>,
			account_id: T::AccountId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			GeneticAnalystVerifierKey::<T>::put(&account_id);

			Self::deposit_event(Event::UpdateGeneticAnalystAdminKeySuccessful(account_id));

			Ok(Pays::No.into())
		}
	}
}

impl<T: Config> GeneticAnalystInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type Moment = MomentOf<T>;
	type Balance = BalanceOf<T>;
	type GeneticAnalystInfo = GeneticAnalystInfo<HashOf<T>, MomentOf<T>>;
	type GeneticAnalyst = GeneticAnalystOf<T>;
	type VerificationStatus = VerificationStatus;
	type AvailabilityStatus = AvailabilityStatus;

	fn create_genetic_analyst(
		account_id: &T::AccountId,
		genetic_analyst_info: &Self::GeneticAnalystInfo,
	) -> Result<Self::GeneticAnalyst, Self::Error> {
		if GeneticAnalysts::<T>::contains_key(account_id) {
			return Err(Error::<T>::GeneticAnalystAlreadyRegistered)
		}
		let genetic_analyst = GeneticAnalyst::new(account_id.clone(), genetic_analyst_info.clone());
		// Insert to Storage
		GeneticAnalysts::<T>::insert(account_id, &genetic_analyst);

		// Increment Count
		Self::add_genetic_analyst_count();

		Ok(genetic_analyst)
	}

	fn update_genetic_analyst(
		account_id: &T::AccountId,
		genetic_analyst_info: &Self::GeneticAnalystInfo,
	) -> Result<Self::GeneticAnalyst, Self::Error> {
		let genetic_analyst = GeneticAnalysts::<T>::get(account_id);
		if genetic_analyst == None {
			return Err(Error::<T>::GeneticAnalystDoesNotExist)
		}
		let mut genetic_analyst = genetic_analyst.unwrap();

		genetic_analyst.update_info(genetic_analyst_info.clone());

		GeneticAnalysts::<T>::insert(account_id, &genetic_analyst);

		Ok(genetic_analyst)
	}

	fn update_genetic_analyst_verification_status(
		genetic_analyst_verifier_key: &T::AccountId,
		account_id: &T::AccountId,
		status: &Self::VerificationStatus,
	) -> Result<Self::GeneticAnalyst, Self::Error> {
		if genetic_analyst_verifier_key.clone() != GeneticAnalystVerifierKey::<T>::get() {
			return Err(Error::<T>::Unauthorized)
		}

		let genetic_analyst = GeneticAnalysts::<T>::get(account_id);
		if genetic_analyst == None {
			return Err(Error::<T>::GeneticAnalystDoesNotExist)
		}

		let mut genetic_analyst = genetic_analyst.unwrap();
		genetic_analyst.verification_status = status.clone();

		if !genetic_analyst.stake_status.is_staked() {
			return Err(Error::<T>::GeneticAnalystIsNotStaked)
		}

		if status.is_rejected() {
			if T::GeneticAnalysisOrders::is_pending_genetic_analysis_order_by_seller_exist(
				account_id,
			) {
				return Err(Error::<T>::GeneticAnalystHasPendingOrders)
			}

			if !Self::is_pallet_balance_sufficient_for_refund(genetic_analyst.stake_amount) {
				return Err(Error::<T>::InsufficientPalletFunds)
			}

			let _ = Self::transfer_balance(
				&Self::account_id(),
				account_id,
				genetic_analyst.stake_amount,
			);

			genetic_analyst.stake_amount = 0u128.saturated_into();
			genetic_analyst.stake_status = StakeStatus::Unstaked;
			genetic_analyst.availability_status = AvailabilityStatus::Unavailable;
		}

		GeneticAnalysts::<T>::insert(account_id, &genetic_analyst);

		Ok(genetic_analyst)
	}

	fn update_genetic_analyst_availability_status(
		account_id: &T::AccountId,
		status: &Self::AvailabilityStatus,
	) -> Result<Self::GeneticAnalyst, Self::Error> {
		let genetic_analyst = GeneticAnalysts::<T>::get(account_id);
		if genetic_analyst == None {
			return Err(Error::<T>::GeneticAnalystDoesNotExist)
		}

		if !status.is_available() &&
			T::GeneticAnalysisOrders::is_pending_genetic_analysis_order_by_seller_exist(
				account_id,
			) {
			return Err(Error::<T>::GeneticAnalystHasPendingOrders)
		}

		let mut genetic_analyst = genetic_analyst.unwrap();
		genetic_analyst.availability_status = status.clone();

		GeneticAnalysts::<T>::insert(account_id, &genetic_analyst);

		Ok(genetic_analyst)
	}

	fn stake_genetic_analyst(
		account_id: &T::AccountId,
	) -> Result<Self::GeneticAnalyst, Self::Error> {
		let genetic_analyst = GeneticAnalysts::<T>::get(account_id);
		if genetic_analyst == None {
			return Err(Error::<T>::GeneticAnalystDoesNotExist)
		}

		let mut genetic_analyst = genetic_analyst.unwrap();
		if genetic_analyst.stake_status.is_staked() {
			return Err(Error::<T>::GeneticAnalystAlreadyStaked)
		}

		if !genetic_analyst.stake_status.is_waiting_for_unstaked() {
			if !Self::is_balance_sufficient_for_staking(account_id) {
				return Err(Error::<T>::InsufficientFunds)
			}

			genetic_analyst.stake_amount = Self::transfer_balance(
				account_id,
				&Self::account_id(),
				Self::get_required_stake_balance(),
			);
		}
		genetic_analyst.stake_status = StakeStatus::Staked;
		genetic_analyst.unstake_at = MomentOf::<T>::default();
		genetic_analyst.retrieve_unstake_at = MomentOf::<T>::default();

		GeneticAnalysts::<T>::insert(account_id, &genetic_analyst);

		Ok(genetic_analyst)
	}

	fn unstake_genetic_analyst(
		account_id: &T::AccountId,
	) -> Result<Self::GeneticAnalyst, Self::Error> {
		let genetic_analyst = GeneticAnalysts::<T>::get(account_id);
		if genetic_analyst == None {
			return Err(Error::<T>::GeneticAnalystDoesNotExist)
		}

		let mut genetic_analyst = genetic_analyst.unwrap();
		if !genetic_analyst.stake_status.is_staked() {
			return Err(Error::<T>::GeneticAnalystIsNotStaked)
		}

		if T::GeneticAnalysisOrders::is_pending_genetic_analysis_order_by_seller_exist(account_id) {
			return Err(Error::<T>::GeneticAnalystHasPendingOrders)
		}

		let now = pallet_timestamp::Pallet::<T>::get();
		genetic_analyst.stake_status = StakeStatus::WaitingForUnstaked;
		genetic_analyst.availability_status = AvailabilityStatus::Unavailable;
		genetic_analyst.unstake_at = now;
		genetic_analyst.retrieve_unstake_at = Self::get_unstake_time(now);

		GeneticAnalysts::<T>::insert(account_id, &genetic_analyst);

		Ok(genetic_analyst)
	}

	fn retrieve_unstake_amount(
		admin_key: &T::AccountId,
		account_id: &T::AccountId,
	) -> Result<Self::GeneticAnalyst, Self::Error> {
		if admin_key.clone() != GeneticAnalystVerifierKey::<T>::get() {
			return Err(Error::<T>::Unauthorized)
		}

		let genetic_analyst = GeneticAnalysts::<T>::get(account_id);
		if genetic_analyst == None {
			return Err(Error::<T>::GeneticAnalystDoesNotExist)
		}

		let mut genetic_analyst = genetic_analyst.unwrap();
		if !genetic_analyst.stake_status.is_waiting_for_unstaked() {
			return Err(Error::<T>::GeneticAnalystIsNotWaitingForUnstake)
		}

		if !Self::check_if_unstake_time(genetic_analyst.retrieve_unstake_at) {
			return Err(Error::<T>::GeneticAnalystCannotUnstakeBeforeUnstakeTime)
		}

		if !Self::is_pallet_balance_sufficient_for_refund(genetic_analyst.stake_amount) {
			return Err(Error::<T>::InsufficientPalletFunds)
		}

		let _ =
			Self::transfer_balance(&Self::account_id(), account_id, genetic_analyst.stake_amount);

		genetic_analyst.stake_amount = 0u128.saturated_into();
		genetic_analyst.stake_status = StakeStatus::Unstaked;
		genetic_analyst.unstake_at = MomentOf::<T>::default();
		genetic_analyst.retrieve_unstake_at = MomentOf::<T>::default();

		GeneticAnalysts::<T>::insert(account_id, &genetic_analyst);

		Ok(genetic_analyst)
	}

	fn delete_genetic_analyst(
		account_id: &T::AccountId,
	) -> Result<Self::GeneticAnalyst, Self::Error> {
		let genetic_analyst = GeneticAnalysts::<T>::get(account_id);
		if genetic_analyst == None {
			return Err(Error::<T>::GeneticAnalystDoesNotExist)
		}
		let genetic_analyst = genetic_analyst.unwrap();
		// Delete genetic_analyst's qualifications
		for qualification_id in &genetic_analyst.qualifications {
			let _result =
				T::GeneticAnalystQualifications::delete_qualification(account_id, qualification_id);
		}
		GeneticAnalysts::<T>::remove(&genetic_analyst.account_id);
		Self::sub_genetic_analyst_count();

		Ok(genetic_analyst)
	}

	fn update_minimum_stake_amount(
		account_id: &T::AccountId,
		amount: Self::Balance,
	) -> Result<(), Self::Error> {
		if account_id.clone() != GeneticAnalystVerifierKey::<T>::get() {
			return Err(Error::<T>::Unauthorized)
		}

		Self::set_minimum_stake_amount(amount);

		Ok(())
	}

	fn update_unstake_time(
		account_id: &T::AccountId,
		moment: Self::Moment,
	) -> Result<(), Self::Error> {
		if account_id.clone() != GeneticAnalystVerifierKey::<T>::get() {
			return Err(Error::<T>::Unauthorized)
		}

		Self::set_unstake_time(moment);

		Ok(())
	}

	fn update_admin_key(
		account_id: &T::AccountId,
		admin_key: &T::AccountId,
	) -> Result<(), Self::Error> {
		if account_id.clone() != GeneticAnalystVerifierKey::<T>::get() {
			return Err(Error::<T>::Unauthorized)
		}

		GeneticAnalystVerifierKey::<T>::put(admin_key);

		Ok(())
	}

	fn genetic_analyst_by_account_id(account_id: &T::AccountId) -> Option<Self::GeneticAnalyst> {
		Self::genetic_analyst_by_account_id(account_id)
	}
}

use frame_support::{sp_runtime::SaturatedConversion, traits::ExistenceRequirement::KeepAlive};

impl<T: Config> Pallet<T> {
	// Add genetic_analyst count
	pub fn add_genetic_analyst_count() {
		let genetic_analyst_count = <GeneticAnalystCount<T>>::get().unwrap_or(0);
		<GeneticAnalystCount<T>>::put(genetic_analyst_count.wrapping_add(1));
	}

	// Subtract genetic_analyst count
	pub fn sub_genetic_analyst_count() {
		let genetic_analyst_count = <GeneticAnalystCount<T>>::get().unwrap_or(1);
		GeneticAnalystCount::<T>::put(genetic_analyst_count - 1);
	}

	/// The injected pallet ID
	pub fn get_pallet_id() -> AccountIdOf<T> {
		T::PalletId::get().into_account()
	}

	/// The account ID that holds the funds
	pub fn account_id() -> AccountIdOf<T> {
		<PalletAccount<T>>::get()
	}

	pub fn get_balance_by_account_id(account_id: &AccountIdOf<T>) -> BalanceOf<T> {
		T::Currency::free_balance(account_id)
	}

	pub fn get_required_stake_balance() -> BalanceOf<T> {
		<MinimumStakeAmount<T>>::get()
			.unwrap_or_else(|| 50000000000000000000000u128.saturated_into())
	}

	/// Is the balance sufficient for staking
	pub fn is_balance_sufficient_for_staking(account_id: &AccountIdOf<T>) -> bool {
		let balance = T::Currency::free_balance(account_id);
		balance >= Self::get_required_stake_balance()
	}

	/// Transfer balance
	pub fn transfer_balance(
		source: &AccountIdOf<T>,
		dest: &AccountIdOf<T>,
		amount: BalanceOf<T>,
	) -> BalanceOf<T> {
		let _ = T::Currency::transfer(source, dest, amount, KeepAlive);
		Self::set_total_staked_amount();
		amount
	}

	/// Is the pallet balance sufficient for refund
	pub fn is_pallet_balance_sufficient_for_refund(refund_amount: BalanceOf<T>) -> bool {
		let balance = T::Currency::free_balance(&Self::account_id());
		balance >= refund_amount
	}

	/// Set current total stake amount
	pub fn set_minimum_stake_amount(amount: BalanceOf<T>) {
		MinimumStakeAmount::<T>::put(amount);
	}

	/// Set unstake time
	pub fn set_unstake_time(moment: MomentOf<T>) {
		UnstakeTime::<T>::put(moment);
	}

	/// Get unstake time
	pub fn get_unstake_time(moment: MomentOf<T>) -> MomentOf<T> {
		if let Some(time) = UnstakeTime::<T>::get() {
			return time + moment
		}
		moment
	}

	/// Check unstake time
	pub fn check_if_unstake_time(moment: MomentOf<T>) -> bool {
		let now = pallet_timestamp::Pallet::<T>::get();
		moment <= now
	}

	/// Set current total staked amount
	pub fn set_total_staked_amount() {
		let balance = T::Currency::free_balance(&Self::account_id());
		TotalStakedAmount::<T>::put(balance);
	}
}

impl<T: Config> GeneticAnalystServiceOwner<T> for Pallet<T> {
	type Owner = GeneticAnalyst<T::AccountId, T::Hash, MomentOf<T>, BalanceOf<T>>;

	/// User can create genetic_analyst_service if he/she is a genetic_analyst and has set ethereum address
	fn can_create_genetic_analyst_service(user_id: &T::AccountId) -> bool {
		GeneticAnalysts::<T>::contains_key(user_id)
	}

	fn get_owner(id: &T::AccountId) -> Option<Self::Owner> {
		GeneticAnalysts::<T>::get(id)
	}

	fn associate(owner_id: &T::AccountId, genetic_analyst_service_id: &T::Hash) {
		<GeneticAnalysts<T>>::mutate(owner_id, |genetic_analyst| {
			match genetic_analyst {
				None => (), // If genetic_analyst does not exist, do nothing
				Some(genetic_analyst) => {
					genetic_analyst.add_service(*genetic_analyst_service_id);
				},
			}
		});
	}

	fn disassociate(owner_id: &T::AccountId, genetic_analyst_service_id: &T::Hash) {
		GeneticAnalysts::<T>::mutate(owner_id, |genetic_analyst| match genetic_analyst {
			None => (),
			Some(genetic_analyst) => {
				genetic_analyst.remove_service(*genetic_analyst_service_id);
			},
		});
	}
}

impl<T: Config> GeneticAnalystQualificationOwner<T> for Pallet<T> {
	type Owner = GeneticAnalyst<T::AccountId, T::Hash, MomentOf<T>, BalanceOf<T>>;

	/// User can create qualification if he/she is a genetic_analyst
	fn can_create_qualification(user_id: &T::AccountId) -> bool {
		GeneticAnalysts::<T>::contains_key(user_id)
	}

	fn get_owner(id: &T::AccountId) -> Option<Self::Owner> {
		GeneticAnalysts::<T>::get(id)
	}

	fn associate(owner_id: &T::AccountId, qualification_id: &T::Hash) {
		<GeneticAnalysts<T>>::mutate(owner_id, |genetic_analyst| {
			match genetic_analyst {
				None => (), // If genetic_analyst does not exist, do nothing
				Some(genetic_analyst) => {
					genetic_analyst.add_qualification(*qualification_id);
				},
			}
		});
	}

	fn disassociate(owner_id: &T::AccountId, qualification_id: &T::Hash) {
		GeneticAnalysts::<T>::mutate(owner_id, |genetic_analyst| match genetic_analyst {
			None => (),
			Some(genetic_analyst) => {
				genetic_analyst.remove_qualification(*qualification_id);
			},
		});
	}
}

/// GeneticAnalystsProvider Trait Implementation
impl<T: Config> GeneticAnalystsProvider<T> for Pallet<T> {
	fn is_genetic_analyst_available(id: &T::AccountId) -> Option<bool> {
		let genetic_analyst =
			<Self as GeneticAnalystInterface<T>>::genetic_analyst_by_account_id(id);
		genetic_analyst.map(|genetic_analyst| genetic_analyst.is_available())
	}
}
