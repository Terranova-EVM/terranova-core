use cosmwasm_std::{DepsMut, Env, Response, Storage};
use evm::{H160, H256};

use crate::ContractError;

pub fn process(deps: DepsMut, caller_address_bytes: [u8; 20], full_tx_hash: [u8; 32], chunk_index: u8, chunk_data: Vec<u8>) -> Result<Response, ContractError> {
    todo!();

    let caller_address = H160::from_slice(&caller_address_bytes);
    let tx_hash = H256::from_slice(&full_tx_hash);

    validate()?;

    execute(deps.storage, caller_address, tx_hash, chunk_index, chunk_data)
}

pub fn validate() -> Result<(), ContractError> {
    Ok(())
}

pub fn execute(storage: &mut dyn Storage, caller_address: H160, tx_hash: H256, chunk_index: u8, chunk_data: Vec<u8>) -> Result<Response, ContractError> {
    let response = Response::new();

    Ok(response)
}
