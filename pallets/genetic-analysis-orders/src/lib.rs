#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod functions;
pub mod impl_genetic_analysis_orders;
pub mod interface;
pub mod migrations;
pub mod types;
pub mod weights;

use frame_support::{
	codec::{Decode, Encode},
	pallet_prelude::*,
	sp_runtime::{
		traits::{AccountIdConversion, Hash},
		RuntimeDebug, SaturatedConversion,
	},
	sp_std::convert::TryInto,
	traits::{Currency, StorageVersion},
	PalletId,
};
use primitives_price_and_currency::{CurrencyType, Price};
use primitives_tracking_id::TrackingId;
use sp_std::{prelude::*, vec};
use traits_genetic_analysis::{GeneticAnalysisProvider, GeneticAnalysisTracking};

use traits_genetic_analyst_services::{GeneticAnalystServiceInfo, GeneticAnalystServicesProvider};
use traits_genetic_analysts::GeneticAnalystsProvider;
use traits_genetic_data::{GeneticData, GeneticDataProvider};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use interface::GeneticAnalysisOrderInterface;
pub use types::*;
pub use weights::WeightInfo;

/// The current storage version.
const STORAGE_VERSION: StorageVersion = StorageVersion::new(3);

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use frame_support::{dispatch::DispatchResultWithPostInfo, traits::tokens::fungibles};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Assets: fungibles::InspectMetadata<
				<Self as frame_system::Config>::AccountId,
				AssetId = AssetId,
				Balance = AssetBalance,
			> + fungibles::Transfer<<Self as frame_system::Config>::AccountId>;
		type GeneticAnalysts: GeneticAnalystsProvider<Self>;
		type GeneticAnalystServices: GeneticAnalystServicesProvider<Self, BalanceOf<Self>>;
		type GeneticData: GeneticDataProvider<Self>;
		type GeneticAnalysis: GeneticAnalysisProvider<Self>;
		type Currency: Currency<<Self as frame_system::Config>::AccountId>;
		type GeneticAnalysisOrdersWeightInfo: WeightInfo;
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

	// ---- Types --------------------------------------------
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;
	pub type HashOf<T> = <T as frame_system::Config>::Hash;
	pub type CurrencyOf<T> = <T as self::Config>::Currency;
	pub type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;
	pub type GeneticAnalysisOrderOf<T> =
		GeneticAnalysisOrder<HashOf<T>, AccountIdOf<T>, BalanceOf<T>, MomentOf<T>>;
	type GeneticAnalysisOrderIdsOf<T> = Vec<HashOf<T>>;
	// -------------------------------------------------------

	// ------ Storage --------------------------
	#[pallet::storage]
	#[pallet::getter(fn genetic_analysis_order_by_id)]
	pub type GeneticAnalysisOrders<T> =
		StorageMap<_, Blake2_128Concat, HashOf<T>, GeneticAnalysisOrderOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn genetic_analysis_orders_by_customer_id)]
	pub type GeneticAnalysisOrdersByCustomer<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, GeneticAnalysisOrderIdsOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn genetic_analysis_orders_by_genetic_analyst_id)]
	pub type GeneticAnalysisOrdersBySeller<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, GeneticAnalysisOrderIdsOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn pending_genetic_analysis_orders_by_genetic_analyst_id)]
	pub type PendingGeneticAnalysisOrdersBySeller<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, GeneticAnalysisOrderIdsOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn last_genetic_analysis_order_by_customer_id)]
	pub type LastGeneticAnalysisOrderByCustomer<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, HashOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn admin_key)]
	pub type EscrowKey<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn treasury_key)]
	pub type TreasuryKey<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn pallet_id)]
	pub type PalletAccount<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn total_escrow_amount)]
	pub type TotalEscrowAmount<T> = StorageValue<_, BalanceOf<T>>;
	// -----------------------------------------

	// ----- Genesis Configs ------------------
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub escrow_key: Option<T::AccountId>,
		pub treasury_key: Option<T::AccountId>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { escrow_key: None, treasury_key: None }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			if let Some(ref escrow_key) = self.escrow_key {
				EscrowKey::<T>::put(escrow_key);
			}
			if let Some(ref treasury_key) = self.treasury_key {
				TreasuryKey::<T>::put(treasury_key);
			}
			PalletAccount::<T>::put(<Pallet<T>>::get_pallet_id());
		}
	}
	// ----------------------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// GeneticAnalysisOrder created
		/// parameters, [GeneticAnalysisOrder]
		GeneticAnalysisOrderCreated(GeneticAnalysisOrderOf<T>),
		/// GeneticAnalysisOrder paid
		/// parameters, [GeneticAnalysisOrder]
		GeneticAnalysisOrderPaid(GeneticAnalysisOrderOf<T>),
		/// GeneticAnalysisOrder Fulfilled
		/// parameters, [GeneticAnalysisOrder]
		GeneticAnalysisOrderFulfilled(GeneticAnalysisOrderOf<T>),
		/// GeneticAnalysisOrder Refunded
		/// parameters, [GeneticAnalysisOrder]
		GeneticAnalysisOrderRefunded(GeneticAnalysisOrderOf<T>),
		/// GeneticAnalysisOrder Cancelled
		/// parameters, [GeneticAnalysisOrder]
		GeneticAnalysisOrderCancelled(GeneticAnalysisOrderOf<T>),
		/// Update GeneticAnalysisOrder escrow key
		/// parameters. [who]
		UpdateGeneticAnalysisOrderEscrowKeySuccessful(AccountIdOf<T>),
		/// Update GeneticAnalysisOrder treasury key
		/// parameters. [who]
		UpdateGeneticAnalysisOrderTreasuryKeySuccessful(AccountIdOf<T>),
		/// GeneticAnalysisOrder Not Found
		/// parameters, []
		GeneticAnalysisOrderNotFound,
		/// GeneticAnalysisOrder Failed
		/// parameters, [GeneticAnalysisOrder]
		GeneticAnalysisOrderFailed(GeneticAnalysisOrderOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// GeneticData id does not exist
		GeneticDataDoesNotExist,
		/// Not owner of GeneticData
		NotOwnerOfGeneticData,
		/// GeneticAnalystService id does not exist
		GeneticAnalystServiceDoesNotExist,
		/// GeneticAnalysisOrder does not exist
		GeneticAnalysisOrderNotFound,
		/// Unauthorized to cancel genetic_analysis_order - user is not the customer who created
		/// the genetic_analysis_order
		UnauthorizedGeneticAnalysisOrderCancellation,
		// Genetic Analysis is ongoing, cannot be cancelled
		OngoingGeneticAnalysisOrderCannotBeCancelled,
		/// Can not fulfill genetic_analysis_order before Specimen is processed
		GeneticAnalysisNotSuccessfullyProcessed,
		/// Refund not allowed, GeneticAnalysisOrder is not expired yet
		GeneticAnalysisOrderNotYetExpired,
		/// Unauthorized Account
		Unauthorized,
		/// Insufficient funds
		InsufficientFunds,
		/// Error on creating DNA sample
		GeneticAnalysisInitalizationError,
		/// Customer eth address not found
		CustomerEthAddressNotFound,
		/// Seller eth address not found
		SellerEthAddressNotFound,
		/// GeneticAnalystService Price Index not found
		PriceIndexNotFound,
		// GeneticAnalyst is unavailable
		GeneticAnalystUnavailable,
		// Dispatch Errors
		Module,
		Other,
		BadOrigin,
		CannotLookup,
		ConsumerRemaining,
		TooManyConsumers,
		NoProviders,
		Token,
		Arithmetic,
		WrongAssetIdFormat,
		AssetIdNotFound,
		GeneticAnalysisOrderCannotBePaid,
		GeneticAnalysisOrderCannotBeCancelled,
		GeneticAnalysisOrderCannotBeFulfilled,
		GeneticAnalysisOrderCannotBeRefunded,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::GeneticAnalysisOrdersWeightInfo::create_genetic_analysis_order())]
		pub fn create_genetic_analysis_order(
			origin: OriginFor<T>,
			genetic_data_id: T::Hash,
			service_id: T::Hash,
			price_index: u32,
			customer_box_public_key: T::Hash,
			genetic_link: Vec<u8>,
			asset_id: Option<u32>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as GeneticAnalysisOrderInterface<T>>::create_genetic_analysis_order(
				&who,
				&genetic_data_id,
				&service_id,
				price_index,
				&customer_box_public_key,
				&genetic_link,
				asset_id,
			) {
				Ok(genetic_analysis_order) => {
					Self::deposit_event(Event::<T>::GeneticAnalysisOrderCreated(
						genetic_analysis_order,
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::GeneticAnalysisOrdersWeightInfo::cancel_genetic_analysis_order())]
		pub fn cancel_genetic_analysis_order(
			origin: OriginFor<T>,
			genetic_analysis_order_id: T::Hash,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as GeneticAnalysisOrderInterface<T>>::cancel_genetic_analysis_order(
				&who,
				&genetic_analysis_order_id,
			) {
				Ok(genetic_analysis_order) => {
					Self::deposit_event(Event::<T>::GeneticAnalysisOrderCancelled(
						genetic_analysis_order,
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::GeneticAnalysisOrdersWeightInfo::set_genetic_analysis_order_paid())]
		pub fn set_genetic_analysis_order_paid(
			origin: OriginFor<T>,
			genetic_analysis_order_id: T::Hash,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as GeneticAnalysisOrderInterface<T>>::set_genetic_analysis_order_paid(
				&who,
				&genetic_analysis_order_id,
			) {
				Ok(genetic_analysis_order) => {
					Self::deposit_event(Event::<T>::GeneticAnalysisOrderPaid(
						genetic_analysis_order,
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::GeneticAnalysisOrdersWeightInfo::fulfill_genetic_analysis_order())]
		pub fn fulfill_genetic_analysis_order(
			origin: OriginFor<T>,
			genetic_analysis_order_id: T::Hash,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as GeneticAnalysisOrderInterface<T>>::fulfill_genetic_analysis_order(
				&who,
				&genetic_analysis_order_id,
			) {
				Ok(genetic_analysis_order) => {
					Self::deposit_event(Event::<T>::GeneticAnalysisOrderFulfilled(
						genetic_analysis_order,
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::GeneticAnalysisOrdersWeightInfo::set_genetic_analysis_order_refunded())]
		pub fn set_genetic_analysis_order_refunded(
			origin: OriginFor<T>,
			genetic_analysis_order_id: T::Hash,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as GeneticAnalysisOrderInterface<T>>::set_genetic_analysis_order_refunded(
				&who,
				&genetic_analysis_order_id,
			) {
				Ok(genetic_analysis_order) => {
					Self::deposit_event(Event::<T>::GeneticAnalysisOrderRefunded(
						genetic_analysis_order,
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::GeneticAnalysisOrdersWeightInfo::update_escrow_key())]
		pub fn update_escrow_key(
			origin: OriginFor<T>,
			account_id: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as GeneticAnalysisOrderInterface<T>>::update_escrow_key(&who, &account_id) {
				Ok(_) => {
					Self::deposit_event(Event::UpdateGeneticAnalysisOrderEscrowKeySuccessful(
						who.clone(),
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(0)]
		pub fn sudo_update_escrow_key(
			origin: OriginFor<T>,
			account_id: T::AccountId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			EscrowKey::<T>::put(&account_id);

			Self::deposit_event(Event::UpdateGeneticAnalysisOrderEscrowKeySuccessful(account_id));

			Ok(Pays::No.into())
		}

		#[pallet::weight(T::GeneticAnalysisOrdersWeightInfo::update_treasury_key())]
		pub fn update_treasury_key(
			origin: OriginFor<T>,
			account_id: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as GeneticAnalysisOrderInterface<T>>::update_treasury_key(&who, &account_id)
			{
				Ok(_) => {
					Self::deposit_event(Event::UpdateGeneticAnalysisOrderTreasuryKeySuccessful(
						who.clone(),
					));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(0)]
		pub fn sudo_update_treasury_key(
			origin: OriginFor<T>,
			account_id: T::AccountId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			TreasuryKey::<T>::put(&account_id);

			Self::deposit_event(Event::UpdateGeneticAnalysisOrderTreasuryKeySuccessful(account_id));

			Ok(Pays::No.into())
		}
	}
}
