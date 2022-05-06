use cosmwasm_std::{DepsMut, Env, Response, Storage};
use evm::{H160, H256};

use crate::ContractError;

pub fn process(deps: DepsMut, env: Env, caller_address_bytes: [u8; 20], full_tx_hash: [u8; 32], chunk_count: u8) -> Result<Response, ContractError> {
    todo!();
    
    let caller_address_h160 = H160::from_slice(&caller_address_bytes);
    let tx_hash = H256::from_slice(&full_tx_hash);

    validate()?;

    execute(deps.storage, caller_address_h160, tx_hash, chunk_count)
}

pub fn validate() -> Result<(), ContractError> {
    Ok(())
}

pub fn execute(storage: &mut dyn Storage, caller_address: H160, tx_hash: H256, chunk_count: u8) -> Result<Response, ContractError> {
    let response = Response::new();

    Ok(response)
}
