#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint256};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::message::{execute_simple_transaction, store_transaction_chunk, execute_chunked_transaction};
use crate::message::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::utils::{parse_h160, parse_hex};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:terranova";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // Only trusted operators can make calls
    // TODO: First check that the message sender is an operator

    match msg {
        ExecuteMsg::ExecuteRawEthereumTx { caller_evm_address, unsigned_tx } => {
            execute_simple_transaction::process(deps, env, caller_evm_address, unsigned_tx)
        }
        ExecuteMsg::StoreTxChunk { caller_evm_address, full_tx_hash, chunk_index, chunk_data } => {
            store_transaction_chunk::process(deps, caller_evm_address, full_tx_hash, chunk_index, chunk_data)
        }
        ExecuteMsg::ExecuteChunkedEthereumTx { caller_evm_address, full_tx_hash, chunk_count } => {
            execute_chunked_transaction::process(deps, env, caller_evm_address, full_tx_hash, chunk_count)
        }
        _ => panic!("Not implemented")
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        _ => Ok(to_binary(&0_i32)?)
    }
}
