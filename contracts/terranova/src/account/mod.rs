use cosmwasm_std::{Uint256};
use cw_storage_plus::PrimaryKey;
use evm::H160;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

//#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EthereumAccount {
    /// Ethereum address
    pub address: H160,

    /// Solana/(Cosmwasm?) account nonce
    /// TODO: This is copied from Neon. Do we need this? (I believe not)
    pub bump_seed: u8,

    /// Ethereum account nonce
    pub trx_count: u64,

    /// Bytecode of contract account, [] if user account
    /// TODO: Is this the right type to use?
    pub contract_code: Vec<u8>,

    /// ETH token account
    pub balance: Uint256,

    /// Read-write lock
    pub rw_blocked: bool,

    /// Read-only lock counter
    pub ro_blocked_count: u8,
}
