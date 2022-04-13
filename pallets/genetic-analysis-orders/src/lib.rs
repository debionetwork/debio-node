#![cfg_attr(not(feature = "std"), no_std)]

pub mod interface;
pub mod weights;
use interface::GeneticAnalysisOrderInterface;

use frame_support::{
	codec::{Decode, Encode},
	pallet_prelude::*,
	sp_runtime::traits::{AccountIdConversion, Hash},
	sp_std::convert::TryInto,
	traits::{Currency, ExistenceRequirement},
	PalletId,
};
pub use pallet::*;
use primitives_price_and_currency::{CurrencyType, Price};
use primitives_tracking_id::TrackingId;
pub use scale_info::TypeInfo;
use sp_std::{prelude::*, vec};
use traits_genetic_analysis::{GeneticAnalysisProvider, GeneticAnalysisTracking};
use traits_genetic_analysis_orders::{
	GeneticAnalysisOrderEventEmitter, GeneticAnalysisOrderStatusUpdater,
};
use traits_genetic_analyst_services::{GeneticAnalystServiceInfo, GeneticAnalystServicesProvider};
use traits_genetic_analysts::GeneticAnalystsProvider;
use traits_genetic_data::{GeneticData, GeneticDataProvider};
pub use weights::WeightInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

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
	pub genetic_data_id: Hash,
	pub genetic_analysis_tracking_id: TrackingId,
	pub currency: CurrencyType,
	pub prices: Vec<Price<Balance>>,
	pub additional_prices: Vec<Price<Balance>>,
	pub total_price: Balance,
	pub status: GeneticAnalysisOrderStatus,
	pub created_at: Moment,
	pub updated_at: Moment,
	pub genetic_link: Vec<u8>,
}
#[allow(clippy::too_many_arguments)]
impl<Hash, AccountId, Balance, Moment: Default>
	GeneticAnalysisOrder<Hash, AccountId, Balance, Moment>
{
	pub fn new(
		id: Hash,
		service_id: Hash,
		customer_id: AccountId,
		customer_box_public_key: Hash,
		seller_id: AccountId,
		genetic_data_id: Hash,
		genetic_analysis_tracking_id: TrackingId,
		genetic_link: Vec<u8>,
		currency: CurrencyType,
		prices: Vec<Price<Balance>>,
		additional_prices: Vec<Price<Balance>>,
		total_price: Balance,
		created_at: Moment,
	) -> Self {
		Self {
			id,
			service_id,
			customer_id,
			customer_box_public_key,
			seller_id,
			genetic_data_id,
			genetic_analysis_tracking_id,
			genetic_link,
			currency,
			prices,
			additional_prices,
			status: GeneticAnalysisOrderStatus::default(),
			total_price,
			created_at,
			updated_at: Moment::default(),
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
		type GeneticAnalysts: GeneticAnalystsProvider<Self>;
		type GeneticAnalystServices: GeneticAnalystServicesProvider<Self, BalanceOf<Self>>;
		type GeneticData: GeneticDataProvider<Self>;
		type GeneticAnalysis: GeneticAnalysisProvider<Self>;
		type Currency: Currency<<Self as frame_system::Config>::AccountId>;
		type GeneticAnalysisOrdersWeightInfo: WeightInfo;
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
	pub type EscrowKey<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn pallet_id)]
	pub type PalletAccount<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn total_escrow_amount)]
	pub type TotalEscrowAmount<T> = StorageValue<_, BalanceOf<T>>;
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
		/// Insufficient pallet funds
		InsufficientPalletFunds,
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
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as GeneticAnalysisOrderInterface<T>>::create_genetic_analysis_order(
				&who,
				&genetic_data_id,
				&service_id,
				price_index,
				&customer_box_public_key,
				&genetic_link,
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
	}
}

impl<T: Config> GeneticAnalysisOrderInterface<T> for Pallet<T> {
	type GeneticAnalysisOrder = GeneticAnalysisOrderOf<T>;
	type Error = Error<T>;

