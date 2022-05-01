mod backend;

use std::{collections::{BTreeMap, BTreeSet}, cell::RefCell};

use cosmwasm_std::{Addr, Env, DepsMut};
use evm::{H160, U256, H256};

use crate::account::{EvmAccount, EvmContract};

/// TODO: Figure out why EthereumAccount (EvmAccount) in Neon has a lifetime specifier
/// 
/// A thin enum wrapper for Ethereum 
enum Account {
    User(EvmAccount),
    Contract(EvmAccount, EvmContract),
}

/// Access to the persistent state of the EVM stored between transactions -
/// this is also referred to as "ephemeral state" (kinda confusing ik).\ 
/// This includes the info (nonce, balance, etc.) of all existing EVM accounts, as well as each contract's storage.
/// 
/// This is used to update the state actually stored in Cosmwasm storage, only *after* the successful completion of an EVM transaction.
/// 
/// State changes *during* a transaction are applied to ExecutorSubstates, which hold all of the updates to accounts incurred by the transaction.
/// At the successful completion of a transaction, the Executor takes the produced ExecutorState, which holds the last produced ExecutorSubstate
///  are then applied to the actual stored EVM state through this interface.
pub struct CwStorageInterface<'a> {
    /// The NOVA cw20 token mint address
    token_mint: Addr,

    /// Cosmwasm environment\ 
    /// Gives access to block info, as well as chainid (can we just use this for chainid needed by StorageInterface?)\ 
    /// Maybe the cw env chain_id should be separate from Terranova EVM chain ID,
    cw_env: Env,
    cw_deps: DepsMut<'a>,

    ///
    /// In Neon, Account is an enum, either account(EvmAccount), contract(ethaccount, ethcontract)
    ethereum_accounts: BTreeMap<H160, Account>,
    empty_ethereum_accounts: RefCell<BTreeSet<H160>>,

    chain_id: u64, 
}

/// TODO: Document this better
/// Equivalent to the AccountStorage trait in Neon
/// A generic interface to a backend to provide to an ExecutorState, giving the EVM access 
/// to storage (related to SSTORE, SLOAD opcodes) and info about the current block
pub trait StorageInterface {
    /// Get NOVA's cw20 token mint
    fn token_mint(&self) -> &Addr;

    /// Get block number of EVM state
    fn block_number(&self) -> U256;
    /// Get block timestamp of EVM state
    fn block_timestamp(&self) -> U256;
    /// Get block hash of EVM state
    fn block_hash(&self, number: U256) -> H256;
    /// Get chain id of EVM
    fn chain_id(&self) -> u64;
    
    /// Check if EVM account exists
    fn exists(&self, address: &H160) -> bool;
    /// Get EVM account nonce
    fn nonce(&self, address: &H160) -> U256;
    /// Get EVM account balance
    fn balance(&self, address: &H160) -> U256;

    /// Get code size of EVM contract account
    fn code_size(&self, address: &H160) -> usize;
    /// Get code hash of EVM contract account
    fn code_hash(&self, address: &H160) -> H256;
    /// Get code data of EVM contract account
    fn code(&self, address: &H160) -> Vec<u8>;
    /// Get valids data - valid jumps associated with the code for an EVM contract account
    fn valids(&self, address: &H160) -> Vec<u8>;
    /// Get data from EVM storage
    fn storage(&self, address: &H160, index: &U256) -> U256;
}
