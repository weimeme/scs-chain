// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate chain configurations.

use hex_literal::hex;
use polkadot_sdk::*;
use sp_core::{U256, H160};
use sp_std::collections::btree_map::BTreeMap;
use std::str::FromStr;
use kitchensink_runtime::{

	AccountId,
	constants::currency::*, wasm_binary_unwrap, Block, MaxNominations, SessionKeys, StakerStatus,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::ChainSpecExtension;
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_beefy::ecdsa_crypto::AuthorityId as BeefyId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::ecdsa;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_mixnet::types::AuthorityId as MixnetId;
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};

pub use kitchensink_runtime::RuntimeGenesisConfig;
pub use node_primitives::{Balance, Signature};

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
const STASH: Balance = ENDOWMENT / 1000;

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

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<Extensions>;
/// Flaming Fir testnet generator
// pub fn flaming_fir_config() -> Result<ChainSpec, String> {
// 	ChainSpec::from_json_bytes(&include_bytes!("../res/flaming-fir.json")[..])
// }

fn session_keys(
	// ed25519
	grandpa: GrandpaId,
	// sr25519
	babe: BabeId,
	// sr25519
	im_online: ImOnlineId,
	// sr25519
	authority_discovery: AuthorityDiscoveryId,
	// sr25519
	mixnet: MixnetId,
	// ecdsa
	beefy: BeefyId,
) -> SessionKeys {
	SessionKeys { grandpa, babe, im_online, authority_discovery, mixnet, beefy }
}

fn configure_accounts_for_staging_testnet() -> (
	Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
		MixnetId,
		BeefyId,
	)>,
	AccountId,
	Vec<AccountId>,
) {
	#[rustfmt::skip]
	// stash, controller, session-key, beefy id
	// generated with secret:
	// for i in 1 2 3 4 ; do for j in stash controller; do subkey inspect "$secret"/fir/$j/$i; done; done
	//
	// and
	//
	// for i in 1 2 3 4 ; do for j in session; do subkey inspect --scheme ed25519 "$secret"//fir//$j//$i; done; done
	//
	// and
	//
	// for i in 1 2 3 4 ; do for j in session; do subkey inspect --scheme ecdsa "$secret"//fir//$j//$i; done; done

	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
		MixnetId,
		BeefyId,
	)> = vec![
		(
			H160::from_str("1000000000000000000000000000000000000001").expect("internal H160 is valid; qed").into(),
			H160::from_str("1000000000000000000000000000000000000001").expect("internal H160 is valid; qed").into(),
			// 5Fb9ayurnxnaXj56CjmyQLBiadfRCqUbL2VWNbbe1nZU6wiC
			array_bytes::hex2array_unchecked("9becad03e6dcac03cee07edebca5475314861492cdfc96a2144a67bbe9699332")
				.unchecked_into(),
			// 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
			array_bytes::hex2array_unchecked("6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106")
				.unchecked_into(),
			// 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
			array_bytes::hex2array_unchecked("6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106")
				.unchecked_into(),
			// 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
			array_bytes::hex2array_unchecked("6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106")
				.unchecked_into(),
			// 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
			array_bytes::hex2array_unchecked("6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106")
				.unchecked_into(),
			// 5DMLFcDdLLQbw696YfHaWBpQR99HwR456ycSCfr6L7KXGYK8
			array_bytes::hex2array_unchecked("035560fafa241739869360aa4b32bc98953172ceb41a19c6cc1a27962fb3d1ecec")
				.unchecked_into(),
		),
		(
			H160::from_str("1000000000000000000000000000000000000002").expect("internal H160 is valid; qed").into(),
			H160::from_str("1000000000000000000000000000000000000002").expect("internal H160 is valid; qed").into(),
			// 5EockCXN6YkiNCDjpqqnbcqd4ad35nU4RmA1ikM4YeRN4WcE
			array_bytes::hex2array_unchecked("7932cff431e748892fa48e10c63c17d30f80ca42e4de3921e641249cd7fa3c2f")
				.unchecked_into(),
			// 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
			array_bytes::hex2array_unchecked("482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e")
				.unchecked_into(),
			// 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
			array_bytes::hex2array_unchecked("482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e")
				.unchecked_into(),
			// 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
			array_bytes::hex2array_unchecked("482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e")
				.unchecked_into(),
			// 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
			array_bytes::hex2array_unchecked("482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e")
				.unchecked_into(),
			// 5FYk11kNtB4178wLKJ2RNoUzzcjgRUciFe3SJDVZXhqX4dzG
			array_bytes::hex2array_unchecked("02da1ab255ed888ee3e19b73d335fc13160b3eb10456c2d17c6a8ea7de403d2445")
				.unchecked_into(),
		),
		(
			H160::from_str("1000000000000000000000000000000000000003").expect("internal H160 is valid; qed").into(),
			H160::from_str("1000000000000000000000000000000000000003").expect("internal H160 is valid; qed").into(),
			// 5E1jLYfLdUQKrFrtqoKgFrRvxM3oQPMbf6DfcsrugZZ5Bn8d
			array_bytes::hex2array_unchecked("5633b70b80a6c8bb16270f82cca6d56b27ed7b76c8fd5af2986a25a4788ce440")
				.unchecked_into(),
			// 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
			array_bytes::hex2array_unchecked("482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a")
				.unchecked_into(),
			// 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
			array_bytes::hex2array_unchecked("482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a")
				.unchecked_into(),
			// 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
			array_bytes::hex2array_unchecked("482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a")
				.unchecked_into(),
			// 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
			array_bytes::hex2array_unchecked("482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a")
				.unchecked_into(),
			// 5GQx4FToRBPqfani6o7owFJE1UstiviqbPP7HPWyvtXWWukn
			array_bytes::hex2array_unchecked("036a818b3f59579c5fbbe4fede64f49dbf090ba883eb2a175d5ca90e5adb5f0b3e")
				.unchecked_into(),
		),
	];

	// generated with secret: subkey inspect "$secret"/fir
	// fixme
	let root_key: AccountId = H160::from_str("1000000000000000000000000000000000000001").expect("internal H160 is valid; qed").into();

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];
	(initial_authorities, root_key, endowed_accounts)
}

