// Copyright 2017-2020 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Polkadot chain configurations.

use beefy_primitives::crypto::AuthorityId as BeefyId;
#[cfg(feature = "cherry-native")]
use cherry_runtime as cherry;
#[cfg(feature = "cherry-native")]
use cherry_runtime_constants::currency::DOLLARS as CHER;
use grandpa::AuthorityId as GrandpaId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_staking::Forcing;
use polkadot_primitives::v2::{AccountId, AccountPublic, AssignmentId, ValidatorId};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;

use sc_chain_spec::{ChainSpecExtension, ChainType};
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::{traits::IdentifyAccount, Perbill};
use telemetry::TelemetryEndpoints;

#[cfg(feature = "cherry-native")]
const POLKADOT_STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const DEFAULT_PROTOCOL_ID: &str = "dot";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<polkadot_primitives::v2::Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<polkadot_primitives::v2::Block>,
	/// The light sync state.
	///
	/// This value will be set by the `sync-state rpc` implementation.
	pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// The `ChainSpec` parameterized for the polkadot runtime.
#[cfg(feature = "cherry-native")]
pub type CherryChainSpec = service::GenericChainSpec<cherry::GenesisConfig, Extensions>;

// Dummy chain spec, in case when we don't have the native runtime.
pub type DummyChainSpec = service::GenericChainSpec<(), Extensions>;

// Dummy chain spec, but that is fine when we don't have the native runtime.
#[cfg(not(feature = "cherry-native"))]
pub type CherryChainSpec = DummyChainSpec;

pub fn cherry_config() -> Result<CherryChainSpec, String> {
	CherryChainSpec::from_json_bytes(&include_bytes!("../chain-specs/polkadot.json")[..])
}

pub fn cherry_testnet_config() -> Result<CherryChainSpec, String> {
	CherryChainSpec::from_json_bytes(&include_bytes!("../chain-specs/cherry-testnet.json")[..])
}

/// The default parachains host configuration.
#[cfg(any(feature = "cherry-native"))]
fn default_parachains_host_configuration(
) -> polkadot_runtime_parachains::configuration::HostConfiguration<
	polkadot_primitives::v2::BlockNumber,
> {
	use polkadot_primitives::v2::{MAX_CODE_SIZE, MAX_POV_SIZE};

	polkadot_runtime_parachains::configuration::HostConfiguration {
		validation_upgrade_cooldown: 2u32,
		validation_upgrade_delay: 2,
		code_retention_period: 1200,
		max_code_size: MAX_CODE_SIZE,
		max_pov_size: MAX_POV_SIZE,
		max_head_data_size: 32 * 1024,
		group_rotation_frequency: 20,
		chain_availability_period: 4,
		thread_availability_period: 4,
		max_upward_queue_count: 8,
		max_upward_queue_size: 1024 * 1024,
		max_downward_message_size: 1024 * 1024,
		ump_service_total_weight: 100_000_000_000,
		max_upward_message_size: 50 * 1024,
		max_upward_message_num_per_candidate: 5,
		hrmp_sender_deposit: 0,
		hrmp_recipient_deposit: 0,
		hrmp_channel_max_capacity: 8,
		hrmp_channel_max_total_size: 8 * 1024,
		hrmp_max_parachain_inbound_channels: 4,
		hrmp_max_parathread_inbound_channels: 4,
		hrmp_channel_max_message_size: 1024 * 1024,
		hrmp_max_parachain_outbound_channels: 4,
		hrmp_max_parathread_outbound_channels: 4,
		hrmp_max_message_num_per_candidate: 5,
		dispute_period: 6,
		no_show_slots: 2,
		n_delay_tranches: 25,
		needed_approvals: 2,
		relay_vrf_modulo_samples: 2,
		zeroth_delay_tranche_width: 0,
		minimum_validation_upgrade_delay: 5,
		..Default::default()
	}
}

#[cfg(any(feature = "cherry-native"))]
#[test]
fn default_parachains_host_configuration_is_consistent() {
	default_parachains_host_configuration().panic_if_not_consistent();
}

