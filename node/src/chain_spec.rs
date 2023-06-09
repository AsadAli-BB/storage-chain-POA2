use hex_literal::hex;
use sc_service::{ChainType, Properties};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{ecdsa, Pair, Public, H160, H256, U256};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use storage_chain_runtime::{
	currency::*, opaque::SessionKeys, AccountId, Balance, BalancesConfig, EVMConfig,
	EthereumConfig, GenesisAccount, GenesisConfig, GrandpaConfig, SessionConfig, SudoConfig,
	SystemConfig, WASM_BINARY,
};
// use sp_runtime::traits::{IdentifyAccount, Verify};
use std::{collections::BTreeMap, default::Default};
// use frame_benchmarking::frame_support::metadata::StorageEntryModifier::Default;
use libsecp256k1::{PublicKey, PublicKeyFormat};
use sha3::{Digest, Keccak256};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

// type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate a crypto pair from seed
fn get_from_secret<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(seed, None)
		.unwrap_or_else(|_| panic!("Invalid string '{}'", seed))
		.public()
}

// / Helper function to generate an account ID from seed
// fn get_account_id_from_secret<TPublic: Public>(seed: &str) -> AccountId
// 	where
// 		AccountPublic: From<<TPublic::Pair as Pair>::Public>,
// {
// 	AccountPublic::from(get_from_secret::<TPublic>(seed)).into_account()
// }

// / Helper function to generate an authority key for Aura
fn get_authority_keys_from_secret(seed: &str) -> (AccountId, AuraId, GrandpaId) {
	(
		// get_account_id_from_secret::<ed25519::Public>(seed),
		AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")),
		get_from_secret::<AuraId>(seed),
		get_from_secret::<GrandpaId>(seed),
	)
}

fn session_keys(aura: AuraId, grandpa: GrandpaId) -> SessionKeys {
	SessionKeys { aura, grandpa }
}

pub fn chainspec_properties() -> Properties {
	let mut properties = Properties::new();
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("tokenSymbol".into(), "STOR".into());
	properties
}

const ALITH: &str = "0xB3C58D472c03CC571EbC97b42BDA4D38a83dF21D";
const BALTATHAR: &str = "0xa22688CbDB4C8d84Bc315C8c1fA7d3B407926411";
const CHARLETH: &str = "0x73852AF9EA8c4744DAb5F50f2943c6Ba9a3D6ebC";
const DOROTHY: &str = "0xC743FF582A879d9Ab1314c0c80ec5DaE66d39E0b";

// const ETHAN: &str = "0xFf64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB";

