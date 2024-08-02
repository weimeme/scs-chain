use std::{
    collections::BTreeMap,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};
use polkadot_sdk::*;
pub use crate::cli::{EthConfiguration, BackendType};
use futures::{future, prelude::*};
// Substrate
use sc_client_api::BlockchainEvents;
use sc_executor::HostFunctions;
use sc_network_sync::SyncingService;
use sc_service::{error::Error as ServiceError, Configuration, TaskManager};
use sp_api::ConstructRuntimeApi;
use sp_core::H256;
use sp_runtime::traits::Block as BlockT;
// Frontier
pub use fc_consensus::FrontierBlockImport;
use fc_rpc::EthTask;
pub use fc_rpc_core::types::{FeeHistoryCache, FeeHistoryCacheLimit, FilterPool};
pub use fc_storage::{StorageOverride, StorageOverrideHandler};

use crate::client::{FullBackend, FullClient};

/// Frontier DB backend type.
pub type FrontierBackend<B, C> = fc_db::Backend<B, C>;

pub fn db_config_dir(config: &Configuration) -> PathBuf {
    config.base_path.config_dir(config.chain_spec.id())
}



pub struct FrontierPartialComponents {
    pub filter_pool: Option<FilterPool>,
    pub fee_history_cache: FeeHistoryCache,
    pub fee_history_cache_limit: FeeHistoryCacheLimit,
}

pub fn new_frontier_partial(
    config: &EthConfiguration,
) -> Result<FrontierPartialComponents, ServiceError> {
    Ok(FrontierPartialComponents {
        filter_pool: Some(Arc::new(Mutex::new(BTreeMap::new()))),
        fee_history_cache: Arc::new(Mutex::new(BTreeMap::new())),
        fee_history_cache_limit: config.fee_history_limit,
    })
}

/// A set of APIs that ethereum-compatible runtimes must implement.
pub trait EthCompatRuntimeApiCollection<Block: BlockT>:
sp_api::ApiExt<Block>
+ fp_rpc::ConvertTransactionRuntimeApi<Block>
+ fp_rpc::EthereumRuntimeRPCApi<Block>
{
}

impl<Block, Api> EthCompatRuntimeApiCollection<Block> for Api
    where
        Block: BlockT,
        Api: sp_api::ApiExt<Block>
        + fp_rpc::ConvertTransactionRuntimeApi<Block>
        + fp_rpc::EthereumRuntimeRPCApi<Block>,
{
}