#[cfg(feature = "cherry-native")]
fn polkadot_session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	para_validator: ValidatorId,
	para_assignment: AssignmentId,
	authority_discovery: AuthorityDiscoveryId,
	// beefy: BeefyId,
) -> cherry::SessionKeys {
	cherry::SessionKeys {
		babe,
		grandpa,
		im_online,
		para_validator,
		para_assignment,
		authority_discovery,
	}
}

#[cfg(feature = "cherry-native")]
fn cherry_staging_testnet_config_genesis(wasm_binary: &[u8]) -> cherry::GenesisConfig {
	use hex_literal::hex;
	use sp_core::crypto::UncheckedInto;

	// subkey inspect "$SECRET"
	let endowed_accounts = vec![
		//5DsokUQ2C7FaAGthmrfBdsNiBm3jaGgaiWgntjFVJZCrBscY
		hex!["e2e842322079eceea8b1e13b620230093eb75e4c4a9483c22bd681e50393dd42"].into(),
		//5GKgU9MjnqX5L5hgYx9Rke3bvxCZDfJr4xbA1WCNTb7Ne9Ho
		hex!["349c797d90a548178d9d40c0fd10fbe73d27ed4ef2e2e01062e1bbe98158df70"].into(),
	];

	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
	)> = vec![
		(
			//5DsokUQ2C7FaAGthmrfBdsNiBm3jaGgaiWgntjFVJZCrBscY
			hex!["e2e842322079eceea8b1e13b620230093eb75e4c4a9483c22bd681e50393dd42"].into(),
			//5DAqsiJcKppwPB3myqzKy1UBtdTcFTxrq65N6JhWpWpqHyLc
			hex!["66a7d5c3189b24be2239a6284773995715d391d80fd2551e149a6871e710c769"].into(),
			//5E8LmUjqWWREdW8T1bZwQBNsBdtQrnWhTNRthDu1ytA4qXz2
			hex!["e2e842322079eceea8b1e13b620230093eb75e4c4a9483c22bd681e50393dd42"]
				.unchecked_into(),
			//5HTJdWSTTqfNeyuyD8YW1HzdfpB6RmH59DQXVRincBXJD5Es
			hex!["0e199f15cf86601c54c30937bfbb00bef78da10e039a783821f13086db93cf08"]
				.unchecked_into(),
			//5FvMY5LtbB9xU3WzJthdyMGjoGFFkC2BgP6X2ZGXTY2RPGmC
			hex!["e2e842322079eceea8b1e13b620230093eb75e4c4a9483c22bd681e50393dd42"]
				.unchecked_into(),
			//5H3AtRtBjgdWQjp2xpcGE8jTGhSoaZPvnWVr27WcuxcA3FcD
			hex!["e2e842322079eceea8b1e13b620230093eb75e4c4a9483c22bd681e50393dd42"]
				.unchecked_into(),
			//5Gee8ozHizRgvtes3JHtWX3v6dQZ2WrRZjEmFrEyV5dmdZqr
			hex!["e2e842322079eceea8b1e13b620230093eb75e4c4a9483c22bd681e50393dd42"]
				.unchecked_into(),
			//5HGgVGs7etUBg3GSbsSwSzpTWfLnicS5EWUL9DXDm9PjvLbo
			hex!["e2e842322079eceea8b1e13b620230093eb75e4c4a9483c22bd681e50393dd42"]
				.unchecked_into(),
		),
		(
			//5GKgU9MjnqX5L5hgYx9Rke3bvxCZDfJr4xbA1WCNTb7Ne9Ho
			hex!["349c797d90a548178d9d40c0fd10fbe73d27ed4ef2e2e01062e1bbe98158df70"].into(),
			//5HbtQdnamyzzUKkfR6a7pnmKZNpci8LqckBAFppYNydBjGi8
			hex!["bc118c801753c4bbde9d256bd5b0f072ad33158af6271f832f56a834635e5874"].into(),
			//5HGViLeoQMxyTKcYgztb7yu4ztKaWwUPrrLvPdidbdLh4QRf
			hex!["349c797d90a548178d9d40c0fd10fbe73d27ed4ef2e2e01062e1bbe98158df70"]
				.unchecked_into(),
			//5H4SSdpRXcc2m9Cn4ZWWQov2Z1UjhMUaYaB7oazcsQn71NKc
			hex!["d111bd1910077ec92353dd47da94e2afdebec2827ac5fee6516c6b363afaca8d"]
				.unchecked_into(),
			//5HKDSaUg2m5X6cNyH231i6cCfb8yXPmxeYRTFyPvRz5Z2Cn8
			hex!["349c797d90a548178d9d40c0fd10fbe73d27ed4ef2e2e01062e1bbe98158df70"]
				.unchecked_into(),
			//5F6yZNMnbxsDws4C18KdBQW9TtoBVXmXdkC9jZ94BE5RGfdq
			hex!["349c797d90a548178d9d40c0fd10fbe73d27ed4ef2e2e01062e1bbe98158df70"]
				.unchecked_into(),
			//5DCfJh5BLtBfUmZaY3oXZCEhYTqBL6745TpTdNoBHEfdWpPn
			hex!["349c797d90a548178d9d40c0fd10fbe73d27ed4ef2e2e01062e1bbe98158df70"]
				.unchecked_into(),
			//5DvwSvqiGoND9u2dSUU9T8a2hs23Btsga4YRfmcNRMFWCv5F
			hex!["349c797d90a548178d9d40c0fd10fbe73d27ed4ef2e2e01062e1bbe98158df70"]
				.unchecked_into(),
		),
	];

	const ENDOWMENT: u128 = 100 * CHER;
	const STASH: u128 = 250_000 * CHER;

	cherry::GenesisConfig {
		system: cherry::SystemConfig { code: wasm_binary.to_vec() },
		balances: cherry::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k: &AccountId| (k.clone(), ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.1.clone(), STASH)))
				.collect(),
		},
		indices: cherry::IndicesConfig { indices: vec![] },
		session: cherry::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.1.clone(),
						polkadot_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: cherry::StakingConfig {
			validator_count: 2,
			minimum_validator_count: 2,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.1.clone(), x.0.clone(), STASH, cherry::StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.1.clone()).collect(),
			force_era: Forcing::ForceNone,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		phragmen_election: Default::default(),
		// democracy: Default::default(),
		council: cherry::CouncilConfig { members: vec![], phantom: Default::default() },
		technical_committee: cherry::TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		},
		technical_membership: Default::default(),
		babe: cherry::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(cherry::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		authority_discovery: cherry::AuthorityDiscoveryConfig { keys: vec![] },
		claims: cherry::ClaimsConfig { claims: vec![], vesting: vec![] },
		vesting: cherry::VestingConfig { vesting: vec![] },
		treasury: Default::default(),
		hrmp: Default::default(),
		configuration: cherry::ConfigurationConfig {
			config: default_parachains_host_configuration(),
		},
		paras: Default::default(),
		xcm_pallet: Default::default(),
		assets: cherry::AssetsConfig { assets: vec![], metadata: vec![], accounts: vec![] },
		gilt: Default::default(),
	}
}

