use cosmwasm_std::{Deps, DepsMut, StdResult};
use cw_storage_plus::Map;
use evm::{H160, H256};

use crate::ContractError;

/// Key: (caller_address, tx_hash, chunk_index)
/// Value: The chunk data
/// The third key in the tuple should just be one u8, it has to be wrapped in a byte slice to function as a PrimaryKey
const CHUNK_STORAGE: Map<(&H160, &H256, &[u8]), Vec<u8>> = Map::new("chunk_storage");

/// If a chunk has already been stored at this address/hash/index tuple, it will be overwritten without warning
pub fn write_chunk(deps: DepsMut, caller_address: H160, tx_hash: H256, chunk_index: u8, chunk_data: Vec<u8>) -> StdResult<()> {
    CHUNK_STORAGE.save(
        deps.storage,
        (&caller_address, &tx_hash, &[chunk_index]),
        &chunk_data
    )
}

pub fn collect_chunks(deps: Deps, caller_address: H160, tx_hash: H256, chunk_count: u8) -> Result<Vec<u8>, ContractError> {
    let mut collected_bytes: Vec<u8> = Vec::new();

    for i in 0..chunk_count {
        let mut chunk = CHUNK_STORAGE.may_load(deps.storage, (&caller_address, &tx_hash, &[i]))?
            .ok_or(ContractError::TxChunksMissing)?;

        collected_bytes.append(&mut chunk);
    }

    Ok(collected_bytes)
}