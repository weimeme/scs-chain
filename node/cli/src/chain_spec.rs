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
const ENDOWMENT: Balance = 100 * DOLLARS;
const STASH: Balance = ENDOWMENT;

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
	// for i in 1 2 3 ; do for j in stash controller; do subkey inspect "$secret"/fir/$j/$i; done; done
	//
	// and
	//
	// for i in 1 2 3 ; do for j in session; do subkey inspect --scheme ed25519 "$secret"//fir//$j//$i; done; done
	//
	// and
	//
	// for i in 1 2 3 ; do for j in session; do subkey inspect --scheme ecdsa "$secret"//fir//$j//$i; done; done

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
			// fixme
			H160::from_str("0x6Cf000856e98d31D35ed49FfD87ce6027A65D4f2").expect("internal H160 is valid; qed").into(),
			H160::from_str("0x14a38ebBC12316D678a2f3BA1941637c9E3090AF").expect("internal H160 is valid; qed").into(),
			// 5FqDUrCH112RHzKoihRtv71XDNThRhkCfP9iHyHWZi5Sj82U
			array_bytes::hex2array_unchecked("a6a738828c1fd3d438de6f9a9e4ff0f5cc3dcdeed062b1950d56a0769be3e462")
				.unchecked_into(),
			// 5HVpGS8qGVYUQpz5SBYYYYexjQxLujZrFuT1Yr9UX5KxpvsF
			array_bytes::hex2array_unchecked("f053e127aced17928a9ded609366027fa80dea0101a8255cdee0a7581ba4bf64")
				.unchecked_into(),
			// 5HVpGS8qGVYUQpz5SBYYYYexjQxLujZrFuT1Yr9UX5KxpvsF
			array_bytes::hex2array_unchecked("f053e127aced17928a9ded609366027fa80dea0101a8255cdee0a7581ba4bf64")
				.unchecked_into(),
			// 5HVpGS8qGVYUQpz5SBYYYYexjQxLujZrFuT1Yr9UX5KxpvsF
			array_bytes::hex2array_unchecked("f053e127aced17928a9ded609366027fa80dea0101a8255cdee0a7581ba4bf64")
				.unchecked_into(),
			// 5HVpGS8qGVYUQpz5SBYYYYexjQxLujZrFuT1Yr9UX5KxpvsF
			array_bytes::hex2array_unchecked("f053e127aced17928a9ded609366027fa80dea0101a8255cdee0a7581ba4bf64")
				.unchecked_into(),
			// 5HVpGS8qGVYUQpz5SBYYYYexjQxLujZrFuT1Yr9UX5KxpvsF
			array_bytes::hex2array_unchecked("700fcc2547efc641f58b4f2528ca52112e0fa7bfcfac4eb672c8ea7d1d9f29b1")
				.unchecked_into(),
		),
		(
			H160::from_str("0x3C53131b57B966aB755a88D458B2D60cD17Fd1FC").expect("internal H160 is valid; qed").into(),
			H160::from_str("0xFCec624D3ACF3fCD4979195014EB18e0150f6E2D").expect("internal H160 is valid; qed").into(),
			// 5FMAvJiAMDJgvm3WWMF7ahouBeMtf5qsXYSv7hacuSic8TW5
			array_bytes::hex2array_unchecked("9143ba611eee5bb7bc7d41dfc30429e405ef42be6734d3ca5f86f2ab6299129b")
				.unchecked_into(),
			// 5GCEEHnYhuCBtkLyaUnStomyoJAwX3jS2yhUKct6gK9wcip3
			array_bytes::hex2array_unchecked("b6ade54294ac176068f9474df6137a4f7f1a692de610a571091cd8b2ae94e674")
				.unchecked_into(),
			// 5GCEEHnYhuCBtkLyaUnStomyoJAwX3jS2yhUKct6gK9wcip3
			array_bytes::hex2array_unchecked("b6ade54294ac176068f9474df6137a4f7f1a692de610a571091cd8b2ae94e674")
				.unchecked_into(),
			// 5GCEEHnYhuCBtkLyaUnStomyoJAwX3jS2yhUKct6gK9wcip3
			array_bytes::hex2array_unchecked("b6ade54294ac176068f9474df6137a4f7f1a692de610a571091cd8b2ae94e674")
				.unchecked_into(),
			// 5GCEEHnYhuCBtkLyaUnStomyoJAwX3jS2yhUKct6gK9wcip3
			array_bytes::hex2array_unchecked("b6ade54294ac176068f9474df6137a4f7f1a692de610a571091cd8b2ae94e674")
				.unchecked_into(),
			// 5DnnuaPZq8FYZFHJsyF8WDfTcU7f4t2u8rjX8yEjhbkc3Q7r
			array_bytes::hex2array_unchecked("4c558a6b5c5b52280a0a856c4ecad7e7437c91bfe8f0d3c2256962cbc4d8bfd0")
				.unchecked_into(),
		),
		(
			H160::from_str("0xAcf2628C421137F6cb3E7D9c5B235B44ffdf9952").expect("internal H160 is valid; qed").into(),
			H160::from_str("0x778bb7D4E268AFcC0c535FC89d3b02b7c1e29C5e").expect("internal H160 is valid; qed").into(),
			// 5Dz8XhNjxggbpFtwzBTHAhc2dohnhdrcsno9TWkEyBZCKzZo
			array_bytes::hex2array_unchecked("54fb4bd83cf76c27d8d3ad1fdc929ea72cc83dd17f1b1a684c66219e1f773f8a")
				.unchecked_into(),
			// 5GBhA2kTNNQqFrDu8uU9DBRHVBVaSF65QXgXwA4ps7SA2DNz
			array_bytes::hex2array_unchecked("b6454b04a97110d57295eeb2809489f51f7b90b2371182c9c6e5a25d10e7467a")
				.unchecked_into(),
			// 5GBhA2kTNNQqFrDu8uU9DBRHVBVaSF65QXgXwA4ps7SA2DNz
			array_bytes::hex2array_unchecked("b6454b04a97110d57295eeb2809489f51f7b90b2371182c9c6e5a25d10e7467a")
				.unchecked_into(),
			// 5GBhA2kTNNQqFrDu8uU9DBRHVBVaSF65QXgXwA4ps7SA2DNz
			array_bytes::hex2array_unchecked("b6454b04a97110d57295eeb2809489f51f7b90b2371182c9c6e5a25d10e7467a")
				.unchecked_into(),
			// 5GBhA2kTNNQqFrDu8uU9DBRHVBVaSF65QXgXwA4ps7SA2DNz
			array_bytes::hex2array_unchecked("b6454b04a97110d57295eeb2809489f51f7b90b2371182c9c6e5a25d10e7467a")
				.unchecked_into(),
			// 5GPGYxmZifrfgGbokX1k8uYYjhL5uAi4Tt5efrZXCcyh49zJ
			array_bytes::hex2array_unchecked("bf19694c3135f399bc82cb51b81a9726a36e3db22d98941b08c97155c67fb8cb")
				.unchecked_into(),
		),
	];

	let root_key: AccountId = H160::from_str("79BD79C274C845E8a29378513c0053b19395E863").expect("internal H160 is valid; qed").into();

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
		.map(|x| (x.0.clone(), x.1.clone(), stash, StakerStatus::Validator))
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
	endowed_accounts: Option<Vec<AccountId>>,
	evm_chain_id: u32,
) -> serde_json::Value {
	let (initial_authorities, endowed_accounts, num_endowed_accounts, stakers) =
		configure_accounts(initial_authorities, initial_nominators, endowed_accounts, STASH);

	// let evm_accounts = {
	// 	let mut map = BTreeMap::new();
		// map.insert(
		// 	// H160 address of Alice dev account
		// 	// Derived from SS58 (42 prefix) address
		// 	// SS58: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
		// 	// hex: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
		// 	// Using the full hex key, truncating to the first 20 bytes (the first 40 hex chars)
		// 	H160::from_str("d43593c715fdd31c61141abd04a99fd6822c8558")
		// 		.expect("internal H160 is valid; qed"),
		// 	fp_evm::GenesisAccount {
		// 		balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
		// 			.expect("internal U256 is valid; qed"),
		// 		code: Default::default(),
		// 		nonce: Default::default(),
		// 		storage: Default::default(),
		// 	},
		// );
		// map.insert(
		// 	// H160 address of CI test runner account
		// 	H160::from_str("6be02d1d3665660d22ff9624b7be0551ee1ac91b")
		// 		.expect("internal H160 is valid; qed"),
		// 	fp_evm::GenesisAccount {
		// 		balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
		// 			.expect("internal U256 is valid; qed"),
		// 		code: Default::default(),
		// 		nonce: Default::default(),
		// 		storage: Default::default(),
		// 	},
		// );
		// map.insert(
		// 	// H160 address for benchmark usage
		// 	H160::from_str("1000000000000000000000000000000000000001")
		// 		.expect("internal H160 is valid; qed"),
		// 	fp_evm::GenesisAccount {
		// 		nonce: U256::from(1),
		// 		balance: U256::from(1_000_000_000_000_000_000_000_000u128),
		// 		storage: Default::default(),
		// 		code: vec![0x00],
		// 	},
		// );
	// 	map
	// };

	serde_json::json!({
		"balances": {
			"balances": endowed_accounts.iter().cloned().map(|x| (x, ENDOWMENT)).collect::<Vec<_>>(),
		},
		"session": {
			"keys": initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.1.clone(),
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
		// "assets": {
		// 	"assets": vec![],
		// },
		"nominationPools": {
			"minCreateBond": 10 * DOLLARS,
			"minJoinBond": 1 * DOLLARS,
		},
		"evmChainId": { "chainId": evm_chain_id },
		// "evm": { "accounts": evm_accounts },
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