/// Returns the properties for the [`PolkadotChainSpec`].
pub fn cherry_chain_spec_properties() -> serde_json::map::Map<String, serde_json::Value> {
	serde_json::json!({
		"tokenDecimals": 10,
	})
	.as_object()
	.expect("Map given; qed")
	.clone()
}

/// Polkadot staging testnet config.
#[cfg(feature = "cherry-native")]
pub fn cherry_staging_testnet_config() -> Result<CherryChainSpec, String> {
	let wasm_binary = cherry::WASM_BINARY.ok_or("Cherry development wasm not available")?;
	let boot_nodes = vec![];

	Ok(CherryChainSpec::from_genesis(
		"Cherry Staging Testnet",
		"cherry_staging_testnet",
		ChainType::Live,
		move || cherry_staging_testnet_config_genesis(wasm_binary),
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(POLKADOT_STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Cherry Staging telemetry url is valid; qed"),
		),
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Some(cherry_chain_spec_properties()),
		Default::default(),
	))
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(
	seed: &str,
) -> (
	AccountId,
	AccountId,
	BabeId,
	GrandpaId,
	ImOnlineId,
	ValidatorId,
	AssignmentId,
	AuthorityDiscoveryId,
	BeefyId,
) {
	let keys = get_authority_keys_from_seed_no_beefy(seed);
	(keys.0, keys.1, keys.2, keys.3, keys.4, keys.5, keys.6, keys.7, get_from_seed::<BeefyId>(seed))
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed_no_beefy(
	seed: &str,
) -> (
	AccountId,
	AccountId,
	BabeId,
	GrandpaId,
	ImOnlineId,
	ValidatorId,
	AssignmentId,
	AuthorityDiscoveryId,
) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<ValidatorId>(seed),
		get_from_seed::<AssignmentId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

fn testnet_accounts() -> Vec<AccountId> {
	vec![
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
	]
}

/// Helper function to create polkadot `GenesisConfig` for testing
#[cfg(feature = "cherry-native")]
pub fn cherry_testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
	)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> cherry::GenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);
	let num_endowed_accounts = endowed_accounts.len();

	const ENDOWMENT: u128 = 1_000_000 * CHER;
	const STASH: u128 = ENDOWMENT;

	cherry::GenesisConfig {
		system: cherry::SystemConfig { code: wasm_binary.to_vec() },
		indices: cherry::IndicesConfig { indices: vec![] },
		balances: cherry::BalancesConfig {
			balances: endowed_accounts.iter().map(|k| (k.clone(), ENDOWMENT)).collect(),
		},
		session: cherry::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						polkadot_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: cherry::StakingConfig {
			minimum_validator_count: 1,
			validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, cherry::StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::NotForcing,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		phragmen_election: Default::default(),
		// democracy: polkadot::DemocracyConfig::default(),
		council: cherry::CouncilConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.map(|member| (member))
				.collect(),
			phantom: Default::default(),
		},
		technical_committee: cherry::TechnicalCommitteeConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.map(|member| (member))
				.collect(),
			phantom: Default::default(),
		},
		technical_membership: Default::default(),
		babe: cherry::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(cherry::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		authority_discovery: cherry::AuthorityDiscoveryConfig { keys: vec![] },
		claims: cherry::ClaimsConfig { claims: vec![], vesting: vec![] },
		vesting: cherry::VestingConfig { vesting: vec![] },
		treasury: Default::default(),
		hrmp: Default::default(),
		configuration: cherry::ConfigurationConfig {
			config: default_parachains_host_configuration(),
		},
		paras: Default::default(),
		xcm_pallet: Default::default(),
		assets: cherry::AssetsConfig {
			assets: vec![(999, root_key.clone(), true, 1)],
			metadata: vec![(999, "Governance Token".into(), "tGov".into(), 0)],
			accounts: vec![(999, root_key.clone(), 10)],
		},
		gilt: Default::default(),
	}
}

