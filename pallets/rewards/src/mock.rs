use crate as rewards;
use frame_support::parameter_types;
use sp_core::H256;
use sp_io::TestExternalities;
use sp_runtime::{
	testing::Header,
	traits::{AccountIdLookup, BlakeTwo256, IdentifyAccount, Verify},
    MultiSignature
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
		Rewards: rewards::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type AccountId = AccountId;
	type Call = Call;
	type Lookup = AccountIdLookup<AccountId, ()>;
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
	type AccountData = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
}

impl rewards::Config for Runtime {
    type Event = Event;
    type Currency = Balances;
    type Reward = ();
    type Slash = ();
}

pub struct ExternalityBuilder;

impl ExternalityBuilder {
	pub fn build() -> TestExternalities {
		let mut storage = system::GenesisConfig::<Runtime>::default().build_storage().unwrap();
		storage.extend(
			GenesisConfig::<Runtime> {
				rewards: RewardsConfig {
					rewarder_key: hex_literal::hex!["d86cd72037edd033b21e5a54fb8ecb687effe90e0af2d12c1c23acba2021ac56"].into(),
					total_reward_amount: 25_000_000 * 1_000_000_000_000_000_000,
				},
			}.build_storage()
				.unwrap()
		);
		storage.into()
	}
}
