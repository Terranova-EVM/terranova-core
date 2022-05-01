use std::{cell::RefMut, collections::BTreeMap};

use cosmwasm_std::Uint256;
use evm::{H160, U256};
use serde::{Serialize, Deserialize};

/// TODO: Document this
#[derive(Debug, Serialize, Deserialize)]
pub struct EvmAccount {
    /// Ethereum address
    pub address: H160,

    /// Solana/(Cosmwasm?) account nonce
    /// TODO: This is copied from Neon. Do we need this? (I believe not)
    pub bump_seed: u8,

    /// Ethereum account nonce
    pub trx_count: u64,

    /// TODO: Document this, explain why a contract account's H160 address can be used in 
    /// our Cosmwasm implementation of EVM as the key to its storage, rather than how it's done in Ethereum
    pub contract_storage_key: Option<H160>,

    /// EVM native balance 
    pub balance: U256,

    /// Read-write lock
    pub rw_blocked: bool,

    /// Read-only lock counter
    pub ro_blocked_count: u8,
}

/// TODO: Document this, THE TYPE CHOICES FOR THIS ARE ALL REALLY LAZY, COME BACK TO THIS
/// For every EvmAccount struct that is a contract account, there will be a corresponding EvmContract
/// stored in the CONTRACTS map, with the same key, the account's H160 address, used. The H160 address is stored
/// as Some(H160_address) in the contract_storage_key field of EvmAccount to make this relationship clear.
#[derive(Debug, Serialize, Deserialize)]
pub struct EvmContract {
    /// Contract code size
    pub code_size: u32,

    /// TODO: Neon uses RefMut for this, as in the below line - why? \ 
    /// pub code: RefMut<'a, [u8]>,
    pub code: Vec<u8>,
    
    
    /// TODO: Neon uses RefMut for this, as in the below line - why? \ 
    /// pub valids: RefMut<'a, [u8]>,
    pub valids: Vec<u8>,

    /// TODO: Think about the choice of data structure here \ 
    /// Neon has their own custom implementation of a HAMT for this:
    /// Try to understand why they did this - why was it necessary? (specific to Solana? or no...)
    /// For this proof-of-concept, I'm going to use this and just see if it works
    pub storage: BTreeMap<U256, U256>
}