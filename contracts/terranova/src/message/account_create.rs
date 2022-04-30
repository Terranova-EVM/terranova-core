use cosmwasm_std::{StdResult, DepsMut};

pub fn process(deps: DepsMut, operator: String, eth_address: [u8; 20], contract_byte_code: Vec<u8>) -> StdResult<()> {
    Ok(())
}