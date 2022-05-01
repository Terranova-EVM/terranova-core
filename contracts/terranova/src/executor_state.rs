use std::{cell::RefCell, collections::{BTreeMap, BTreeSet}, mem};

use evm::{Transfer, H160, backend::{Log, Apply}, U256, H256, ExitError, Valids};
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

/// TODO: Document this\ 
/// Alright I started being lazy and pasting in code without understanding every internal implementation detail
/// and just making the changes needed to get signatures to match\ 
/// I'm hoping these functions will just work out of the box since my implementation of
/// StorageInterface has the same set of methods as AccountStorage in Neon and I put a good amount of consideration into it
impl ExecutorSubstate {
    #[allow(clippy::missing_const_for_fn)]
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

    /// Returns an immutable reference on executor metadata.
    #[must_use]
    pub const fn metadata(&self) -> &ExecutorMetadata {
        &self.metadata
    }

    /// Returns the mutable reference on executor metadata.
    pub fn metadata_mut(&mut self) -> &mut ExecutorMetadata {
        &mut self.metadata
    }

    /// TODO: Document this\ 
    /// Deconstructs the executor, returns state to be applied.
    /// # Panics
    /// Panics if the executor is not in the top-level substate.
    #[must_use]
    pub fn deconstruct<B: StorageInterface>(
        mut self,
        backend: &B,
    ) -> ApplyState {
        assert!(self.parent.is_none());

        let mut applies = Vec::<Apply<BTreeMap<U256, U256>>>::new();

        let mut addresses = BTreeSet::new();

        for address in self.accounts.keys() {
            addresses.insert(*address);
        }

        for (address, _) in self.storages.keys() {
            addresses.insert(*address);
        }

        for address in addresses {
            if self.deletes.contains(&address) {
                continue;
            }

            let mut storage = BTreeMap::new();
            for ((oa, ok), ov) in &self.storages {
                if *oa == address {
                    storage.insert(*ok, *ov);
                }
            }

            let apply = {
                let account = self.accounts.remove(&address).unwrap_or_else(
                    || ExecutorAccount {
                        nonce: backend.nonce(&address),
                        code: None,
                        valids: None,
                        reset: false,
                    }
                );

                Apply::Modify {
                    address,
                    nonce: account.nonce,
                    code_and_valids: account.code.zip(account.valids),
                    storage,
                    reset_storage: account.reset,
                }
            };

            applies.push(apply);
        }

        for address in self.deletes {
            applies.push(Apply::Delete { address });
        }

        // let erc20_allowances = self.erc20_allowances.take();
        // let mut erc20_approves = Vec::with_capacity(erc20_allowances.len());
        // for ((owner, spender, contract, mint), value) in erc20_allowances {
        //     let approve = ERC20Approve { owner, spender, contract, mint, value };
        //     erc20_approves.push(approve);
        // }

        (applies, self.logs, self.transfers) //, self.spl_transfers, self.spl_approves, self.withdrawals, erc20_approves)
    }

    /// Creates new instance of `ExecutorSubstate` when entering next execution of a call or create.
    pub fn enter(&mut self, is_static: bool) {
        let mut entering = Self {
            metadata: self.metadata.spit_child(is_static),
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
        };
        mem::swap(&mut entering, self);

        self.parent = Some(Box::new(entering));
    }

