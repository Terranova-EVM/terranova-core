use std::{cell::RefCell, collections::{BTreeSet, BTreeMap}};

use cosmwasm_std::{Addr, Env, DepsMut};

use crate::ContractError;

use super::CwStorageInterface;

impl<'a> CwStorageInterface<'a> {
    pub fn new(cw_deps: DepsMut<'a>, cw_env: Env, token_mint: Addr,  chain_id: u64) -> Result<Self, ContractError> {
        Ok(Self {
            cw_deps,
            cw_env,
            token_mint,
            ethereum_accounts: BTreeMap::new(),
            empty_ethereum_accounts: RefCell::new(BTreeSet::new()),
            chain_id,
        })
    }
}