#[cfg(feature = "cherry-native")]
fn cherry_development_config_genesis(wasm_binary: &[u8]) -> cherry::GenesisConfig {
	cherry_testnet_genesis(
		wasm_binary,
		vec![get_authority_keys_from_seed_no_beefy("Alice")],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Polkadot development config (single validator Alice)
#[cfg(feature = "cherry-native")]
pub fn cherry_development_config() -> Result<CherryChainSpec, String> {
	let wasm_binary = cherry::WASM_BINARY.ok_or("Cherry development wasm not available")?;

	Ok(CherryChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		move || cherry_development_config_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Some(cherry_chain_spec_properties()),
		Default::default(),
	))
}

#[cfg(feature = "cherry-native")]
fn cherry_local_testnet_genesis(wasm_binary: &[u8]) -> cherry::GenesisConfig {
	cherry_testnet_genesis(
		wasm_binary,
		vec![
			get_authority_keys_from_seed_no_beefy("Alice"),
			get_authority_keys_from_seed_no_beefy("Bob"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Polkadot local testnet config (multivalidator Alice + Bob)
#[cfg(feature = "cherry-native")]
pub fn cherry_local_testnet_config() -> Result<CherryChainSpec, String> {
	let wasm_binary = cherry::WASM_BINARY.ok_or("Cherry development wasm not available")?;

	Ok(CherryChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		move || cherry_local_testnet_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Some(cherry_chain_spec_properties()),
		Default::default(),
	))
}
