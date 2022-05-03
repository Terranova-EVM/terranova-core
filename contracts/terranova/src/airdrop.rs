use cosmwasm_std::{DepsMut, Env};
use evm::H160;

use crate::storage::CwStorageInterface;
use crate::config;

/// Set the native balance of the given addr to 100_000
pub fn airdrop_write_balance(deps: DepsMut, env: Env, addr: H160) {
    let mut backend = CwStorageInterface::new(deps, env, config::token_mint_dummy(), config::chain_id_dummy()).unwrap();
    backend.airdrop_write_balance(&addr);
}
