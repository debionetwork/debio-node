use frame_support::{parameter_types, traits::ConstU64, PalletId};
use pallet_balances::AccountData;
use scale_info::TypeInfo;
use sp_core::{Decode, Encode, RuntimeDebug, H256};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

use primitives_profile_roles::ProfileRoles;

#[derive(Clone, Copy, PartialEq, Eq, Encode, Decode, Default, RuntimeDebug, TypeInfo)]
pub struct EthereumAddress(pub [u8; 20]);

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub type AccountId = u64;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		Balances: pallet_balances,
		Labs: labs,
		Services: services,
		Certifications: certifications,
		UserProfile: user_profile,
		Orders: orders,
		GeneticTesting: genetic_testing,
		Timestamp: pallet_timestamp,
		RandomnessCollectiveFlip: pallet_randomness_collective_flip,
		Assets: pallet_assets,
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type AccountId = AccountId;
	type RuntimeCall = RuntimeCall;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
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
	pub const LabPalletId: PalletId = PalletId(*b"dbio/lab");
	pub const OrderPalletId: PalletId = PalletId(*b"dbio/ord");
}

impl pallet_timestamp::Config for Test {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = Moment;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

impl genetic_testing::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Orders = Orders;
	type RandomnessSource = RandomnessCollectiveFlip;
	type GeneticTestingWeightInfo = ();
}

impl pallet_randomness_collective_flip::Config for Test {}

impl orders::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Services = Services;
	type GeneticTesting = GeneticTesting;
	type Currency = Balances;
	type Assets = Assets;
	type OrdersWeightInfo = ();
	type PalletId = OrderPalletId;
}

type Balance = u64;

parameter_types! {
	pub static ExistentialDeposit: Balance = 0;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

pub type AssetId = u32;
pub type AssetBalance = u128;

parameter_types! {
	pub const ApprovalDeposit: Balance = 1;
	pub const AssetDeposit: Balance = 1;
	pub const MetadataDepositBase: Balance = 1;
	pub const MetadataDepositPerByte: Balance = 1;
	pub const StringLimit: u32 = 50;
}

impl pallet_assets::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = AssetBalance;
	type AssetId = AssetId;
	type Currency = Balances;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type AssetAccountDeposit = ConstU64<10>;
	type AssetDeposit = AssetDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = StringLimit;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = ();
}

impl labs::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type Services = Services;
	type Orders = Orders;
	type PalletId = LabPalletId;
	type Certifications = Certifications;
	type EthereumAddress = EthereumAddress;
	type ProfileRoles = ProfileRoles;
	type UserProfile = UserProfile;
	type LabWeightInfo = ();
}

impl services::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type ServiceOwner = Labs;
	type WeightInfo = ();
}

impl certifications::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type CertificationOwner = Labs;
	type WeightInfo = ();
}

impl user_profile::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type EthereumAddress = EthereumAddress;
	type ProfileRoles = ProfileRoles;
	type WeightInfo = ();
}

#[cfg(test)]
use sp_io::TestExternalities;

#[cfg(test)]
pub struct ExternalityBuilder {
	existential_deposit: u64,
}

#[cfg(test)]
impl Default for ExternalityBuilder {
	fn default() -> Self {
		Self { existential_deposit: 1 }
	}
}

#[cfg(test)]
impl ExternalityBuilder {
	pub fn existential_deposit(mut self, existential_deposit: u64) -> Self {
		self.existential_deposit = existential_deposit;
		self
	}
	pub fn set_associated_consts(&self) {
		EXISTENTIAL_DEPOSIT.with(|v| *v.borrow_mut() = self.existential_deposit);
	}
	pub fn build(&self) -> TestExternalities {
		self.set_associated_consts();
		let mut storage = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
		pallet_balances::GenesisConfig::<Test> { balances: { vec![] } }
			.assimilate_storage(&mut storage)
			.unwrap();
		let mut ext = sp_io::TestExternalities::new(storage);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}
