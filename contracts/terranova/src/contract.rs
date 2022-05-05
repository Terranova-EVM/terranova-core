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
    use crate::storage::backend::{ACCOUNTS, CONTRACTS};
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

        // SimpleStorage.sol, compiled by Neel using Truffle deploy, nonce: 0
        let trx_hex = "0xf903928001830186a08080b903876080604052336000806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555034801561005057600080fd5b50610327806100606000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c8063445df0ac146100465780638da5cb5b14610064578063fdacd57614610082575b600080fd5b61004e61009e565b60405161005b9190610179565b60405180910390f35b61006c6100a4565b60405161007991906101d5565b60405180910390f35b61009c60048036038101906100979190610221565b6100c8565b005b60015481565b60008054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b60008054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610156576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161014d906102d1565b60405180910390fd5b8060018190555050565b6000819050919050565b61017381610160565b82525050565b600060208201905061018e600083018461016a565b92915050565b600073ffffffffffffffffffffffffffffffffffffffff82169050919050565b60006101bf82610194565b9050919050565b6101cf816101b4565b82525050565b60006020820190506101ea60008301846101c6565b92915050565b600080fd5b6101fe81610160565b811461020957600080fd5b50565b60008135905061021b816101f5565b92915050565b600060208284031215610237576102366101f0565b5b60006102458482850161020c565b91505092915050565b600082825260208201905092915050565b7f546869732066756e6374696f6e206973207265737472696374656420746f207460008201527f686520636f6e74726163742773206f776e657200000000000000000000000000602082015250565b60006102bb60338361024e565b91506102c68261025f565b604082019050919050565b600060208201905081810360008301526102ea816102ae565b905091905056fea26469706673582212202772102c6bf65909b8a1ed2bbf4af4f042e10a1097612ea089ae207eeb38038964736f6c634300080d0033";
        let trx = parse_hex(&trx_hex);

        let msg = ExecuteMsg::CallFromRawEthereumTX { 
            caller_evm_address: sender_addr.to_fixed_bytes(), 
            unsigned_tx: trx 
        };

        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        println!("{:?}", res);

        let sender_addr: H160 = parse_h160("0xB34e2213751c5d8e9a31355fcA6F1B4FA5bB6bE1");

        // SimpleStorage.sol, compiled by Neel using Truffle deploy, nonce: 1
        let trx_hex = "0xf903920101830186a08080b903876080604052336000806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555034801561005057600080fd5b50610327806100606000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c8063445df0ac146100465780638da5cb5b14610064578063fdacd57614610082575b600080fd5b61004e61009e565b60405161005b9190610179565b60405180910390f35b61006c6100a4565b60405161007991906101d5565b60405180910390f35b61009c60048036038101906100979190610221565b6100c8565b005b60015481565b60008054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b60008054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610156576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161014d906102d1565b60405180910390fd5b8060018190555050565b6000819050919050565b61017381610160565b82525050565b600060208201905061018e600083018461016a565b92915050565b600073ffffffffffffffffffffffffffffffffffffffff82169050919050565b60006101bf82610194565b9050919050565b6101cf816101b4565b82525050565b60006020820190506101ea60008301846101c6565b92915050565b600080fd5b6101fe81610160565b811461020957600080fd5b50565b60008135905061021b816101f5565b92915050565b600060208284031215610237576102366101f0565b5b60006102458482850161020c565b91505092915050565b600082825260208201905092915050565b7f546869732066756e6374696f6e206973207265737472696374656420746f207460008201527f686520636f6e74726163742773206f776e657200000000000000000000000000602082015250565b60006102bb60338361024e565b91506102c68261025f565b604082019050919050565b600060208201905081810360008301526102ea816102ae565b905091905056fea26469706673582212202772102c6bf65909b8a1ed2bbf4af4f042e10a1097612ea089ae207eeb38038964736f6c634300080d0033";
        let trx = parse_hex(&trx_hex);

        let msg = ExecuteMsg::CallFromRawEthereumTX { 
            caller_evm_address: sender_addr.to_fixed_bytes(), 
            unsigned_tx: trx 
        };

        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        println!("{:?}", res);
    }
}