/// Helper function to get an `AccountId` from an ECDSA Key Pair.
pub fn get_account_id_from_pair(pair: ecdsa::Pair) -> Option<AccountId> {
	let decompressed = PublicKey::parse_slice(&pair.public().0, Some(PublicKeyFormat::Compressed))
		.ok()?
		.serialize();

	let mut m = [0u8; 64];
	m.copy_from_slice(&decompressed[1..65]);

	Some(H160::from(H256::from_slice(Keccak256::digest(&m).as_slice())).into())
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![(
					array_bytes::hex_n_into_unchecked(ALITH),
					get_from_secret::<AuraId>("//Alice"),
					get_from_secret::<GrandpaId>("//Alice"),
				)],
				// Sudo account
				AccountId::from(hex!("6Ff7bE9856B8D3e1c9b65a20f4daA41c47e0D516")),
				// get_account_id_from_secret::<ed25519::Public>("//Alice"),
				// Pre-funded accounts
				vec![
					AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")),
					AccountId::from(hex!("7ed8c8a0C4d1FeA01275fE13F0Ef23bce5CBF8C3")),
					AccountId::from(hex!("3263236Cbc327B5519E373CC591318e56e7c5081")),
					// get_account_id_from_seed::<ecdsa::Public>("Bob"),
					// get_account_id_from_seed::<ecdsa::Public>("Alice//stash"),
					// get_account_id_from_seed::<ecdsa::Public>("Bob//stash"),
					// get_account_id_from_secret::<ed25519::Public>("//Alice"),
					// get_account_id_from_secret::<ed25519::Public>("//Bob"),
					// get_account_id_from_secret::<sr25519::Public>("//Alice"),
					// get_account_id_from_secret::<sr25519::Public>("//Bob"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		Some(chainspec_properties()),
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
					(
						array_bytes::hex_n_into_unchecked(ALITH),
						get_from_secret::<AuraId>("//Alice"),
						get_from_secret::<GrandpaId>("//Alice"),
					),
					(
						array_bytes::hex_n_into_unchecked(BALTATHAR),
						get_from_secret::<AuraId>("//Bob"),
						get_from_secret::<GrandpaId>("//Bob"),
					),
					// (
					// 	array_bytes::hex_n_into_unchecked(CHARLETH),
					// 	get_from_secret::<AuraId>("//Charlie"),
					// 	get_from_secret::<GrandpaId>("//Charlie"),
					// ),
					// (
					// 	array_bytes::hex_n_into_unchecked(DOROTHY),
					// 	get_from_secret::<AuraId>("//Dave"),
					// 	get_from_secret::<GrandpaId>("//Dave"),
					// ),
					// get_account_id_from_seed::<sr25519::Public>("Alice"),
					// get_account_id_from_seed::<sr25519::Public>("Bob"),
					// get_account_id_from_seed::<sr25519::Public>("Charlie"),
					// get_account_id_from_seed::<sr25519::Public>("Dave"),
					// get_account_id_from_seed::<sr25519::Public>("Eve"),
					// get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					// get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					// get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					// get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					// get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					// get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					// get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),

				],
				// Sudo account
				AccountId::from(hex!("B3C58D472c03CC571EbC97b42BDA4D38a83dF21D")),
				// Pre-funded accounts
				vec![
					array_bytes::hex_n_into_unchecked(ALITH),
					array_bytes::hex_n_into_unchecked(BALTATHAR),
					array_bytes::hex_n_into_unchecked(CHARLETH),
					array_bytes::hex_n_into_unchecked(DOROTHY),
					// AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")),
					// AccountId::from(hex!("7ed8c8a0C4d1FeA01275fE13F0Ef23bce5CBF8C3")),
					// AccountId::from(hex!("3263236Cbc327B5519E373CC591318e56e7c5081")),
					// get_account_id_from_seed::<ecdsa::Public>("Alice"),
					// get_account_id_from_seed::<ecdsa::Public>("Bob"),
					// get_account_id_from_seed::<ecdsa::Public>("Charlie"),
					// get_account_id_from_seed::<ecdsa::Public>("Dave"),
					// get_account_id_from_seed::<ecdsa::Public>("Eve"),
					// get_account_id_from_seed::<ecdsa::Public>("Ferdie"),
					// get_account_id_from_seed::<ecdsa::Public>("Alice//stash"),
					// get_account_id_from_seed::<ecdsa::Public>("Bob//stash"),
					// get_account_id_from_seed::<ecdsa::Public>("Charlie//stash"),
					// get_account_id_from_seed::<ecdsa::Public>("Dave//stash"),
					// get_account_id_from_seed::<ecdsa::Public>("Eve//stash"),
					// get_account_id_from_seed::<ecdsa::Public>("Ferdie//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		Some(chainspec_properties()),
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	const ENDOWMENT: Balance = 2_500_000_000 * STOR;
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k.clone(), ENDOWMENT / initial_authorities.len() as u128))
				.collect(),
		},
		aura: Default::default(),
		// AuraConfig {
		// 	authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		// },
		// grandpa: Default::default(),
		grandpa: GrandpaConfig {
			// authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
			authorities: vec![
				
			],
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
		evm: EVMConfig {
			accounts: {
				let mut accounts = BTreeMap::new();
				accounts.insert(
					H160::from_slice(&hex!("6Be02d1d3665660d22FF9624b7BE0551ee1Ac91b")),
					GenesisAccount {
						nonce: U256::zero(),
						balance: Default::default(),
						code: vec![],
						storage: BTreeMap::new(),
					},
				);
				accounts
			},
		},
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.0.clone(), session_keys(x.1.clone(), x.2.clone())))
				.collect::<Vec<_>>(),
		},
		ethereum: EthereumConfig {},
		base_fee: Default::default(),
	}
}
