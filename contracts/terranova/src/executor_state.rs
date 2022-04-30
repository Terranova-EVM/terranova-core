use std::{cell::RefCell, collections::{BTreeMap, BTreeSet}};

use evm::{Transfer, H160, backend::{Log, Apply}, U256, H256};
use serde::{Serialize, Deserialize};

use crate::storage::StorageInterface;

/// TODO: Document this
/// Each of these structs is tied to an EVM H160 address in the accounts field of ExecutorSubstate
#[derive(Clone, Debug, Serialize, Deserialize)]
struct ExecutorAccount {
    /// TODO: Document this
    pub nonce: U256,

    /// TODO: Document this
    #[serde(with = "serde_bytes")]
    pub code: Option<Vec<u8>>,

    /// TODO: Document this
    #[serde(with = "serde_bytes")]
    pub valids: Option<Vec<u8>>,

    /// TODO: Document this
    pub reset: bool,
}

/// TODO: Document this maybe a bit better
/// Represents additional data attached to an executor.
#[derive(Serialize, Deserialize)]
pub struct ExecutorMetadata {
    /// TODO: Document this
    is_static: bool,

    /// Stack depth (I think)
    depth: Option<usize>,

    block_number: U256,

    block_timestamp: U256,
}

/// TODO: Document this
impl ExecutorMetadata {

}

/// TODO: Document this better
/// Represents the state of executor abstracted away from a backend.
#[derive(Serialize, Deserialize)]
pub struct ExecutorSubstate {
    metadata: ExecutorMetadata,
    parent: Option<Box<ExecutorSubstate>>,
    logs: Vec<Log>,
    transfers: Vec<Transfer>,
    block_hashes: RefCell<BTreeMap<U256, H256>>,
    accounts: BTreeMap<H160, ExecutorAccount>,
    balances: RefCell<BTreeMap<H160, U256>>,
    storages: BTreeMap<(H160, U256), U256>,
    // spl_balances: RefCell<BTreeMap<Pubkey, u64>>,
    // spl_decimals: RefCell<BTreeMap<Pubkey, u8>>,
    // spl_supply: RefCell<BTreeMap<Pubkey, u64>>,
    // spl_transfers: Vec<SplTransfer>,
    // spl_approves: Vec<SplApprove>,
    // withdrawals: Vec<Withdraw>, // Withdraw from Neon to Solana
    // erc20_allowances: RefCell<BTreeMap<(H160, H160, H160, Pubkey), U256>>,
    deletes: BTreeSet<H160>,
    // query_account_cache: query::AccountCache,
}

/// TODO: Document this
pub type ApplyState = (Vec::<Apply<BTreeMap<U256, U256>>>, Vec<Log>, Vec<Transfer>); //, Vec<SplTransfer>, Vec<SplApprove>, Vec<Withdraw>, Vec<ERC20Approve>);

/// TODO: Document this
impl ExecutorSubstate {

}

/// TODO: Document this
pub struct ExecutorState<'a, B: StorageInterface> {
    backend: &'a B,
    substate: Box<ExecutorSubstate>,
}