use cosmwasm_std::{Addr, DepsMut, Env, Response};
use evm::H160;

use crate::{
    transaction::UnsignedTransaction, 
    storage::{CwStorageInterface}, 
    config::{token_mint_dummy, chain_id_dummy},
    ContractError, 
    executor::Machine
};

pub fn process(deps: DepsMut, env: Env, caller_address_bytes: [u8; 20], unsigned_tx: Vec<u8>) -> Result<Response, ContractError> {
    let caller_address = H160::from_slice(&caller_address_bytes);
    let trx = UnsignedTransaction::from_rlp(&unsigned_tx)?;

    let storage = CwStorageInterface::new_mut(
        deps, 
        env, 
        token_mint_dummy(), 
        chain_id_dummy()
    )?;
    validate()?;

    execute(storage, caller_address, trx)
}

/// Implement this later
/// Contract validation, as well as validating EVM transaction signature
pub fn validate() -> Result<(), ContractError> {
    Ok(())
}

pub fn execute(mut storage: CwStorageInterface<DepsMut>, caller_address: H160, trx: UnsignedTransaction) -> Result<Response, ContractError> {
    let (exit_reason, return_value, apply_state, used_gas, response) = {
        let mut executor = Machine::new(caller_address, &storage)?;
        executor.gasometer_mut().record_transaction_size(&trx);

        let response: Response = match trx.to {
            Some(code_address) => {
                executor.call_begin(
                    caller_address, 
                    code_address,
                    trx.call_data,
                    trx.value, 
                    trx.gas_limit
                )?
            },
            None => {
                executor.create_begin(
                    caller_address,
                    trx.call_data,
                    trx.value,
                    trx.gas_limit
                )?
            },
        };

        let (result, exit_reason) = executor.execute();
        debug_print!("result, exit_reason of executor.execute(): {:?}, {:?}", result, exit_reason);
        let steps_executed = executor.get_steps_executed();
        executor.gasometer_mut().pad_evm_steps(steps_executed);

        let used_gas = executor.used_gas();
        if used_gas > trx.gas_limit {
            (evm::ExitError::OutOfGas.into(), vec![], None, trx.gas_limit, response)
        } else {
            let apply = if exit_reason.is_succeed() {
                let executor_state = executor.into_state();
                Some(executor_state.deconstruct())
            } else {
                None 
            };

            (exit_reason, result, apply, used_gas, response)
        }
    };

    debug_print!("exit_reason: {:?}", exit_reason);
    let response = response
        .set_data(return_value);

    // TODO: Gas payment and calculation

    if let Some(apply_state) = apply_state {
        storage.apply_state_change(apply_state)?;
    } else {
        // Transaction ended with error, no state to apply
        // Increment nonce here. Normally it is incremented inside apply_state_change
        storage.increment_nonce(&caller_address)?;
    }

    Ok(response)
}