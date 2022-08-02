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
		Labs: labs::{Pallet, Call, Storage, Event<T>},
		Services: services::{Pallet, Call, Storage, Event<T>},
		UserProfile: user_profile::{Pallet, Call, Storage, Event<T>},
		Orders: orders::{Pallet, Call, Storage, Config<T>, Event<T>},
		GeneticTesting: genetic_testing::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
	pub const LabPalletId: PalletId = PalletId(*b"dbio/lab");
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
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

type Balance = u64;

parameter_types! {
	pub const ExistentialDeposit: Balance = 10;
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

impl labs::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type Services = Services;
	type Orders = Orders;
	type Certifications = Certifications;
	type EthereumAddress = EthereumAddress;
	type ProfileRoles = ProfileRoles;
	type UserProfile = UserProfile;
	type LabWeightInfo = ();
	type PalletId = LabPalletId;
}

impl services::Config for Test {
	type Currency = Balances;
	type ServiceOwner = Labs;
}

impl user_profile::Config for Runtime {
	type Event = Event;
	type EthereumAddress = EthereumAddress;
}

impl genetic_testing::Config for Test {
	type Event = Event;
	type Orders = Orders;
	type RandomnessSource = RandomnessCollectiveFlip;
}

impl orders::Config for Test {
	type Event = Event;
	type Services = Services;
	type GeneticTesting = GeneticTesting;
	type Currency = Balances;
}

pub struct ExternalityBuilder;

impl ExternalityBuilder {
	pub fn build() -> TestExternalities {
		let mut storage = system::GenesisConfig::<Runtime>::default().build_storage().unwrap();
		storage.extend(
			GenesisConfig::<Runtime> {
				orders: OrdersConfig {
					escrow_key: hex![
						"18c79faa6203d8b8349b19cc72cc6bfd008c243ea998435847abf6618756ca0b"
					]
					.into(),
				},
			}
			.build_storage()
			.unwrap(),
		);
		storage.into()
	}
}
