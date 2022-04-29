use cosmwasm_std::{StdResult, DepsMut, Uint256};
use evm::H160;
use crate::{account_storage::ACCOUNTS, account::EthereumAccount};

pub fn process(deps: DepsMut, operator: String, eth_address: [u8; 20], contract_byte_code: Vec<u8>) -> StdResult<()> {
    let eth_address_H160 = H160::from(eth_address);
    
    let account = EthereumAccount {
        address: eth_address_H160,
        bump_seed: 0_u8, // TODO: Right now this is being treated as useless. Figure out if it actually is
        trx_count: 0_u64,
        contract_code: contract_byte_code,
        balance: Uint256::zero(),
        rw_blocked: false,
        ro_blocked_count: 0_u8,
    };

    ACCOUNTS.save(deps.storage, eth_address, &account)?;

    Ok(())
}