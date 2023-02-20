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

pub use crate::interface::LabInterface;

use frame_support::{
	pallet_prelude::*,
	sp_runtime::{traits::AccountIdConversion, RuntimeDebug, SaturatedConversion},
	traits::{Currency, ExistenceRequirement, StorageVersion},
	PalletId,
};
use primitives_area_code::{CityCode, CountryCode, CountryRegionCode, RegionCode};
use primitives_stake_status::{StakeStatus, StakeStatusTrait};
use primitives_verification_status::VerificationStatus;

use traits_certifications::CertificationOwnerInfo;
use traits_labs::LabsProvider;
use traits_order::{OrderEventEmitter, OrderStatusUpdater};
use traits_services::ServiceOwnerInfo;
use traits_user_profile::UserProfileProvider;

// LabInfo Struct
// Used as parameter of dispatchable calls
// Until update Rust compiler, clippy will have false positives: https://github.com/rust-lang/rust-clippy/issues/8867
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct LabInfo<Hash>
where
	Hash: PartialEq + Eq,
{
	pub box_public_key: Hash,
	pub name: Vec<u8>,
	pub email: Vec<u8>,
	pub country: CountryCode,
	pub region: RegionCode,
	pub city: CityCode,
	pub address: Vec<u8>,
	pub phone_number: Vec<u8>,
	pub website: Vec<u8>,
	pub latitude: Option<Vec<u8>>,
	pub longitude: Option<Vec<u8>>,
	pub profile_image: Option<Vec<u8>>,
}

// Lab Struct
// the fields (excluding account_id and services) come from LabInfo struct
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct Lab<AccountId, Hash, Moment, Balance>
where
	Hash: PartialEq + Eq,
{
	pub account_id: AccountId,
	pub services: Vec<Hash>,
	pub certifications: Vec<Hash>,
	pub verification_status: VerificationStatus,
	pub info: LabInfo<Hash>,
	pub stake_amount: Balance,
	pub stake_status: StakeStatus,
	pub unstake_at: Moment,
	pub retrieve_unstake_at: Moment,
}

