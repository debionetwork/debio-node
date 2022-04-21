#![cfg(test)]

use super::*;

use frame_support::{parameter_types, PalletId};
use sp_io::TestExternalities;
use sp_runtime::{
	testing::Header,
	traits::{AccountIdLookup, IdentifyAccount, Verify},
	MultiSignature,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		GeneticData: genetic_data::{Pallet, Call, Storage, Event<T>},
		GeneticAnalysts: genetic_analysts::{Pallet, Call, Storage, Event<T>},
		GeneticAnalystServices: genetic_analyst_services::{Pallet, Call, Storage, Event<T>},
		GeneticAnalystQualifications: genetic_analyst_qualifications::{Pallet, Call, Storage, Event<T>},
		GeneticAnalysis: genetic_analysis::{Pallet, Call, Storage, Event<T>},
		GeneticAnalysisOrders: genetic_analysis_orders::{Pallet, Call, Storage, Config<T>, Event<T>},
		UserProfile: user_profile::{Pallet, Call, Storage, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
		RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage},
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
	type Call = Call;
	type Lookup = AccountIdLookup<AccountId, ()>;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = sp_core::H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type Header = sp_runtime::testing::Header;
	type Event = Event;
	type Origin = Origin;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type AccountData = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
}

impl pallet_timestamp::Config for Test {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = Moment;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

impl pallet_randomness_collective_flip::Config for Test {}

type Balance = u64;

parameter_types! {
	pub const ExistentialDeposit: Balance = 10;
	pub const GeneticAnalysisOrdersEscrowPalletId: PalletId = PalletId(*b"dbio/esc");
	pub const GeneticAnalysisOrdersEscrowPalletId: PalletId = PalletId(*b"dbio/esc");
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = Balance;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

impl genetic_data::Config for Test {
	type Event = Event;
	type GeneticDataWeightInfo = ();
}

impl genetic_analysts::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type GeneticAnalystServices = GeneticAnalystServices;
	type GeneticAnalystQualifications = GeneticAnalystQualifications;
	type EthereumAddress = EthereumAddress;
	type UserProfile = UserProfile;
	type GeneticAnalystWeightInfo = ();
}

impl genetic_analyst_services::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type GeneticAnalystServiceOwner = GeneticAnalysts;
	type WeightInfo = ();
}

impl genetic_analyst_qualifications::Config for Test {
	type Event = Event;
	type GeneticAnalystQualificationOwner = GeneticAnalysts;
	type WeightInfo = ();
}

impl genetic_analysis::Config for Test {
	type Event = Event;
	type RandomnessSource = RandomnessCollectiveFlip;
	type GeneticAnalysisOrders = GeneticAnalysisOrders;
	type GeneticAnalysisWeightInfo = ();
}

impl genetic_analysis_orders::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type GeneticData = GeneticData;
	type GeneticAnalysis = GeneticAnalysis;
	type GeneticAnalystServices = GeneticAnalystServices;
	type GeneticAnalysisOrdersWeightInfo = ();
	type PalletId = GeneticAnalysisOrdersEscrowPalletId;
}

impl user_profile::Config for Test {
	type Event = Event;
	type EthereumAddress = EthereumAddress;
}

pub struct ExternalityBuilder;

impl ExternalityBuilder {
	pub fn build() -> TestExternalities {
		let storage = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
		TestExternalities::from(storage)
	}
}