    /// Commits the state on exit of call or creation.
    /// # Panics
    /// Panics on incorrect exit sequence or if an address not found in known accounts.
    /// # Errors
    /// May return one of `ExitError` variants.
    pub fn exit_commit(&mut self) -> Result<(), ExitError> {
        let mut exited = *self.parent.take().expect("Cannot commit on root substate");
        mem::swap(&mut exited, self);

        self.metadata.swallow_commit(exited.metadata)?;
        self.logs.append(&mut exited.logs);
        self.balances.borrow_mut().append(&mut exited.balances.borrow_mut());
        self.transfers.append(&mut exited.transfers);

        // self.spl_balances.borrow_mut().append(&mut exited.spl_balances.borrow_mut());
        // self.spl_decimals.borrow_mut().append(&mut exited.spl_decimals.borrow_mut());
        // self.spl_supply.borrow_mut().append(&mut exited.spl_supply.borrow_mut());
        // self.spl_transfers.append(&mut exited.spl_transfers);
        // self.spl_approves.append(&mut exited.spl_approves);

        // self.withdrawals.append(&mut exited.withdrawals);

        // self.erc20_allowances.borrow_mut().append(&mut exited.erc20_allowances.borrow_mut());

        let mut resets = BTreeSet::new();
        for (address, account) in &exited.accounts {
            if account.reset {
                resets.insert(*address);
            }
        }
        let mut reset_keys = BTreeSet::new();
        for (address, key) in self.storages.keys() {
            if resets.contains(address) {
                reset_keys.insert((*address, *key));
            }
        }
        for (address, key) in reset_keys {
            self.storages.remove(&(address, key));
        }

        resets = BTreeSet::new();
        for (address, account) in &self.accounts {
            if account.reset {
                resets.insert(*address);
            }
        }
        self.accounts.append(&mut exited.accounts);
        self.storages.append(&mut exited.storages);
        self.deletes.append(&mut exited.deletes);

        for address in &resets {
            if self.accounts.contains_key(address){
                self.accounts.get_mut(address).unwrap().reset = true;
            }
        }

        Ok(())
    }

    /// Reverts the state on exit of call or creation.
    /// # Panics
    /// Panics on incorrect exit sequence.
    /// # Errors
    /// May return one of `ExitError` variants.
    pub fn exit_revert(&mut self) -> Result<(), ExitError> {
        let mut exited = *self.parent.take().expect("Cannot discard on root substate");
        mem::swap(&mut exited, self);

        self.metadata.swallow_revert(exited.metadata)?;

        Ok(())
    }

    /// Discards the state on exit of call or creation.
    /// # Panics
    /// Panics on incorrect exit sequence.
    /// # Errors
    /// May return one of `ExitError` variants.
    pub fn exit_discard(&mut self) -> Result<(), ExitError> {
        let mut exited = *self.parent.take().expect("Cannot discard on root substate");
        mem::swap(&mut exited, self);

        self.metadata.swallow_discard(exited.metadata)?;

        Ok(())
    }

    fn known_account(&self, address: H160) -> Option<&ExecutorAccount> {
        match self.accounts.get(&address) {
            Some(account) => Some(account),
            None => self.parent.as_ref().and_then(|parent| parent.known_account(address))
        }
    }

    /// Returns copy of basic account information if the `address` represents a known account.
    /// Returns `None` if the account is not known.
    #[must_use]
    pub fn known_nonce(&self, address: H160) -> Option<U256> {
        self.known_account(address).map(|acc| acc.nonce)
    }

    /// Returns copy of code stored in account if the `address` represents a known account.
    /// Returns `None` if the account is not known.
    #[must_use]
    pub fn known_code(&self, address: H160) -> Option<Vec<u8>> {
        self.known_account(address).and_then(|acc| acc.code.clone())
    }

    /// Returns copy of `valids` bit array stored in account if the `address` represents a known account.
    /// Returns `None` if the account is not known.
    #[must_use]
    pub fn known_valids(&self, address: H160) -> Option<Vec<u8>> {
        self.known_account(address).and_then(|acc| acc.valids.clone())
    }


    /// Checks if an account is empty: does not contain balance, nonce and code.
    /// Returns `None` if the account is not known.
    #[must_use]
    pub fn known_empty(&self, address: H160) -> Option<bool> {
        if let Some(balance) = self.known_balance(&address) {
            if balance != U256::zero() {
                return Some(false);
            }
        } else {
            return None;
        }

        if let Some(account) = self.known_account(address) {
            if account.nonce != U256::zero() {
                return Some(false);
            }

            if let Some(code) = &account.code {
                return Some(code.is_empty());
            }
        }

        None
    }