impl<AccountId, Hash, Moment: Default, Balance: Default> Lab<AccountId, Hash, Moment, Balance>
where
	Hash: PartialEq + Eq,
{
	pub fn new(account_id: AccountId, info: LabInfo<Hash>) -> Self {
		Self {
			account_id,
			services: Vec::<Hash>::new(),
			certifications: Vec::<Hash>::new(),
			verification_status: VerificationStatus::default(),
			info,
			unstake_at: Moment::default(),
			retrieve_unstake_at: Moment::default(),
			stake_amount: Balance::default(),
			stake_status: StakeStatus::default(),
		}
	}

	fn update_info(&mut self, info: LabInfo<Hash>) {
		self.info = info;
	}

	fn get_country(&self) -> &CountryCode {
		&self.info.country
	}

	fn get_region(&self) -> &RegionCode {
		&self.info.region
	}

	fn get_city(&self) -> &CityCode {
		&self.info.city
	}

	// Returns CountryCode-RegionCode -> XX-YYY
	fn get_country_region(&self) -> CountryRegionCode {
		CountryRegionCode::build_country_region_code(self.get_country(), self.get_region())
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

	pub fn add_certification(&mut self, certification_id: Hash) {
		self.certifications.push(certification_id);
	}

	pub fn remove_certification(&mut self, certification_id: Hash) {
		if let Some(pos) = &self.certifications.iter().position(|x| *x == certification_id) {
			self.certifications.remove(*pos);
		}
	}
}

impl<T, AccountId, Hash, Moment: Default, Balance: Default> ServiceOwnerInfo<T>
	for Lab<AccountId, Hash, Moment, Balance>
where
	Hash: PartialEq + Eq,
	T: frame_system::Config<AccountId = AccountId>,
{
	fn get_id(&self) -> &AccountId {
		self.get_account_id()
	}
}

impl<T, AccountId, Hash, Moment: Default, Balance: Default> CertificationOwnerInfo<T>
	for Lab<AccountId, Hash, Moment, Balance>
where
	Hash: PartialEq + Eq,
	T: frame_system::Config<AccountId = AccountId>,
{
	fn get_owner_id(&self) -> &AccountId {
		self.get_account_id()
	}
}

/// The current storage version.
const STORAGE_VERSION: StorageVersion = StorageVersion::new(3);

#[frame_support::pallet]
pub mod pallet {
	use crate::{interface::LabInterface, Lab, LabInfo, *};
	use codec::EncodeLike;
	use frame_support::{dispatch::DispatchResultWithPostInfo, traits::Currency};
	use frame_system::pallet_prelude::*;
	pub use sp_std::prelude::*;
	pub use traits_certifications::{CertificationOwner, CertificationsProvider};
	pub use traits_services::{ServiceOwner, ServicesProvider};

	#[pallet::config]
	/// Configure the pallet by specifying the parameters and types on which it depends.
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Currency: Currency<Self::AccountId>;
		type Services: ServicesProvider<Self, BalanceOf<Self>>;
		type Orders: OrderEventEmitter<Self> + OrderStatusUpdater<Self>;
		type Certifications: CertificationsProvider<Self>;
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
		type ProfileRoles: Clone
			+ Copy
			+ PartialEq
			+ Eq
			+ Encode
			+ EncodeLike
			+ Decode
			+ Default
			+ TypeInfo
			+ sp_std::fmt::Debug;
		type UserProfile: UserProfileProvider<Self, Self::EthereumAddress, Self::ProfileRoles>;
		type LabWeightInfo: WeightInfo;
		/// Currency type for this pallet.
		#[pallet::constant]
		type PalletId: Get<PalletId>;
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

	// ---- Types ----------------------
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type HashOf<T> = <T as frame_system::Config>::Hash;
	pub type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;
	pub type LabOf<T> = Lab<AccountIdOf<T>, HashOf<T>, MomentOf<T>, BalanceOf<T>>;
	pub type CurrencyOf<T> = <T as self::Config>::Currency;
	pub type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;
	pub type UserProfileOf<T> = <T as self::Config>::UserProfile;

	// ----- Storage ------------------
	/// Get Lab by account id
	/// AccountId => Lab
	#[pallet::storage]
	#[pallet::getter(fn lab_by_account_id)]
	pub type Labs<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, LabOf<T>>;

	/// Get LabId by Country-Region, City
	/// (CountryRegionCode, CityCode) => Vec<AccountId>
	#[pallet::storage]
	#[pallet::getter(fn labs_by_country_region_city)]
	pub type LabsByCountryRegionCity<T> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		CountryRegionCode,
		Blake2_128Concat,
		CityCode,
		Vec<AccountIdOf<T>>,
	>;

	/// Get total lab count
	/// u32
	#[pallet::storage]
	#[pallet::getter(fn lab_count)]
	pub type LabCount<T> = StorageValue<_, u64>;

	/// Get total lab count by Country-Region, City
	/// (CountryRegionCode, CityCode) => u32
	#[pallet::storage]
	#[pallet::getter(fn lab_count_by_country_region_city)]
	pub type LabCountByCountryRegionCity<T> =
		StorageDoubleMap<_, Blake2_128Concat, CountryRegionCode, Blake2_128Concat, CityCode, u64>;

	#[pallet::storage]
	#[pallet::getter(fn admin_key)]
	pub type LabVerifierKey<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn pallet_id)]
	pub type PalletAccount<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

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
		pub lab_verifier_key: Option<T::AccountId>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { lab_verifier_key: None }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			if let Some(ref lab_verifier_key) = self.lab_verifier_key {
				LabVerifierKey::<T>::put(lab_verifier_key);
			}
			UnstakeTime::<T>::put(MomentOf::<T>::default());
			PalletAccount::<T>::put(<Pallet<T>>::get_pallet_id());
			<Pallet<T>>::set_minimum_stake_amount(50000000000000000000000u128.saturated_into());
			<Pallet<T>>::set_total_staked_amount();
		}
	}
	// ----------------------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// User AccountId registered as lab
		/// parameters. [Lab, who]
		LabRegistered(LabOf<T>, AccountIdOf<T>),
		/// Lab information updated
		/// parameters. [Lab, who]
		LabUpdated(LabOf<T>, AccountIdOf<T>),
		/// Lab verification updated
		/// parameters. [Lab, who]
		LabUpdateVerificationStatus(LabOf<T>, AccountIdOf<T>),
		/// Lab deregistered
		/// parameters. [Lab, who]
		LabDeregistered(LabOf<T>, AccountIdOf<T>),
		/// Lab stake successful
		/// parameters. [Lab, who]
		LabStakeSuccessful(LabOf<T>, AccountIdOf<T>),
		/// Lab unstake successful
		/// parameters. [Lab, who]
		LabUnstakeSuccessful(LabOf<T>, AccountIdOf<T>),
		/// Lab retrive unstake amount
		/// parameters. [Lab, who]
		LabRetrieveUnstakeAmount(LabOf<T>, AccountIdOf<T>),
		/// Update Lab minimum stake successful
		/// parameters. [who]
		UpdateLabMinimumStakeSuccessful(AccountIdOf<T>),
		/// Update Lab unstake time successful
		/// parameters. [who]
		UpdateLabUnstakeTimeSuccessful(AccountIdOf<T>),
		/// Update Lab admin key
		/// parameters. [who]
		UpdateLabAdminKeySuccessful(AccountIdOf<T>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Account already has lab registered
		LabAlreadyRegistered,
		/// Lab identified by the AccountId does not exist
		LabDoesNotExist,
		/// Lab is not the owner of the service
		LabIsNotOwner,
		/// Account already has genetic_analyst staked
		LabAlreadyStaked,
		/// Insufficient funds
		InsufficientFunds,
		/// Insufficient pallet funds
		InsufficientPalletFunds,
		/// Account has not staked
		LabIsNotStaked,
		/// Unauthorized access to extrinsic
		Unauthorized,
		// Lab has pending orders
		LabHasPendingOrders,
		// Lab not waiting for unstake
		LabIsNotWaitingForUnstake,
		// Lab cannot unstake now
		LabCannotUnstakeBeforeUnstakeTime,
		// Dispatch Errors
		Module,
		Other,
		BadOrigin,
		CannotLookup,
		TooManyConsumers,
		ConsumerRemaining,
		NoProviders,
		Token,
		Arithmetic,
		// Setting profile role failed,
		FailedToSetProfileRole,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::LabWeightInfo::register_lab())]
		pub fn register_lab(
			origin: OriginFor<T>,
			lab_info: LabInfo<HashOf<T>>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match Self::create_lab(&who, &lab_info) {
				Ok(lab) => match UserProfileOf::<T>::set_account_profile_role_to_lab(&who) {
					Ok(_) => {
						Self::deposit_event(Event::LabRegistered(lab, who.clone()));
						Ok(().into())
					},
					Err(_) => Err(Error::<T>::FailedToSetProfileRole.into()),
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::LabWeightInfo::update_lab())]
		pub fn update_lab(
			origin: OriginFor<T>,
			lab_info: LabInfo<HashOf<T>>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as LabInterface<T>>::update_lab(&who, &lab_info) {
				Ok(lab) => {
					Self::deposit_event(Event::LabUpdated(lab, who.clone()));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::LabWeightInfo::update_lab_verification_status())]
		pub fn update_lab_verification_status(
			origin: OriginFor<T>,
			account_id: T::AccountId,
			lab_verification_status: VerificationStatus,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as LabInterface<T>>::update_lab_verification_status(
				&who,
				&account_id,
				&lab_verification_status,
			) {
				Ok(lab) => {
					Self::deposit_event(Event::LabUpdateVerificationStatus(lab, who.clone()));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::LabWeightInfo::deregister_lab())]
		pub fn deregister_lab(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			// Check if user is a lab
			let lab = Self::lab_by_account_id(&who);
			if lab.is_none() {
				return Err(Error::<T>::LabDoesNotExist.into())
			}

			match <Self as LabInterface<T>>::delete_lab(&who) {
				Ok(lab) => {
					Self::deposit_event(Event::LabDeregistered(lab, who.clone()));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::LabWeightInfo::stake_lab())]
		pub fn stake_lab(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as LabInterface<T>>::stake_lab(&who) {
				Ok(lab) => {
					Self::deposit_event(Event::LabStakeSuccessful(lab, who.clone()));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::LabWeightInfo::unstake_lab())]
		pub fn unstake_lab(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as LabInterface<T>>::unstake_lab(&who) {
				Ok(lab) => {
					Self::deposit_event(Event::LabUnstakeSuccessful(lab, who.clone()));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::LabWeightInfo::retrieve_unstake_amount())]
		pub fn retrieve_unstake_amount(
			origin: OriginFor<T>,
			account_id: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as LabInterface<T>>::retrieve_unstake_amount(&who, &account_id) {
				Ok(lab) => {
					Self::deposit_event(Event::LabRetrieveUnstakeAmount(lab, who.clone()));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::LabWeightInfo::update_minimum_stake_amount())]
		pub fn update_minimum_stake_amount(
			origin: OriginFor<T>,
			amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as LabInterface<T>>::update_minimum_stake_amount(&who, amount) {
				Ok(_) => {
					Self::deposit_event(Event::UpdateLabMinimumStakeSuccessful(who.clone()));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::LabWeightInfo::update_unstake_time())]
		pub fn update_unstake_time(
			origin: OriginFor<T>,
			amount: MomentOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as LabInterface<T>>::update_unstake_time(&who, amount) {
				Ok(_) => {
					Self::deposit_event(Event::UpdateLabUnstakeTimeSuccessful(who.clone()));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::LabWeightInfo::update_admin_key())]
		pub fn update_admin_key(
			origin: OriginFor<T>,
			account_id: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as LabInterface<T>>::update_admin_key(&who, &account_id) {
				Ok(_) => {
					Self::deposit_event(Event::UpdateLabAdminKeySuccessful(who.clone()));
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

			LabVerifierKey::<T>::put(&account_id);

			Self::deposit_event(Event::UpdateLabAdminKeySuccessful(account_id));

			Ok(Pays::No.into())
		}
	}
}

impl<T: Config> LabInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type Moment = MomentOf<T>;
	type Balance = BalanceOf<T>;
	type LabInfo = LabInfo<HashOf<T>>;
	type Lab = LabOf<T>;
	type VerificationStatus = VerificationStatus;

	fn create_lab(
		account_id: &T::AccountId,
		lab_info: &Self::LabInfo,
	) -> Result<Self::Lab, Self::Error> {
		if Labs::<T>::contains_key(account_id) {
			return Err(Error::<T>::LabAlreadyRegistered)
		}
		let lab = Lab::new(account_id.clone(), lab_info.clone());
		// Insert to Storage
		Labs::<T>::insert(account_id, &lab);
		Self::insert_lab_id_to_location(&lab);

		// Increment Count
		Self::add_lab_count();
		Self::add_lab_count_by_location(&lab);

		Ok(lab)
	}

	fn update_lab(
		account_id: &T::AccountId,
		lab_info: &Self::LabInfo,
	) -> Result<Self::Lab, Self::Error> {
		let lab = Labs::<T>::get(account_id);
		if lab.is_none() {
			return Err(Error::<T>::LabDoesNotExist)
		}
		let mut lab = lab.unwrap();
		let mut is_location_changed = false;

		// If location is updated, remove the lab from the old location
		// Also update service locations
		if lab.get_country() != &lab_info.country ||
			lab.get_region() != &lab_info.region ||
			lab.get_city() != &lab_info.city
		{
			// removed old location
			Self::remove_lab_id_from_location(&lab);
			Self::sub_lab_count_by_location(&lab);

			is_location_changed = true;
		}

		lab.update_info(lab_info.clone());

		if is_location_changed {
			// insert new location
			Self::insert_lab_id_to_location(&lab);
			Self::add_lab_count_by_location(&lab);
		}

		Labs::<T>::insert(account_id, &lab);

		Ok(lab)
	}

	fn update_lab_verification_status(
		lab_verifier_key: &T::AccountId,
		account_id: &T::AccountId,
		status: &Self::VerificationStatus,
	) -> Result<Self::Lab, Self::Error> {
		if lab_verifier_key.clone() != LabVerifierKey::<T>::get().unwrap() {
			return Err(Error::<T>::Unauthorized)
		}

		let lab = Labs::<T>::get(account_id);
		if lab.is_none() {
			return Err(Error::<T>::LabDoesNotExist)
		}
		let mut lab = lab.unwrap();
		lab.verification_status = status.clone();
		Labs::<T>::insert(account_id, &lab);
		Ok(lab)
	}

	fn delete_lab(account_id: &T::AccountId) -> Result<Self::Lab, Self::Error> {
		let lab = Labs::<T>::get(account_id);
		if lab.is_none() {
			return Err(Error::<T>::LabDoesNotExist)
		}
		let lab = lab.unwrap();
		// Delete lab's services
		for service_id in &lab.services {
			let _result = T::Services::delete_service(account_id, service_id);
		}
		// Delete lab's certifications
		for certification_id in &lab.certifications {
			let _result = T::Certifications::delete_certification(account_id, certification_id);
		}
		Self::remove_lab_id_from_location(&lab);
		Self::sub_lab_count_by_location(&lab);
		Labs::<T>::remove(&lab.account_id);
		Self::sub_lab_count();

		Ok(lab)
	}

	fn stake_lab(account_id: &T::AccountId) -> Result<Self::Lab, Self::Error> {
		let lab = Labs::<T>::get(account_id);
		if lab.is_none() {
			return Err(Error::<T>::LabDoesNotExist)
		}

		let mut lab = lab.unwrap();
		if lab.stake_status.is_staked() {
			return Err(Error::<T>::LabAlreadyStaked)
		}

		if !lab.stake_status.is_waiting_for_unstaked() {
			if !Self::is_balance_sufficient_for_staking(account_id) {
				return Err(Error::<T>::InsufficientFunds)
			}

			match CurrencyOf::<T>::transfer(
				account_id,
				&Self::account_id(),
				Self::get_required_stake_balance(),
				ExistenceRequirement::KeepAlive,
			) {
				Ok(_) => {
					lab.stake_amount = Self::get_required_stake_balance();
				},
				Err(dispatch) => match dispatch {
					sp_runtime::DispatchError::Other(_) => return Err(Error::<T>::Other),
					sp_runtime::DispatchError::CannotLookup => return Err(Error::<T>::CannotLookup),
					sp_runtime::DispatchError::BadOrigin => return Err(Error::<T>::BadOrigin),
					sp_runtime::DispatchError::TooManyConsumers =>
						return Err(Error::<T>::TooManyConsumers),
					sp_runtime::DispatchError::ConsumerRemaining =>
						return Err(Error::<T>::ConsumerRemaining),
					sp_runtime::DispatchError::NoProviders => return Err(Error::<T>::NoProviders),
					sp_runtime::DispatchError::Token(_) => return Err(Error::<T>::Token),
					sp_runtime::DispatchError::Arithmetic(_) => return Err(Error::<T>::Arithmetic),
					sp_runtime::DispatchError::Module(_) => return Err(Error::<T>::Arithmetic),
					sp_runtime::DispatchError::Transactional(_) =>
						return Err(Error::<T>::Arithmetic),
				},
			}
		}
		lab.stake_status = StakeStatus::Staked;
		lab.unstake_at = MomentOf::<T>::default();
		lab.retrieve_unstake_at = MomentOf::<T>::default();

		Labs::<T>::insert(account_id, &lab);

		Ok(lab)
	}

	fn unstake_lab(account_id: &T::AccountId) -> Result<Self::Lab, Self::Error> {
		let lab = Labs::<T>::get(account_id);
		if lab.is_none() {
			return Err(Error::<T>::LabDoesNotExist)
		}

		let mut lab = lab.unwrap();
		if !lab.stake_status.is_staked() {
			return Err(Error::<T>::LabIsNotStaked)
		}

		if T::Orders::is_pending_order_by_seller_exist(account_id) {
			return Err(Error::<T>::LabHasPendingOrders)
		}

		let now = pallet_timestamp::Pallet::<T>::get();
		lab.stake_status = StakeStatus::WaitingForUnstaked;
		lab.unstake_at = now;
		lab.retrieve_unstake_at = Self::get_unstake_time(now);

		Labs::<T>::insert(account_id, &lab);

		Ok(lab)
	}

	fn retrieve_unstake_amount(
		admin_key: &T::AccountId,
		account_id: &T::AccountId,
	) -> Result<Self::Lab, Self::Error> {
		if admin_key.clone() != LabVerifierKey::<T>::get().unwrap() {
			return Err(Error::<T>::Unauthorized)
		}

		let lab = Labs::<T>::get(account_id);
		if lab.is_none() {
			return Err(Error::<T>::LabDoesNotExist)
		}

		let mut lab = lab.unwrap();
		if !lab.stake_status.is_waiting_for_unstaked() {
			return Err(Error::<T>::LabIsNotWaitingForUnstake)
		}

		if !Self::check_if_unstake_time(lab.retrieve_unstake_at) {
			return Err(Error::<T>::LabCannotUnstakeBeforeUnstakeTime)
		}

		if !Self::is_pallet_balance_sufficient_for_refund(lab.stake_amount) {
			return Err(Error::<T>::InsufficientPalletFunds)
		}

		match CurrencyOf::<T>::transfer(
			&Self::account_id(),
			account_id,
			lab.stake_amount,
			ExistenceRequirement::AllowDeath,
		) {
			Ok(_) => (),
			Err(dispatch) => match dispatch {
				sp_runtime::DispatchError::Other(_) => return Err(Error::<T>::Other),
				sp_runtime::DispatchError::CannotLookup => return Err(Error::<T>::CannotLookup),
				sp_runtime::DispatchError::BadOrigin => return Err(Error::<T>::BadOrigin),
				sp_runtime::DispatchError::TooManyConsumers =>
					return Err(Error::<T>::TooManyConsumers),
				sp_runtime::DispatchError::ConsumerRemaining =>
					return Err(Error::<T>::ConsumerRemaining),
				sp_runtime::DispatchError::NoProviders => return Err(Error::<T>::NoProviders),
				sp_runtime::DispatchError::Token(_) => return Err(Error::<T>::Token),
				sp_runtime::DispatchError::Arithmetic(_) => return Err(Error::<T>::Arithmetic),
				sp_runtime::DispatchError::Module(_) => return Err(Error::<T>::Arithmetic),
				sp_runtime::DispatchError::Transactional(_) => return Err(Error::<T>::Arithmetic),
			},
		}

		lab.stake_amount = 0u128.saturated_into();
		lab.stake_status = StakeStatus::Unstaked;
		lab.unstake_at = MomentOf::<T>::default();
		lab.retrieve_unstake_at = MomentOf::<T>::default();

		Labs::<T>::insert(account_id, &lab);

		Ok(lab)
	}

	fn update_minimum_stake_amount(
		account_id: &T::AccountId,
		amount: Self::Balance,
	) -> Result<(), Self::Error> {
		if account_id.clone() != LabVerifierKey::<T>::get().unwrap() {
			return Err(Error::<T>::Unauthorized)
		}

		Self::set_minimum_stake_amount(amount);

		Ok(())
	}

	fn update_unstake_time(
		account_id: &T::AccountId,
		moment: Self::Moment,
	) -> Result<(), Self::Error> {
		if account_id.clone() != LabVerifierKey::<T>::get().unwrap() {
			return Err(Error::<T>::Unauthorized)
		}

		Self::set_unstake_time(moment);

		Ok(())
	}

	fn update_admin_key(
		account_id: &T::AccountId,
		admin_key: &T::AccountId,
	) -> Result<(), Self::Error> {
		if account_id.clone() != LabVerifierKey::<T>::get().unwrap() {
			return Err(Error::<T>::Unauthorized)
		}

		LabVerifierKey::<T>::put(admin_key);

		Ok(())
	}

	fn labs_by_country_region_city(
		country_region_code: &CountryRegionCode,
		city_code: &CityCode,
	) -> Option<Vec<T::AccountId>> {
		Self::labs_by_country_region_city(country_region_code, city_code)
	}

	fn lab_by_account_id(account_id: &T::AccountId) -> Option<Self::Lab> {
		Self::lab_by_account_id(account_id)
	}
}

impl<T: Config> Pallet<T> {
	pub fn insert_lab_id_to_location(lab: &LabOf<T>) {
		let country_region_code = lab.get_country_region();
		let city_code = lab.get_city();
		let lab_account_id = lab.get_account_id();

		match LabsByCountryRegionCity::<T>::get(&country_region_code, city_code) {
			None => {
				let labs = vec![lab_account_id.clone()];
				LabsByCountryRegionCity::<T>::insert(&country_region_code, city_code, labs);
			},
			Some(mut labs) => {
				labs.push(lab_account_id.clone());
				LabsByCountryRegionCity::<T>::insert(&country_region_code, city_code, labs);
			},
		}
	}

	pub fn remove_lab_id_from_location(lab: &LabOf<T>) {
		let country_region_code = lab.get_country_region();
		let city_code = lab.get_city();
		let lab_account_id = lab.get_account_id();

		// Get the lab_account_id list
		let mut labs_by_location =
			LabsByCountryRegionCity::<T>::get(&country_region_code, city_code).unwrap_or_default();
		// Remove id from the list
		labs_by_location.retain(|l_id| l_id != lab_account_id);
		//  Put back the list to storage
		LabsByCountryRegionCity::<T>::insert(&country_region_code, city_code, labs_by_location);
	}

	// Add lab count
	pub fn add_lab_count() {
		let lab_count = <LabCount<T>>::get().unwrap_or(0);
		<LabCount<T>>::put(lab_count.wrapping_add(1));
	}

	// Add lab count by location
	pub fn add_lab_count_by_location(lab: &LabOf<T>) {
		let country_region_code = lab.get_country_region();
		let city_code = lab.get_city();

		let lab_count =
			<LabCountByCountryRegionCity<T>>::get(country_region_code.clone(), city_code.clone())
				.unwrap_or(0);
		<LabCountByCountryRegionCity<T>>::insert(
			country_region_code,
			city_code.clone(),
			lab_count.wrapping_add(1),
		);
	}

	// Subtract lab count
	pub fn sub_lab_count() {
		let lab_count = <LabCount<T>>::get().unwrap_or(1);
		LabCount::<T>::put(lab_count - 1);
	}

	// Subtract lab count by location
	pub fn sub_lab_count_by_location(lab: &LabOf<T>) {
		let country_region_code = lab.get_country_region();
		let city_code = lab.get_city();

		let lab_count =
			LabCountByCountryRegionCity::<T>::get(country_region_code.clone(), city_code.clone())
				.unwrap_or(1);
		LabCountByCountryRegionCity::<T>::insert(
			country_region_code,
			city_code.clone(),
			lab_count - 1,
		);
	}

	/// The injected pallet ID
	pub fn get_pallet_id() -> AccountIdOf<T> {
		T::PalletId::get().into_account_truncating()
	}

	/// The account ID that holds the funds
	pub fn account_id() -> AccountIdOf<T> {
		<PalletAccount<T>>::get().unwrap()
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

	fn lab_verification_status(account_id: &T::AccountId) -> Option<VerificationStatus> {
		let lab = Self::lab_by_account_id(account_id)?;
		Some(lab.verification_status)
	}
}

impl<T: Config> ServiceOwner<T> for Pallet<T> {
	type Owner = Lab<T::AccountId, T::Hash, MomentOf<T>, BalanceOf<T>>;

	/// User can create service if he/she is a lab and has set ethereum address
	fn can_create_service(user_id: &T::AccountId) -> bool {
		Labs::<T>::contains_key(user_id)
	}

	fn get_owner(id: &T::AccountId) -> Option<Self::Owner> {
		Labs::<T>::get(id)
	}

	fn associate(owner_id: &T::AccountId, service_id: &T::Hash) {
		<Labs<T>>::mutate(owner_id, |lab| {
			match lab {
				None => (), // If lab does not exist, do nothing
				Some(lab) => {
					lab.add_service(*service_id);
				},
			}
		});
	}

	fn disassociate(owner_id: &T::AccountId, service_id: &T::Hash) {
		Labs::<T>::mutate(owner_id, |lab| match lab {
			None => (),
			Some(lab) => {
				lab.remove_service(*service_id);
			},
		});
	}
}

impl<T: Config> CertificationOwner<T> for Pallet<T> {
	type Owner = Lab<T::AccountId, T::Hash, MomentOf<T>, BalanceOf<T>>;

	/// User can create certification if he/she is a lab
	fn can_create_certification(user_id: &T::AccountId) -> bool {
		Labs::<T>::contains_key(user_id)
	}

	fn get_owner(id: &T::AccountId) -> Option<Self::Owner> {
		Labs::<T>::get(id)
	}

	fn associate(owner_id: &T::AccountId, certification_id: &T::Hash) {
		<Labs<T>>::mutate(owner_id, |lab| {
			match lab {
				None => (), // If lab does not exist, do nothing
				Some(lab) => {
					lab.add_certification(*certification_id);
				},
			}
		});
	}

	fn disassociate(owner_id: &T::AccountId, certification_id: &T::Hash) {
		Labs::<T>::mutate(owner_id, |lab| match lab {
			None => (),
			Some(lab) => {
				lab.remove_certification(*certification_id);
			},
		});
	}
}

impl<T: Config> LabsProvider<T> for Pallet<T> {
	fn lab_verification_status(account_id: &T::AccountId) -> Option<VerificationStatus> {
		Self::lab_verification_status(account_id)
	}

	fn is_lab_exists(account_id: &T::AccountId) -> bool {
		Self::lab_by_account_id(account_id).is_some()
	}
}