fn staging_testnet_config_genesis() -> serde_json::Value {
	let (initial_authorities, root_key, endowed_accounts) =
		configure_accounts_for_staging_testnet();
	// 测试网的链id是1969
	testnet_genesis(initial_authorities, vec![], root_key, Some(endowed_accounts), 1969u32)
}

/// Staging testnet config.
pub fn staging_testnet_config() -> ChainSpec {
	ChainSpec::builder(wasm_binary_unwrap(), Default::default())
		.with_name("SCS Testnet")
		.with_id("scs_testnet")
		.with_properties(serde_json::from_str("{\"isEthereum\": true, \"tokenDecimals\": 18, \"tokenSymbol\": \"TSCS\"}")
							 .expect("Provided valid json map"),)
		.with_chain_type(ChainType::Live)
		.with_genesis_config_patch(staging_testnet_config_genesis())
		.with_telemetry_endpoints(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		)
		.build()
}

/// Helper function to generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed.
pub fn authority_keys_from_seed(
	seed: &str,
) -> (AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId, MixnetId, BeefyId)
{
	(
		get_account_id_from_seed::<ecdsa::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<ecdsa::Public>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
		get_from_seed::<MixnetId>(seed),
		get_from_seed::<BeefyId>(seed),
	)
}

/// Helper function to generate stash, controller and session key from seed.
pub fn authority_keys_from_alice(
) -> (AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId, MixnetId, BeefyId)
{
	let seed = "Alice";
	(
		AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")),
		AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
		get_from_seed::<MixnetId>(seed),
		get_from_seed::<BeefyId>(seed),
	)
}