	fn create_genetic_analysis_order(
		customer_id: &T::AccountId,
		genetic_data_id: &T::Hash,
		genetic_analyst_service_id: &T::Hash,
		price_index: u32,
		customer_box_public_key: &T::Hash,
		genetic_link: &[u8],
	) -> Result<Self::GeneticAnalysisOrder, Self::Error> {
		let genetic_analyst_service = match T::GeneticAnalystServices::genetic_analyst_service_by_id(
			genetic_analyst_service_id,
		) {
			Some(_genetic_analyst_service) => _genetic_analyst_service,
			None => return Err(Error::<T>::GeneticAnalystServiceDoesNotExist),
		};

		let seller_id = genetic_analyst_service.get_owner_id();
		if !T::GeneticAnalysts::is_genetic_analyst_available(seller_id).unwrap() {
			// If _bool is false, then genetic analyst is unavailable
			return Err(Error::<T>::GeneticAnalystUnavailable)
		}

		let genetic_data = T::GeneticData::genetic_data_by_id(genetic_data_id);
		if genetic_data.is_none() {
			return Err(Error::<T>::GeneticDataDoesNotExist)
		}

		let genetic_data = genetic_data.unwrap();
		if customer_id.clone() != genetic_data.get_owner_id().clone() {
			return Err(Error::<T>::NotOwnerOfGeneticData)
		}

		let prices_by_currency = genetic_analyst_service.get_prices_by_currency();
		if prices_by_currency.is_empty() ||
			prices_by_currency.len() - 1 < price_index.try_into().unwrap()
		{
			return Err(Error::<T>::PriceIndexNotFound)
		}

		let price_by_currency = &prices_by_currency[price_index as usize];

		let total_price = &price_by_currency.total_price;
		let currency = &price_by_currency.currency;
		let prices = &price_by_currency.price_components;
		let additional_prices = &price_by_currency.additional_prices;

		let now = pallet_timestamp::Pallet::<T>::get();

		// Initialize GeneticAnalysis
		let genetic_analysis_order_id =
			Self::generate_genetic_analysis_order_id(customer_id, genetic_analyst_service_id);
		let genetic_analysis = T::GeneticAnalysis::register_genetic_analysis(
			seller_id,
			customer_id,
			&genetic_analysis_order_id,
		);
		if genetic_analysis.is_err() {
			return Err(Error::<T>::GeneticAnalysisInitalizationError)
		}
		let genetic_analysis = genetic_analysis.ok().unwrap();

		let genetic_analysis_order = GeneticAnalysisOrder::new(
			genetic_analysis_order_id,
			*genetic_analyst_service_id,
			customer_id.clone(),
			*customer_box_public_key,
			seller_id.clone(),
			*genetic_data_id,
			genetic_analysis.get_genetic_analysis_tracking_id().clone(),
			genetic_link.to_vec(),
			currency.clone(),
			prices.clone(),
			additional_prices.clone(),
			*total_price,
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

		let genetic_analysis =
			T::GeneticAnalysis::genetic_analysis_by_genetic_analysis_tracking_id(
				&genetic_analysis_order.genetic_analysis_tracking_id,
			)
			.unwrap();
		if !genetic_analysis.is_registered() {
			return Err(Error::<T>::OngoingGeneticAnalysisOrderCannotBeCancelled)
		}

		if genetic_analysis_order.status == GeneticAnalysisOrderStatus::Paid {
			if !Self::is_pallet_balance_sufficient_for_transfer(genetic_analysis_order.total_price)
			{
				return Err(Error::<T>::InsufficientPalletFunds)
			}

			let _ = Self::transfer_balance(
				&Self::account_id(),
				&genetic_analysis_order.customer_id,
				genetic_analysis_order.total_price,
			);
		}

		// Delete dna sample associated with the genetic_analysis_order
		let _genetic_analysis = T::GeneticAnalysis::delete_genetic_analysis(
			&genetic_analysis_order.genetic_analysis_tracking_id,
		);

		let genetic_analysis_order = Self::update_genetic_analysis_order_status(
			genetic_analysis_order_id,
			GeneticAnalysisOrderStatus::Cancelled,
		)
		.unwrap();

		Ok(genetic_analysis_order)
	}

	fn set_genetic_analysis_order_paid(
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

		let genetic_analysis_order = genetic_analysis_order.unwrap();
		if !Self::is_balance_sufficient_for_payment(
			&genetic_analysis_order.customer_id,
			genetic_analysis_order.total_price,
		) {
			return Err(Error::<T>::InsufficientFunds)
		}

		let _ = Self::transfer_balance(
			&genetic_analysis_order.customer_id,
			&Self::account_id(),
			genetic_analysis_order.total_price,
		);

		let genetic_analysis_order = Self::update_genetic_analysis_order_status(
			genetic_analysis_order_id,
			GeneticAnalysisOrderStatus::Paid,
		);

		Ok(genetic_analysis_order.unwrap())
	}

	fn fulfill_genetic_analysis_order(
		escrow_account_id: &T::AccountId,
		genetic_analysis_order_id: &T::Hash,
	) -> Result<Self::GeneticAnalysisOrder, Self::Error> {
		// Only the admin can fulfill the genetic_analysis_order
		if escrow_account_id.clone() != EscrowKey::<T>::get() {
			return Err(Error::<T>::Unauthorized)
		}

		let genetic_analysis_order = GeneticAnalysisOrders::<T>::get(genetic_analysis_order_id);
		if genetic_analysis_order.is_none() {
			return Err(Error::<T>::GeneticAnalysisOrderNotFound)
		}
		let genetic_analysis_order = genetic_analysis_order.unwrap();

		let genetic_analysis = T::GeneticAnalysis::genetic_analysis_by_genetic_analysis_tracking_id(
			&genetic_analysis_order.genetic_analysis_tracking_id,
		);
		if !genetic_analysis.unwrap().process_success() {
			return Err(Error::<T>::GeneticAnalysisNotSuccessfullyProcessed)
		}

		if !Self::is_pallet_balance_sufficient_for_transfer(genetic_analysis_order.total_price) {
			return Err(Error::<T>::InsufficientPalletFunds)
		}

		let _ = Self::transfer_balance(
			&Self::account_id(),
			&genetic_analysis_order.seller_id,
			genetic_analysis_order.total_price,
		);

		let genetic_analysis_order = Self::update_genetic_analysis_order_status(
			genetic_analysis_order_id,
			GeneticAnalysisOrderStatus::Fulfilled,
		);

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

		let genetic_analysis_order_can_be_refunded =
			Self::genetic_analysis_order_can_be_refunded(genetic_analysis_order.clone().unwrap());
		if !genetic_analysis_order_can_be_refunded {
			return Err(Error::<T>::GeneticAnalysisOrderNotYetExpired)
		}

		let genetic_analysis_order = genetic_analysis_order.unwrap();
		if !Self::is_pallet_balance_sufficient_for_transfer(genetic_analysis_order.total_price) {
			return Err(Error::<T>::InsufficientPalletFunds)
		}

		let _ = Self::transfer_balance(
			&Self::account_id(),
			&genetic_analysis_order.customer_id,
			genetic_analysis_order.total_price,
		);

		let genetic_analysis_order = Self::update_genetic_analysis_order_status(
			genetic_analysis_order_id,
			GeneticAnalysisOrderStatus::Refunded,
		);
		Ok(genetic_analysis_order.unwrap())
	}

	fn update_escrow_key(
		account_id: &T::AccountId,
		escrow_key: &T::AccountId,
	) -> Result<(), Self::Error> {
		if account_id.clone() != EscrowKey::<T>::get() {
			return Err(Error::<T>::Unauthorized)
		}

		EscrowKey::<T>::put(escrow_key);

		Ok(())
	}

	fn is_pending_genetic_analysis_order_ids_by_seller_exist(account_id: &T::AccountId) -> bool {
		match PendingGeneticAnalysisOrdersBySeller::<T>::get(account_id) {
			Some(_arr) => !_arr.is_empty(),
			None => false,
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn generate_genetic_analysis_order_id(
		customer_id: &T::AccountId,
		service_id: &T::Hash,
	) -> T::Hash {
		let mut customer_id_bytes = customer_id.encode();
		let mut service_id_bytes = service_id.encode();
		let account_info = frame_system::Pallet::<T>::account(customer_id);
		let mut nonce_bytes = account_info.nonce.encode();

		customer_id_bytes.append(&mut service_id_bytes);
		customer_id_bytes.append(&mut nonce_bytes);

		let seed = &customer_id_bytes;
		T::Hashing::hash(seed)
	}

	pub fn update_genetic_analysis_order_status(
		genetic_analysis_order_id: &T::Hash,
		status: GeneticAnalysisOrderStatus,
	) -> Option<GeneticAnalysisOrderOf<T>> {
		GeneticAnalysisOrders::<T>::mutate(genetic_analysis_order_id, |genetic_analysis_order| {
			match genetic_analysis_order {
				None => None,
				Some(genetic_analysis_order) => {
					genetic_analysis_order.status = status;
					genetic_analysis_order.updated_at = pallet_timestamp::Pallet::<T>::get();
					Some(genetic_analysis_order.clone())
				},
			}
		})
	}

	pub fn insert_genetic_analysis_order_to_storage(
		genetic_analysis_order: &GeneticAnalysisOrderOf<T>,
	) {
		GeneticAnalysisOrders::<T>::insert(&genetic_analysis_order.id, genetic_analysis_order);
		LastGeneticAnalysisOrderByCustomer::<T>::insert(
			&genetic_analysis_order.customer_id,
			&genetic_analysis_order.id,
		);
		Self::insert_genetic_analysis_order_id_into_genetic_analysis_orders_by_seller(
			genetic_analysis_order,
		);
		Self::insert_genetic_analysis_order_id_into_pending_genetic_analysis_orders_by_seller(
			genetic_analysis_order,
		);
		Self::insert_genetic_analysis_order_id_into_genetic_analysis_orders_by_customer(
			genetic_analysis_order,
		);
	}

	pub fn insert_genetic_analysis_order_id_into_genetic_analysis_orders_by_seller(
		genetic_analysis_order: &GeneticAnalysisOrderOf<T>,
	) {
		match GeneticAnalysisOrdersBySeller::<T>::get(&genetic_analysis_order.seller_id) {
			None => {
				GeneticAnalysisOrdersBySeller::<T>::insert(
					&genetic_analysis_order.seller_id,
					vec![genetic_analysis_order.id],
				);
			},
			Some(mut genetic_analysis_orders) => {
				genetic_analysis_orders.push(genetic_analysis_order.id);
				GeneticAnalysisOrdersBySeller::<T>::insert(
					&genetic_analysis_order.seller_id,
					genetic_analysis_orders,
				);
			},
		}
	}

	pub fn insert_genetic_analysis_order_id_into_genetic_analysis_orders_by_customer(
		genetic_analysis_order: &GeneticAnalysisOrderOf<T>,
	) {
		match GeneticAnalysisOrdersByCustomer::<T>::get(&genetic_analysis_order.customer_id) {
			None => {
				GeneticAnalysisOrdersByCustomer::<T>::insert(
					&genetic_analysis_order.customer_id,
					vec![genetic_analysis_order.id],
				);
			},
			Some(mut genetic_analysis_orders) => {
				genetic_analysis_orders.push(genetic_analysis_order.id);
				GeneticAnalysisOrdersByCustomer::<T>::insert(
					&genetic_analysis_order.customer_id,
					genetic_analysis_orders,
				);
			},
		}
	}

	pub fn insert_genetic_analysis_order_id_into_pending_genetic_analysis_orders_by_seller(
		genetic_analysis_order: &GeneticAnalysisOrderOf<T>,
	) {
		match PendingGeneticAnalysisOrdersBySeller::<T>::get(&genetic_analysis_order.seller_id) {
			None => {
				PendingGeneticAnalysisOrdersBySeller::<T>::insert(
					&genetic_analysis_order.seller_id,
					vec![genetic_analysis_order.id],
				);
			},
			Some(mut genetic_analysis_orders) => {
				genetic_analysis_orders.push(genetic_analysis_order.id);
				PendingGeneticAnalysisOrdersBySeller::<T>::insert(
					&genetic_analysis_order.seller_id,
					genetic_analysis_orders,
				);
			},
		}
	}

	pub fn remove_genetic_analysis_order_id_from_pending_genetic_analysis_orders_by_seller(
		seller_id: &T::AccountId,
		genetic_analysis_order_id: &T::Hash,
	) {
		let mut genetic_analysis_orders =
			PendingGeneticAnalysisOrdersBySeller::<T>::get(seller_id).unwrap_or_default();
		genetic_analysis_orders.retain(|o_id| o_id != genetic_analysis_order_id);
		PendingGeneticAnalysisOrdersBySeller::<T>::insert(seller_id, genetic_analysis_orders);
	}

	pub fn genetic_analysis_order_can_be_refunded(
		genetic_analysis_order: GeneticAnalysisOrderOf<T>,
	) -> bool {
		let genetic_analysis =
			T::GeneticAnalysis::genetic_analysis_by_genetic_analysis_tracking_id(
				&genetic_analysis_order.genetic_analysis_tracking_id,
			)
			.unwrap();
		if !genetic_analysis.is_rejected() {
			return false
		}
		true
	}

	/// The injected pallet ID
	pub fn get_pallet_id() -> AccountIdOf<T> {
		T::PalletId::get().into_account()
	}

	/// The account ID that holds the funds
	pub fn account_id() -> AccountIdOf<T> {
		<PalletAccount<T>>::get()
	}

	/// Transfer balance
	pub fn transfer_balance(
		source: &AccountIdOf<T>,
		dest: &AccountIdOf<T>,
		amount: BalanceOf<T>,
	) -> BalanceOf<T> {
		let _ = T::Currency::transfer(source, dest, amount, ExistenceRequirement::KeepAlive);
		Self::set_escrow_amount();
		amount
	}

	/// Is the balance sufficient for payment
	pub fn is_balance_sufficient_for_payment(
		account_id: &AccountIdOf<T>,
		price: BalanceOf<T>,
	) -> bool {
		let balance = T::Currency::free_balance(account_id);
		balance >= price
	}

	/// Is the pallet balance sufficient for transfer
	pub fn is_pallet_balance_sufficient_for_transfer(price: BalanceOf<T>) -> bool {
		let balance = T::Currency::free_balance(&Self::account_id());
		balance >= price
	}

	/// Set current escrow amount
	pub fn set_escrow_amount() {
		TotalEscrowAmount::<T>::put(T::Currency::free_balance(&Self::account_id()));
	}
}

impl<T: Config> GeneticAnalysisOrderEventEmitter<T> for Pallet<T> {
	fn emit_event_genetic_analysis_order_failed(genetic_analysis_order_id: &HashOf<T>) {
		match Self::genetic_analysis_order_by_id(genetic_analysis_order_id) {
			None => Self::deposit_event(Event::GeneticAnalysisOrderNotFound),
			Some(genetic_analysis_order) =>
				Self::deposit_event(Event::GeneticAnalysisOrderFailed(genetic_analysis_order)),
		}
	}
}

impl<T: Config> GeneticAnalysisOrderStatusUpdater<T> for Pallet<T> {
	fn update_status_failed(genetic_analysis_order_id: &HashOf<T>) {
		match Self::genetic_analysis_order_by_id(genetic_analysis_order_id) {
			None => Self::deposit_event(Event::GeneticAnalysisOrderNotFound),
			Some(genetic_analysis_order) => {
				Self::update_genetic_analysis_order_status(
					&genetic_analysis_order.id,
					GeneticAnalysisOrderStatus::Failed,
				);
			},
		}
	}

	fn remove_genetic_analysis_order_id_from_pending_genetic_analysis_orders_by_seller(
		seller_id: &AccountIdOf<T>,
		genetic_analysis_order_id: &HashOf<T>,
	) {
		Self::remove_genetic_analysis_order_id_from_pending_genetic_analysis_orders_by_seller(
			seller_id,
			genetic_analysis_order_id,
		);
	}

	fn is_pending_genetic_analysis_order_by_seller_exist(seller_id: &AccountIdOf<T>) -> bool {
		Self::is_pending_genetic_analysis_order_ids_by_seller_exist(seller_id)
	}
}
