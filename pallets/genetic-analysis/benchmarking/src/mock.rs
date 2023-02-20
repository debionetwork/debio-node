#![cfg(test)]

use frame_support::{parameter_types, traits::ConstU128, PalletId};
use frame_system::EnsureRoot;
use pallet_balances::AccountData;

use sp_runtime::{
	traits::{AccountIdLookup, IdentifyAccount, Verify},
	MultiSignature,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

use primitives_ethereum_address::EthereumAddress;
use primitives_profile_roles::ProfileRoles;

pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		Balances: pallet_balances,
		GeneticData: genetic_data,
		GeneticAnalysts: genetic_analysts,
		GeneticAnalystServices: genetic_analyst_services,
		GeneticAnalystQualifications: genetic_analyst_qualifications,
		GeneticAnalysis: genetic_analysis,
		GeneticAnalysisOrders: genetic_analysis_orders,
		UserProfile: user_profile,
		Timestamp: pallet_timestamp,
		RandomnessCollectiveFlip: pallet_randomness_collective_flip,
		OctopusAssets: pallet_assets::<Instance1>,
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
	type Lookup = AccountIdLookup<AccountId, ()>;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = sp_core::H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type Header = sp_runtime::testing::Header;
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

/// The native token, uses 18 decimals of precision.
pub mod currency {
	use super::Balance;

	pub const UNITS: Balance = 1_000_000_000_000_000_000;
	pub const DOLLARS: Balance = UNITS;
}

pub type OctopusAssetId = u32;
pub type OctopusAssetBalance = u128;

parameter_types! {
	pub const ApprovalDeposit: Balance = currency::DOLLARS;
	pub const AssetDeposit: Balance = 100 * currency::DOLLARS;
	pub const MetadataDepositBase: Balance = 10 * currency::DOLLARS;
	pub const MetadataDepositPerByte: Balance = currency::DOLLARS;
	pub const StringLimit: u32 = 50;
}

impl pallet_assets::Config<pallet_assets::Instance1> for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = OctopusAssetBalance;
	type AssetId = OctopusAssetId;
	type Currency = Balances;
	type ForceOrigin = EnsureRoot<AccountId>;
	type AssetAccountDeposit = ConstU128<{ currency::DOLLARS }>;
	type AssetDeposit = AssetDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = StringLimit;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = ();
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

impl pallet_randomness_collective_flip::Config for Test {}

type Balance = u128;

parameter_types! {
	pub const ExistentialDeposit: Balance = 10;
	pub const GeneticAnalystPalletId: PalletId = PalletId(*b"dbio/gen");
	pub const GeneticAnalysisOrdersEscrowPalletId: PalletId = PalletId(*b"dbio/esc");
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

impl genetic_data::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type GeneticDataWeightInfo = ();
}

impl genetic_analysts::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type PalletId = GeneticAnalystPalletId;
	type GeneticAnalysisOrders = GeneticAnalysisOrders;
	type GeneticAnalystServices = GeneticAnalystServices;
	type GeneticAnalystQualifications = GeneticAnalystQualifications;
	type EthereumAddress = EthereumAddress;
	type ProfileRoles = ProfileRoles;
	type UserProfile = UserProfile;
	type GeneticAnalystWeightInfo = ();
}

impl genetic_analyst_services::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type GeneticAnalystServiceOwner = GeneticAnalysts;
	type WeightInfo = ();
}

impl genetic_analyst_qualifications::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type GeneticAnalystQualificationOwner = GeneticAnalysts;
	type WeightInfo = ();
}

impl genetic_analysis::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RandomnessSource = RandomnessCollectiveFlip;
	type GeneticAnalysisOrders = GeneticAnalysisOrders;
	type GeneticAnalysisWeightInfo = ();
}

impl genetic_analysis_orders::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type Assets = OctopusAssets;
	type GeneticData = GeneticData;
	type GeneticAnalysts = GeneticAnalysts;
	type GeneticAnalysis = GeneticAnalysis;
	type GeneticAnalystServices = GeneticAnalystServices;
	type GeneticAnalysisOrdersWeightInfo = ();
	type PalletId = GeneticAnalysisOrdersEscrowPalletId;
}

impl user_profile::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type EthereumAddress = EthereumAddress;
	type ProfileRoles = ProfileRoles;
	type WeightInfo = ();
}
