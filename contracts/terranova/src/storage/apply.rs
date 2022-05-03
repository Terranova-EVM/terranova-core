use std::{collections::BTreeMap, convert::TryInto};

use cosmwasm_std::{StdError, Order};
use evm::{backend::Apply, U256, H160, Transfer};

use crate::{storage::{CwStorageInterface}, executor_state::ApplyState, ContractError, account::{EvmAccount, EvmContract}};

use super::{backend::{ACCOUNTS, CONTRACTS, CONTRACT_STORAGE}, StorageInterface};

/// Write operations on the backend EVM state
/// Methods to apply the results of a completed transaction to persistent EVM state
impl<'a> CwStorageInterface<'a> {
    pub fn apply_state_change(
        &mut self,
        state: ApplyState,
    ) -> Result<(), ContractError> {
        let (
            applies,
            logs,
            transfers,
        ) = state;

        debug_print!("Applies begin");

        if !transfers.is_empty() {
            self.apply_transfers(transfers)?;
        }

        if !applies.is_empty() {
            self.apply_contract_results(applies)?;
        }

        debug_print!("Applies done");

        for log in logs {
            // Do nothing for now
        }

        Ok(())
    }

    /// Public method to increment account nonce
    pub fn increment_nonce(&mut self, address: &H160) -> Result<(), ContractError> {
        if !ACCOUNTS.has(self.cw_deps.storage, address) {
            self.init_new_account(address)?;
        }

        let mut account = ACCOUNTS.may_load(self.cw_deps.storage, address)?
            .ok_or(StdError::NotFound { kind: "EvmAccount".to_string() })?;

        account.trx_count = account.trx_count.checked_add(1)
            .ok_or_else(|| E!(ContractError::NonceOverflow; "Account {} - nonce overflow", address))?;

        ACCOUNTS.save(self.cw_deps.storage, address, &account)?;

        Ok(())
    }

    pub fn airdrop_write_balance(&mut self, address: &H160) {
        println!("Setting balance of {} to 100,000,000", address);
        if !ACCOUNTS.has(self.cw_deps.storage, address) {
            self.init_new_account(address).unwrap();
        }

        self.write_balance(address, U256::from(100_000_000)).unwrap();
    }

    /// This takes the actual raw contract bytecode that should be written, NOT the contract initialization bytecode in a contract create messaage
    pub fn airdrop_deploy_contract(&mut self, address: &H160, code: Vec<u8>) {
        println!("Deploying a contract to {}", address);
        let valids = evm::Valids::compute(&code);

        self.update_contract_account(*address, U256::one(), Some((code, valids)), BTreeMap::new(), false).unwrap();
    }

    /// This could be either a user or contract account, however the same initialization state will be set for both 
    /// (with contract_storage_key set to None). Additional logic should be implemented after this to initialize
    /// the contract code and update this account's contract_storage_key field if the account is a contract account.
    fn init_new_account(&mut self, address: &H160) -> Result<(), ContractError> {
        // If account already exists in the EVM state, do nothing
        if !ACCOUNTS.has(self.cw_deps.storage, address) {
            ACCOUNTS.save(
                self.cw_deps.storage,
                address,
                &EvmAccount::new_user_account(address)
            )?;
        }

        Ok(())
    }

    /// TODO: 
    fn delete_account(&mut self, address: &H160) -> Result<(), ContractError> {
        // Accounts can only be deleted by calling suicide() in contract code
        assert!(CONTRACTS.has(self.cw_deps.storage, address));

        ACCOUNTS.remove(self.cw_deps.storage, address);
        CONTRACTS.remove(self.cw_deps.storage, address);

        Ok(())
    }

    /// This should be called if an account with this address has been confirmed to exist in the ACCOUNTS map\ 
    /// Will return an error otherwise
    fn write_balance(&mut self, addr: &H160, new_balance: U256) -> Result<(), ContractError> {
        ACCOUNTS.update(
            self.cw_deps.storage, 
            addr, 
            |maybe_account| {
                if let Some(mut account) = maybe_account {
                    account.balance = new_balance;
                    Ok(account)
                } else {
                    Err(StdError::NotFound { kind: "EvmAccount".to_string() })
                }
            }
        )?;

        Ok(())
    }

    /// This should be called if an account with this address has been confirmed to exist in the ACCOUNTS map\ 
    /// Will return an error otherwise
    fn write_nonce(&mut self, address: &H160, new_nonce: u64) -> Result<(), ContractError> {
        ACCOUNTS.update(
            self.cw_deps.storage, 
            address, 
            |maybe_account| {
                if let Some(mut account) = maybe_account {
                    account.trx_count = new_nonce;
                    Ok(account)
                } else {
                    Err(StdError::NotFound { kind: "EvmAccount".to_string() })
                }
            }
        )?;

        Ok(())
    }

