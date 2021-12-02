use beefy_primitives::crypto::AuthorityId as BeefyId;
use debio_runtime::{
	currency::UNITS as DBIO,
	opaque::{Block, SessionKeys},
	AccountId, BabeConfig, Balance, BalancesConfig, GenesisConfig, LabsConfig, UserProfileConfig,
	OctopusAppchainConfig, OctopusLposConfig, OrdersConfig, RewardsConfig, ServiceRequestConfig,
	SessionConfig, Signature, SudoConfig, SystemConfig, BABE_GENESIS_EPOCH_CONFIG, WASM_BINARY,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_octopus_appchain::AuthorityId as OctopusId;
use sc_chain_spec::ChainSpecExtension;
use sc_service::{ChainType, Properties};
use serde::{Deserialize, Serialize};
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};

use hex_literal::hex;

use frame_support::PalletId;
use sp_runtime::traits::AccountIdConversion;

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
	/// The light sync state extension used by the sync-state rpc.
	pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

fn session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	beefy: BeefyId,
	octopus: OctopusId,
) -> SessionKeys {
	SessionKeys { babe, grandpa, im_online, beefy, octopus }
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(
	seed: &str,
	stash_amount: Balance,
) -> (AccountId, BabeId, GrandpaId, ImOnlineId, BeefyId, OctopusId, Balance) {
	(
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<BeefyId>(seed),
		get_from_seed::<OctopusId>(seed),
		stash_amount,
	)
}

/// Helper function to generate an properties
pub fn get_properties(symbol: &str, decimals: u32, ss58format: u32) -> Properties {
	let mut properties = Properties::new();
	properties.insert("tokenSymbol".into(), symbol.into());
	properties.insert("tokenDecimals".into(), decimals.into());
	properties.insert("ss58Format".into(), ss58format.into());

	properties
}

/// Helper function to generate appchain config
pub fn appchain_config(
	anchor_contract: &str,
	asset_id_by_name: &str,
	premined_amount: Balance,
	era_payout: Balance,
) -> (String, String, Balance, Balance) {
	(anchor_contract.to_string(), asset_id_by_name.to_string(), premined_amount, era_payout)
}

pub fn octopus_mainnet_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../../resources/mainnet.json")[..])
}

pub fn octopus_testnet_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../../resources/testnet.json")[..])
}

