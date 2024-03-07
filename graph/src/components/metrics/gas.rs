use crate::blockchain::BlockPtr;
use crate::components::store::{DeploymentId, Entity};
use crate::data::store::Id;
use crate::env::ENV_VARS;
use crate::schema::EntityType;
use crate::spawn;
use crate::util::cache_weight::CacheWeight;
use anyhow::{anyhow, Result};
use csv::Writer;
use object_store::gcp::GoogleCloudStorageBuilder;
use object_store::path::Path;
use object_store::ObjectStore;
use serde::Serialize;
use slog::{error, info, Logger};
use std::sync::RwLock;
use std::{collections::HashMap, sync::Arc};
use url::Url;

pub struct GasMetrics {
    pub gas_counter: Arc<RwLock<HashMap<String, u64>>>,
    pub op_counter: Arc<RwLock<HashMap<String, u64>>>,
    pub read_bytes_counter: Arc<RwLock<HashMap<(EntityType, Id), u64>>>,
    pub write_bytes_counter: Arc<RwLock<HashMap<(EntityType, Id), u64>>>,
}

impl GasMetrics {
    pub fn new() -> Self {
        let gas_counter = Arc::new(RwLock::new(HashMap::new()));
        let op_counter = Arc::new(RwLock::new(HashMap::new()));
        let read_bytes_counter = Arc::new(RwLock::new(HashMap::new()));
        let write_bytes_counter = Arc::new(RwLock::new(HashMap::new()));

        GasMetrics {
            gas_counter,
            op_counter,
            read_bytes_counter,
            write_bytes_counter,
        }
    }

    fn serialize_to_csv<T: Serialize, U: Serialize, I: IntoIterator<Item = T>>(
        data: I,
        column_names: U,
    ) -> Result<String> {
        let mut wtr = Writer::from_writer(vec![]);
        wtr.serialize(column_names)?;
        for record in data {
            wtr.serialize(record)?;
        }
        wtr.flush()?;
        Ok(String::from_utf8(wtr.into_inner()?)?)
    }

    fn map_to_csv(data: &HashMap<String, u64>, column_names: (&str, &str)) -> Result<String> {
        Self::serialize_to_csv(data.iter().map(|(key, value)| (key, value)), column_names)
    }

    fn entity_stats_to_csv(
        data: &HashMap<(EntityType, Id), u64>,
        column_names: (&str, &str, &str),
    ) -> Result<String> {
        Self::serialize_to_csv(
            data.iter()
                .map(|((entity_type, id), value)| (entity_type.typename(), id.to_string(), value)),
            column_names,
        )
    }

    async fn write_csv_to_store(bucket: &str, path: &str, data: String) -> Result<()> {
        let data_bytes = data.into_bytes();

        let bucket =
            Url::parse(&bucket).map_err(|e| anyhow!("Failed to parse bucket url: {}", e))?;
        let store = GoogleCloudStorageBuilder::from_env()
            .with_url(bucket)
            .build()?;

        store.put(&Path::parse(path)?, data_bytes.into()).await?;

        Ok(())
    }

