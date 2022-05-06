use cosmwasm_std::{DepsMut, Env, Response, Storage};
use evm::{H160, H256};

use crate::ContractError;
use crate::tx_chunk;
use crate::utils::keccak256_h256;

use super::execute_simple_transaction;

pub fn process(deps: DepsMut, env: Env, caller_address_bytes: [u8; 20], full_tx_hash: [u8; 32], chunk_count: u8) -> Result<Response, ContractError> {
    let caller_address = H160::from_slice(&caller_address_bytes);
    let full_tx_hash = H256::from_slice(&full_tx_hash);

    let collected_tx_bytes = tx_chunk::collect_chunks(deps.as_ref(), caller_address, full_tx_hash, chunk_count)?;
    
    // Check that the hash of the merged transaction chunks match the provided transaction hash
    if full_tx_hash != keccak256_h256(&collected_tx_bytes) {
        return Err(ContractError::InvalidTxChunks)
    }

    // Validation and execution will be handled in execute_simple_transaction
    execute_simple_transaction::process(deps, env, caller_address_bytes, collected_tx_bytes)
}