pub fn mainnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?;
	let properties = get_properties("DBIO", 18, 42);
	let total_reward_balance = 25_000_000 * DBIO;

	Ok(ChainSpec::from_genesis(
		// Name
		"DeBio",
		// ID
		"debio",
		ChainType::Live,
		move || {
			genesis(
				// WASM Binary
				wasm_binary,
				// Sudo account
				// 5Eqe2cvQPYJRbpQ3BGgVYYZn43zyGJatXNfCw36hZaiNU3LC
				hex!["7abd9f5aa885700a6a7c91fb5d6409a0c697cce3018c0191c398f3e15c64847e"].into(),
				// Initial PoA authorities
				vec![
					(
						// 5Eqe2cvQPYJRbpQ3BGgVYYZn43zyGJatXNfCw36hZaiNU3LC
						hex!["7abd9f5aa885700a6a7c91fb5d6409a0c697cce3018c0191c398f3e15c64847e"]
							.into(),
						// 5Giq6cT3PREcXj2U3d9JNqcq6B9V6LvEmDsyK2kLJvyzmCS8
						hex!["ce04a1ee223de8cbc6245da685d1148daa7bf3fe634168a8eb4553cbc5ff943a"]
							.unchecked_into(),
						// 5HB3gUbYrKQEFseuR8Ky65NxxHvn7vS34tabe2nCoZZ8d8sV
						hex!["e2035fc7e3d8e0b65dec0c79886428b42707451735b49f96f05431f92e826ce1"]
							.unchecked_into(),
						// 5CXv6qkhma5V1U5tQw3xr5xaBywCbVqBTtQjW5fn1FmiA6Yn
						hex!["14c0d8168ae31b1aa2bb410834b495a15d49e352dd2f7d98b64cbbbc423f7934"]
							.unchecked_into(),
						// KW7ZQom54xAXrm6ywL3Up6XTvViKVE7ZjMe4rTcUu4oYV2JvZ
						hex!["02ccfa1aef0c9f012f055e216ae7546556cadd09896215edbf837b07cb6362b1f8"]
							.unchecked_into(),
						// 5CiN6MhnULuztDnFiCx4XBWDsEWVSgu3cukBF9Q7SRWRCXXj
						hex!["1cb8cb3b111f12e7b30330641094ea13a2f32800416922abbf9f4de182aab07e"]
							.unchecked_into(),
						// Stash amount
						100 * DBIO,
					),
					(
						// 5DwE8Hedc1WkgbmWQSv4CrHurDPRyR85BVmhC1fNhLfGtbsB
						hex!["52c466320e6f6f9a37d24250b04ae4bc0b15f5416dd01d580d25355958230a39"]
							.into(),
						// 5CqZgRLiyJMqhK7W53r4aQ2rzWZ1jarHr1HFd43oX6KCFhY4
						hex!["2236849aeeefef86ef93f73db747ce312608cae2bba80ca2ed720f9db3113773"]
							.unchecked_into(),
						// 5EJ7JGhKyRcZLJsQbT2gdFcNk2Lkiyo2FgPHWVVhRZS4jcfs
						hex!["62b1916ab9f422ba36c3d962f964d8fa6ea6d99331c08b953c2723b4e0ec5986"]
							.unchecked_into(),
						// 5FULzzASLz6SbLEaxvDefwuqvvb8w6XzTqEJ2DMSNopDAZy4
						hex!["96bc616963a5d997aa14d4bb57c9ba3c956709b3a4c78bf5847437defcc1884d"]
							.unchecked_into(),
						// KW9z6vxHeTgBbBKJaCoDBQC7Wb43CgUEZQ3mzPe5pobB4N3Yc
						hex!["0338478804f8f14bcaab2b7c3ec85481ac4c59994ff5f92125901465b6b7c177e0"]
							.unchecked_into(),
						// 5CvXhikX2VJ7DLaf9Nto7fJWZ3PyP3orPyjHF1CH5fBX6ACZ
						hex!["2600167ed0dad19e81de33ebba35ba8945e4fd0ef4da40fbc760294391332132"]
							.unchecked_into(),
						// Stash amount
						100 * DBIO,
					),
					(
						// 5EfNPLqe76zfPQtLemkqCjMYYAjPCmdGW3jJfd4P7sfoMC8r
						hex!["72e882a7bc0a3d545fbc7189694fd752351e58cd934ee931d0ead5c605fd0003"]
							.into(),
						// 5HEZ8CGV2uNyrtQGz3Y6ZHpVR7QwLArSKZnraLq6yWUeVjRg
						hex!["e4b039cac52f2a2d3326b15473eca58022be2a50c5a7ccad159a75a6f2ad9c4d"]
							.unchecked_into(),
						// 5CJU2aPav9MSfKNHBzhMasC5hTNkEhGNiZkXTu9eH9JX2JGQ
						hex!["0a7ee1eba7f93ad5eabfee8b2e56b8933022acd51b27d6b7d1309b142e6d562a"]
							.unchecked_into(),
						// 5HGWB6TtjsGmKuokpB2NwyH2FSFX9uWSXsG6VFDESJjX5M79
						hex!["e62cc9355c8362a41263bbca72f6373b8248f419085d2447aa7aee53d3f05b6a"]
							.unchecked_into(),
						// KW7PS5VWU3CFHvc4P8yuz3FrnFn9MAY9AEorwE2RJhs3E5NLc
						hex!["02c55dee371aa4ad6c2eb7b030354d38e2be227f8f221c440a83b2b0a71d42f8f9"]
							.unchecked_into(),
						// 5H3DcYd1BBHU2vKMS5oFiW3sxJFeXKFYTgK3aqtMVQvfLNGN
						hex!["dc0ad884f6fa637f99a93665f27c022b2c6efad1c42ea3bb15f1d0edc3c24164"]
							.unchecked_into(),
						// Stash amount
						100 * DBIO,
					),
					(
						// 5HbEiquaeAfPSAj5X8MsUSAByD3Pq6zHKXzGscXyrM1mZKGG
						hex!["f4766e38cb1ee4f254f1f6c2f536914f9fed811cd6de21621ef1aa06ab23ea41"]
							.into(),
						// 5F9PNKLau1G817Y8nKAVKJFvyna8g71rJbegXgAcWzixxZqH
						hex!["8846b03e6c2c53eec0288e00c1f0d5b8b87e0f34dd7495e355b922c8ff41d630"]
							.unchecked_into(),
						// 5HgHkNZhiuFnu8KH5WetSm21q2wVq1NxHTT3AdjPfDoTG3xp
						hex!["f850d85ea2c7f440f04361a614f42ddf93d82490c5a9bb9e10efd966d1aaea1e"]
							.unchecked_into(),
						// 5DfhkVDwi3zcbDQajpfZXJkfPVdhR5VpMi1r89bW88fAWpfT
						hex!["46ed76be7187614a109d45a8528a6e0c6e4f7c05c1f035b6a49764374bedd40b"]
							.unchecked_into(),
						// KWDwYPXkvLXeZBpuVdnjmPPH7Zjxz3BsfeuCDoB1xA1gtEwcc
						hex!["03e7446dcfb7d23bbb4a98a7fe93ea9e4be5ae1734e3d7d96645c9432db5dedb37"]
							.unchecked_into(),
						// 5Dm2RRgAyoxeFPpDj1D6LgtVEpLEW1S26nRkFZBkQi5j67Hq
						hex!["4afc8b54d1dae1f069fc4a976dc9d11c1f71ca3ef8d878a216fdd6a695705b55"]
							.unchecked_into(),
						// Stash amount
						100 * DBIO,
					),
				],
				// Pre-funded accounts
				vec![
					(
						// Octopus Foundation Valiadator 1 account
						// 5Eqe2cvQPYJRbpQ3BGgVYYZn43zyGJatXNfCw36hZaiNU3LC
						hex!["7abd9f5aa885700a6a7c91fb5d6409a0c697cce3018c0191c398f3e15c64847e"]
							.into(),
						// Balance amount
						510 * DBIO,
					),
					(
						// Octopus Foundation Valiadator 2 account
						// 5DwE8Hedc1WkgbmWQSv4CrHurDPRyR85BVmhC1fNhLfGtbsB
						hex!["52c466320e6f6f9a37d24250b04ae4bc0b15f5416dd01d580d25355958230a39"]
							.into(),
						// Balance amount
						10 * DBIO,
					),
					(
						// Octopus Foundation Valiadator 3 account
						// 5EfNPLqe76zfPQtLemkqCjMYYAjPCmdGW3jJfd4P7sfoMC8r
						hex!["72e882a7bc0a3d545fbc7189694fd752351e58cd934ee931d0ead5c605fd0003"]
							.into(),
						// Balance amount
						10 * DBIO,
					),
					(
						// Octopus Foundation Valiadator 4 account
						// 5HbEiquaeAfPSAj5X8MsUSAByD3Pq6zHKXzGscXyrM1mZKGG
						hex!["f4766e38cb1ee4f254f1f6c2f536914f9fed811cd6de21621ef1aa06ab23ea41"]
							.into(),
						// Balance amount
						10 * DBIO,
					),
					(
						// Pallet ID Account
						PalletId(*b"Rewards!").into_account(),
						// Pallet ID rewards amount
						total_reward_balance,
					),
				],
				// Appchain config
				appchain_config(
					// Appchain Relay Contract
					"debionetwork.octopus-registry.near",
					// Appchain Asset Id by Name
					"usdc.testnet",
					// Premined Amount
					87_500_000 * DBIO,
					// Era Payout
					4_657 * DBIO,
				),
				// API admin account
				// 5EU2uVTEVWkAsLj7fTkkxk72BBDk1bJPDSrPK4Xh1EfEWprA
				hex!["6a433c17606024462262cfa318b4f45f929b5d807e91ab1793b03172a4e8005b"].into(),
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("debio"),
		// Properties
		Some(properties),
		// Extensions
		Default::default(),
	))
}

pub fn testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?;
	let properties = get_properties("DBIO", 18, 42);
	let total_reward_balance = 25_000_000 * DBIO;

	Ok(ChainSpec::from_genesis(
		// Name
		"DeBio Testnet",
		// ID
		"debio_testnet",
		ChainType::Live,
		move || {
			genesis(
				// WASM Binary
				wasm_binary,
				// Sudo account
				// 5FR73HBVwSpPjnPsBZPDVyuHQS1KE8jvSL3pSud6F6HZcuBA
				hex!["9443a63297b9f5b4e2569ee17225011db11a537066bce62d018acbcfda88f947"].into(),
				// Initial PoA authorities
				vec![
					(
						// 5FRzbdg5WEQQPu34pdowRehCfA4rgZuDQE4bQEbcWGnthegY
						hex!["94f135526ec5fe830e0cbc6fd58683cb2d9ee06522cd9a2c0481268c5c73674f"]
							.into(),
						// 5GQqSQ3R8DxUJTpRPPCJZv4foEYrLpDn3858oYSivB21XXB8
						hex!["c04b5d176ccb7ff1bf8014baae55614ae998c4d9c476598a95869216dfd3c356"]
							.unchecked_into(),
						// 5GEGWDRGKRJyjj7rvt8RWU7AcjgVz7TwXPnebTqr8FQugir6
						hex!["b83c0b10820f068051e62f1362b681f1539567f6225f55925bd5d7b53fa0e6d0"]
							.unchecked_into(),
						// 5GYqx4xhtHDNk4Lnq3SDVpCUakayFGaEgNNyhtky4FaRMZ8Q
						hex!["c6670c893b8bc9f9b883d40d0ea1a8dbce71ce3de8d57cf467f1026cb927384c"]
							.unchecked_into(),
						// KWDm4WP68DG4Kg3ADTmZQmPWuWp74xf7C25ydqrhxtwk6m9h1
						hex!["03df461db93e4434e2f629134b171d3b2e075ad629839f1c697b547750f1e07248"]
							.unchecked_into(),
						// 5FZuGnwJVzhHDfqz9GYXuopc86xqqJk2oQ98UBRqmVcXvjJd
						hex!["9af93f1d2cf773015a1baee1028edb612a25f56fde130f5da2ac488c2884e619"]
							.unchecked_into(),
						// Stash amount
						100 * DBIO,
					),
					(
						// 5GVtTAfgTZrRf4gs2fDmGMPrCUWZx4eD7gzYvoCSxCuke67t
						hex!["c425bbf59c7bf49e4fcc6547539d84ba8ecd2fb171f5b83cde3571d45d0c8224"]
							.into(),
						// 5DJ474dCD9qGx8r8tCXkJUchwS6dEFAvBzLizoXyVsXpHCuE
						hex!["366a95cd6d3a75b2e7094b3cfc0ef40d3d5ad37e9394667a57ec2270899f9074"]
							.unchecked_into(),
						// 5DBw1qYaGN8DoD6y2uYizf12qFHRsB2u5fonhaW4tNVsrzgb
						hex!["31bf3ecf6785449ee1fa52cebc512e3638399e889c1a72640d836378d3b99889"]
							.unchecked_into(),
						// 5DCQDScgxoSqEqqXHqoDGGCeoPhJuuCaxuUBKo2oqjGNWnUE
						hex!["321acf19ab3edaaf90601c4fff9e8ad0e9e63dc19fb39915ff0776e10c5d4e62"]
							.unchecked_into(),
						// KW54T4BS4gQaa5vZX9ykSkBr9bY4MdTSzMaWnUPijwRUUEh8p
						hex!["025e6b175317ab988d6d18f8e083e9b3bcf3de711798228044274c8d8ff85f0f10"]
							.unchecked_into(),
						// 5D7J3Gnjyt1hzRRX1n2g89tpjhhYspvtgLxMVLdVv8DV44NL
						hex!["2e35cb3f34b1e1d456981f4cc39a57b35a5287146925a9a95b66feec76b3e339"]
							.unchecked_into(),
						// Stash amount
						100 * DBIO,
					),
					(
						// 5Gs3JPQknr3LbPuP5kuva2XxijodjU1sAAWncES719ZfuhCq
						hex!["d447acbfe7761c0cfba8341e616275caca6401637308ee123b77082a40095331"]
							.into(),
						// 5Dyr6DRdQrDGe1cidZ4qnv5k8sAVyLkzqyVH9xDU1t7HGgyg
						hex!["54c3f4dd996025cf3a0f793ce058a0c614541ef6fccb939d3483fff21065b374"]
							.unchecked_into(),
						// 5DDUj1ez9MWqGo32KXNLCFYgBTqNvKzbKQ2FJVAK5KWYBZ5d
						hex!["32ed3c5d8a3ee2589b9ea0a53593a606d866385e73aa2d1f6cfe0f1fdf06702f"]
							.unchecked_into(),
						// 5H9YaNETz66q5oXc9D1Z2RTSCvXhdqfgfDMdZpGd34hqWFSk
						hex!["e0de26c436aeb19b1b66a22976914134df559f66583962c0a78367e4e095d341"]
							.unchecked_into(),
						// KW7xD3xSTGLeNgi42SJ9MfSsv3u1rv8zBCdtGcm8vG8VkZFr2
						hex!["02de5d2d86fffbdb27187a58354d130aa6ff876ee5a2114f2c42b8b147d3753272"]
							.unchecked_into(),
						// 5E2Lp4CRph6cYTmq1P8sUyQPP22P3FzFT5FegwovLgwc2bYF
						hex!["56ab21d481aeb3f93b756bbd78b0e38c4db567b1663b75c7d672bd3f7eaefb5d"]
							.unchecked_into(),
						// Stash amount
						100 * DBIO,
					),
					(
						// 5DXNnoH2EMDQ1kk2JM7ZEutZFqZD8m6A6E5wZKfJVXjoJEez
						hex!["4093acd03283fa2d53d3b684b2a7ce3118ceb047b869f6c000d041578420de22"]
							.into(),
						// 5HTtvH6s59JKYyuSoc9CUV6V9WRZWVv6HsC4Xo4yXrZtQ9zX
						hex!["eedd0ce50d355a9590ba7b3ac0f4116e00bfe5ed1684d7eef630371dbd018110"]
							.unchecked_into(),
						// 5GFmQyUvApYnZRYqRwTosBQXrQrRwE42SFrV7KQ2UFBDx2dg
						hex!["b9609b704d6f8b242c51ee79b4b23c7c09350bda96c0d01e6c214d26a778e8f4"]
							.unchecked_into(),
						// 5FH4WtPN93KETuHWRn4q7oikRjPozP4cXtreXVia1oQ3eaQf
						hex!["8e21307acc160a81d92addae65e26b0bb047a016bd881eb234b6db693b09a074"]
							.unchecked_into(),
						// KWALuXvqYgmoPymKwg8v5qNSckbeFLGEXGjq2LeS2LMjo7vFM
						hex!["03482551b40c34c48a8d84ba1b0a3f48eb230cf923b1c21dafbc1d7295212e4a88"]
							.unchecked_into(),
						// 5FJAQA33PTTwr6VNGKecxJjyiL6jN5oPHG77WRdinj97LEPb
						hex!["8ef83df12c339f41551d44a83bf82c6dfbf944bb2c6caa8b4761600b09dc324a"]
							.unchecked_into(),
						// Stash amount
						100 * DBIO,
					),
				],
				// Pre-funded accounts
				vec![
					(
						// Sudo account
						// 5FR73HBVwSpPjnPsBZPDVyuHQS1KE8jvSL3pSud6F6HZcuBA
						hex!["9443a63297b9f5b4e2569ee17225011db11a537066bce62d018acbcfda88f947"]
							.into(),
						// Balance amount
						510 * DBIO,
					),
					(
						// Octopus Foundation Valiadator 1 account
						// 5FRzbdg5WEQQPu34pdowRehCfA4rgZuDQE4bQEbcWGnthegY
						hex!["94f135526ec5fe830e0cbc6fd58683cb2d9ee06522cd9a2c0481268c5c73674f"]
							.into(),
						// Balance amount
						10 * DBIO,
					),
					(
						// Octopus Foundation Valiadator 2 account
						// 5GVtTAfgTZrRf4gs2fDmGMPrCUWZx4eD7gzYvoCSxCuke67t
						hex!["c425bbf59c7bf49e4fcc6547539d84ba8ecd2fb171f5b83cde3571d45d0c8224"]
							.into(),
						// Balance amount
						10 * DBIO,
					),
					(
						// Octopus Foundation Valiadator 3 account
						// 5Gs3JPQknr3LbPuP5kuva2XxijodjU1sAAWncES719ZfuhCq
						hex!["d447acbfe7761c0cfba8341e616275caca6401637308ee123b77082a40095331"]
							.into(),
						// Balance amount
						10 * DBIO,
					),
					(
						// Octopus Foundation Valiadator 4 account
						// 5DXNnoH2EMDQ1kk2JM7ZEutZFqZD8m6A6E5wZKfJVXjoJEez
						hex!["4093acd03283fa2d53d3b684b2a7ce3118ceb047b869f6c000d041578420de22"]
							.into(),
						// Balance amount
						10 * DBIO,
					),
					(
						// Pallet ID Account
						PalletId(*b"Rewards!").into_account(),
						// Pallet ID rewards amount
						total_reward_balance,
					),
				],
				// Appchain config
				appchain_config(
					// Appchain Relay Contract
					"debionetwork.registry.test_oct.testnet",
					// Appchain Asset Id by Name
					"usdc.testnet",
					// Premined Amount
					87_500_000 * DBIO,
					// Era Payout
					4_657 * DBIO,
				),
				// API admin account
				// 5FpcRYvUMB3bNRdbj5YDwKeGHKVeWmdjnzY45RdTJSoSGcKN
				hex!["a63135764844b7b889f0447cc5127c4aa1b78fb998878549bf66ed7b0ee49753"].into(),
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("debio-testnet"),
		// Properties
		Some(properties),
		// Extensions
		Default::default(),
	))
}

pub fn staging_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?;
	let properties = get_properties("DBIO", 18, 42);
	let total_reward_balance = 25_000_000 * DBIO;

	Ok(ChainSpec::from_genesis(
		// Name
		"Debio Staging Testnet",
		// ID
		"debio_staging_testnet",
		ChainType::Live,
		move || {
			genesis(
				// WASM Binary
				wasm_binary,
				// Sudo account
				// 5CB5udaxY6zFqApVHWPQTGTW5FszotkXKAUD48fvi5Y7FSR2
				hex!["04ddb3f730857ed801327da2242dff4d4d85e25b33c43db6f328d55904247f40"].into(),
				// Initial PoA authorities
				vec![
					(
						// 5DWyDncRWXBuQHwJkwndcxD8EpiNjC5aUpkvQvH5pKWW31kS
						hex!["4044558c867f510c90406c029d4132552cff769af982df767536607126f20b3e"]
							.into(),
						// 5DWyDncRWXBuQHwJkwndcxD8EpiNjC5aUpkvQvH5pKWW31kS
						hex!["4044558c867f510c90406c029d4132552cff769af982df767536607126f20b3e"]
							.unchecked_into(),
						// 5F3w32CUTSdx6tVtKqtmX5ySxv2EWtf5ALozHXdm1sRmDRyn
						hex!["841dd15656fe6f518d7f834be42c8ebac03856b973fe0cfe884d3bf63f54044e"]
							.unchecked_into(),
						// 5DWyDncRWXBuQHwJkwndcxD8EpiNjC5aUpkvQvH5pKWW31kS
						hex!["4044558c867f510c90406c029d4132552cff769af982df767536607126f20b3e"]
							.unchecked_into(),
						// KWE8SLkhBFQDYC6mp9BXGFvtHHoXWgVspsnrHhXXwGxdd52J1
						hex!["03ef93c4f8f2b34f0945ac57f666555f9a6a211cbb7f21118dfc8049100347162d"]
							.unchecked_into(),
						// 5DWyDncRWXBuQHwJkwndcxD8EpiNjC5aUpkvQvH5pKWW31kS
						hex!["4044558c867f510c90406c029d4132552cff769af982df767536607126f20b3e"]
							.unchecked_into(),
						// Stash amount
						100 * DBIO,
					),
					(
						// 5CaJm3bpWi3ieWYHcbz4xd7MrF8Njma4p7tGTBwemRbYnknT
						hex!["16939c61baa637549e3a90277790655b5c5ce0e60ea9688559f9da587b2cb419"]
							.into(),
						// 5CaJm3bpWi3ieWYHcbz4xd7MrF8Njma4p7tGTBwemRbYnknT
						hex!["16939c61baa637549e3a90277790655b5c5ce0e60ea9688559f9da587b2cb419"]
							.unchecked_into(),
						// 5Ca1whan2BfjBU4JYkcK2HwkkJxcgUvgkXHd1q3sn1kMM2iX
						hex!["165b008e2e3181f869893b237406e847daa043d90bf80e62bb8a8442281afac0"]
							.unchecked_into(),
						// 5CaJm3bpWi3ieWYHcbz4xd7MrF8Njma4p7tGTBwemRbYnknT
						hex!["16939c61baa637549e3a90277790655b5c5ce0e60ea9688559f9da587b2cb419"]
							.unchecked_into(),
						// KW6TMKohWcZBna5xBkcthEuGX1p5R7gn6z8eqCMBDjEWBLUbA
						hex!["029c1ead8e295430573bb984b8b38c9479b7a9a236725d7c2090182fd38bf4d9b5"]
							.unchecked_into(),
						// 5CaJm3bpWi3ieWYHcbz4xd7MrF8Njma4p7tGTBwemRbYnknT
						hex!["16939c61baa637549e3a90277790655b5c5ce0e60ea9688559f9da587b2cb419"]
							.unchecked_into(),
						// Stash amount
						100 * DBIO,
					),
				],
				// Pre-funded accounts
				vec![
					(
						// Sudo account
						// 5CB5udaxY6zFqApVHWPQTGTW5FszotkXKAUD48fvi5Y7FSR2
						hex!["04ddb3f730857ed801327da2242dff4d4d85e25b33c43db6f328d55904247f40"]
							.into(),
						// Balance amount
						12_498_000 * DBIO,
					),
					(
						// Valiadator 1 account
						// 5DWyDncRWXBuQHwJkwndcxD8EpiNjC5aUpkvQvH5pKWW31kS
						hex!["4044558c867f510c90406c029d4132552cff769af982df767536607126f20b3e"]
							.into(),
						// Balance amount
						1_000 * DBIO,
					),
					(
						// Valiadator 2 account
						// 5CaJm3bpWi3ieWYHcbz4xd7MrF8Njma4p7tGTBwemRbYnknT
						hex!["16939c61baa637549e3a90277790655b5c5ce0e60ea9688559f9da587b2cb419"]
							.into(),
						// Balance amount
						1_000 * DBIO,
					),
					(
						// Pallet ID Account
						PalletId(*b"Rewards!").into_account(),
						// Pallet ID rewards amount
						total_reward_balance,
					),
				],
				// Appchain config
				appchain_config(
					// Appchain Relay Contract
					"",
					// Appchain Asset Id by Name
					"usdc.testnet",
					// Premined Amount
					87_500_000 * DBIO,
					// Era Payout
					4_657 * DBIO,
				),
				// API admin account
				// 5ELYNFhFz9tauMxfjgTGhd6sRbnndddEXqh3UxWsPi6Rjajg
				hex!["648c728f7fcf0ae26a44410cf0ba4ea15b27b3169a4f809a14097680b8d0bc53"].into(),
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("debio-staging-testnet"),
		// Properties
		Some(properties),
		// Extensions
		Default::default(),
	))
}

pub fn development_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?;
	let properties = get_properties("DBIO", 18, 42);
	let total_reward_balance = 25_000_000 * DBIO;

	Ok(ChainSpec::from_genesis(
		// Name
		"DeBio Development Testnet",
		// ID
		"debio_development_testnet",
		ChainType::Live,
		move || {
			genesis(
				// WASM Binary
				wasm_binary,
				// Sudo account
				// 5G3nLeySH5sFzD9WPKt2kB3KNVnazsZykaFfotouvjf1RZWY
				hex!["b03cc727c3c98eab988e5acfa815f6e6ed1939060471adaa78d2e39bbb1fc50b"].into(),
				// Initial PoA authorities
				vec![
					(
						// 5FNUtTJn1hhx1JEBrtWz9yaGx7M19hGhWZonxaJFHFu6SQ6C
						hex!["92437599810542e6c9e435290225920cb7b8174a949ed8f67b3413c6435ad76c"]
							.into(),
						// 5FNUtTJn1hhx1JEBrtWz9yaGx7M19hGhWZonxaJFHFu6SQ6C
						hex!["92437599810542e6c9e435290225920cb7b8174a949ed8f67b3413c6435ad76c"]
							.unchecked_into(),
						// 5DZQ8hkpX2STvCDKxnisDS4M3wKr8T4irH7Kb6pi1opYWicR
						hex!["421eaffb5d5601b080f546fa8be621d26085a2743b4d935d2b8dd83c2cecaa39"]
							.unchecked_into(),
						// 5FNUtTJn1hhx1JEBrtWz9yaGx7M19hGhWZonxaJFHFu6SQ6C
						hex!["92437599810542e6c9e435290225920cb7b8174a949ed8f67b3413c6435ad76c"]
							.unchecked_into(),
						// KW39i1yj3MYMcCaF5QZUbk8FBPbEzbrn1E6A3Xdmw4beErUGT
						hex!["0209f537ca85f50055cf9553d72c8a594516a915b6c040109ed5450da0185c3ff1"]
							.unchecked_into(),
						// 5FNUtTJn1hhx1JEBrtWz9yaGx7M19hGhWZonxaJFHFu6SQ6C
						hex!["92437599810542e6c9e435290225920cb7b8174a949ed8f67b3413c6435ad76c"]
							.unchecked_into(),
						// Stash amount
						100 * DBIO,
					),
					(
						// 5DF6RP41YxxgE8yemXAH47aJo9313TG7pVvx1utM4a9WnKk5
						hex!["3428a50b8746e28304b67a2a8dfd5fc40c0ee17c28ce129c5db1ac42c4e9905a"]
							.into(),
						// 5DF6RP41YxxgE8yemXAH47aJo9313TG7pVvx1utM4a9WnKk5
						hex!["3428a50b8746e28304b67a2a8dfd5fc40c0ee17c28ce129c5db1ac42c4e9905a"]
							.unchecked_into(),
						// 5CetaryC3UwJEwSJvo8GzLVM4kxejioSfjmoZyAX4TKPSNuq
						hex!["1a1274a58903a684d89cd926735137961a795d798b250926f7c8867b487549d8"]
							.unchecked_into(),
						// 5DF6RP41YxxgE8yemXAH47aJo9313TG7pVvx1utM4a9WnKk5
						hex!["3428a50b8746e28304b67a2a8dfd5fc40c0ee17c28ce129c5db1ac42c4e9905a"]
							.unchecked_into(),
						// KW2ywDzHXAmvvcCZu14szXHdsXka9Xuez4Q1RPuXMkw2VTZYk
						hex!["020281390b3b2a5f25dcda82477a2da7a00a2570724b24d60e82446a63f81db4c7"]
							.unchecked_into(),
						// 5DF6RP41YxxgE8yemXAH47aJo9313TG7pVvx1utM4a9WnKk5
						hex!["3428a50b8746e28304b67a2a8dfd5fc40c0ee17c28ce129c5db1ac42c4e9905a"]
							.unchecked_into(),
						// Stash amount
						100 * DBIO,
					),
				],
				// Pre-funded accounts
				vec![
					(
						// Sudo account
						// 5G3nLeySH5sFzD9WPKt2kB3KNVnazsZykaFfotouvjf1RZWY
						hex!["b03cc727c3c98eab988e5acfa815f6e6ed1939060471adaa78d2e39bbb1fc50b"]
							.into(),
						// Balance amount
						12_498_000 * DBIO,
					),
					(
						// Valiadator 1 account
						// 5FNUtTJn1hhx1JEBrtWz9yaGx7M19hGhWZonxaJFHFu6SQ6C
						hex!["92437599810542e6c9e435290225920cb7b8174a949ed8f67b3413c6435ad76c"]
							.into(),
						// Balance amount
						1_000 * DBIO,
					),
					(
						// Valiadator 2 account
						// 5DF6RP41YxxgE8yemXAH47aJo9313TG7pVvx1utM4a9WnKk5
						hex!["3428a50b8746e28304b67a2a8dfd5fc40c0ee17c28ce129c5db1ac42c4e9905a"]
							.into(),
						// Balance amount
						1_000 * DBIO,
					),
					(
						// Pallet ID Account
						PalletId(*b"Rewards!").into_account(),
						// Pallet ID rewards amount
						total_reward_balance,
					),
				],
				// Appchain config
				appchain_config(
					// Appchain Relay Contract
					"",
					// Appchain Asset Id by Name
					"usdc.testnet",
					// Premined Amount
					87_500_000 * DBIO,
					// Era Payout
					4_657 * DBIO,
				),
				// API admin account
				// C8KpmHUFT7HJbNLv74cXrtT1w9LF1W3WduN8nVGQUySSJTF
				hex!["02c2cffef38fbf56b32d6a49eeeecc0e3345a1e0549cd8817d52f6cf2e414152"].into(),
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("debio-development-testnet"),
		// Properties
		Some(properties),
		// Extensions
		Default::default(),
	))
}

pub fn local_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?;
	let properties = get_properties("DBIO", 18, 42);
	let total_reward_balance = 25_000_000 * DBIO;

	Ok(ChainSpec::from_genesis(
		// Name
		"DeBio Local",
		// ID
		"debio_local",
		ChainType::Local,
		move || {
			genesis(
				// WASM Binary
				wasm_binary,
				// Sudo account
				// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Initial PoA authorities
				vec![
					// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
					authority_keys_from_seed("Alice", 100 * DBIO),
					// 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
					authority_keys_from_seed("Bob", 100 * DBIO),
				],
				// Pre-funded accounts
				vec![
					(
						// Sudo account, Validator, 1and API admin account
						// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						// Balance amount
						12_499_000 * DBIO,
					),
					(
						// Validator 2
						// 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						// Balance amount
						1_000 * DBIO,
					),
					(
						// Pallet ID Account
						PalletId(*b"Rewards!").into_account(),
						// Pallet ID rewards amount
						total_reward_balance,
					)
				],
				// Appchain config
				appchain_config(
					// Appchain Relay Contract
					"",
					// Appchain Asset Id by Name
					"usdc.testnet",
					// Premined Amount
					87_500_000 * DBIO,
					// Era Payout
					4_657 * DBIO,
				),
				// API admin account
				// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
				get_account_id_from_seed::<sr25519::Public>("Alice"),
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("debio-local"),
		// Properties
		Some(properties),
		// Extensions
		Default::default(),
	))
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?;
	let properties = get_properties("DBIO", 18, 42);
	let total_reward_balance = 25_000_000 * DBIO;

	Ok(ChainSpec::from_genesis(
		// Name
		"DeBio Development",
		// ID
		"debio_development",
		ChainType::Development,
		move || {
			genesis(
				// WASM Binary
				wasm_binary,
				// Sudo account
				// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Initial PoA authorities
				vec![
					// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
					authority_keys_from_seed("Alice", 100 * DBIO),
				],
				// Pre-funded accounts
				vec![
					(
						// Sudo account and API admin account
						// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						// Balance amount
						12_497_500 * DBIO,
					),
					(
						// Pallet ID Account
						PalletId(*b"Rewards!").into_account(),
						// Pallet ID rewards amount
						total_reward_balance,
					)
				],
				// Appchain config
				appchain_config(
					// Appchain Relay Contract
					"",
					// Appchain Asset Id by Name
					"usdc.testnet",
					// Premined Amount
					87_500_000 * DBIO,
					// Era Payout
					4_657 * DBIO,
				),
				// API admin account
				// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
				get_account_id_from_seed::<sr25519::Public>("Alice"),
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("debio-development"),
		// Properties
		Some(properties),
		// Extensions
		Default::default(),
	))
}

/// Configure initial storage state for FRAME modules.
fn genesis(
	wasm_binary: &[u8],
	root_key: AccountId,
	initial_authorities: Vec<(
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		BeefyId,
		OctopusId,
		Balance,
	)>,
	endowed_accounts: Vec<(AccountId, Balance)>,
	appchain_config: (String, String, Balance, Balance),
	api_admin_key: AccountId,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		sudo: SudoConfig { key: root_key },
		babe: BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		beefy: Default::default(),
		assets: Default::default(),
		balances: BalancesConfig {
			balances: endowed_accounts.iter().map(|x| (x.0.clone(), x.1)).collect(),
		},
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(
							x.1.clone(),
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
						),
					)
				})
				.collect(),
		},
		octopus_appchain: OctopusAppchainConfig {
			anchor_contract: appchain_config.0,
			asset_id_by_name: vec![(appchain_config.1, 0)],
			premined_amount: appchain_config.2,
			validators: initial_authorities.iter().map(|x| (x.0.clone(), x.6)).collect(),
		},
		octopus_lpos: OctopusLposConfig { era_payout: appchain_config.3, ..Default::default() },
		orders: OrdersConfig { escrow_key: api_admin_key.clone() },
		rewards: RewardsConfig { rewarder_key: api_admin_key.clone() },
		labs: LabsConfig { lab_verifier_key: api_admin_key.clone() },
		service_request: ServiceRequestConfig { admin_key: api_admin_key.clone() },
		user_profile: UserProfileConfig { admin_key: api_admin_key.clone() }
	}
}
