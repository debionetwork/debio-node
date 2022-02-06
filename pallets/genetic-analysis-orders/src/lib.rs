#![cfg_attr(not(feature = "std"), no_std)]

pub mod interface;
pub mod weights;
use interface::GeneticAnalysisOrderInterface;

use frame_support::{
	codec::{Decode, Encode},
	pallet_prelude::*,
	traits::Currency,
};
pub use pallet::*;
pub use scale_info::TypeInfo;
use sp_std::{prelude::*, vec};
use traits_genetic_analysis::{GeneticAnalysisProvider};
use traits_genetic_analysis_orders::{GeneticAnalysisOrderEventEmitter, GeneticAnalysisOrderStatusUpdater};
use traits_genetic_analyst_services::{
	GeneticAnalystServicesProvider,
};
use primitives_price_and_currency::{CurrencyType, Price};
use primitives_tracking_id::TrackingId;
pub use weights::WeightInfo;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum GeneticAnalysisOrderStatus {
	Unpaid,
	Paid,
	Fulfilled,
	Refunded,
	Cancelled,
	Failed,
}
impl Default for GeneticAnalysisOrderStatus {
	fn default() -> Self {
		GeneticAnalysisOrderStatus::Unpaid
	}
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct GeneticAnalysisOrder<Hash, AccountId, Balance, Moment> {
	pub id: Hash,
	pub service_id: Hash,
	pub customer_id: AccountId,
	pub customer_box_public_key: Hash,
	pub seller_id: AccountId,
	pub genetic_analysis_tracking_id: TrackingId,
	pub currency: CurrencyType,
	pub prices: Vec<Price<Balance>>,
	pub additional_prices: Vec<Price<Balance>>,
	pub status: GeneticAnalysisOrderStatus,
	pub created_at: Moment,
	pub updated_at: Moment,
}
#[allow(clippy::too_many_arguments)]
impl<Hash, AccountId, Balance, Moment> GeneticAnalysisOrder<Hash, AccountId, Balance, Moment> {
	pub fn new(
		id: Hash,
		service_id: Hash,
		customer_id: AccountId,
		customer_box_public_key: Hash,
		seller_id: AccountId,
		genetic_analysis_tracking_id: TrackingId,
		currency: CurrencyType,
		prices: Vec<Price<Balance>>,
		additional_prices: Vec<Price<Balance>>,
		created_at: Moment,
		updated_at: Moment,
	) -> Self {
		Self {
			id,
			service_id,
			customer_id,
			customer_box_public_key,
			seller_id,
			genetic_analysis_tracking_id,
			currency,
			prices,
			additional_prices,
			status: GeneticAnalysisOrderStatus::default(),
			created_at,
			updated_at,
		}
	}

	pub fn get_id(&self) -> &Hash {
		&self.id
	}

	pub fn get_created_at(&self) -> &Moment {
		&self.created_at
	}

	pub fn get_service_id(&self) -> &Hash {
		&self.service_id
	}
}

#[frame_support::pallet]
pub mod pallet {
	use crate::*;
	use frame_support::dispatch::DispatchResultWithPostInfo;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type GeneticAnalystServices: GeneticAnalystServicesProvider<Self, BalanceOf<Self>>;
		type GeneticAnalysis: GeneticAnalysisProvider<Self>;
		type Currency: Currency<<Self as frame_system::Config>::AccountId>;
		type GeneticAnalysisOrdersWeightInfo: WeightInfo;
	}

	// ----- This is template code, every pallet needs this ---
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
	// --------------------------------------------------------

	// ---- Types --------------------------------------------
	type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;
	pub type HashOf<T> = <T as frame_system::Config>::Hash;
	pub type CurrencyOf<T> = <T as self::Config>::Currency;
	pub type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;
	pub type GeneticAnalysisOrderOf<T> = GeneticAnalysisOrder<HashOf<T>, AccountIdOf<T>, BalanceOf<T>, MomentOf<T>>;
	type GeneticAnalysisOrderIdsOf<T> = Vec<HashOf<T>>;
	// -------------------------------------------------------