    /// Returns value of record stored in a account if the `address` represents a known account.
    /// Returns zero if the account is in reset state (empty storage).
    /// Returns `None` if a record with the key does not exist or the account is not known.
    #[must_use]
    pub fn known_storage(&self, address: H160, key: U256) -> Option<U256> {
        if let Some(value) = self.storages.get(&(address, key)) {
            return Some(*value);
        }

        if let Some(account) = self.accounts.get(&address) {
            if account.reset {
                return Some(U256::zero());
            }
        }

        if let Some(parent) = self.parent.as_ref() {
            return parent.known_storage(address, key);
        }

        None
    }

    /// Returns zero if the account is in reset state (empty storage).
    /// Returns `None` if the account is not in reset state or is not known.
    #[must_use]
    pub fn known_original_storage(&self, address: H160, key: U256) -> Option<U256> {
        if let Some(account) = self.accounts.get(&address) {
            if account.reset {
                return Some(U256::zero());
            }
        }

        if let Some(parent) = self.parent.as_ref() {
            return parent.known_original_storage(address, key);
        }

        None
    }

    /// Checks if an account has been deleted.
    #[must_use]
    pub fn deleted(&self, address: H160) -> bool {
        if self.deletes.contains(&address) {
            return true;
        }

        if let Some(parent) = self.parent.as_ref() {
            return parent.deleted(address);
        }

        false
    }

    #[must_use]
    fn account_mut<B: StorageInterface>(&mut self, address: H160, backend: &B) -> &mut ExecutorAccount {
        #[allow(clippy::map_entry)]
        if !self.accounts.contains_key(&address) {
            let account = self.known_account(address).cloned().map_or_else(
                || ExecutorAccount {
                    nonce: backend.nonce(&address),
                    code: None,
                    valids: None,
                    reset: false,
                },
                |mut v| {
                    v.reset = false;
                    v
                },
            );
            self.accounts.insert(address, account);
        }

        self.accounts
            .get_mut(&address)
            .expect("New account was just inserted")
    }

    /// Increments nonce of an account: increases it by 1.
    pub fn inc_nonce<B: StorageInterface>(&mut self, address: H160, backend: &B) {
        let account = self.account_mut(address, backend);

        let (nonce, _overflow) = account.nonce.overflowing_add(U256::one());
        account.nonce = nonce;
    }

    /// Adds or changes a record in the storage of given account.
    pub fn set_storage(&mut self, address: H160, key: U256, value: U256) {
        self.storages.insert((address, key), value);
    }

    /// Clears the storage of an account and marks the account as reset.
    pub fn reset_storage<B: StorageInterface>(&mut self, address: H160, backend: &B) {
        let mut removing = Vec::new();

        for (oa, ok) in self.storages.keys() {
            if *oa == address {
                removing.push(*ok);
            }
        }

        for ok in removing {
            self.storages.remove(&(address, ok));
        }

        self.account_mut(address, backend).reset = true;
    }

    /// Adds an Ethereum event log record.
    pub fn log(&mut self, address: H160, topics: Vec<H256>, data: Vec<u8>) {
        self.logs.push(Log {
            address,
            topics,
            data,
        });
    }

    /// Marks an account as deleted.
    pub fn set_deleted(&mut self, address: H160) {
        self.deletes.insert(address);
    }

    /// Initializes a contract account with it's code and corresponding bit array of valid jumps.
    pub fn set_code<B: StorageInterface>(&mut self, address: H160, code: Vec<u8>, backend: &B) {
        self.account_mut(address, backend).valids = Some(Valids::compute(&code));
        self.account_mut(address, backend).code = Some(code);
    }

