use std::{cell::RefCell, collections::{BTreeSet, BTreeMap}};

use cosmwasm_std::{Addr, Env, DepsMut};

use crate::ContractError;

use super::{CwStorageInterface, Readable, Writable};

impl<S: Readable> CwStorageInterface<S> {
    pub fn new_ref(cw_deps: S, cw_env: Env, token_mint: Addr,  chain_id: u64) -> Result<Self, ContractError> {
        Ok(Self {
            cw_deps,
            cw_env,
            token_mint,
            evm_accounts: BTreeMap::new(),
            empty_evm_accounts: RefCell::new(BTreeSet::new()),
            chain_id,
        })
    }
}

impl<S: Readable + Writable> CwStorageInterface<S> {
    pub fn new_mut(cw_deps: S, cw_env: Env, token_mint: Addr,  chain_id: u64) -> Result<Self, ContractError> {
        Ok(Self {
            cw_deps,
            cw_env,
            token_mint,
            evm_accounts: BTreeMap::new(),
            empty_evm_accounts: RefCell::new(BTreeSet::new()),
            chain_id,
        })
    }
}
