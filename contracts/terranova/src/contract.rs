#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint256};
use cw2::set_contract_version;
use evm::H160;

use crate::airdrop::airdrop_write_balance;
use crate::error::ContractError;
use crate::message::{execute_simple_transaction, store_transaction_chunk, execute_chunked_transaction, raw_ethereum_query, EvmAccountResponse};
use crate::message::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::storage::backend::ACCOUNTS;
use crate::utils::{parse_h160, parse_hex};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:terranova";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let addr: H160 = parse_h160("0xB34e2213751c5d8e9a31355fcA6F1B4FA5bB6bE1");

    airdrop_write_balance(deps, env, addr);

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
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::RawEthereumQuery { caller_evm_address, unsigned_tx } => {
            to_binary(
                &raw_ethereum_query::process(deps, env, caller_evm_address, unsigned_tx)?
            ).map_err(|e| e.into())
        }
        QueryMsg::QueryEvmAccount { evm_address } => {
            to_binary(
                &query_account(deps, evm_address)?
            ).map_err(|e| e.into())
        }
        _ => Ok(to_binary(&0_i32)?)
    }
}

fn query_account(deps: Deps, address_bytes: [u8; 20]) -> Result<EvmAccountResponse, ContractError> {
    ACCOUNTS.load(
        deps.storage,
        &H160::from_slice(&address_bytes)
    ).map(|acc| EvmAccountResponse {
        balance: Uint256::from_be_bytes(acc.balance.to_bytes()),
        nonce: acc.trx_count
    }).map_err(|e| e.into())
}
