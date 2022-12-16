use frame_support::{construct_runtime, parameter_types};
use pallet_balances::AccountData;
use scale_info::TypeInfo;
use sp_core::{Decode, Encode, RuntimeDebug, H256};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

#[derive(Clone, Copy, PartialEq, Eq, Encode, Decode, Default, RuntimeDebug, TypeInfo)]
pub struct EthereumAddress(pub [u8; 20]);

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub type AccountId = u64;

construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		HealthProfessional: health_professional::{Pallet, Call, Storage, Event<T>},
		HealthProfessionalQualification: health_professional_qualification::{Pallet, Call, Storage, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
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
pub const MILLISECS_PER_BLOCK: Moment = 10;
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

impl health_professional::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type HealthProfessionalQualifications = HealthProfessionalQualification;
	type HealthProfessionalWeightInfo = ();
}

impl health_professional_qualification::Config for Test {
	type Event = Event;
	type HealthProfessionalQualificationOwner = HealthProfessional;
	type WeightInfo = ();
}

#[cfg(test)]
use sp_io::TestExternalities;

#[cfg(test)]
use frame_system as system;

#[cfg(test)]
pub fn account_key(s: &str) -> u64 {
	match s {
		"admin" => 1,
		"customer" => 2,
		"doctor" => 3,
		_ => 4,
	}
}

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

		let mut storage = system::GenesisConfig::default().build_storage::<Test>().unwrap();

		let admin = account_key("admin");
		let customer = account_key("customer");
		let doctor = account_key("doctor");
		let other = account_key("other");

		pallet_balances::GenesisConfig::<Test> {
			balances: vec![(admin, 100), (customer, 200), (doctor, 300), (other, 400)],
		}
		.assimilate_storage(&mut storage)
		.unwrap();

		let mut ext = sp_io::TestExternalities::new(storage);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}
