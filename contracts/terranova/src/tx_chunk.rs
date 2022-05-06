use cw_storage_plus::Map;
use evm::{H160, H256};

const CHUNK_STORAGE: Map<(H160, H256, u8), Vec<u8>> = Map::new("chunk_storage");