    /// Flushes gas and op metrics to an object storage asynchronously, clearing metrics maps afterward.
    ///
    /// Serializes metrics to CSV and spawns tasks for object storage upload, logging successes or errors.
    /// Metrics are organized by block number and subgraph ID in the object storage bucket.
    /// Returns `Ok(())` to indicate the upload process has started.
    pub fn flush_metrics_to_store(
        &self,
        logger: &Logger,
        block_ptr: BlockPtr,
        subgraph_id: DeploymentId,
    ) -> Result<()> {
        let logger = logger.clone();
        let gas_data = Self::map_to_csv(&self.gas_counter.read().unwrap(), ("method", "gas"))?;
        let op_data = Self::map_to_csv(&self.op_counter.read().unwrap(), ("method", "count"))?;
        let read_bytes_data = Self::entity_stats_to_csv(
            &self.read_bytes_counter.read().unwrap(),
            ("entity", "id", "bytes"),
        )?;
        let write_bytes_data = Self::entity_stats_to_csv(
            &self.write_bytes_counter.read().unwrap(),
            ("entity", "id", "bytes"),
        )?;
        let bucket = &ENV_VARS.dips_metrics_object_store_url;

        match bucket {
            Some(bucket) => {
                spawn(async move {
                    let gas_file = format!("{}/{}/gas.csv", subgraph_id, block_ptr.number);
                    let op_file = format!("{}/{}/op.csv", subgraph_id, block_ptr.number);
                    let read_bytes_file =
                        format!("{}/{}/read_bytes.csv", subgraph_id, block_ptr.number);
                    let write_bytes_file =
                        format!("{}/{}/write_bytes.csv", subgraph_id, block_ptr.number);

                    match Self::write_csv_to_store(bucket, &gas_file, gas_data).await {
                        Ok(_) => {
                            info!(
                                logger,
                                "Wrote gas metrics to object-store for block {}", block_ptr.number
                            );
                        }
                        Err(e) => {
                            error!(logger, "Error writing gas metrics to object-store: {}", e)
                        }
                    }

                    match Self::write_csv_to_store(bucket, &op_file, op_data).await {
                        Ok(_) => {
                            info!(
                                logger,
                                "Wrote op metrics to object-store for block {}", block_ptr.number
                            );
                        }
                        Err(e) => error!(logger, "Error writing op metrics to object-store: {}", e),
                    }

                    match Self::write_csv_to_store(bucket, &read_bytes_file, read_bytes_data).await
                    {
                        Ok(_) => {
                            info!(
                                logger,
                                "Wrote read bytes metrics to object-store for block {}",
                                block_ptr.number
                            );
                        }
                        Err(e) => {
                            error!(
                                logger,
                                "Error writing read bytes metrics to object-store: {}", e
                            )
                        }
                    }

                    match Self::write_csv_to_store(bucket, &write_bytes_file, write_bytes_data)
                        .await
                    {
                        Ok(_) => {
                            info!(
                                logger,
                                "Wrote write bytes metrics to object-store for block {}",
                                block_ptr.number
                            );
                        }
                        Err(e) => {
                            error!(
                                logger,
                                "Error writing write bytes metrics to object-store: {}", e
                            )
                        }
                    }
                });
            }
            None => return Err(anyhow!("Failed to parse object store URL"))?,
        }

        self.reset_counters();

        Ok(())
    }

    pub fn mock() -> Arc<Self> {
        Arc::new(Self::new())
    }

    // Reset all counters
    pub fn reset_counters(&self) {
        self.gas_counter.write().unwrap().clear();
        self.op_counter.write().unwrap().clear();
        self.read_bytes_counter.write().unwrap().clear();
        self.write_bytes_counter.write().unwrap().clear();
    }

    pub fn track_gas(&self, method: &str, gas_used: u64) {
        let mut map = self.gas_counter.write().unwrap(); //
        let counter = map.entry(method.to_string()).or_insert(0);
        *counter += gas_used;
    }

    pub fn track_operations(&self, method: &str, op_count: u64) {
        let mut map = self.op_counter.write().unwrap();
        let counter = map.entry(method.to_string()).or_insert(0);
        *counter += op_count;
    }

    pub fn track_entity_read(&self, entity_type: &EntityType, entity: &Entity) {
        let mut map = self.read_bytes_counter.write().unwrap();

        let counter = map.entry((entity_type.clone(), entity.id())).or_insert(0);
        *counter += entity.weight() as u64;
    }

    pub fn track_entity_write(&self, entity_type: &EntityType, entity: &Entity) {
        let mut map = self.write_bytes_counter.write().unwrap();

        let counter = map.entry((entity_type.clone(), entity.id())).or_insert(0);
        *counter += entity.weight() as u64;
    }

    pub fn track_entity_read_batch(&self, entity_type: &EntityType, entities: &[Entity]) {
        let mut map = self.read_bytes_counter.write().unwrap();

        for entity in entities {
            let counter = map.entry((entity_type.clone(), entity.id())).or_insert(0);
            *counter += entity.weight() as u64;
        }
    }

    pub fn track_entity_write_batch(&self, entity_type: &EntityType, entities: &[Entity]) {
        let mut map = self.write_bytes_counter.write().unwrap();

        for entity in entities {
            let counter = map.entry((entity_type.clone(), entity.id())).or_insert(0);
            *counter += entity.weight() as u64;
        }
    }
}
