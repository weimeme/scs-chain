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

use polkadot_sdk::*;

// use super::benchmarking::{inherent_benchmark_data, RemarkBuilder, TransferKeepAliveBuilder};
use crate::{
    chain_spec, service,
    service::{new_partial, FullClient},
    Cli, Subcommand,
};
use frame_benchmarking_cli::*;
use common_runtime::opaque::Block;

#[cfg(feature="scs")]
use kitchensink_mainnet_runtime::{ExistentialDeposit, RuntimeApi};
#[cfg(feature="tscs")]
use kitchensink_testnet_runtime::{ExistentialDeposit, RuntimeApi};

use sc_network::{Litep2pNetworkBackend, NetworkBackend};
// use node_primitives::Block;
use sc_cli::{Result, SubstrateCli};
use sc_service::PartialComponents;
// use sp_keyring::Sr25519Keyring;
use sp_runtime::traits::HashingFor;

use std::sync::Arc;

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "Substrate Node".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        env!("CARGO_PKG_DESCRIPTION").into()
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/paritytech/polkadot-sdk/issues/new".into()
    }

    fn copyright_start_year() -> i32 {
        2017
    }

    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
        #[cfg(feature="scs")]
        let spec = match id {
            "" => {
                return Err(
                    "Please specify which chain you want to run, e.g. --dev or --chain=local"
                        .into(),
                )
            },
            "dev" => Box::new(chain_spec::mainnet::development_config()),
            // "local" => Box::new(chain_spec::local_testnet_config()),	// (
            // 		// 	AccountId::from(hex!("3C53131b57B966aB755a88D458B2D60cD17Fd1FC")),
            // 		// 	AccountId::from(hex!("FCec624D3ACF3fCD4979195014EB18e0150f6E2D")),
            // 		// 	// H160::from_str("3C53131b57B966aB755a88D458B2D60cD17Fd1FC").expect("internal H160 is valid; qed").into(),
            // 		// 	// H160::from_str("FCec624D3ACF3fCD4979195014EB18e0150f6E2D").expect("internal H160 is valid; qed").into(),
            // 		// 	// 5FMAvJiAMDJgvm3WWMF7ahouBeMtf5qsXYSv7hacuSic8TW5
            // 		// 	array_bytes::hex2array_unchecked("9143ba611eee5bb7bc7d41dfc30429e405ef42be6734d3ca5f86f2ab6299129b")
            // 		// 		.unchecked_into(),
            // 		// 	// 5GCEEHnYhuCBtkLyaUnStomyoJAwX3jS2yhUKct6gK9wcip3
            // 		// 	array_bytes::hex2array_unchecked("b6ade54294ac176068f9474df6137a4f7f1a692de610a571091cd8b2ae94e674")
            // 		// 		.unchecked_into(),
            // 		// 	// 5GCEEHnYhuCBtkLyaUnStomyoJAwX3jS2yhUKct6gK9wcip3
            // 		// 	array_bytes::hex2array_unchecked("b6ade54294ac176068f9474df6137a4f7f1a692de610a571091cd8b2ae94e674")
            // 		// 		.unchecked_into(),
            // 		// 	// 5GCEEHnYhuCBtkLyaUnStomyoJAwX3jS2yhUKct6gK9wcip3
            // 		// 	array_bytes::hex2array_unchecked("b6ade54294ac176068f9474df6137a4f7f1a692de610a571091cd8b2ae94e674")
            // 		// 		.unchecked_into(),
            // 		// 	// 5GCEEHnYhuCBtkLyaUnStomyoJAwX3jS2yhUKct6gK9wcip3
            // 		// 	array_bytes::hex2array_unchecked("b6ade54294ac176068f9474df6137a4f7f1a692de610a571091cd8b2ae94e674")
            // 		// 		.unchecked_into(),
            // 		// 	// 5DnnuaPZq8FYZFHJsyF8WDfTcU7f4t2u8rjX8yEjhbkc3Q7r
            // 		// 	array_bytes::hex2array_unchecked("03dfb0588ca98d1974feb9c5ac4d8ac5c9b877051e07c80a3db431a75d4861b734")
            // 		// 		.unchecked_into(),
            // 		// ),
            // "fir" | "flaming-fir" => Box::new(chain_spec::flaming_fir_config()?),
            "staging" => Box::new(chain_spec::mainnet::tscs_config()?),
            "local" => Box::new(chain_spec::mainnet::staging_testnet_config()),
            path => Box::new(chain_spec::mainnet::ChainSpec::from_json_file(
                std::path::PathBuf::from(path),
            )?),
        };

        #[cfg(feature="tscs")]
        let spec = match id {
            "" => {
                return Err(
                    "Please specify which chain you want to run, e.g. --dev or --chain=local"
                        .into(),
                )
            },
            "dev" => Box::new(chain_spec::testnet::development_config()),
            // "local" => Box::new(chain_spec::local_testnet_config()),	// (
            // 		// 	AccountId::from(hex!("3C53131b57B966aB755a88D458B2D60cD17Fd1FC")),
            // 		// 	AccountId::from(hex!("FCec624D3ACF3fCD4979195014EB18e0150f6E2D")),
            // 		// 	// H160::from_str("3C53131b57B966aB755a88D458B2D60cD17Fd1FC").expect("internal H160 is valid; qed").into(),
            // 		// 	// H160::from_str("FCec624D3ACF3fCD4979195014EB18e0150f6E2D").expect("internal H160 is valid; qed").into(),
            // 		// 	// 5FMAvJiAMDJgvm3WWMF7ahouBeMtf5qsXYSv7hacuSic8TW5
            // 		// 	array_bytes::hex2array_unchecked("9143ba611eee5bb7bc7d41dfc30429e405ef42be6734d3ca5f86f2ab6299129b")
            // 		// 		.unchecked_into(),
            // 		// 	// 5GCEEHnYhuCBtkLyaUnStomyoJAwX3jS2yhUKct6gK9wcip3
            // 		// 	array_bytes::hex2array_unchecked("b6ade54294ac176068f9474df6137a4f7f1a692de610a571091cd8b2ae94e674")
            // 		// 		.unchecked_into(),
            // 		// 	// 5GCEEHnYhuCBtkLyaUnStomyoJAwX3jS2yhUKct6gK9wcip3
            // 		// 	array_bytes::hex2array_unchecked("b6ade54294ac176068f9474df6137a4f7f1a692de610a571091cd8b2ae94e674")
            // 		// 		.unchecked_into(),
            // 		// 	// 5GCEEHnYhuCBtkLyaUnStomyoJAwX3jS2yhUKct6gK9wcip3
            // 		// 	array_bytes::hex2array_unchecked("b6ade54294ac176068f9474df6137a4f7f1a692de610a571091cd8b2ae94e674")
            // 		// 		.unchecked_into(),
            // 		// 	// 5GCEEHnYhuCBtkLyaUnStomyoJAwX3jS2yhUKct6gK9wcip3
            // 		// 	array_bytes::hex2array_unchecked("b6ade54294ac176068f9474df6137a4f7f1a692de610a571091cd8b2ae94e674")
            // 		// 		.unchecked_into(),
            // 		// 	// 5DnnuaPZq8FYZFHJsyF8WDfTcU7f4t2u8rjX8yEjhbkc3Q7r
            // 		// 	array_bytes::hex2array_unchecked("03dfb0588ca98d1974feb9c5ac4d8ac5c9b877051e07c80a3db431a75d4861b734")
            // 		// 		.unchecked_into(),
            // 		// ),
            // "fir" | "flaming-fir" => Box::new(chain_spec::flaming_fir_config()?),
            "staging" => Box::new(chain_spec::testnet::tscs_config()?),
            "local" => Box::new(chain_spec::testnet::staging_testnet_config()),
            path => Box::new(chain_spec::testnet::ChainSpec::from_json_file(
                std::path::PathBuf::from(path),
            )?),
        };
        Ok(spec)
    }

}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
    let cli = Cli::from_args();

    match &cli.subcommand {
        None => {
            let runner = cli.create_runner(&cli.run)?;
            runner.run_node_until_exit(|config| async move {
                service::new_full(config, cli.eth.clone(), cli).map_err(sc_cli::Error::Service)
            })
        }
        Some(Subcommand::Inspect(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            runner.sync_run(|config| cmd.run::<Block, RuntimeApi>(config))
        }
        // Some(Subcommand::Benchmark(cmd)) => {
        // 	let runner = cli.create_runner(cmd)?;
        //
        // 	runner.sync_run(|config| {
        // 		// This switch needs to be in the client, since the client decides
        // 		// which sub-commands it wants to support.
        // 		match cmd {
        // 			BenchmarkCmd::Pallet(cmd) => {
        // 				if !cfg!(feature = "runtime-benchmarks") {
        // 					return Err(
        // 						"Runtime benchmarking wasn't enabled when building the node. \
        // 					You can enable it with `--features runtime-benchmarks`."
        // 							.into(),
        // 					)
        // 				}
        //
        // 				cmd.run_with_spec::<HashingFor<Block>, sp_statement_store::runtime_api::HostFunctions>(Some(config.chain_spec))
        // 			},
        // 			BenchmarkCmd::Block(cmd) => {
        // 				// ensure that we keep the task manager alive
        // 				let partial = new_partial(&config, None)?;
        // 				cmd.run(partial.client)
        // 			},
        // 			#[cfg(not(feature = "runtime-benchmarks"))]
        // 			BenchmarkCmd::Storage(_) => Err(
        // 				"Storage benchmarking can be enabled with `--features runtime-benchmarks`."
        // 					.into(),
        // 			),
        // 			#[cfg(feature = "runtime-benchmarks")]
        // 			BenchmarkCmd::Storage(cmd) => {
        // 				// ensure that we keep the task manager alive
        // 				let partial = new_partial(&config, None)?;
        // 				let db = partial.backend.expose_db();
        // 				let storage = partial.backend.expose_storage();
        //
        // 				cmd.run(config, partial.client, db, storage)
        // 			},
        // 			BenchmarkCmd::Overhead(cmd) => {
        // 				// ensure that we keep the task manager alive
        // 				let partial = new_partial(&config, None)?;
        // 				let ext_builder = RemarkBuilder::new(partial.client.clone());
        //
        // 				cmd.run(
        // 					config,
        // 					partial.client,
        // 					inherent_benchmark_data()?,
        // 					Vec::new(),
        // 					&ext_builder,
        // 				)
        // 			},
        // 			BenchmarkCmd::Extrinsic(cmd) => {
        // 				// ensure that we keep the task manager alive
        // 				let partial = service::new_partial(&config, None)?;
        // 				// Register the *Remark* and *TKA* builders.
        // 				let ext_factory = ExtrinsicFactory(vec![
        // 					Box::new(RemarkBuilder::new(partial.client.clone())),
        // 					Box::new(TransferKeepAliveBuilder::new(
        // 						partial.client.clone(),
        // 						Sr25519Keyring::Alice.to_account_id(),
        // 						ExistentialDeposit::get(),
        // 					)),
        // 				]);
        //
        // 				cmd.run(
        // 					partial.client,
        // 					inherent_benchmark_data()?,
        // 					Vec::new(),
        // 					&ext_factory,
        // 				)
        // 			},
        // 			BenchmarkCmd::Machine(cmd) =>
        // 				cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone()),
        // 		}
        // 	})
        // },
        Some(Subcommand::Key(cmd)) => cmd.run(&cli),
        Some(Subcommand::Sign(cmd)) => cmd.run(),
        Some(Subcommand::Verify(cmd)) => cmd.run(),
        Some(Subcommand::Vanity(cmd)) => cmd.run(),
        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        }
        Some(Subcommand::CheckBlock(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    import_queue,
                    ..
                } = new_partial::<Litep2pNetworkBackend>(&config, &cli.eth, None)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        }
        Some(Subcommand::ExportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    ..
                } = new_partial::<Litep2pNetworkBackend>(&config, &cli.eth, None)?;
                Ok((cmd.run(client, config.database), task_manager))
            })
        }
        Some(Subcommand::ExportState(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    ..
                } = new_partial::<Litep2pNetworkBackend>(&config, &cli.eth, None)?;
                Ok((cmd.run(client, config.chain_spec), task_manager))
            })
        }
        Some(Subcommand::ImportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    import_queue,
                    ..
                } = new_partial::<Litep2pNetworkBackend>(&config, &cli.eth, None)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        }
        Some(Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.database))
        }
        Some(Subcommand::Revert(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    backend,
                    ..
                } = new_partial::<Litep2pNetworkBackend>(&config, &cli.eth, None)?;
                let aux_revert = Box::new(|client: Arc<FullClient>, backend, blocks| {
                    sc_consensus_babe::revert(client.clone(), backend, blocks)?;
                    sc_consensus_grandpa::revert(client, blocks)?;
                    Ok(())
                });
                Ok((cmd.run(client, backend, Some(aux_revert)), task_manager))
            })
        }
        Some(Subcommand::ChainInfo(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run::<Block>(&config))
        }

        // 测试专用
        Some(_) => {
            unreachable!()
        }
    }
}