	// ------ Storage --------------------------
	#[pallet::storage]
	#[pallet::getter(fn genetic_analysis_order_by_id)]
	pub type GeneticAnalysisOrders<T> = StorageMap<_, Blake2_128Concat, HashOf<T>, GeneticAnalysisOrderOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn genetic_analysis_orders_by_customer_id)]
	pub type GeneticAnalysisOrdersByCustomer<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, GeneticAnalysisOrderIdsOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn genetic_analysis_orders_by_lab_id)]
	pub type GeneticAnalysisOrdersBySeller<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, GeneticAnalysisOrderIdsOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn last_genetic_analysis_order_by_customer_id)]
	pub type LastGeneticAnalysisOrderByCustomer<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, HashOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn admin_key)]
	pub type EscrowKey<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;
	// -----------------------------------------

	// ----- Genesis Configs ------------------
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub escrow_key: T::AccountId,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { escrow_key: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			EscrowKey::<T>::put(&self.escrow_key);
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
		/// GeneticAnalysisOrder Not Found
		/// parameters, []
		GeneticAnalysisOrderNotFound,
		/// GeneticAnalysisOrder Failed
		/// parameters, [GeneticAnalysisOrder]
		GeneticAnalysisOrderFailed(GeneticAnalysisOrderOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// GeneticAnalystService id does not exist
		GeneticAnalystServiceDoesNotExist,
		/// GeneticAnalysisOrder does not exist
		GeneticAnalysisOrderNotFound,
		/// Unauthorized to fulfill genetic_analysis_order - user is not the seller who owns the service
		UnauthorizedGeneticAnalysisOrderFulfillment,
		/// Unauthorized to cancel genetic_analysis_order - user is not the customer who created the genetic_analysis_order
		UnauthorizedGeneticAnalysisOrderCancellation,
		/// Can not fulfill genetic_analysis_order before Specimen is processed
		GeneticAnalysisNotSuccessfullyProcessed,
		/// Refund not allowed, GeneticAnalysisOrder is not expired yet
		GeneticAnalysisOrderNotYetExpired,
		/// Unauthorized Account
		Unauthorized,
		/// Error on creating DNA sample
		GeneticAnalysisInitalizationError,
		/// Customer eth address not found
		CustomerEthAddressNotFound,
		/// Seller eth address not found
		SellerEthAddressNotFound,
		/// GeneticAnalystService Price Index not found
		PriceIndexNotFound,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::GeneticAnalysisOrdersWeightInfo::create_genetic_analysis_order())]
		pub fn create_genetic_analysis_order(
			origin: OriginFor<T>,
			service_id: T::Hash,
			price_index: u32,
			customer_box_public_key: T::Hash,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as GeneticAnalysisOrderInterface<T>>::create_genetic_analysis_order(
				&who,
				&service_id,
				price_index,
				&customer_box_public_key,
			) {
				Ok(genetic_analysis_order) => {
					Self::deposit_event(Event::<T>::GeneticAnalysisOrderCreated(genetic_analysis_order));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::GeneticAnalysisOrdersWeightInfo::cancel_genetic_analysis_order())]
		pub fn cancel_genetic_analysis_order(origin: OriginFor<T>, genetic_analysis_order_id: T::Hash) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as GeneticAnalysisOrderInterface<T>>::cancel_genetic_analysis_order(&who, &genetic_analysis_order_id) {
				Ok(genetic_analysis_order) => {
					Self::deposit_event(Event::<T>::GeneticAnalysisOrderCancelled(genetic_analysis_order));
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

			match <Self as GeneticAnalysisOrderInterface<T>>::set_genetic_analysis_order_paid(&who, &genetic_analysis_order_id) {
				Ok(genetic_analysis_order) => {
					Self::deposit_event(Event::<T>::GeneticAnalysisOrderPaid(genetic_analysis_order));
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

			match <Self as GeneticAnalysisOrderInterface<T>>::fulfill_genetic_analysis_order(&who, &genetic_analysis_order_id) {
				Ok(genetic_analysis_order) => {
					Self::deposit_event(Event::<T>::GeneticAnalysisOrderFulfilled(genetic_analysis_order));
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

			match <Self as GeneticAnalysisOrderInterface<T>>::set_genetic_analysis_order_refunded(&who, &genetic_analysis_order_id) {
				Ok(genetic_analysis_order) => {
					Self::deposit_event(Event::<T>::GeneticAnalysisOrderRefunded(genetic_analysis_order));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}
	}
}

impl<T: Config> GeneticAnalysisOrderInterface<T> for Pallet<T> {
	type GeneticAnalysisOrder = GeneticAnalysisOrderOf<T>;
	type Error = Error<T>;

	fn create_genetic_analysis_order(
		customer_id: &T::AccountId,
		service_id: &T::Hash,
		price_index: u32,
		customer_box_public_key: &T::Hash,
	) -> Result<Self::GeneticAnalysisOrder, Self::Error> {
		let service = T::GeneticAnalystServices::genetic_analyst_service_by_id(service_id);
		if service.is_none() {
			return Err(Error::<T>::GeneticAnalystServiceDoesNotExist)
		}
		let service = service.unwrap();
		let genetic_analysis_order_id = Self::generate_genetic_analysis_order_id(customer_id, service_id);
		let seller_id = service.get_owner_id();
		let prices_by_currency = service.get_prices_by_currency();

		if prices_by_currency.is_empty() ||
			prices_by_currency.len() - 1 < price_index.try_into().unwrap()
		{
			return Err(Error::<T>::PriceIndexNotFound)
		}

		let price_by_currency = &prices_by_currency[price_index as usize];

		let currency = &price_by_currency.currency;
		let prices = &price_by_currency.price_components;
		let additional_prices = &price_by_currency.additional_prices;

		let now = pallet_timestamp::Pallet::<T>::get();

		// Initialize GeneticAnalysis
		let genetic_analysis = T::GeneticAnalysis::register_genetic_analysis(seller_id, customer_id, &genetic_analysis_order_id);
		if genetic_analysis.is_err() {
			return Err(Error::<T>::GeneticAnalysisInitalizationError)
		}
		let genetic_analysis = genetic_analysis.ok().unwrap();

		let genetic_analysis_order = GeneticAnalysisOrder::new(
			genetic_analysis_order_id,
			*service_id,
			customer_id.clone(),
			*customer_box_public_key,
			seller_id.clone(),
			genetic_analysis.get_tracking_id().clone(),
			currency.clone(),
			prices.clone(),
			additional_prices.clone(),
			now,
			now,
		);
		Self::insert_genetic_analysis_order_to_storage(&genetic_analysis_order);

		Ok(genetic_analysis_order)
	}

	fn cancel_genetic_analysis_order(
		customer_id: &T::AccountId,
		genetic_analysis_order_id: &T::Hash,
	) -> Result<Self::GeneticAnalysisOrder, Self::Error> {
		let genetic_analysis_order = GeneticAnalysisOrders::<T>::get(genetic_analysis_order_id);
		if genetic_analysis_order.is_none() {
			return Err(Error::<T>::GeneticAnalysisOrderNotFound)
		}
		let genetic_analysis_order = genetic_analysis_order.unwrap();

		if genetic_analysis_order.customer_id != customer_id.clone() {
			return Err(Error::<T>::UnauthorizedGeneticAnalysisOrderCancellation)
		}

		// Delete dna sample associated with the genetic_analysis_order
		let _genetic_analysis = T::GeneticAnalysis::delete_genetic_analysis(&genetic_analysis_order.genetic_analysis_tracking_id);

		let genetic_analysis_order = Self::update_genetic_analysis_order_status(genetic_analysis_order_id, GeneticAnalysisOrderStatus::Cancelled).unwrap();

		Ok(genetic_analysis_order)
	}

	fn set_genetic_analysis_order_paid(
		escrow_account_id: &T::AccountId,
		genetic_analysis_order_id: &T::Hash,
	) -> Result<Self::GeneticAnalysisOrder, Self::Error> {
		if escrow_account_id.clone() != EscrowKey::<T>::get() {
			return Err(Error::<T>::Unauthorized)
		}

		let genetic_analysis_order = Self::update_genetic_analysis_order_status(genetic_analysis_order_id, GeneticAnalysisOrderStatus::Paid);
		if genetic_analysis_order.is_none() {
			return Err(Error::<T>::GeneticAnalysisOrderNotFound)
		}

		Ok(genetic_analysis_order.unwrap())
	}

	fn fulfill_genetic_analysis_order(
		seller_id: &T::AccountId,
		genetic_analysis_order_id: &T::Hash,
	) -> Result<Self::GeneticAnalysisOrder, Self::Error> {
		let genetic_analysis_order = GeneticAnalysisOrders::<T>::get(genetic_analysis_order_id);
		if genetic_analysis_order.is_none() {
			return Err(Error::<T>::GeneticAnalysisOrderNotFound)
		}
		let genetic_analysis_order = genetic_analysis_order.unwrap();

		// Only the seller can fulfill the genetic_analysis_order
		if genetic_analysis_order.seller_id != seller_id.clone() {
			return Err(Error::<T>::UnauthorizedGeneticAnalysisOrderFulfillment)
		}

		let genetic_analysis =
			T::GeneticAnalysis::genetic_analysis_by_genetic_analysis_tracking_id(&genetic_analysis_order.genetic_analysis_tracking_id);
		if !genetic_analysis.unwrap().process_success() {
			return Err(Error::<T>::GeneticAnalysisNotSuccessfullyProcessed)
		}

		let genetic_analysis_order = Self::update_genetic_analysis_order_status(genetic_analysis_order_id, GeneticAnalysisOrderStatus::Fulfilled);

		Ok(genetic_analysis_order.unwrap())
	}

	fn set_genetic_analysis_order_refunded(
		escrow_account_id: &T::AccountId,
		genetic_analysis_order_id: &T::Hash,
	) -> Result<Self::GeneticAnalysisOrder, Self::Error> {
		if escrow_account_id.clone() != EscrowKey::<T>::get() {
			return Err(Error::<T>::Unauthorized)
		}

		let genetic_analysis_order = GeneticAnalysisOrders::<T>::get(genetic_analysis_order_id);
		if genetic_analysis_order.is_none() {
			return Err(Error::<T>::GeneticAnalysisOrderNotFound)
		}

		let genetic_analysis_order_can_be_refunded = Self::genetic_analysis_order_can_be_refunded(genetic_analysis_order.unwrap());
		if !genetic_analysis_order_can_be_refunded {
			return Err(Error::<T>::GeneticAnalysisOrderNotYetExpired)
		}

		let genetic_analysis_order = Self::update_genetic_analysis_order_status(genetic_analysis_order_id, GeneticAnalysisOrderStatus::Refunded);
		Ok(genetic_analysis_order.unwrap())
	}
}

use frame_support::{sp_runtime::traits::Hash, sp_std::convert::TryInto};

impl<T: Config> Pallet<T> {
	pub fn generate_genetic_analysis_order_id(customer_id: &T::AccountId, service_id: &T::Hash) -> T::Hash {
		let mut customer_id_bytes = customer_id.encode();
		let mut service_id_bytes = service_id.encode();
		let account_info = frame_system::Pallet::<T>::account(customer_id);
		let mut nonce_bytes = account_info.nonce.encode();

		customer_id_bytes.append(&mut service_id_bytes);
		customer_id_bytes.append(&mut nonce_bytes);

		let seed = &customer_id_bytes;
		T::Hashing::hash(seed)
	}

	pub fn update_genetic_analysis_order_status(genetic_analysis_order_id: &T::Hash, status: GeneticAnalysisOrderStatus) -> Option<GeneticAnalysisOrderOf<T>> {
		GeneticAnalysisOrders::<T>::mutate(genetic_analysis_order_id, |genetic_analysis_order| match genetic_analysis_order {
			None => None,
			Some(genetic_analysis_order) => {
				genetic_analysis_order.status = status;
				genetic_analysis_order.updated_at = pallet_timestamp::Pallet::<T>::get();
				Some(genetic_analysis_order.clone())
			},
		})
	}

	pub fn insert_genetic_analysis_order_to_storage(genetic_analysis_order: &GeneticAnalysisOrderOf<T>) {
		GeneticAnalysisOrders::<T>::insert(&genetic_analysis_order.id, genetic_analysis_order);
		LastGeneticAnalysisOrderByCustomer::<T>::insert(&genetic_analysis_order.customer_id, &genetic_analysis_order.id);
		Self::insert_genetic_analysis_order_id_into_genetic_analysis_orders_by_seller(genetic_analysis_order);
		Self::insert_genetic_analysis_order_id_into_genetic_analysis_orders_by_customer(genetic_analysis_order);
	}

	pub fn insert_genetic_analysis_order_id_into_genetic_analysis_orders_by_seller(genetic_analysis_order: &GeneticAnalysisOrderOf<T>) {
		match GeneticAnalysisOrdersBySeller::<T>::get(&genetic_analysis_order.seller_id) {
			None => {
				GeneticAnalysisOrdersBySeller::<T>::insert(&genetic_analysis_order.seller_id, vec![genetic_analysis_order.id]);
			},
			Some(mut genetic_analysis_orders) => {
				genetic_analysis_orders.push(genetic_analysis_order.id);
				GeneticAnalysisOrdersBySeller::<T>::insert(&genetic_analysis_order.seller_id, genetic_analysis_orders);
			},
		}
	}

	pub fn insert_genetic_analysis_order_id_into_genetic_analysis_orders_by_customer(genetic_analysis_order: &GeneticAnalysisOrderOf<T>) {
		match GeneticAnalysisOrdersByCustomer::<T>::get(&genetic_analysis_order.customer_id) {
			None => {
				GeneticAnalysisOrdersByCustomer::<T>::insert(&genetic_analysis_order.customer_id, vec![genetic_analysis_order.id]);
			},
			Some(mut genetic_analysis_orders) => {
				genetic_analysis_orders.push(genetic_analysis_order.id);
				GeneticAnalysisOrdersByCustomer::<T>::insert(&genetic_analysis_order.customer_id, genetic_analysis_orders);
			},
		}
	}

	pub fn remove_genetic_analysis_order_id_from_genetic_analysis_orders_by_seller(seller_id: &T::AccountId, genetic_analysis_order_id: &T::Hash) {
		let mut genetic_analysis_orders = GeneticAnalysisOrdersBySeller::<T>::get(seller_id).unwrap_or_default();
		genetic_analysis_orders.retain(|o_id| o_id != genetic_analysis_order_id);
		GeneticAnalysisOrdersBySeller::<T>::insert(seller_id, genetic_analysis_orders);
	}

	pub fn remove_genetic_analysis_order_id_from_genetic_analysis_orders_by_customer(customer_id: &T::AccountId, genetic_analysis_order_id: &T::Hash) {
		let mut genetic_analysis_orders = GeneticAnalysisOrdersByCustomer::<T>::get(customer_id).unwrap_or_default();
		genetic_analysis_orders.retain(|o_id| o_id != genetic_analysis_order_id);
		GeneticAnalysisOrdersByCustomer::<T>::insert(customer_id, genetic_analysis_orders);
	}

	pub fn genetic_analysis_order_can_be_refunded(genetic_analysis_order: GeneticAnalysisOrderOf<T>) -> bool {
		let genetic_analysis =
			T::GeneticAnalysis::genetic_analysis_by_genetic_analysis_tracking_id(&genetic_analysis_order.genetic_analysis_tracking_id).unwrap();
		if !genetic_analysis.is_rejected() {
			return false
		}
		true
	}
}

impl<T: Config> GeneticAnalysisOrderEventEmitter<T> for Pallet<T> {
	fn emit_event_genetic_analysis_order_failed(genetic_analysis_order_id: &HashOf<T>) {
		match Self::genetic_analysis_order_by_id(genetic_analysis_order_id) {
			None => Self::deposit_event(Event::GeneticAnalysisOrderNotFound),
			Some(genetic_analysis_order) => Self::deposit_event(Event::GeneticAnalysisOrderFailed(genetic_analysis_order)),
		}
	}
}

impl<T: Config> GeneticAnalysisOrderStatusUpdater<T> for Pallet<T> {
	fn update_status_failed(genetic_analysis_order_id: &HashOf<T>) {
		match Self::genetic_analysis_order_by_id(genetic_analysis_order_id) {
			None => Self::deposit_event(Event::GeneticAnalysisOrderNotFound),
			Some(genetic_analysis_order) => {
				Self::update_genetic_analysis_order_status(&genetic_analysis_order.id, GeneticAnalysisOrderStatus::Failed);
			},
		}
	}
}
