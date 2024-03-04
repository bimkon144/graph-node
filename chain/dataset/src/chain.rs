use crate::runtime::RuntimeAdapter;
use crate::{data_source::*, TriggerData, TriggerFilter, TriggersAdapter};
use anyhow::{anyhow, Error};
use graph::blockchain::client::ChainClient;
use graph::blockchain::{
    BasicBlockchainBuilder, BlockIngestor, BlockTime, EmptyNodeCapabilities, NoopRuntimeAdapter,
};
use graph::components::store::DeploymentCursorTracker;
use graph::env::EnvVars;
use graph::indexer::block_stream::IndexerBlockStream;
use graph::indexer::store::{SledIndexerStore, DB_NAME};
use graph::prelude::{
    BlockHash, CheapClone, DeploymentHash, EthereumCallCache, LoggerFactory, MetricsRegistry,
};
use graph::{
    blockchain::{
        self,
        block_stream::{BlockStream, FirehoseCursor},
        BlockPtr, Blockchain, BlockchainKind, IngestorError, RuntimeAdapter as RuntimeAdapterTrait,
    },
    components::store::DeploymentLocator,
    data::subgraph::UnifiedMappingApiVersion,
    prelude::{async_trait, BlockNumber, ChainStore},
    slog::Logger,
};
use graph_chain_ethereum::network::EthereumNetworkAdapters;

use std::sync::Arc;

#[derive(Default, Debug, Clone)]
pub struct Block {
    pub hash: BlockHash,
    pub number: BlockNumber,
    pub data: Box<[u8]>,
}

impl blockchain::Block for Block {
    fn ptr(&self) -> BlockPtr {
        BlockPtr {
            hash: self.hash.clone(),
            number: self.number,
        }
    }

    fn parent_ptr(&self) -> Option<BlockPtr> {
        None
    }

    fn timestamp(&self) -> BlockTime {
        BlockTime::NONE
    }
}

pub struct Chain {
    pub(crate) eth_adapters: Option<Arc<EthereumNetworkAdapters>>,
    pub(crate) call_cache: Arc<dyn EthereumCallCache>,
    pub(crate) logger_factory: LoggerFactory,
    pub(crate) metrics_registry: Arc<MetricsRegistry>,
}

impl Chain {
    pub fn new(
        eth_adapters: Option<Arc<EthereumNetworkAdapters>>,
        eth_call_cache: Arc<dyn EthereumCallCache>,
        logger_factory: LoggerFactory,
        metrics_registry: Arc<MetricsRegistry>,
    ) -> Self {
        Self {
            logger_factory,
            metrics_registry,
            eth_adapters,
            call_cache: eth_call_cache,
        }
    }
}

impl std::fmt::Debug for Chain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "chain: substreams")
    }
}

#[async_trait]
impl Blockchain for Chain {
    const KIND: BlockchainKind = BlockchainKind::Dataset;

    type Client = ();
    type Block = Block;
    type DataSource = DataSource;
    type UnresolvedDataSource = UnresolvedDataSource;

    type DataSourceTemplate = NoopDataSourceTemplate;
    type UnresolvedDataSourceTemplate = NoopDataSourceTemplate;

    /// Trigger data as parsed from the triggers adapter.
    type TriggerData = TriggerData;

    /// Decoded trigger ready to be processed by the mapping.
    /// New implementations should have this be the same as `TriggerData`.
    type MappingTrigger = TriggerData;

    /// Trigger filter used as input to the triggers adapter.
    type TriggerFilter = TriggerFilter;

    type NodeCapabilities = EmptyNodeCapabilities<Self>;

    fn triggers_adapter(
        &self,
        _log: &DeploymentLocator,
        _capabilities: &Self::NodeCapabilities,
        _unified_api_version: UnifiedMappingApiVersion,
    ) -> Result<Arc<dyn blockchain::TriggersAdapter<Self>>, Error> {
        Ok(Arc::new(TriggersAdapter {}))
    }

    async fn new_block_stream(
        &self,
        _deployment: DeploymentLocator,
        _store: impl DeploymentCursorTracker,
        _start_blocks: Vec<BlockNumber>,
        filter: Arc<Self::TriggerFilter>,
        _unified_api_version: UnifiedMappingApiVersion,
    ) -> Result<Box<dyn BlockStream<Self>>, Error> {
        let deployment: &str = "QmagGaBm7FL9uQWg1bk52Eb3LTN4owkvxEKkirtyXNLQc9";
        let hash = DeploymentHash::new(deployment).unwrap();
        let db = Arc::new(sled::open(DB_NAME).unwrap());
        let store = Arc::new(
            SledIndexerStore::new(
                db,
                &hash,
                graph::indexer::store::StateSnapshotFrequency::Never,
            )
            .unwrap(),
        );
        let logger = graph::log::logger(true);

        let metrics = Arc::new(MetricsRegistry::mock());
        let handler = filter
            .handler
            .as_ref()
            .ok_or(anyhow!("Expected dataset block stream to have a handler"))?
            .clone();

        Ok(Box::new(IndexerBlockStream::<Self>::new(
            hash.clone(),
            store,
            None,
            vec![],
            vec![],
            logger.clone(),
            handler,
            metrics,
        )))
    }

    fn is_refetch_block_required(&self) -> bool {
        false
    }
    async fn refetch_firehose_block(
        &self,
        _logger: &Logger,
        _cursor: FirehoseCursor,
    ) -> Result<Block, Error> {
        unimplemented!("This chain does not support Dynamic Data Sources. is_refetch_block_required always returns false, this shouldn't be called.")
    }

    fn chain_store(&self) -> Arc<dyn ChainStore> {
        unimplemented!("Dataset chain has no chain_store");
    }

    async fn block_pointer_from_number(
        &self,
        _logger: &Logger,
        number: BlockNumber,
    ) -> Result<BlockPtr, IngestorError> {
        // This is the same thing TriggersAdapter does, not sure if it's going to work but
        // we also don't yet have a good way of getting this value until we sort out the
        // chain store.
        // TODO(filipe): Fix this once the chain_store is correctly setup for substreams.
        Ok(BlockPtr {
            hash: BlockHash::from(vec![0xff; 32]),
            number,
        })
    }
    fn runtime_adapter(&self) -> Arc<dyn RuntimeAdapterTrait<Self>> {
        match self.eth_adapters {
            None => Arc::new(NoopRuntimeAdapter::default()),
            Some(ref adapters) => Arc::new(RuntimeAdapter {
                eth_adapters: adapters.cheap_clone(),
                call_cache: self.call_cache.cheap_clone(),
            }),
        }
    }

    fn chain_client(&self) -> Arc<ChainClient<Self>> {
        Arc::new(ChainClient::Rpc(()))
    }

    fn block_ingestor(&self) -> anyhow::Result<Box<dyn BlockIngestor>> {
        anyhow::bail!("Datasets don't use block ingestors")
    }
}
