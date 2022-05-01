use std::{cell::RefCell, collections::{BTreeMap, BTreeSet}};

use evm::{Transfer, H160, backend::{Log, Apply}, U256, H256, ExitError};
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
    /// TODO: Document this\ 
    /// Neon description: "Creates new empty metadata with specified gas limit."
    /// <-- wtf I don't see anything that has to do with gas limit???
    #[must_use]
    pub fn new<B: StorageInterface>(backend: &B) -> Self {
        Self {
            is_static: false,
            depth: None,
            block_number: backend.block_number(),
            block_timestamp: backend.block_timestamp()
        }
    }

    #[allow(clippy::needless_pass_by_value, clippy::unused_self)]
    pub fn swallow_commit(&mut self, _other: Self) -> Result<(), ExitError> {
    	// The following fragment deleted in the mainstream code:
        // if let Some(runtime) = self.runtime.borrow_mut().as_ref() {
        //     let return_value = other.borrow().runtime().unwrap().machine().return_value();
        //     runtime.set_return_data(return_value);
        // }

        Ok(())
    }

    #[allow(clippy::needless_pass_by_value, clippy::unused_self)]
    pub fn swallow_revert(&mut self, _other: Self) -> Result<(), ExitError> {
        Ok(())
    }

    /// Records gas usage on discard (actually does nothing).
    #[allow(clippy::needless_pass_by_value, clippy::unused_self, clippy::unnecessary_wraps)]
    pub fn swallow_discard(&mut self, _other: Self) -> Result<(), ExitError> {
        Ok(())
    }

    /// Creates new instance of metadata when entering next frame of execution.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn spit_child(&self, is_static: bool) -> Self {
        Self {
            is_static: is_static || self.is_static,
            depth: match self.depth {
                None => Some(0),
                Some(n) => Some(n + 1),
            },
            block_number: self.block_number,
            block_timestamp: self.block_timestamp,
        }
    }

    #[must_use]
    pub const fn is_static(&self) -> bool {
        self.is_static
    }

    /// Returns current depth of frame of execution.
    #[must_use]
    pub const fn depth(&self) -> Option<usize> {
        self.depth
    }

    #[must_use]
    pub const fn block_number(&self) -> &U256 {
        &self.block_number
    }

    #[must_use]
    pub const fn block_timestamp(&self) -> &U256 {
        &self.block_timestamp
    }
}

/// TODO: Document this better
/// Represents the state of an executor abstracted away from a backend.
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
    #[must_use]
    pub fn new<B: StorageInterface>(backend: &B) -> Self {
        Self {
            metadata: ExecutorMetadata::new(backend),
            parent: None,
            logs: Vec::new(),
            transfers: Vec::new(),
            block_hashes: RefCell::new(BTreeMap::new()),
            accounts: BTreeMap::new(),
            balances: RefCell::new(BTreeMap::new()),
            storages: BTreeMap::new(),
            // spl_balances: RefCell::new(BTreeMap::new()),
            // spl_decimals: RefCell::new(BTreeMap::new()),
            // spl_supply: RefCell::new(BTreeMap::new()),
            // spl_transfers: Vec::new(),
            // spl_approves: Vec::new(),
            // withdrawals: Vec::new(),
            // erc20_allowances: RefCell::new(BTreeMap::new()),
            deletes: BTreeSet::new(),
            // query_account_cache: query::AccountCache::new(),
        }
    }
}

/// TODO: Document this
pub struct ExecutorState<'a, B: StorageInterface> {
    backend: &'a B,
    substate: Box<ExecutorSubstate>,
}
