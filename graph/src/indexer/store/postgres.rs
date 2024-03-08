use std::sync::Arc;

use async_trait::async_trait;

use crate::anyhow::Result;
use crate::blockchain::BlockPtr;
use crate::components::store::{DeploymentLocator, WritableStore};
use crate::indexer::{BlockSender, EncodedTriggers, State};
use crate::{components::store::BlockNumber, indexer::IndexerStore};

pub struct PostgresIndexerDB {
    store: Arc<dyn WritableStore>,
    deployment: DeploymentLocator,
}

impl PostgresIndexerDB {
    pub fn new(store: Arc<dyn WritableStore>, deployment: DeploymentLocator) -> Self {
        Self { store, deployment }
    }
}

#[async_trait]
impl IndexerStore for PostgresIndexerDB {
    async fn get_last_stable_block(&self) -> Result<Option<BlockNumber>> {
        Ok(self.store.block_ptr().map(|b| b.block_number()))
    }

    async fn stream_from(&self, bn: BlockNumber, bs: BlockSender) -> Result<()> {
        unimplemented!()
    }
    async fn get(&self, bn: BlockNumber) -> Result<Option<EncodedTriggers>> {
        unimplemented!()
    }
    async fn set(&self, bn: BlockPtr, state: &State, triggers: EncodedTriggers) -> Result<()> {
        unimplemented!()
    }
    async fn get_state(&self, bn: BlockNumber) -> Result<State> {
        unimplemented!()
    }
    async fn set_last_stable_block(&self, bn: BlockNumber) -> Result<()> {
        unimplemented!()
    }
}
