use super::*;

#[allow(unused)]
use crate::{Pallet as MenstrualSubscription, AccountKeyType, Config};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{sp_runtime::SaturatedConversion, traits::Currency};
use frame_system::RawOrigin;
use primitives_menstrual_status::MenstrualSubscriptionStatus;
use primitives_price_and_currency::CurrencyType;
use primitives_duration::MenstrualSubscriptionDuration;

const SEED: u32 = 0;

benchmarks! {
	add_menstrual_subscription {
		// Initial account
		let caller: T::AccountId = whitelisted_caller();
		let admin: T::AccountId = account("admin", 0, SEED);

		// Set Admin Key
		let root = <T as frame_system::Config>::Origin::from(RawOrigin::Root);
		let _ = MenstrualSubscription::<T>::sudo_update_key(root, AccountKeyType::AdminKey(admin.clone()));

		// Default balance
		let subscription_price = 1_000_000_000_000_000_000u128.saturated_into();

		// Set price
		let admin_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(admin));
		let _ = MenstrualSubscription::<T>::set_menstrual_subscription_price(
			admin_origin,
			MenstrualSubscriptionDuration::default(),
			CurrencyType::default(),
			subscription_price,
			None,
		);
	}: add_menstrual_subscription(
		RawOrigin::Signed(caller),
		MenstrualSubscriptionDuration::default(),
		CurrencyType::DBIO
	)

	change_menstrual_subscription_status {
		// Initial account
		let caller: T::AccountId = whitelisted_caller();
		let admin: T::AccountId = account("admin", 0, SEED);

		// Set Admin Key
		let root = <T as frame_system::Config>::Origin::from(RawOrigin::Root);
		let _ = MenstrualSubscription::<T>::sudo_update_key(root, AccountKeyType::AdminKey(admin.clone()));

		// Default balance
		let init_balance = 2_000_000_000_000_000_000u128.saturated_into();
		let subscription_price = 1_000_000_000_000_000_000u128.saturated_into();

		// Caller init balance
		let _ = <T as Config>::Currency::deposit_creating(
			&caller,
			init_balance
		);

		// Set price
		let admin_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(admin.clone()));
		let _ = MenstrualSubscription::<T>::set_menstrual_subscription_price(
			admin_origin,
			MenstrualSubscriptionDuration::default(),
			CurrencyType::default(),
			subscription_price,
			None,
		);

		// Add subscription
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let _ = MenstrualSubscription::<T>::add_menstrual_subscription(
			caller_origin.clone(),
			MenstrualSubscriptionDuration::default(),
			CurrencyType::DBIO,
		);

		let ids = MenstrualSubscription::<T>::menstrual_subscription_by_address_id(caller.clone()).unwrap();

		let _ = MenstrualSubscription::<T>::set_menstrual_subscription_paid(
			caller_origin,
			ids[0],
		);
	}: change_menstrual_subscription_status(
		RawOrigin::Signed(admin),
		ids[0],
		MenstrualSubscriptionStatus::default()
	)

	set_menstrual_subscription_paid {
		// Initial Account
		let caller: T::AccountId = whitelisted_caller();
		let admin: T::AccountId = account("admin", 0, SEED);
		let treasure: T::AccountId = account("treasure", 0, SEED);

		// Set Key
		let root = <T as frame_system::Config>::Origin::from(RawOrigin::Root);
		let _ = MenstrualSubscription::<T>::sudo_update_key(root.clone(), AccountKeyType::AdminKey(admin.clone()));
		let _ = MenstrualSubscription::<T>::sudo_update_key(root.clone(), AccountKeyType::TreasuryKey(treasure.clone()));

		// Default balance
		let init_balance = 2_000_000_000_000_000_000u128.saturated_into();
		let subscription_price = 1_000_000_000_000_000_000u128.saturated_into();

		// Caller init balance
		let _ = <T as Config>::Currency::deposit_creating(
			&caller,
			init_balance
		);

		// Set price
		let admin_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(admin));
		let _ = MenstrualSubscription::<T>::set_menstrual_subscription_price(
			admin_origin,
			MenstrualSubscriptionDuration::default(),
			CurrencyType::default(),
			subscription_price,
			None,
		);

		// Add Subscription
		let caller_origin = T::Origin::from(RawOrigin::Signed(caller.clone()));
		let _ = MenstrualSubscription::<T>::add_menstrual_subscription(
			caller_origin,
			MenstrualSubscriptionDuration::default(),
			CurrencyType::DBIO,
		);

		let ids = MenstrualSubscription::<T>::menstrual_subscription_by_address_id(caller.clone()).unwrap();
	}: set_menstrual_subscription_paid(
		RawOrigin::Signed(caller),
		ids[0]
	)

	set_menstrual_subscription_price {
		let caller: T::AccountId = whitelisted_caller();

		// Set Admin Key
		let root = <T as frame_system::Config>::Origin::from(RawOrigin::Root);
		let _ = MenstrualSubscription::<T>::sudo_update_key(root.clone(), AccountKeyType::AdminKey(caller.clone()));
	}: set_menstrual_subscription_price(
		RawOrigin::Signed(caller),
		MenstrualSubscriptionDuration::default(),
		CurrencyType::default(),
		1000000000000000000u128.saturated_into(),
		None
	)
}

impl_benchmark_test_suite! {MenstrualSubscription, crate::mock::ExternalityBuilder::build(), crate::mock::Test}
