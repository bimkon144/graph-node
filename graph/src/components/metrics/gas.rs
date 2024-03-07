use crate::blockchain::BlockPtr;
use crate::components::store::DeploymentId;
use crate::env::ENV_VARS;
use crate::spawn;
use anyhow::{anyhow, Result};
use csv::Writer;
use object_store::gcp::GoogleCloudStorageBuilder;
use object_store::path::Path;
use object_store::ObjectStore;
use slog::{error, info, Logger};
use std::sync::RwLock;
use std::{collections::HashMap, sync::Arc};
use url::Url;

pub struct GasMetrics {
    pub gas_counter: Arc<RwLock<HashMap<String, u64>>>,
    pub op_counter: Arc<RwLock<HashMap<String, u64>>>,
}

impl GasMetrics {
    pub fn new() -> Self {
        let gas_counter = Arc::new(RwLock::new(HashMap::new()));
        let op_counter = Arc::new(RwLock::new(HashMap::new()));

        GasMetrics {
            gas_counter,
            op_counter,
        }
    }

    // Converts the map to CSV and returns it as a String
    fn map_to_csv(data: &HashMap<String, u64>) -> Result<String> {
        let mut wtr = Writer::from_writer(vec![]);
        for (key, value) in data {
            wtr.serialize((key, value))?;
        }
        wtr.flush()?;
        Ok(String::from_utf8(wtr.into_inner()?)?)
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
        let gas_data = Self::map_to_csv(&self.gas_counter.read().unwrap())?;
        let op_data = Self::map_to_csv(&self.op_counter.read().unwrap())?;
        let bucket = &ENV_VARS.dips_metrics_object_store_url;

        match bucket {
            Some(bucket) => {
                spawn(async move {
                    let gas_file = format!("{}/{}/gas.csv", subgraph_id, block_ptr.number);
                    let op_file = format!("{}/{}/op.csv", subgraph_id, block_ptr.number);

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
                });
            }
            None => return Err(anyhow!("Failed to parse gas metrics object store URL"))?,
        }

        // Clear the maps
        self.gas_counter.write().unwrap().clear();
        self.op_counter.write().unwrap().clear();

        Ok(())
    }

    pub fn mock() -> Arc<Self> {
        Arc::new(Self::new())
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
}
