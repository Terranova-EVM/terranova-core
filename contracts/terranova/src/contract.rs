#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint256};
use cw2::set_contract_version;

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
            transaction_execute_simple::process(deps, env, caller_evm_address, unsigned_tx)
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
    use cosmwasm_std::{coins, from_binary, Order};
    use evm::{H160, U256};
    use serde::{Serialize, Serializer};
    use crate::airdrop::{airdrop_write_balance, airdrop_deploy_contract, get_backend};
    use crate::storage::backend::{ACCOUNTS, CONTRACTS, CONTRACT_STORAGE};
    use env_logger;

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

    #[test]
    fn simple_user_user_transfer() {
        env_logger::init();
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg { };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let sender_addr: H160 = parse_h160("0xB34e2213751c5d8e9a31355fcA6F1B4FA5bB6bE1");
        
        // This addresses differs by sender_addr by one character in the middle
        let receiver_addr: H160 = parse_h160("0xB34e2213751c5d8e9a31755fcA6F1B4FA5bB6bE1");

        airdrop_write_balance(deps.as_mut(), mock_env(), sender_addr);
        
        // Python (3.0+) script for creating rlp-encoded raw unsigned transaction
        // import rlp
        // 
        // class NoChainTrx(rlp.Serializable):
        //     fields = (
        //         ('nonce', rlp.codec.big_endian_int),
        //         ('gasPrice', rlp.codec.big_endian_int),
        //         ('gasLimit', rlp.codec.big_endian_int),
        //         ('toAddress', rlp.codec.binary),
        //         ('value', rlp.codec.big_endian_int),
        //         ('callData', rlp.codec.binary),
        //     )
        
        //     @classmethod
        //     def fromString(cls, s):
        //         return rlp.decode(s, NoChainTrx)
        
        // tx = NoChainTrx(
        //     100, # nonce
        //     1, # gasPrice
        //     100000, # gasLimit
        //     bytes.fromhex('0xB34e2213751c5d8e9a31755fcA6F1B4FA5bB6bE1'), # toAddress
        //     123456, # value
        //     bytes.fromhex('') # callData
        // )
        
        // rlp.encode(tx).hex()
            
        // let trx_hex = "0xe06401830186a094d3cda913deb6f67967b99d67acdfa1712c2936018301e24080";
        let trx_hex = "0xe00101830186a094b34e2213751c5d8e9a31755fca6f1b4fa5bb6be18301e24080";
        let trx = parse_hex(&trx_hex);

        let msg = ExecuteMsg::CallFromRawEthereumTX { 
            caller_evm_address: sender_addr.to_fixed_bytes(), 
            unsigned_tx: trx 
        };

        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        assert_eq!(99876544, ACCOUNTS.load(deps.as_ref().storage, &sender_addr).unwrap().balance.as_u128());
        assert_eq!(123456, ACCOUNTS.load(deps.as_ref().storage, &receiver_addr).unwrap().balance.as_u128());
    }

    #[test]
    fn simple_contract_deploy() {
        env_logger::init();
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg { };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        
        let sender_addr: H160 = parse_h160("0xB34e2213751c5d8e9a31355fcA6F1B4FA5bB6bE1");

        // SimpleStorage.sol, compiled on truffle remix browser
        let trx_hex = "0xf901838001839896808080b90178608060405263075bcd1560005534801561001857600080fd5b50610150806100286000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c80632e64cec11461003b5780636057361d14610059575b600080fd5b610043610075565b60405161005091906100d9565b60405180910390f35b610073600480360381019061006e919061009d565b61007e565b005b60008054905090565b8060008190555050565b60008135905061009781610103565b92915050565b6000602082840312156100b3576100b26100fe565b5b60006100c184828501610088565b91505092915050565b6100d3816100f4565b82525050565b60006020820190506100ee60008301846100ca565b92915050565b6000819050919050565b600080fd5b61010c816100f4565b811461011757600080fd5b5056fea2646970667358221220facec4d53246a2f266ae2009faa8f2b2d003a6565a11f23d636d33a0424178ad64736f6c63430008070033";
        let trx = parse_hex(&trx_hex);

        let msg = ExecuteMsg::CallFromRawEthereumTX { 
            caller_evm_address: sender_addr.to_fixed_bytes(), 
            unsigned_tx: trx 
        };

        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        println!("{:?}", res);
    }

    #[test]
    fn simple_contract_interact() {
        env_logger::init();
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg { };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        
        let sender_addr: H160 = parse_h160("0xB34e2213751c5d8e9a31355fcA6F1B4FA5bB6bE1");
        // airdrop_write_balance(deps.as_mut(), mock_env(), sender_addr);
        
        // SimpleStorage.sol, compiled on truffle remix browser
        let trx_hex = "0xf901808001839896808080b90175608060405260aa60005534801561001557600080fd5b50610150806100256000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c80632e64cec11461003b5780636057361d14610059575b600080fd5b610043610075565b60405161005091906100d9565b60405180910390f35b610073600480360381019061006e919061009d565b61007e565b005b60008054905090565b8060008190555050565b60008135905061009781610103565b92915050565b6000602082840312156100b3576100b26100fe565b5b60006100c184828501610088565b91505092915050565b6100d3816100f4565b82525050565b60006020820190506100ee60008301846100ca565b92915050565b6000819050919050565b600080fd5b61010c816100f4565b811461011757600080fd5b5056fea2646970667358221220b65bdaef17cddab79670f4265ba7f40ee7d3c93b549cac6537012e5ac8ee7f5064736f6c63430008070033";
        
        let trx = parse_hex(&trx_hex);
        let msg = ExecuteMsg::CallFromRawEthereumTX { 
            caller_evm_address: sender_addr.to_fixed_bytes(), 
            unsigned_tx: trx
        };

        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        assert_eq!("create", res.attributes[0].value);
        assert_eq!("address", res.attributes[1].key);
        // Contract address creation is determinisitic, should always be this
        //println!("Contract address: {}", res.attributes[1].value);
        assert_eq!("0xff3b783539a1a7a53ecacfb1c0778274c670f35b", res.attributes[1].value);
        
        let contract_addr = parse_h160(&res.attributes[1].value);

        debug_print!("Calling retrieve function");

        // Call retrieve() function of SimpleStorage contract
        let trx_hex = "0xe180018398968094ff3b783539a1a7a53ecacfb1c0778274c670f35b80842e64cec1";
        let trx = parse_hex(&trx_hex);
        let msg = ExecuteMsg::CallFromRawEthereumTX { 
            caller_evm_address: sender_addr.to_fixed_bytes(), 
            unsigned_tx: trx 
        };

        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        assert_eq!(170, U256::from_big_endian_fast(res.data.as_ref().unwrap().as_slice()).as_u128());

        // Call store(0xbb)
        let trx_hex = "0xf84180018398968094ff3b783539a1a7a53ecacfb1c0778274c670f35b80a46057361d00000000000000000000000000000000000000000000000000000000000000bb";
        let trx = parse_hex(&trx_hex);
        let msg = ExecuteMsg::CallFromRawEthereumTX { 
            caller_evm_address: sender_addr.to_fixed_bytes(), 
            unsigned_tx: trx 
        };

        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        // Call retrieve() function of SimpleStorage contract
        let trx_hex = "0xe180018398968094ff3b783539a1a7a53ecacfb1c0778274c670f35b80842e64cec1";
        let trx = parse_hex(&trx_hex);
        let msg = ExecuteMsg::CallFromRawEthereumTX { 
            caller_evm_address: sender_addr.to_fixed_bytes(), 
            unsigned_tx: trx 
        };

        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap(); 
        assert_eq!(187, U256::from_big_endian_fast(res.data.as_ref().unwrap().as_slice()).as_u128());
    }
}
