use debio_runtime::{
	currency::DBIO, AccountId, BalancesConfig, GenesisConfig, Signature, GrandpaConfig,
	SudoConfig, SystemConfig, BabeConfig, 
	WASM_BINARY, OrdersConfig, RewardsConfig,
};
use sc_service::{ChainType, Properties};
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};

use debio_runtime::BeefyConfig;
use debio_runtime::{
	opaque::SessionKeys, Balance, ImOnlineConfig, SessionConfig,
};
use debio_runtime::{OctopusAppchainConfig, OctopusLposConfig};
use beefy_primitives::crypto::AuthorityId as BeefyId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_octopus_appchain::AuthorityId as OctopusId;
use pallet_octopus_lpos::StakerStatus;
use sp_consensus_babe::AuthorityId as BabeId;

use hex_literal::hex;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

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
	s: &str,
) -> (AccountId, BabeId, GrandpaId, ImOnlineId, BeefyId, OctopusId) {
	(
		get_account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<BabeId>(s),
		get_from_seed::<GrandpaId>(s),
		get_from_seed::<ImOnlineId>(s),
		get_from_seed::<BeefyId>(s),
		get_from_seed::<OctopusId>(s),
	)
}

/// Helper function to generate properties.
pub fn get_properties(symbol: &str, decimals: u32, ss58format: u32) -> Properties {
    let mut properties = Properties::new();
    properties.insert("tokenSymbol".into(), symbol.into());
    properties.insert("tokenDecimals".into(), decimals.into());
    properties.insert("ss58format".into(), ss58format.into());

    properties
}

pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary =
        WASM_BINARY.ok_or_else(|| "Development wasm binary not available".to_string())?;
    let properties = get_properties("DBIO", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"Debio Dev Net",
		// ID
		"debio_dev_net",
		ChainType::Development,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
			],
			vec![],
			// Sudo account
			// 5EpzDTRWDoVTnE31ybM2tse77CkZyG2eKC58Z3gbALHphHN6
			hex!["7a3e54fe532670c009cc839a7a9b8578239d08ed5234909d991da8ba39f45346"].into(),
			// Orders Pallet admin key
			// API Server   5GRjDZsTCatwWfNosGF8QRAPR1zYPJ7jJppt224tjE7x8cSx
			hex!["c0f9aaa3ce6b6c57eadc5fef443aaf8152fa8e49a8fc684ecc47c3304fdf3c0c"].into(),
			// Rewarders Pallet admin key
			// API Server   5GRjDZsTCatwWfNosGF8QRAPR1zYPJ7jJppt224tjE7x8cSx
			hex!["c0f9aaa3ce6b6c57eadc5fef443aaf8152fa8e49a8fc684ecc47c3304fdf3c0c"].into(),
			// Pre-funded accounts
			vec![
				// Sudo         5EpzDTRWDoVTnE31ybM2tse77CkZyG2eKC58Z3gbALHphHN6
				hex!["7a3e54fe532670c009cc839a7a9b8578239d08ed5234909d991da8ba39f45346"].into(),
				// Faucet       5HbNav6B8wUj8F9jRCVEcL6a576iHP8HJhfSfZM7fEHnRs2X
				hex!["f490e69c55aa14d06bb5d62d12b81db20f3c125d6ea5d1cfddfcf98767272e6b"].into(),
				// API Server   5GRjDZsTCatwWfNosGF8QRAPR1zYPJ7jJppt224tjE7x8cSx
				hex!["c0f9aaa3ce6b6c57eadc5fef443aaf8152fa8e49a8fc684ecc47c3304fdf3c0c"].into(),
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		Some(properties),
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary =
        WASM_BINARY.ok_or_else(|| "Development wasm binary not available".to_string())?;
    let properties = get_properties("DBIO", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"Debio Local Testnet",
		// ID
		"debio_local_testnet",
		ChainType::Local,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
				authority_keys_from_seed("Bob"),
			],
			vec![],
			// Sudo account
			// 5EpzDTRWDoVTnE31ybM2tse77CkZyG2eKC58Z3gbALHphHN6
			hex!["7a3e54fe532670c009cc839a7a9b8578239d08ed5234909d991da8ba39f45346"].into(),
			// Orders Pallet admin key
			// API Server   5GRjDZsTCatwWfNosGF8QRAPR1zYPJ7jJppt224tjE7x8cSx
			hex!["c0f9aaa3ce6b6c57eadc5fef443aaf8152fa8e49a8fc684ecc47c3304fdf3c0c"].into(),
			// Rewarders Pallet admin key
			// API Server   5GRjDZsTCatwWfNosGF8QRAPR1zYPJ7jJppt224tjE7x8cSx
			hex!["c0f9aaa3ce6b6c57eadc5fef443aaf8152fa8e49a8fc684ecc47c3304fdf3c0c"].into(),
			// Pre-funded accounts
			vec![
				// Sudo     5EpzDTRWDoVTnE31ybM2tse77CkZyG2eKC58Z3gbALHphHN6
				hex!["7a3e54fe532670c009cc839a7a9b8578239d08ed5234909d991da8ba39f45346"].into(),
				// Faucet   5HbNav6B8wUj8F9jRCVEcL6a576iHP8HJhfSfZM7fEHnRs2X
				hex!["f490e69c55aa14d06bb5d62d12b81db20f3c125d6ea5d1cfddfcf98767272e6b"].into(),
				// API Server   5GRjDZsTCatwWfNosGF8QRAPR1zYPJ7jJppt224tjE7x8cSx
				hex!["c0f9aaa3ce6b6c57eadc5fef443aaf8152fa8e49a8fc684ecc47c3304fdf3c0c"].into(),
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		Some(properties),
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, BabeId, GrandpaId, ImOnlineId, BeefyId, OctopusId)>,
	initial_nominators: Vec<AccountId>,
	root_key: AccountId,
	orders_escrow_key: AccountId,
    rewarder_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	// stakers: all validators and nominators.
	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), STASH, StakerStatus::Validator))
		.chain(initial_nominators.iter().map(|x| {
			use rand::{seq::SliceRandom, Rng};
			let limit = (16 as usize).min(initial_authorities.len());
			let count = rng.gen::<usize>() % limit;
			let nominations = initial_authorities
				.as_slice()
				.choose_multiple(&mut rng, count)
				.into_iter()
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), STASH, StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();

	const ENDOWMENT: Balance = 33_333_333 * DBIO;
	const STASH: Balance = 100 * DBIO;

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|x| (x, ENDOWMENT)).collect(),
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
				.collect::<Vec<_>>(),
		},
		octopus_lpos: OctopusLposConfig {
			stakers,
			..Default::default()
		},
		sudo: SudoConfig { key: root_key },
		babe: BabeConfig {
			authorities: vec![],
			epoch_config: Some(debio_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		im_online: ImOnlineConfig { keys: vec![] },
		grandpa: GrandpaConfig { authorities: vec![] },
		beefy: BeefyConfig { authorities: vec![] },
		octopus_appchain: OctopusAppchainConfig {
			appchain_id: "".to_string(),
			relay_contract: "oct-relay.testnet".to_string(),
			asset_id_by_name: vec![("usdc.testnet".to_string(), 0)],
		},
		orders: OrdersConfig {
			escrow_key: orders_escrow_key,
		},
        rewards: RewardsConfig {
            rewarder_key: rewarder_key,
        },
	}
}
