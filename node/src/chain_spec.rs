use sp_core::{Pair, Public, sr25519};
use node_template_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, SystemConfig, WASM_BINARY, Signature, OrdersConfig, GbioConfig
};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{Verify, IdentifyAccount};
use sc_service::{ChainType, Properties};

use hex_literal::hex;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(
		get_from_seed::<AuraId>(s),
		get_from_seed::<GrandpaId>(s),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm binary not available".to_string())?;

	let mut properties = Properties::new();
	properties.insert("tokenSymbol".into(), "DBIO".into());
	properties.insert("tokenDecimals".into(), 15.into());

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
			// Sudo account
			// 5EpzDTRWDoVTnE31ybM2tse77CkZyG2eKC58Z3gbALHphHN6
			hex!["7a3e54fe532670c009cc839a7a9b8578239d08ed5234909d991da8ba39f45346"].into(),
			// Orders Pallet admin key
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
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
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
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm binary not available".to_string())?;

	let mut properties = Properties::new();
	properties.insert("tokenSymbol".into(), "DBIO".into());
	properties.insert("tokenDecimals".into(), 15.into());

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
			// Sudo account
			// 5EpzDTRWDoVTnE31ybM2tse77CkZyG2eKC58Z3gbALHphHN6
			hex!["7a3e54fe532670c009cc839a7a9b8578239d08ed5234909d991da8ba39f45346"].into(),
			// Orders Pallet admin key
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
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_account_id_from_seed::<sr25519::Public>("Eve"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
				get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
				get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
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
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	orders_escrow_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		frame_system: Some(SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
		}),
		pallet_aura: Some(AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		}),
		pallet_grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		}),
		pallet_sudo: Some(SudoConfig {
			// Assign network admin rights.
			key: root_key,
		}),
		gbio: Some(GbioConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			endowed_accounts: endowed_accounts,
			total_supply: 21000000,
		}),
		orders: Some(OrdersConfig {
			escrow_key: orders_escrow_key,
		})
	}
}
