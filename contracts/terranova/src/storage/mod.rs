// TODO: Make backend private again, it's only pub rn for testing shit
pub mod backend;
mod base;
mod apply;

use std::{collections::{BTreeMap, BTreeSet}, cell::RefCell};

use cosmwasm_std::{Addr, Env, DepsMut, Storage, Deps};
use evm::{H160, U256, H256};

use crate::account::{EvmAccount, EvmContract};

/// Currently unused\ 
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
/// whose results are then applied to the actual stored EVM state through this interface.
pub struct CwStorageInterface<S: Readable> {
    /// Gives write access to Cosmasm storage
    cw_deps: S,

    /// Cosmwasm environment\ 
    /// Gives access to block info, as well as chainid (can we just use this for chainid needed by StorageInterface?)\ 
    /// Maybe the cw env chain_id should be separate from Terranova EVM chain ID,
    cw_env: Env,

    /// The NOVA cw20 token mint address
    token_mint: Addr,

    /// Currently unused\ 
    /// Account is an enum, either account(EvmAccount), contract(ethaccount, ethcontract)
    /// EVM accounts that have been touched so far during the course of a transaction
    /// basically this is just a cache for us.
    evm_accounts: BTreeMap<H160, Account>,

    /// Currently unused\ 
    /// Is there a point to this...?
    empty_evm_accounts: RefCell<BTreeSet<H160>>,

    chain_id: u64, 
}

/// TODO: Document this better
/// A generic interface to a backend to provide to an ExecutorState, giving the EVM access 
/// to read/writes on the persistent state and info about the current block
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

pub trait Readable {
    fn get_ref(&self) -> &dyn Storage;
}

pub trait Writable {
    fn get_mut(&mut self) -> &mut dyn Storage;
}

impl<'a> Readable for DepsMut<'a> {
    fn get_ref(&self) -> &dyn Storage {
        self.storage
    }
}

impl<'a> Writable for DepsMut<'a> {
    fn get_mut(&mut self) -> &mut dyn Storage {
        self.storage
    }
}

impl<'a> Readable for Deps<'a> {
    fn get_ref(&self) -> &dyn Storage {
        self.storage
    }
}