    fn write_deploy_contract(&mut self, address: &H160, code: Vec<u8>, valids: Vec<u8>) -> Result<(), ContractError> {
        // Redundant here, can remove if needed for performance
        if !ACCOUNTS.has(self.cw_deps.storage, address) {
            self.init_new_account(address)?;
        } 

        // Signify that the account is a contract account by setting the contract_storage_key field
        ACCOUNTS.update(
            self.cw_deps.storage,
            address,
            |maybe_account| {
                if let Some(mut account) = maybe_account {
                    account.contract_storage_key = Some(*address);
                    Ok(account)
                } else {
                    Err(StdError::NotFound { kind: "EvmAccount".to_string() })
                }
            }
        )?;

        // Save code and valids to CONTRACT entry, initialize new storage
        CONTRACTS.save(
            self.cw_deps.storage,
            address,
            &EvmContract {
                code_size: code.len().try_into().expect("code.len() never exceeds u32::max"),
                code,
                valids,
            }
        )?;

        Ok(())
    }

    fn write_storage(&mut self, address: &H160, key: U256, value: U256) -> Result<(), ContractError> {
        // Not an entirely necessary check, just for sanity. Remove if needed for performance
        if !CONTRACTS.has(self.cw_deps.storage, address) {
            return Err(StdError::NotFound { kind: "EvmContract".to_string() }.into())
        }

        CONTRACT_STORAGE.save(
            self.cw_deps.storage,
            (address, &key.to_bytes()),
            &value
        )?;

        Ok(())  
    }

    fn write_clear_storage(&mut self, address: &H160) -> Result<(), ContractError> {
        if !CONTRACTS.has(self.cw_deps.storage, address) {
            return Err(StdError::NotFound { kind: "EvmContract".to_string() }.into())
        }

        let storage_keys: Vec<Result<(Vec<u8>, U256), StdError>> = CONTRACT_STORAGE
            .prefix(address)
            .range(self.cw_deps.storage, None, None, Order::Ascending)
            .collect();

        for entry in storage_keys {
            let entry = entry?;

            CONTRACT_STORAGE.remove(
                self.cw_deps.storage,
                (address, &entry.0)
            )
        }

        Ok(()) 
    }

    fn apply_transfers(&mut self, transfers: Vec<Transfer>) -> Result<(), ContractError> {
        debug_print!("apply_transfers: {:?}", transfers);

        for transfer in transfers {
            self.transfer_nova_tokens(transfer.source, transfer.target, transfer.value)?;
        }

        Ok(())
    }
    
    fn transfer_nova_tokens(&mut self, source: H160, target: H160, value: U256) -> Result<(), ContractError> {
        // This check is redundant since the same check happens inside of init_new_account
        // However we may want to replace init_new_account with a different implementation, so keep it here
        if !ACCOUNTS.has(self.cw_deps.storage, &source) {
            self.init_new_account(&source)?;
        }

        if !ACCOUNTS.has(self.cw_deps.storage, &source) {
            self.init_new_account(&source)?;
        }

        let source_balance = self.balance(&source).checked_sub(value)
            .ok_or_else(|| E!(ContractError::InsufficientFunds; "Account {} - insufficient funds, balance = {}", source, self.balance(&source)))?;
        let target_balance = self.balance(&target).checked_add(value)
            .ok_or_else(|| E!(ContractError::BalanceOverflow; "Account {} - balance overflow", target))?;

        self.write_balance(&source, source_balance)?;
        self.write_balance(&target, target_balance)?;
    
        Ok(())
    }

    fn apply_contract_results(
        &mut self,
        values: Vec<Apply<BTreeMap<U256, U256>>>,
    ) -> Result<(), ContractError> {
        debug_print!("Apply contract results");

        for apply in values {
            match apply {
                Apply::Modify {address, nonce, code_and_valids, storage, reset_storage} => {
                    // if is_precompile_address(&address) {
                    //     continue;
                    // }

                    self.update_contract_account(address, nonce, code_and_valids, storage, reset_storage)?;
                },
                Apply::Delete { address } => {
                    self.delete_account(&address)?;
                }
            }
        }

        Ok(())
    }

    fn update_contract_account(
        &mut self,
        address: H160,
        trx_count: U256,
        code_and_valids: Option<(Vec<u8>, Vec<u8>)>,
        storage: BTreeMap<U256, U256>,
        reset_storage: bool
    ) -> Result<(), ContractError> {
        if !ACCOUNTS.has(self.cw_deps.storage, &address) {
            self.init_new_account(&address)?;
        }

        if self.nonce(&address) != trx_count {
            assert!(trx_count > self.nonce(&address));

            if trx_count > U256::from(u64::MAX) {
                return Err!(ContractError::NonceOverflow; "Account {} - nonce overflow", address);
            }

            self.write_nonce(&address, trx_count.as_u64())?;
        }

        if let Some((code, valids)) = code_and_valids {
            self.write_deploy_contract(&address, code, valids)?;
        }

        if reset_storage | storage.is_empty() {
            if reset_storage {
                self.write_clear_storage(&address)?;
            }

            for (key, value) in storage {
                self.write_storage(&address, key, value)?;
            }
        }

        Ok(())
    }
}
