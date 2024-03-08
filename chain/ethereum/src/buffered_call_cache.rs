use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use graph::{
    cheap_clone::CheapClone,
    components::store::EthereumCallCache,
    data::store::ethereum::call,
    prelude::{BlockPtr, CachedEthereumCall},
    slog::{error, Logger},
};

/// A wrapper around an Ethereum call cache that buffers call results in
/// memory for the duration of a block. If `get_call` or `set_call` are
/// called with a different block pointer than the one used in the previous
/// call, the buffer is cleared.
pub struct BufferedCallCache {
    call_cache: Arc<dyn EthereumCallCache>,
    buffer: Arc<Mutex<HashMap<call::Request, call::Retval>>>,
    block: Arc<Mutex<Option<BlockPtr>>>,
}

impl BufferedCallCache {
    pub fn new(call_cache: Arc<dyn EthereumCallCache>) -> Self {
        Self {
            call_cache,
            buffer: Arc::new(Mutex::new(HashMap::new())),
            block: Arc::new(Mutex::new(None)),
        }
    }

    fn check_block(&self, block: &BlockPtr) {
        let mut self_block = self.block.lock().unwrap();
        if self_block.as_ref() != Some(block) {
            *self_block = Some(block.clone());
            self.buffer.lock().unwrap().clear();
        }
    }
}

impl EthereumCallCache for BufferedCallCache {
    fn get_call(
        &self,
        call: &call::Request,
        block: BlockPtr,
    ) -> Result<Option<(call::Retval, call::Source)>, graph::prelude::Error> {
        self.check_block(&block);

        {
            let buffer = self.buffer.lock().unwrap();
            if let Some(result) = buffer.get(&call) {
                return Ok(Some((result.clone(), call::Source::Memory)));
            }
        }

        let result = self.call_cache.get_call(&call, block)?;

        let mut buffer = self.buffer.lock().unwrap();
        if let Some((value, _)) = &result {
            buffer.insert(call.cheap_clone(), value.clone());
        }
        Ok(result)
    }

    fn get_calls_in_block(
        &self,
        block: BlockPtr,
    ) -> Result<Vec<CachedEthereumCall>, graph::prelude::Error> {
        self.call_cache.get_calls_in_block(block)
    }

    fn set_call(
        &self,
        logger: &Logger,
        call: call::Request,
        block: BlockPtr,
        return_value: call::Retval,
    ) -> Result<(), graph::prelude::Error> {
        self.check_block(&block);

        // Enter the call into the in-memory cache immediately so that
        // handlers will find it, but add it to the underlying cache in the
        // background so we do not have to wait for that as it will be a
        // cache backed by the database
        {
            let mut buffer = self.buffer.lock().unwrap();
            buffer.insert(call.cheap_clone(), return_value.clone());
        }

        let cache = self.call_cache.cheap_clone();
        let logger = logger.cheap_clone();
        let _ = graph::spawn_blocking_allow_panic(move || {
            cache
                .set_call(&logger, call.cheap_clone(), block, return_value)
                .map_err(|e| {
                    error!(logger, "BufferedCallCache: call cache set error";
                            "contract_address" => format!("{:?}", call.address),
                            "error" => e.to_string())
                })
        });

        Ok(())
    }
}
