#![cfg_attr(not(feature = "std"), no_std)]

pub mod weights;

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
pub use pallet::*;
pub use scale_info::TypeInfo;
pub use weights::WeightInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod interface;
pub use crate::interface::CustomerInterface;
use frame_support::pallet_prelude::*;
use traits_user_profile::UserProfileProvider;

// CustomerInfo Struct
// Used as parameter of dispatchable calls
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct CustomerInfo {
	pub name: Vec<u8>,
	pub email: Vec<u8>,
	pub address: Vec<u8>,
	pub latitude: Option<Vec<u8>>,
	pub longitude: Option<Vec<u8>>,
	pub profile_image: Option<Vec<u8>>,
}

// Customer Struct
// the fields (excluding account_id) come from CustomerInfo struct
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct Customer<AccountId> {
	pub account_id: AccountId,
	pub info: CustomerInfo,
}

impl<AccountId> Customer<AccountId> {
	pub fn new(account_id: AccountId, info: CustomerInfo) -> Self {
		Self { account_id, info }
	}

	fn update_info(&mut self, info: CustomerInfo) {
		self.info = info;
	}

	pub fn get_account_id(&self) -> &AccountId {
		&self.account_id
	}
}

#[frame_support::pallet]
pub mod pallet {
	use crate::{interface::CustomerInterface, Customer, CustomerInfo, *};
	use codec::EncodeLike;
	use frame_support::{dispatch::DispatchResultWithPostInfo, traits::Currency};
	use frame_system::pallet_prelude::*;
	pub use sp_std::prelude::*;

	#[pallet::config]
	/// Configure the pallet by specifying the parameters and types on which it depends.
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<Self::AccountId>;
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
		type WeightInfo: WeightInfo;
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
	pub type CustomerOf<T> = Customer<AccountIdOf<T>>;

	// ----- Storage ------------------
	/// Get Customer by account id
	/// AccountId => Customer
	#[pallet::storage]
	#[pallet::getter(fn customer_by_account_id)]
	pub type Customers<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, CustomerOf<T>>;

	/// Get total customer count
	/// u32
	#[pallet::storage]
	#[pallet::getter(fn customer_count)]
	pub type CustomerCount<T> = StorageValue<_, u64>;
	// -----------------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// User AccountId registered as customer
		/// parameters. [Customer, who]
		CustomerRegistered(CustomerOf<T>, AccountIdOf<T>),
		/// Customer information updated
		/// parameters. [Customer, who]
		CustomerUpdated(CustomerOf<T>, AccountIdOf<T>),
		/// Customer deleted
		/// parameters. [Customer, who]
		CustomerDeleted(CustomerOf<T>, AccountIdOf<T>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Account already has customer registered
		CustomerAlreadyRegistered,
		/// Customer identified by the AccountId does not exist
		CustomerDoesNotExist,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::register_customer())]
		pub fn register_customer(
			origin: OriginFor<T>,
			customer_info: CustomerInfo,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match Self::create_customer(&who, &customer_info) {
				Ok(customer) => {
					Self::deposit_event(Event::CustomerRegistered(customer, who.clone()));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::update_customer())]
		pub fn update_customer(
			origin: OriginFor<T>,
			customer_info: CustomerInfo,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as CustomerInterface<T>>::update_customer(&who, &customer_info) {
				Ok(customer) => {
					Self::deposit_event(Event::CustomerUpdated(customer, who.clone()));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::deregister_customer())]
		pub fn deregister_customer(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			// Check if user is a customer
			let customer = Self::customer_by_account_id(&who);
			if customer == None {
				return Err(Error::<T>::CustomerDoesNotExist.into())
			}

			match <Self as CustomerInterface<T>>::delete_customer(&who) {
				Ok(customer) => {
					Self::deposit_event(Event::CustomerDeleted(customer, who.clone()));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}
	}
}

impl<T: Config> CustomerInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type CustomerInfo = CustomerInfo;
	type Customer = CustomerOf<T>;

	fn create_customer(
		account_id: &T::AccountId,
		customer_info: &Self::CustomerInfo,
	) -> Result<Self::Customer, Self::Error> {
		if Customers::<T>::contains_key(account_id) {
			return Err(Error::<T>::CustomerAlreadyRegistered)
		}
		let customer = Customer::new(account_id.clone(), customer_info.clone());

		// Insert to Storage
		Customers::<T>::insert(account_id, &customer);

		// Increment Count
		Self::add_customer_count();

		Ok(customer)
	}

	fn update_customer(
		account_id: &T::AccountId,
		customer_info: &Self::CustomerInfo,
	) -> Result<Self::Customer, Self::Error> {
		let customer = Customers::<T>::get(account_id);
		if customer == None {
			return Err(Error::<T>::CustomerDoesNotExist)
		}
		let mut customer = customer.unwrap();

		customer.update_info(customer_info.clone());

		Customers::<T>::insert(account_id, &customer);

		Ok(customer)
	}

	fn delete_customer(account_id: &T::AccountId) -> Result<Self::Customer, Self::Error> {
		let customer = Customers::<T>::get(account_id);
		if customer == None {
			return Err(Error::<T>::CustomerDoesNotExist)
		}
		let customer = customer.unwrap();

		Customers::<T>::remove(&customer.account_id);
		Self::sub_customer_count();

		Ok(customer)
	}

	fn customer_by_account_id(account_id: &T::AccountId) -> Option<Self::Customer> {
		Self::customer_by_account_id(account_id)
	}
}

impl<T: Config> Pallet<T> {
	// Add customer count
	pub fn add_customer_count() {
		let customer_count = <CustomerCount<T>>::get().unwrap_or(0);
		<CustomerCount<T>>::put(customer_count.wrapping_add(1));
	}

	// Subtract customer count
	pub fn sub_customer_count() {
		let customer_count = <CustomerCount<T>>::get().unwrap_or(1);
		CustomerCount::<T>::put(customer_count - 1);
	}
}