    #[must_use]
    pub fn known_balance(&self, address: &H160) -> Option<U256> {
        let balances = self.balances.borrow();

        match balances.get(address) {
            Some(balance) => Some(*balance),
            None => self.parent.as_ref().and_then(|parent| parent.known_balance(address))
        }
    }

    #[must_use]
    pub fn balance<B: StorageInterface>(&self, address: &H160, backend: &B) -> U256 {
        let value = self.known_balance(address);

        value.map_or_else(
            || {
                let balance = backend.balance(address);
                self.balances.borrow_mut().insert(*address, balance);

                balance
            },
            |value| value
        )
    }

    /// Adds a transfer to execute.
    /// # Errors
    /// May return `OutOfFund` if the source has no funds.
    pub fn transfer<B: StorageInterface>(
        &mut self,
        transfer: &Transfer,
        backend: &B,
    ) -> Result<(), ExitError> {
        let new_source_balance = {
            let balance = self.balance(&transfer.source, backend);
            balance.checked_sub(transfer.value).ok_or(ExitError::OutOfFund)?
        };

        let new_target_balance = {
            let balance = self.balance(&transfer.target, backend);
            balance.checked_add(transfer.value).ok_or(ExitError::InvalidRange)?
        };

        let mut balances = self.balances.borrow_mut();
        balances.insert(transfer.source, new_source_balance);
        balances.insert(transfer.target, new_target_balance);

        self.transfers.push(*transfer);

        Ok(())
    }

    /// Resets the balance of an account: sets it to 0.
    pub fn reset_balance(&self, address: H160) {
        let mut balances = self.balances.borrow_mut();
        balances.insert(address, U256::zero());
    }

