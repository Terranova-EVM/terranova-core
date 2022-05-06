use cosmwasm_std::{DepsMut, Env, Response, Storage};
use evm::{H160, H256};

use crate::ContractError;
use crate::tx_chunk;

pub fn process(deps: DepsMut, caller_address_bytes: [u8; 20], full_tx_hash: [u8; 32], chunk_index: u8, chunk_data: Vec<u8>) -> Result<Response, ContractError> {
    let caller_address = H160::from_slice(&caller_address_bytes);
    let tx_hash = H256::from_slice(&full_tx_hash);

    validate()?;

    execute(deps, caller_address, tx_hash, chunk_index, chunk_data)
}

pub fn validate() -> Result<(), ContractError> {
    Ok(())
}

pub fn execute(deps: DepsMut, caller_address: H160, tx_hash: H256, chunk_index: u8, chunk_data: Vec<u8>) -> Result<Response, ContractError> {
    tx_chunk::write_chunk(deps, caller_address, tx_hash, chunk_index, chunk_data)?;

    let response = Response::new()
        .add_attribute("chunk_index", chunk_index.to_string());

    Ok(response)
}