fn configure_accounts(
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
		MixnetId,
		BeefyId,
	)>,
	initial_nominators: Vec<AccountId>,
	endowed_accounts: Option<Vec<AccountId>>,
	stash: Balance,
) -> (
	Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
		MixnetId,
		BeefyId,
	)>,
	Vec<AccountId>,
	usize,
	Vec<(AccountId, AccountId, Balance, StakerStatus<AccountId>)>,
) {
	let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![
			get_account_id_from_seed::<ecdsa::Public>("Alice"),
			get_account_id_from_seed::<ecdsa::Public>("Bob"),
			get_account_id_from_seed::<ecdsa::Public>("Charlie"),
			get_account_id_from_seed::<ecdsa::Public>("Dave"),
			get_account_id_from_seed::<ecdsa::Public>("Eve"),
			get_account_id_from_seed::<ecdsa::Public>("Ferdie"),
			get_account_id_from_seed::<ecdsa::Public>("Alice//stash"),
			get_account_id_from_seed::<ecdsa::Public>("Bob//stash"),
			get_account_id_from_seed::<ecdsa::Public>("Charlie//stash"),
			get_account_id_from_seed::<ecdsa::Public>("Dave//stash"),
			get_account_id_from_seed::<ecdsa::Public>("Eve//stash"),
			get_account_id_from_seed::<ecdsa::Public>("Ferdie//stash"),
		]
	});
	// endow all authorities and nominators.
	// 最初的验证人跟endowed_accounts一样  需要同样的金额
	initial_authorities
		.iter()
		.map(|x| &x.0)
		.chain(initial_nominators.iter())
		.for_each(|x| {
			if !endowed_accounts.contains(x) {
				endowed_accounts.push(x.clone())
			}
		});

	// stakers: all validators and nominators.
	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), x.0.clone(), stash, StakerStatus::Validator))
		.chain(initial_nominators.iter().map(|x| {
			use rand::{seq::SliceRandom, Rng};
			let limit = (MaxNominations::get() as usize).min(initial_authorities.len());
			let count = rng.gen::<usize>() % limit;
			let nominations = initial_authorities
				.as_slice()
				.choose_multiple(&mut rng, count)
				.into_iter()
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), stash, StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();

	let num_endowed_accounts = endowed_accounts.len();

	(initial_authorities, endowed_accounts, num_endowed_accounts, stakers)
}

/// Helper function to create RuntimeGenesisConfig json patch for testing.
pub fn testnet_genesis(
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
		MixnetId,
		BeefyId,
	)>,
	initial_nominators: Vec<AccountId>,
	root_key: AccountId,
	// 收钱账号
	endowed_accounts: Option<Vec<AccountId>>,
	evm_chain_id: u32,
) -> serde_json::Value {
	let (initial_authorities, endowed_accounts, num_endowed_accounts, stakers) =
		configure_accounts(initial_authorities, initial_nominators, endowed_accounts, STASH);

	let evm_accounts = {
		let mut map = BTreeMap::new();
		map.insert(
			// H160 address of Alice dev account
			// Derived from SS58 (42 prefix) address
			// SS58: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
			// hex: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
			// Using the full hex key, truncating to the first 20 bytes (the first 40 hex chars)
			H160::from_str("d43593c715fdd31c61141abd04a99fd6822c8558")
				.expect("internal H160 is valid; qed"),
			fp_evm::GenesisAccount {
				balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
					.expect("internal U256 is valid; qed"),
				code: Default::default(),
				nonce: Default::default(),
				storage: Default::default(),
			},
		);
		map.insert(
			// H160 address of CI test runner account
			H160::from_str("6be02d1d3665660d22ff9624b7be0551ee1ac91b")
				.expect("internal H160 is valid; qed"),
			fp_evm::GenesisAccount {
				balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
					.expect("internal U256 is valid; qed"),
				code: Default::default(),
				nonce: Default::default(),
				storage: Default::default(),
			},
		);
		map.insert(
			// H160 address for benchmark usage
			H160::from_str("1000000000000000000000000000000000000001")
				.expect("internal H160 is valid; qed"),
			fp_evm::GenesisAccount {
				nonce: U256::from(1),
				balance: U256::from(1_000_000_000_000_000_000_000_000u128),
				storage: Default::default(),
				code: vec![0x00],
			},
		);
		map
	};

	serde_json::json!({
		// fixme 这个需要自己设置就行了
		// 如果开发的话使用这个默认配置
		// todo 需要知道验证人需要抵押多少金额 然后给验证人配置金额
		"balances": {
			"balances": endowed_accounts.iter().cloned().map(|x| (x, ENDOWMENT)).collect::<Vec<_>>(),
		},
		"session": {
			"keys": initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						// 固定使用sr25519
						session_keys(
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
		"staking": {
			"validatorCount": initial_authorities.len() as u32,
			"minimumValidatorCount": initial_authorities.len() as u32,
			// todo 要确定这个会不会因为投票排序被踢出去
			"invulnerables": initial_authorities.iter().map(|x| x.0.clone()).collect::<Vec<_>>(),
			"slashRewardFraction": Perbill::from_percent(10),
			"stakers": stakers.clone(),
		},
		// "elections": {
		// 	"members": endowed_accounts
		// 		.iter()
		// 		.take((num_endowed_accounts + 1) / 2)
		// 		.cloned()
		// 		.map(|member| (member, STASH))
		// 		.collect::<Vec<_>>(),
		// },
		// "technicalCommittee": {
		// 	"members": endowed_accounts
		// 		.iter()
		// 		.take((num_endowed_accounts + 1) / 2)
		// 		.cloned()
		// 		.collect::<Vec<_>>(),
		// },
		"sudo": { "key": Some(root_key.clone()) },
		"babe": {
			"epochConfig": Some(kitchensink_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		"society": { "pot": 0 },
		"assets": {
			// This asset is used by the NIS pallet as counterpart currency.
			"assets": vec![(9, get_account_id_from_seed::<ecdsa::Public>("Alice"), true, 1)],
		},
		"nominationPools": {
			"minCreateBond": 10 * DOLLARS,
			"minJoinBond": 1 * DOLLARS,
		},
		"evmChainId": { "chainId": evm_chain_id },
		"evm": { "accounts": evm_accounts },
	})
}


fn development_config_genesis_json() -> serde_json::Value {
	testnet_genesis(
		vec![authority_keys_from_alice()],// vec![AccountId::from(hex!("d43593c715fdd31c61141abd04a99fd6822c8558"))],
		vec![],
		AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")),
		Some(vec![AccountId::from(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac"))]),
		42u32,
	)
}

/// Development config (single validator Alice).
pub fn development_config() -> ChainSpec {
	ChainSpec::builder(wasm_binary_unwrap(), Default::default())
		.with_name("Development")
		.with_id("dev")
		// .with_id()
		.with_chain_type(ChainType::Development)
		// .with_properties(serde_json::from_str("{\"tokenDecimals\": 18, \"tokenSymbol\": \"TSCS\"}")
		.with_properties(serde_json::from_str("{\"isEthereum\": true, \"tokenDecimals\": 18, \"tokenSymbol\": \"TSCS\"}")
							 .expect("Provided valid json map"),)
		.with_genesis_config_patch(development_config_genesis_json())
		.build()
}

// fn local_testnet_genesis() -> serde_json::Value {
// 	testnet_genesis(
// 		vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
// 		vec![],
// 		get_account_id_from_seed::<ecdsa::Public>("Alice"),
// 		None,
// 	)
// }
//
// /// Local testnet config (multivalidator Alice + Bob).
// pub fn local_testnet_config() -> ChainSpec {
// 	ChainSpec::builder(wasm_binary_unwrap(), Default::default())
// 		.with_name("Local Testnet")
// 		.with_id("local_testnet")
// 		.with_chain_type(ChainType::Local)
// 		.with_genesis_config_patch(local_testnet_genesis())
// 		.build()
// }

#[cfg(test)]
pub(crate) mod tests {
	use super::*;
	use crate::service::{new_full_base, NewFullBase};
	use sc_service_test;
	use sp_runtime::BuildStorage;

	/// Local testnet config (single validator - Alice).
	pub fn integration_test_config_with_single_authority() -> ChainSpec {
		ChainSpec::builder(wasm_binary_unwrap(), Default::default())
			.with_name("Integration Test")
			.with_id("test")
			.with_chain_type(ChainType::Development)
			.with_genesis_config_patch(testnet_genesis(
				vec![authority_keys_from_seed("Alice")],
				vec![],
				get_account_id_from_seed::<ecdsa::Public>("Alice"),
				None,
			))
			.build()
	}

	/// Local testnet config (multivalidator Alice + Bob).
	pub fn integration_test_config_with_two_authorities() -> ChainSpec {
		ChainSpec::builder(wasm_binary_unwrap(), Default::default())
			.with_name("Integration Test")
			.with_id("test")
			.with_chain_type(ChainType::Development)
			.with_genesis_config_patch(local_testnet_genesis())
			.build()
	}

	#[test]
	#[ignore]
	fn test_connectivity() {
		sp_tracing::try_init_simple();

		sc_service_test::connectivity(integration_test_config_with_two_authorities(), |config| {
			let NewFullBase { task_manager, client, network, sync, transaction_pool, .. } =
				new_full_base::<sc_network::NetworkWorker<_, _>>(config, None, false, |_, _| ())?;
			Ok(sc_service_test::TestNetComponents::new(
				task_manager,
				client,
				network,
				sync,
				transaction_pool,
			))
		});
	}

	#[test]
	fn test_create_development_chain_spec() {
		development_config().build_storage().unwrap();
	}

	#[test]
	fn test_create_local_testnet_chain_spec() {
		local_testnet_config().build_storage().unwrap();
	}

	// #[test]
	// fn test_staging_test_net_chain_spec() {
	// 	staging_testnet_config().build_storage().unwrap();
	// }
}