    /// Adds an account to list of known accounts if not yet added.
    pub fn touch<B: StorageInterface>(&mut self, address: H160, backend: &B) {
        let _unused = self.account_mut(address, backend);
    }
    /* 
    fn known_spl_balance(&self, address: &Pubkey) -> Option<u64> {
        let spl_balances = self.spl_balances.borrow();

        match spl_balances.get(address) {
            Some(balance) => Some(*balance),
            None => self.parent.as_ref().and_then(|parent| parent.known_spl_balance(address))
        }
    }

    #[must_use]
    pub fn spl_balance<B: StorageInterface>(&self, address: &Pubkey, backend: &B) -> u64 {
        let value = self.known_spl_balance(address);

        value.map_or_else(
            || {
                let balance = backend.get_spl_token_balance(address);
                self.spl_balances.borrow_mut().insert(*address, balance);

                balance
            },
            |value| value
        )
    }

    fn spl_transfer<B: StorageInterface>(&mut self, transfer: SplTransfer, backend: &B) -> Result<(), ExitError> {
        debug_print!("spl_transfer: {:?}", transfer);

        let new_source_balance = {
            let balance = self.spl_balance(&transfer.source_token, backend);
            balance.checked_sub(transfer.value).ok_or(ExitError::OutOfFund)?
        };

        let new_target_balance = {
            let balance = self.spl_balance(&transfer.target_token, backend);
            balance.checked_add(transfer.value).ok_or(ExitError::InvalidRange)?
        };

        let mut spl_balances = self.spl_balances.borrow_mut();
        spl_balances.insert(transfer.source_token, new_source_balance);
        spl_balances.insert(transfer.target_token, new_target_balance);

        self.spl_transfers.push(transfer);

        Ok(())
    }

    fn withdraw<B: StorageInterface>(&mut self, withdraw: Withdraw, backend: &B) -> Result<(), ExitError> {
        debug_print!("withdraw: {:?}", withdraw);

        let new_source_balance = {
            let balance = self.balance(&withdraw.source, backend);
            balance.checked_sub(withdraw.neon_amount).ok_or(ExitError::OutOfFund)?
        };

        let new_target_balance = {
            let balance = self.spl_balance(&withdraw.dest_neon, backend);
            balance.checked_add(withdraw.spl_amount).ok_or(ExitError::InvalidRange)?
        };

        let dest_neon = withdraw.dest_neon;

        let mut balances = self.balances.borrow_mut();
        balances.insert(withdraw.source, new_source_balance);
        self.withdrawals.push(withdraw);

        let mut spl_balances = self.spl_balances.borrow_mut();
        spl_balances.insert(dest_neon, new_target_balance);

        Ok(())
    }

    fn spl_approve(&mut self, approve: SplApprove) {
        self.spl_approves.push(approve);
    }

    fn known_spl_decimals(&self, address: &Pubkey) -> Option<u8> {
        let spl_decimals = self.spl_decimals.borrow();

        match spl_decimals.get(address) {
            Some(decimals) => Some(*decimals),
            None => self.parent.as_ref().and_then(|parent| parent.known_spl_decimals(address))
        }
    }

    #[must_use]
    pub fn spl_decimals<B: StorageInterface>(&self, address: &Pubkey, backend: &B) -> u8 {
        let value = self.known_spl_decimals(address);

        value.map_or_else(
            || {
                let decimals = backend.get_spl_token_decimals(address);
                self.spl_decimals.borrow_mut().insert(*address, decimals);

                decimals
            },
            |value| value
        )
    }

    fn known_spl_supply(&self, address: &Pubkey) -> Option<u64> {
        let spl_supply = self.spl_supply.borrow();

        match spl_supply.get(address) {
            Some(decimals) => Some(*decimals),
            None => self.parent.as_ref().and_then(|parent| parent.known_spl_supply(address))
        }
    }

    #[must_use]
    pub fn spl_supply<B: StorageInterface>(&self, address: &Pubkey, backend: &B) -> u64 {
        let value = self.known_spl_supply(address);

        value.map_or_else(
            || {
                let supply = backend.get_spl_token_supply(address);
                self.spl_supply.borrow_mut().insert(*address, supply);

                supply
            },
            |value| value
        )
    }

    fn known_erc20_allowance(&self, owner: H160, spender: H160, contract: H160, mint: Pubkey) -> Option<U256> {
        let erc20_allowances = self.erc20_allowances.borrow();
        match erc20_allowances.get(&(owner, spender, contract, mint)) {
            Some(&allowance) => Some(allowance),
            None => self.parent.as_ref().and_then(|parent| parent.known_erc20_allowance(owner, spender, contract, mint))
        }
    }

    #[must_use]
    pub fn erc20_allowance<B: StorageInterface>(&self, owner: H160, spender: H160, contract: H160, mint: Pubkey, backend: &B) -> U256 {
        let value = self.known_erc20_allowance(owner, spender, contract, mint);

        value.map_or_else(
            || {
                let allowance = backend.get_erc20_allowance(&owner, &spender, &contract, &mint);

                let key = (owner, spender, contract, mint);
                self.erc20_allowances.borrow_mut().insert(key, allowance);

                allowance
            },
            |value| value
        )
    }

    fn erc20_approve(&mut self, approve: &ERC20Approve) {
        let key = (approve.owner, approve.spender, approve.contract, approve.mint);
        self.erc20_allowances.borrow_mut().insert(key, approve.value);
    }
    */

    fn known_block_hash(&self, number: U256) -> Option<H256> {
        let block_hashes = self.block_hashes.borrow();
        block_hashes.get(&number).copied()
    }

    #[must_use]
    pub fn block_hash<B: StorageInterface>(&self, number: U256, backend: &B) -> H256 {
        let value = self.known_block_hash(number);

        value.map_or_else(
            || {
                let block_hash = backend.block_hash(number);
                self.block_hashes.borrow_mut().insert(number, block_hash);

                block_hash
            },
            |value| value
        )
    }
}

/// TODO: Document this
pub struct ExecutorState<'a, B: StorageInterface> {
    backend: &'a B,
    substate: Box<ExecutorSubstate>,
}
