#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint256};
use cw2::set_contract_version;
use evm::H160;

use crate::error::ContractError;
use crate::message::{self, transaction_execute_simple};
use crate::message::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:terranova";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // Only trusted operators can make calls
    // TODO: First check that the message sender is an operator

    match msg {
        ExecuteMsg::CallFromRawEthereumTX { caller_evm_address, unsigned_tx } => {
            transaction_execute_simple::process(deps, env, caller_evm_address, unsigned_tx)?;
            Ok(Response::new())
        }
        _ => panic!("Not implemented")
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        _ => Ok(to_binary(&0_i32)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};
    use evm::{H160};
    use crate::airdrop::{airdrop_write_balance, airdrop_deploy_contract};

    fn parse_hex(hex_asm: &str) -> Vec<u8> {
        let hex_asm = &hex_asm[2..];
        let hex_chars: Vec<char> = hex_asm.as_bytes().iter().filter_map(|b| {
            let ch = char::from(*b);
            if ('0' <= ch && ch <= '9') || ('a' <= ch && ch <= 'f') || ('A' <= ch && ch <= 'F') {
                Some(ch)
            } else {
                None
            }
        }).collect();
    
        let mut index = 0usize;
        let (odd_chars, even_chars): (Vec<char>, Vec<char>) = hex_chars.into_iter().partition(|_| { 
            index = index + 1;
            index % 2 == 1
        });
    
        odd_chars.into_iter().zip(even_chars.into_iter()).map(|(c0, c1)| {
            fn hexchar2int(ch: char) -> u8 {
                if '0' <= ch && ch <= '9' {
                    ch as u8 - '0' as u8
                } else {
                    0xa + 
                    if 'a' <= ch && ch <= 'f' {
                        ch as u8 - 'a' as u8
                    } else if 'A' <= ch && ch <= 'F' {
                        ch as u8 - 'A' as u8
                    } else {
                        unreachable!()
                    }
                }
            }
            hexchar2int(c0) * 0x10 + hexchar2int(c1)            
        }).collect::<Vec<u8>>()
    }

    fn parse_h160(address_str: &str) -> H160 {
        let vec_u8 = parse_hex(address_str);
        let mut array_u8 = [0_u8; 20];
        for (i, n_u8) in vec_u8.iter().enumerate() {
            array_u8[i] = *n_u8;
        }

        H160(array_u8)
    }

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg { };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    /// TODO
    #[test]
    fn simple_user_user_transfer() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg { };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // TODO
        let sender_addr: H160 = parse_h160("0xd3CdA913deB6f67967B99D67aCDFa1712C293601");

        println!("Sender addr: {}", sender_addr);

        airdrop_write_balance(deps.as_mut(), mock_env(), sender_addr);

        let contract_addr: H160 = parse_h160("0x402B964289Da03f1F26Bf1bAdDc1E34DA8468F9a");
        let contract_code = parse_hex("0608");

        println!("Contract addr: {}", contract_addr);

        airdrop_deploy_contract(deps.as_mut(), mock_env(), sender_addr, contract_code);
    }
}
