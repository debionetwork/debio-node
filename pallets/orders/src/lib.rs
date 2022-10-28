#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;

pub mod functions;
pub mod impl_orders;
pub mod interface;
pub mod migrations;
pub mod types;
pub mod weights;

pub use interface::OrderInterface;
pub use sp_std::{prelude::*, vec};
pub use traits_genetic_testing::{DnaSampleTracking, GeneticTestingProvider};
pub use traits_order::{OrderEventEmitter, OrderStatusUpdater};
pub use traits_services::{types::ServiceFlow, ServiceInfo, ServicesProvider};
pub use types::*;
pub use weights::WeightInfo;

pub use frame_support::traits::StorageVersion;

/// The current storage version.
const STORAGE_VERSION: StorageVersion = StorageVersion::new(2);

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
		sp_runtime::traits::AccountIdConversion,
		traits::{tokens::fungibles, Currency},
		PalletId,
	};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Services: ServicesProvider<Self, BalanceOf<Self>>;
		type GeneticTesting: GeneticTestingProvider<Self>;
		type Currency: Currency<<Self as frame_system::Config>::AccountId>;
		type Assets: fungibles::InspectMetadata<
				<Self as frame_system::Config>::AccountId,
				AssetId = AssetId,
				Balance = AssetBalance,
			> + fungibles::Transfer<<Self as frame_system::Config>::AccountId>;
		type OrdersWeightInfo: WeightInfo;
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

	// ------ Storage --------------------------
	#[pallet::storage]
	#[pallet::getter(fn order_by_id)]
	pub type Orders<T> = StorageMap<_, Blake2_128Concat, HashOf<T>, OrderOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn orders_by_customer_id)]
	pub type OrdersByCustomer<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, OrderIdsOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn orders_by_lab_id)]
	pub type OrdersBySeller<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, OrderIdsOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn pending_genetic_analysis_orders_by_genetic_analyst_id)]
	pub type PendingOrdersBySeller<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, OrderIdsOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn last_order_by_customer_id)]
	pub type LastOrderByCustomer<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, HashOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn admin_key)]
	pub type EscrowKey<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn treasury_key)]
	pub type TreasuryKey<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn pallet_id)]
	pub type PalletAccount<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;
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
			let account_id: T::AccountId = T::PalletId::get().into_account();
			PalletAccount::<T>::put(account_id);

			if let Some(ref escrow_key) = self.escrow_key {
				EscrowKey::<T>::put(escrow_key);
			}

			if let Some(ref treasury_key) = self.treasury_key {
				TreasuryKey::<T>::put(treasury_key);
			}
		}
	}
	// ----------------------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Order created
		/// parameters, [Order]
		OrderCreated(OrderOf<T>),
		/// Order paid
		/// parameters, [Order]
		OrderPaid(OrderOf<T>),
		/// Order Fulfilled
		/// parameters, [Order]
		OrderFulfilled(OrderOf<T>),
		/// Order Refunded
		/// parameters, [Order]
		OrderRefunded(OrderOf<T>),
		/// Order Cancelled
		/// parameters, [Order]
		OrderCancelled(OrderOf<T>),
		/// Order Not Found
		/// parameters, []
		OrderNotFound,
		/// Update Order escrow key
		/// parameters. [who]
		UpdateOrderKeySuccessful(AccountKeyTypeOf<T>),
		/// Order Failed
		/// parameters, [Order]
		OrderFailed(OrderOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Service id does not exist
		ServiceDoesNotExist,
		/// Order does not exist
		OrderNotFound,
		/// Unauthorized to fulfill order - user is not the seller who owns the service
		UnauthorizedOrderFulfillment,
		/// Unauthorized to cancel order - user is not the customer who created the order
		UnauthorizedOrderCancellation,
		// Genetic Testing is ongoing, cannot be cancelled
		OngoingOrderCannotBeCancelled,
		/// Can not fulfill order before Specimen is processed
		DnaSampleNotSuccessfullyProcessed,
		/// Refund not allowed, Order is not expired yet
		OrderNotYetExpired,
		/// Unauthorized Account
		Unauthorized,
		/// Error on creating DNA sample
		DnaSampleInitalizationError,
		/// Customer eth address not found
		CustomerEthAddressNotFound,
		/// Seller eth address not found
		SellerEthAddressNotFound,
		/// Service Price Index not found
		PriceIndexNotFound,
		/// Asset Id not found
		AssetIdNotFound,
		PalletAccountNotFound,
		OrderCannotBeCancelled,
		OrderCannotBePaid,
		OrderCannotBeRefunded,
		OrderCannotBeFulfilled,
		Module,
		Other,
		BadOrigin,
		CannotLookup,
		ConsumerRemaining,
		TooManyConsumers,
		NoProviders,
		Token,
		Arithmetic,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::OrdersWeightInfo::create_order())]
		pub fn create_order(
			origin: OriginFor<T>,
			service_id: T::Hash,
			price_index: u32,
			customer_box_public_key: T::Hash,
			order_flow: ServiceFlow,
			asset_id: Option<u32>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as OrderInterface<T>>::create_order(
				&who,
				&service_id,
				price_index,
				&customer_box_public_key,
				order_flow,
				asset_id,
			) {
				Ok(order) => {
					Self::deposit_event(Event::<T>::OrderCreated(order));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::OrdersWeightInfo::cancel_order())]
		pub fn cancel_order(origin: OriginFor<T>, order_id: T::Hash) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as OrderInterface<T>>::cancel_order(&who, &order_id) {
				Ok(order) => {
					Self::deposit_event(Event::<T>::OrderCancelled(order));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::OrdersWeightInfo::set_order_paid())]
		pub fn set_order_paid(
			origin: OriginFor<T>,
			order_id: T::Hash,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as OrderInterface<T>>::set_order_paid(&who, &order_id) {
				Ok(order) => {
					Self::deposit_event(Event::<T>::OrderPaid(order));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::OrdersWeightInfo::fulfill_order())]
		pub fn fulfill_order(
			origin: OriginFor<T>,
			order_id: T::Hash,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as OrderInterface<T>>::fulfill_order(&who, &order_id) {
				Ok(order) => {
					Self::deposit_event(Event::<T>::OrderFulfilled(order));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::OrdersWeightInfo::set_order_refunded())]
		pub fn set_order_refunded(
			origin: OriginFor<T>,
			order_id: T::Hash,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as OrderInterface<T>>::set_order_refunded(&who, &order_id) {
				Ok(order) => {
					Self::deposit_event(Event::<T>::OrderRefunded(order));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::OrdersWeightInfo::update_key())]
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
				AccountKeyType::EscrowKey(account_id) => {
					let result = EscrowKey::<T>::get().filter(|account_id| account_id == &who);
					ensure!(result.is_some(), Error::<T>::Unauthorized);
					EscrowKey::<T>::put(&account_id);
				},
			};

			Self::deposit_event(Event::UpdateOrderKeySuccessful(account_key_type));

			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn sudo_update_key(
			origin: OriginFor<T>,
			account_key_type: AccountKeyTypeOf<T>,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			match account_key_type.clone() {
				AccountKeyType::TreasuryKey(account_id) => TreasuryKey::<T>::put(&account_id),
				AccountKeyType::EscrowKey(account_id) => EscrowKey::<T>::put(&account_id),
			};

			Self::deposit_event(Event::UpdateOrderKeySuccessful(account_key_type));

			Ok(Pays::No.into())
		}
	}
}
