#![cfg(test)]

use frame_support::{parameter_types, PalletId};
use pallet_balances::AccountData;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

use primitives_ethereum_address::EthereumAddress;
use primitives_profile_roles::ProfileRoles;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub type AccountId = u64;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Orders: orders::{Pallet, Call, Storage, Event<T>},
		Labs: labs::{Pallet, Call, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		ServiceRequest: service_request::{Pallet, Call, Storage, Event<T>},
		Services: services::{Pallet, Call, Storage, Event<T>},
		Certifications: certifications::{Pallet, Call, Storage, Event<T>},
		UserProfile: user_profile::{Pallet, Call, Storage, Event<T>},
		GeneticTesting: genetic_testing::{Pallet, Call, Storage, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
		RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage},
		Assets: pallet_assets::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(1024);
}

impl pallet_randomness_collective_flip::Config for Test {}

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type AccountId = AccountId;
	type Call = Call;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Header = Header;
	type Event = Event;
	type Origin = Origin;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type AccountData = AccountData<Balance>;
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

pub type Moment = u64;
pub const MILLISECS_PER_BLOCK: Moment = 6000;
pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

parameter_types! {
	pub const MinimumPeriod: Moment = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Test {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = Moment;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

type Balance = u64;

parameter_types! {
	pub static ExistentialDeposit: Balance = 0;
	pub const LabPalletId: PalletId = PalletId(*b"dbio/lab");
	pub const OrderPalletId: PalletId = PalletId(*b"dbio/ord");
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

pub type AssetId = u32;
pub type AssetBalance = u128;

parameter_types! {
	pub const ApprovalDeposit: Balance = 1;
	pub const AssetAccountDeposit: Balance = 10;
	pub const AssetDeposit: Balance = 1;
	pub const MetadataDepositBase: Balance = 1;
	pub const MetadataDepositPerByte: Balance = 1;
	pub const StringLimit: u32 = 50;
}

impl pallet_assets::Config for Test {
	type Event = Event;
	type Balance = AssetBalance;
	type AssetId = AssetId;
	type Currency = Balances;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type AssetAccountDeposit = AssetAccountDeposit;
	type AssetDeposit = AssetDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = StringLimit;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = ();
}

parameter_types! {
	pub const UnstakePeriode: Moment = 0;
}

impl service_request::Config for Test {
	type Event = Event;
	type TimeProvider = Timestamp;
	type Currency = Balances;
	type Labs = Labs;
	type Orders = Orders;
	type Services = Services;
	type UnstakePeriode = UnstakePeriode;
	type ServiceRequestWeightInfo = ();
}

impl labs::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type Certifications = Certifications;
	type EthereumAddress = EthereumAddress;
	type Services = Services;
	type Orders = Orders;
	type ProfileRoles = ProfileRoles;
	type UserProfile = UserProfile;
	type PalletId = LabPalletId;
	type LabWeightInfo = ();
}

impl services::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type ServiceOwner = Labs;
	type WeightInfo = ();
}

impl orders::Config for Test {
	type Event = Event;
	type Services = Services;
	type GeneticTesting = GeneticTesting;
	type Currency = Balances;
	type Assets = Assets;
	type OrdersWeightInfo = ();
	type PalletId = OrderPalletId;
}

impl genetic_testing::Config for Test {
	type Event = Event;
	type Orders = Orders;
	type RandomnessSource = RandomnessCollectiveFlip;
	type GeneticTestingWeightInfo = ();
}

impl certifications::Config for Test {
	type Event = Event;
	type CertificationOwner = Labs;
	type WeightInfo = ();
}

impl user_profile::Config for Test {
	type Event = Event;
	type EthereumAddress = EthereumAddress;
	type ProfileRoles = ProfileRoles;
	type WeightInfo = ();
}
