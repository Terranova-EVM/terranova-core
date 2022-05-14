use super::*;
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, from_binary, Order, Uint256, OwnedDeps, DepsMut};
use evm::{H160, U256};
use crate::airdrop::{airdrop_write_balance, airdrop_deploy_contract, get_backend};
use crate::contract::{instantiate, execute, query};
use crate::storage::backend::{ACCOUNTS, CONTRACTS, CONTRACT_STORAGE};
use crate::message::{ExecuteMsg, InstantiateMsg, QueryMsg, RawEthereumQueryResponse, EvmAccountResponse};
use crate::transaction::UnsignedTransaction;
use crate::utils::{parse_h160, parse_hex};
use env_logger;


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

    let msg = ExecuteMsg::ExecuteRawEthereumTx { 
        caller_evm_address: sender_addr.to_fixed_bytes(), 
        unsigned_tx: trx 
    };

    let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    assert_eq!(99876544, ACCOUNTS.load(deps.as_ref().storage, &sender_addr).unwrap().balance.as_u128());
    assert_eq!(123456, ACCOUNTS.load(deps.as_ref().storage, &receiver_addr).unwrap().balance.as_u128());
}

#[test]
fn simple_contract_deploy() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg { };
    let info = mock_info("creator", &coins(1000, "earth"));

    // we can just call .unwrap() to assert this was a success
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    
    let sender_addr: H160 = parse_h160("0xB34e2213751c5d8e9a31355fcA6F1B4FA5bB6bE1");

    // SimpleStorage.sol, compiled on truffle remix browser
    let trx_hex = "0xf901838001839896808080b90178608060405263075bcd1560005534801561001857600080fd5b50610150806100286000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c80632e64cec11461003b5780636057361d14610059575b600080fd5b610043610075565b60405161005091906100d9565b60405180910390f35b610073600480360381019061006e919061009d565b61007e565b005b60008054905090565b8060008190555050565b60008135905061009781610103565b92915050565b6000602082840312156100b3576100b26100fe565b5b60006100c184828501610088565b91505092915050565b6100d3816100f4565b82525050565b60006020820190506100ee60008301846100ca565b92915050565b6000819050919050565b600080fd5b61010c816100f4565b811461011757600080fd5b5056fea2646970667358221220facec4d53246a2f266ae2009faa8f2b2d003a6565a11f23d636d33a0424178ad64736f6c63430008070033";
    let trx = parse_hex(&trx_hex);

    let msg = ExecuteMsg::ExecuteRawEthereumTx { 
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
    // let trx_hex = "0xf9019304018502540be40080834c4b40b90183f901808001839896808080b90175608060405260aa60005534801561001557600080fd5b50610150806100256000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c80632e64cec11461003b5780636057361d14610059575b600080fd5b610043610075565b60405161005091906100d9565b60405180910390f35b610073600480360381019061006e919061009d565b61007e565b005b60008054905090565b8060008190555050565b60008135905061009781610103565b92915050565b6000602082840312156100b3576100b26100fe565b5b60006100c184828501610088565b91505092915050565b6100d3816100f4565b82525050565b60006020820190506100ee60008301846100ca565b92915050565b6000819050919050565b600080fd5b61010c816100f4565b811461011757600080fd5b5056fea2646970667358221220b65bdaef17cddab79670f4265ba7f40ee7d3c93b549cac6537012e5ac8ee7f5064736f6c63430008070033";
    let trx = parse_hex(&trx_hex);
    let msg = ExecuteMsg::ExecuteRawEthereumTx { 
        caller_evm_address: sender_addr.to_fixed_bytes(), 
        unsigned_tx: trx
    };

    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    println!("Result: {:?}", res);
    assert_eq!("create", res.attributes[0].value);
    assert_eq!("address", res.attributes[1].key);
    // Contract address creation is determinisitic, should always be this
    //println!("Contract address: {}", res.attributes[1].value);
    assert_eq!("0xff3b783539a1a7a53ecacfb1c0778274c670f35b", res.attributes[1].value);
    
    let contract_addr = parse_h160(&res.attributes[1].value);

    // Call store(0xbb)
    let trx_hex = "0xf84180018398968094ff3b783539a1a7a53ecacfb1c0778274c670f35b80a46057361d00000000000000000000000000000000000000000000000000000000000000bb";
    let trx = parse_hex(&trx_hex);
    let msg = ExecuteMsg::ExecuteRawEthereumTx { 
        caller_evm_address: sender_addr.to_fixed_bytes(), 
        unsigned_tx: trx 
    };

    let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    // Call retrieve() function of SimpleStorage contract
    let trx_hex = "0xe180018398968094ff3b783539a1a7a53ecacfb1c0778274c670f35b80842e64cec1";
    let trx = parse_hex(&trx_hex);
    let msg = QueryMsg::RawEthereumQuery {
        caller_evm_address: sender_addr.to_fixed_bytes(),
        unsigned_tx: trx
    };

    let res = query(deps.as_ref(), mock_env(), msg).unwrap().to_vec();
    assert_eq!(187, U256::from_big_endian_fast(res.as_slice()).as_u128());
}

#[test]
fn erc20_transfer() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg { };
    let info = mock_info("creator", &coins(1000, "earth"));

    // we can just call .unwrap() to assert this was a success
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    
    let sender_addr: H160 = parse_h160("0xB34e2213751c5d8e9a31355fcA6F1B4FA5bB6bE1");
    
    // Erc20Simple.sol, compiled on truffle remix browser
    // Barebones impl from https://ethereum.org/en/developers/tutorials/understand-the-erc-20-token-smart-contract/
    // Initial supply gets minted to the trx sender's balance
    let trx_hex = "0xf90dd28001839896808080b90dc76080604052678ac7230489e8000060025534801561001c57600080fd5b506002546000803373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002081905550610d56806100716000396000f3fe608060405234801561001057600080fd5b50600436106100935760003560e01c8063313ce56711610066578063313ce5671461013457806370a082311461015257806395d89b4114610182578063a9059cbb146101a0578063dd62ed3e146101d057610093565b806306fdde0314610098578063095ea7b3146100b657806318160ddd146100e657806323b872dd14610104575b600080fd5b6100a0610200565b6040516100ad9190610b27565b60405180910390f35b6100d060048036038101906100cb9190610a66565b610239565b6040516100dd9190610b0c565b60405180910390f35b6100ee61032b565b6040516100fb9190610b49565b60405180910390f35b61011e60048036038101906101199190610a13565b610335565b60405161012b9190610b0c565b60405180910390f35b61013c61069b565b6040516101499190610b64565b60405180910390f35b61016c600480360381019061016791906109a6565b6106a0565b6040516101799190610b49565b60405180910390f35b61018a6106e8565b6040516101979190610b27565b60405180910390f35b6101ba60048036038101906101b59190610a66565b610721565b6040516101c79190610b0c565b60405180910390f35b6101ea60048036038101906101e591906109d3565b6108f5565b6040516101f79190610b49565b60405180910390f35b6040518060400160405280600f81526020017f54657272616e6f7661204552433230000000000000000000000000000000000081525081565b600081600160003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020819055508273ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff167f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925846040516103199190610b49565b60405180910390a36001905092915050565b6000600254905090565b60008060008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000205482111561038257600080fd5b600160008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000205482111561040b57600080fd5b816000808673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020546104559190610bf1565b6000808673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000208190555081600160008673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000205461051f9190610bf1565b600160008673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002081905550816000808573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020546105e99190610b9b565b6000808573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020819055508273ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef846040516106889190610b49565b60405180910390a3600190509392505050565b601281565b60008060008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020549050919050565b6040518060400160405280600481526020017f4e4f56410000000000000000000000000000000000000000000000000000000081525081565b60008060003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000205482111561076e57600080fd5b816000803373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020546107b89190610bf1565b6000803373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002081905550816000808573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020546108449190610b9b565b6000808573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020819055508273ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef846040516108e39190610b49565b60405180910390a36001905092915050565b6000600160008473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002054905092915050565b60008135905061098b81610cf2565b92915050565b6000813590506109a081610d09565b92915050565b6000602082840312156109bc576109bb610cdc565b5b60006109ca8482850161097c565b91505092915050565b600080604083850312156109ea576109e9610cdc565b5b60006109f88582860161097c565b9250506020610a098582860161097c565b9150509250929050565b600080600060608486031215610a2c57610a2b610cdc565b5b6000610a3a8682870161097c565b9350506020610a4b8682870161097c565b9250506040610a5c86828701610991565b9150509250925092565b60008060408385031215610a7d57610a7c610cdc565b5b6000610a8b8582860161097c565b9250506020610a9c85828601610991565b9150509250929050565b610aaf81610c37565b82525050565b6000610ac082610b7f565b610aca8185610b8a565b9350610ada818560208601610c7a565b610ae381610ce1565b840191505092915050565b610af781610c63565b82525050565b610b0681610c6d565b82525050565b6000602082019050610b216000830184610aa6565b92915050565b60006020820190508181036000830152610b418184610ab5565b905092915050565b6000602082019050610b5e6000830184610aee565b92915050565b6000602082019050610b796000830184610afd565b92915050565b600081519050919050565b600082825260208201905092915050565b6000610ba682610c63565b9150610bb183610c63565b9250827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff03821115610be657610be5610cad565b5b828201905092915050565b6000610bfc82610c63565b9150610c0783610c63565b925082821015610c1a57610c19610cad565b5b828203905092915050565b6000610c3082610c43565b9050919050565b60008115159050919050565b600073ffffffffffffffffffffffffffffffffffffffff82169050919050565b6000819050919050565b600060ff82169050919050565b60005b83811015610c98578082015181840152602081019050610c7d565b83811115610ca7576000848401525b50505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b600080fd5b6000601f19601f8301169050919050565b610cfb81610c25565b8114610d0657600080fd5b50565b610d1281610c63565b8114610d1d57600080fd5b5056fea2646970667358221220fd73d39b0f9762fc4ab42cde5b96e2b9cca69e87e5b45403fcc50bc76d377b9b64736f6c63430008070033";
    let trx = parse_hex(&trx_hex);
    let msg = ExecuteMsg::ExecuteRawEthereumTx { 
        caller_evm_address: sender_addr.to_fixed_bytes(),
        unsigned_tx: trx
    };

    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    println!("{:?}", res);
    assert_eq!("0xff3b783539a1a7a53ecacfb1c0778274c670f35b", res.attributes[2].value);

    let contract_address = parse_h160(&res.attributes[2].value);

    /// Get balance of sender_addr
    let trx_hex = "0xf84180018398968094ff3b783539a1a7a53ecacfb1c0778274c670f35b80a470a08231000000000000000000000000b34e2213751c5d8e9a31355fca6f1b4fa5bb6be1";
    let trx = parse_hex(&trx_hex);
    let msg = QueryMsg::RawEthereumQuery {
        caller_evm_address: sender_addr.to_fixed_bytes(),
        unsigned_tx: trx
    };

    let res: RawEthereumQueryResponse = from_binary(&query(deps.as_ref(), mock_env(), msg).unwrap()).unwrap();
    println!("Balance result: {:?}", res);
    assert_eq!(10_000_000_000_000_000_000, U256::from_big_endian_fast(&res.result).as_u128());

    // Transfer 77777 tokens from sender to receiver
    let receiver_addr: H160 = parse_h160("0x2e36b2970ab7A4C955eADD836585c21A087Ab904");

    let trx_hex = "0xf86280018398968094ff3b783539a1a7a53ecacfb1c0778274c670f35b80b844a9059cbb0000000000000000000000002e36b2970ab7a4c955eadd836585c21a087ab9040000000000000000000000000000000000000000000000000000000000012fd1";
    let trx = parse_hex(&trx_hex);
    let msg = ExecuteMsg::ExecuteRawEthereumTx { 
        caller_evm_address: sender_addr.to_fixed_bytes(),
        unsigned_tx: trx
    };

    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    println!("Execute result: {:?}", res);
    // Check new balance of receiver
    let trx_hex = "0xf84180018398968094ff3b783539a1a7a53ecacfb1c0778274c670f35b80a470a082310000000000000000000000002e36b2970ab7a4c955eadd836585c21a087ab904";
    let trx = parse_hex(&trx_hex);
    let msg = QueryMsg::RawEthereumQuery {
        caller_evm_address: receiver_addr.to_fixed_bytes(),
        unsigned_tx: trx
    };

    let res: RawEthereumQueryResponse = from_binary(&query(deps.as_ref(), mock_env(), msg).unwrap()).unwrap();
    println!("Balance result: {:?}", res);
    assert_eq!(77_777, U256::from_big_endian_fast(&res.result).as_u128());
}

#[test]
fn erc20_approve() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg { };
    let info = mock_info("creator", &coins(1000, "earth"));

    // we can just call .unwrap() to assert this was a success
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    
    let sender_addr: H160 = parse_h160("0xB34e2213751c5d8e9a31355fcA6F1B4FA5bB6bE1");
    
    // Erc20Simple.sol, compiled on truffle remix browser
    // Barebones impl from https://ethereum.org/en/developers/tutorials/understand-the-erc-20-token-smart-contract/
    // Initial supply gets minted to the trx sender's balance
    let trx_hex = "0xf90dd28001839896808080b90dc76080604052678ac7230489e8000060025534801561001c57600080fd5b506002546000803373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002081905550610d56806100716000396000f3fe608060405234801561001057600080fd5b50600436106100935760003560e01c8063313ce56711610066578063313ce5671461013457806370a082311461015257806395d89b4114610182578063a9059cbb146101a0578063dd62ed3e146101d057610093565b806306fdde0314610098578063095ea7b3146100b657806318160ddd146100e657806323b872dd14610104575b600080fd5b6100a0610200565b6040516100ad9190610b27565b60405180910390f35b6100d060048036038101906100cb9190610a66565b610239565b6040516100dd9190610b0c565b60405180910390f35b6100ee61032b565b6040516100fb9190610b49565b60405180910390f35b61011e60048036038101906101199190610a13565b610335565b60405161012b9190610b0c565b60405180910390f35b61013c61069b565b6040516101499190610b64565b60405180910390f35b61016c600480360381019061016791906109a6565b6106a0565b6040516101799190610b49565b60405180910390f35b61018a6106e8565b6040516101979190610b27565b60405180910390f35b6101ba60048036038101906101b59190610a66565b610721565b6040516101c79190610b0c565b60405180910390f35b6101ea60048036038101906101e591906109d3565b6108f5565b6040516101f79190610b49565b60405180910390f35b6040518060400160405280600f81526020017f54657272616e6f7661204552433230000000000000000000000000000000000081525081565b600081600160003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020819055508273ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff167f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925846040516103199190610b49565b60405180910390a36001905092915050565b6000600254905090565b60008060008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000205482111561038257600080fd5b600160008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000205482111561040b57600080fd5b816000808673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020546104559190610bf1565b6000808673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000208190555081600160008673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000205461051f9190610bf1565b600160008673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002081905550816000808573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020546105e99190610b9b565b6000808573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020819055508273ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef846040516106889190610b49565b60405180910390a3600190509392505050565b601281565b60008060008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020549050919050565b6040518060400160405280600481526020017f4e4f56410000000000000000000000000000000000000000000000000000000081525081565b60008060003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000205482111561076e57600080fd5b816000803373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020546107b89190610bf1565b6000803373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002081905550816000808573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020546108449190610b9b565b6000808573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020819055508273ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef846040516108e39190610b49565b60405180910390a36001905092915050565b6000600160008473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002054905092915050565b60008135905061098b81610cf2565b92915050565b6000813590506109a081610d09565b92915050565b6000602082840312156109bc576109bb610cdc565b5b60006109ca8482850161097c565b91505092915050565b600080604083850312156109ea576109e9610cdc565b5b60006109f88582860161097c565b9250506020610a098582860161097c565b9150509250929050565b600080600060608486031215610a2c57610a2b610cdc565b5b6000610a3a8682870161097c565b9350506020610a4b8682870161097c565b9250506040610a5c86828701610991565b9150509250925092565b60008060408385031215610a7d57610a7c610cdc565b5b6000610a8b8582860161097c565b9250506020610a9c85828601610991565b9150509250929050565b610aaf81610c37565b82525050565b6000610ac082610b7f565b610aca8185610b8a565b9350610ada818560208601610c7a565b610ae381610ce1565b840191505092915050565b610af781610c63565b82525050565b610b0681610c6d565b82525050565b6000602082019050610b216000830184610aa6565b92915050565b60006020820190508181036000830152610b418184610ab5565b905092915050565b6000602082019050610b5e6000830184610aee565b92915050565b6000602082019050610b796000830184610afd565b92915050565b600081519050919050565b600082825260208201905092915050565b6000610ba682610c63565b9150610bb183610c63565b9250827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff03821115610be657610be5610cad565b5b828201905092915050565b6000610bfc82610c63565b9150610c0783610c63565b925082821015610c1a57610c19610cad565b5b828203905092915050565b6000610c3082610c43565b9050919050565b60008115159050919050565b600073ffffffffffffffffffffffffffffffffffffffff82169050919050565b6000819050919050565b600060ff82169050919050565b60005b83811015610c98578082015181840152602081019050610c7d565b83811115610ca7576000848401525b50505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b600080fd5b6000601f19601f8301169050919050565b610cfb81610c25565b8114610d0657600080fd5b50565b610d1281610c63565b8114610d1d57600080fd5b5056fea2646970667358221220fd73d39b0f9762fc4ab42cde5b96e2b9cca69e87e5b45403fcc50bc76d377b9b64736f6c63430008070033";
    let trx = parse_hex(&trx_hex);
    let msg = ExecuteMsg::ExecuteRawEthereumTx { 
        caller_evm_address: sender_addr.to_fixed_bytes(),
        unsigned_tx: trx
    };

    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    println!("{:?}", res);
    assert_eq!("0xff3b783539a1a7a53ecacfb1c0778274c670f35b", res.attributes[2].value);

    let contract_address = parse_h160(&res.attributes[2].value);

    // Get balance of sender_addr
    let trx_hex = "0xf84180018398968094ff3b783539a1a7a53ecacfb1c0778274c670f35b80a470a08231000000000000000000000000b34e2213751c5d8e9a31355fca6f1b4fa5bb6be1";
    let trx = parse_hex(&trx_hex);
    let msg = QueryMsg::RawEthereumQuery {
        caller_evm_address: sender_addr.to_fixed_bytes(),
        unsigned_tx: trx
    };

    let res: RawEthereumQueryResponse = from_binary(&query(deps.as_ref(), mock_env(), msg).unwrap()).unwrap();
    println!("Balance result: {:?}", res);
    assert_eq!(10_000_000_000_000_000_000, U256::from_big_endian_fast(&res.result).as_u128());

    // Approve address 0x47e0a3ddd614e28670da25b414afad2751741725 to spend sender_addr's tokens
    let approved: H160 = parse_h160("0x47e0a3ddd614e28670da25b414afad2751741725");
    let trx_hex = "0xf8628001830f424094ff3b783539a1a7a53ecacfb1c0778274c670f35b80b844095ea7b300000000000000000000000047e0a3ddd614e28670da25b414afad275174172500ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
    let trx = parse_hex(&trx_hex);
    let msg = ExecuteMsg::ExecuteRawEthereumTx { 
        caller_evm_address: sender_addr.to_fixed_bytes(),
        unsigned_tx: trx
    };

    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    println!("Result of ERC20 approve {:?}", res);

    // Transfor from 256 of the erc20 token from sender_addr to receiver_addr using the approved address as the transaction sender
    let receiver_addr: H160 = parse_h160("0x2e36b2970ab7A4C955eADD836585c21A087Ab904");
    
    let trx_hex = "0xf8828001830f424094ff3b783539a1a7a53ecacfb1c0778274c670f35b80b86423b872dd000000000000000000000000b34e2213751c5d8e9a31355fca6f1b4fa5bb6be10000000000000000000000002e36b2970ab7a4c955eadd836585c21a087ab9040000000000000000000000000000000000000000000000000000000000000100";
    let trx = parse_hex(&trx_hex);
    let msg = ExecuteMsg::ExecuteRawEthereumTx { 
        caller_evm_address: approved.to_fixed_bytes(),
        unsigned_tx: trx
    };

    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    println!("Result of ERC20 transferFrom {:?}", res);
    

    // Check new balance of receiver
    let trx_hex = "0xf84180018398968094ff3b783539a1a7a53ecacfb1c0778274c670f35b80a470a082310000000000000000000000002e36b2970ab7a4c955eadd836585c21a087ab904";
    let trx = parse_hex(&trx_hex);
    let msg = QueryMsg::RawEthereumQuery {
        caller_evm_address: receiver_addr.to_fixed_bytes(),
        unsigned_tx: trx
    };

    let res: RawEthereumQueryResponse = from_binary(&query(deps.as_ref(), mock_env(), msg).unwrap()).unwrap();
    println!("Balance result: {:?}", res);
    assert_eq!(256, U256::from_big_endian_fast(&res.result).as_u128()); 
}

#[test]
fn chunked_transaction() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg { };
    let info = mock_info("creator", &coins(1000, "earth"));

    // we can just call .unwrap() to assert this was a success
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    
    let sender_addr: H160 = parse_h160("0xB34e2213751c5d8e9a31355fcA6F1B4FA5bB6bE1");

    // Erc20Simple.sol, compiled on truffle remix browser
    // Barebones impl from https://ethereum.org/en/developers/tutorials/understand-the-erc-20-token-smart-contract/
    // Initial supply gets minted to the trx sender's balance
    let trx_hex = "0xf90dd28001839896808080b90dc76080604052678ac7230489e8000060025534801561001c57600080fd5b506002546000803373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002081905550610d56806100716000396000f3fe608060405234801561001057600080fd5b50600436106100935760003560e01c8063313ce56711610066578063313ce5671461013457806370a082311461015257806395d89b4114610182578063a9059cbb146101a0578063dd62ed3e146101d057610093565b806306fdde0314610098578063095ea7b3146100b657806318160ddd146100e657806323b872dd14610104575b600080fd5b6100a0610200565b6040516100ad9190610b27565b60405180910390f35b6100d060048036038101906100cb9190610a66565b610239565b6040516100dd9190610b0c565b60405180910390f35b6100ee61032b565b6040516100fb9190610b49565b60405180910390f35b61011e60048036038101906101199190610a13565b610335565b60405161012b9190610b0c565b60405180910390f35b61013c61069b565b6040516101499190610b64565b60405180910390f35b61016c600480360381019061016791906109a6565b6106a0565b6040516101799190610b49565b60405180910390f35b61018a6106e8565b6040516101979190610b27565b60405180910390f35b6101ba60048036038101906101b59190610a66565b610721565b6040516101c79190610b0c565b60405180910390f35b6101ea60048036038101906101e591906109d3565b6108f5565b6040516101f79190610b49565b60405180910390f35b6040518060400160405280600f81526020017f54657272616e6f7661204552433230000000000000000000000000000000000081525081565b600081600160003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020819055508273ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff167f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925846040516103199190610b49565b60405180910390a36001905092915050565b6000600254905090565b60008060008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000205482111561038257600080fd5b600160008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000205482111561040b57600080fd5b816000808673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020546104559190610bf1565b6000808673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000208190555081600160008673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000205461051f9190610bf1565b600160008673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002081905550816000808573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020546105e99190610b9b565b6000808573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020819055508273ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef846040516106889190610b49565b60405180910390a3600190509392505050565b601281565b60008060008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020549050919050565b6040518060400160405280600481526020017f4e4f56410000000000000000000000000000000000000000000000000000000081525081565b60008060003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000205482111561076e57600080fd5b816000803373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020546107b89190610bf1565b6000803373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002081905550816000808573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020546108449190610b9b565b6000808573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020819055508273ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef846040516108e39190610b49565b60405180910390a36001905092915050565b6000600160008473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002054905092915050565b60008135905061098b81610cf2565b92915050565b6000813590506109a081610d09565b92915050565b6000602082840312156109bc576109bb610cdc565b5b60006109ca8482850161097c565b91505092915050565b600080604083850312156109ea576109e9610cdc565b5b60006109f88582860161097c565b9250506020610a098582860161097c565b9150509250929050565b600080600060608486031215610a2c57610a2b610cdc565b5b6000610a3a8682870161097c565b9350506020610a4b8682870161097c565b9250506040610a5c86828701610991565b9150509250925092565b60008060408385031215610a7d57610a7c610cdc565b5b6000610a8b8582860161097c565b9250506020610a9c85828601610991565b9150509250929050565b610aaf81610c37565b82525050565b6000610ac082610b7f565b610aca8185610b8a565b9350610ada818560208601610c7a565b610ae381610ce1565b840191505092915050565b610af781610c63565b82525050565b610b0681610c6d565b82525050565b6000602082019050610b216000830184610aa6565b92915050565b60006020820190508181036000830152610b418184610ab5565b905092915050565b6000602082019050610b5e6000830184610aee565b92915050565b6000602082019050610b796000830184610afd565b92915050565b600081519050919050565b600082825260208201905092915050565b6000610ba682610c63565b9150610bb183610c63565b9250827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff03821115610be657610be5610cad565b5b828201905092915050565b6000610bfc82610c63565b9150610c0783610c63565b925082821015610c1a57610c19610cad565b5b828203905092915050565b6000610c3082610c43565b9050919050565b60008115159050919050565b600073ffffffffffffffffffffffffffffffffffffffff82169050919050565b6000819050919050565b600060ff82169050919050565b60005b83811015610c98578082015181840152602081019050610c7d565b83811115610ca7576000848401525b50505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b600080fd5b6000601f19601f8301169050919050565b610cfb81610c25565b8114610d0657600080fd5b50565b610d1281610c63565b8114610d1d57600080fd5b5056fea2646970667358221220fd73d39b0f9762fc4ab42cde5b96e2b9cca69e87e5b45403fcc50bc76d377b9b64736f6c63430008070033";
    let trx_full = parse_hex(&trx_hex);

    println!("{}", trx_full.len());
    let tx_hash = utils::keccak256_h256(&trx_full);
    let chunks = vec![&trx_full[..1000], &trx_full[1000..2000], &trx_full[2000..3000], &trx_full[3000..]];

    // Store all the chunks
    for (i, chunk) in chunks.iter().enumerate() {
        let msg = ExecuteMsg::StoreTxChunk {
            caller_evm_address: sender_addr.to_fixed_bytes(),
            full_tx_hash: tx_hash.to_fixed_bytes(),
            chunk_index: i as u8,
            chunk_data: chunk.to_vec(),
        };

        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap(); 
    }

    // Make the call on the chunked transaction
    let msg = ExecuteMsg::ExecuteChunkedEthereumTx { 
        caller_evm_address: sender_addr.to_fixed_bytes(), 
        full_tx_hash: tx_hash.to_fixed_bytes(),
        chunk_count: chunks.len() as u8
    };

    let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap(); 
}

#[test]
fn account_query() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg { };
    let info = mock_info("creator", &coins(1000, "earth"));

    // we can just call .unwrap() to assert this was a success
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    
    let addr: H160 = parse_h160("0xB34e2213751c5d8e9a31355fcA6F1B4FA5bB6bE1");

    airdrop_write_balance(deps.as_mut(), mock_env(), addr);
    
    let msg = QueryMsg::QueryEvmAccount { evm_address: addr.to_fixed_bytes() };
    let res: EvmAccountResponse = from_binary(&query(deps.as_ref(), mock_env(), msg).unwrap()).unwrap();
    assert_eq!("100000000", res.balance.to_string()); 
    assert_eq!(0, res.nonce); 
}

#[test]
fn uniswap_v1() {
    env_logger::init();
    // Instantiate contract
    let mut deps = mock_dependencies(&[]);
    let msg = InstantiateMsg { };
    let relayer = mock_info("relayer", &coins(1000, "earth"));
    let _res = instantiate(deps.as_mut(), mock_env(), relayer.clone(), msg).unwrap();


    // Depositor address
    let depositor_addr: H160 = parse_h160("0xB34e2213751c5d8e9a31355fcA6F1B4FA5bB6bE1");
    let msg = QueryMsg::QueryEvmAccount { evm_address: depositor_addr.to_fixed_bytes() };
    let res: EvmAccountResponse = from_binary(&query(deps.as_ref(), mock_env(), msg).unwrap()).unwrap();
    // assert_eq!("100000000", res.balance.to_string()); 
    // assert_eq!(0, res.nonce); 

    // Deploy the NOVA ERC20 contract using depositor_addr
    // Initial supply of 10 NOVA is minted to depositor_addr
    let erc20_tx_hex = "0xf90dd28001839896808080b90dc76080604052678ac7230489e8000060025534801561001c57600080fd5b506002546000803373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002081905550610d56806100716000396000f3fe608060405234801561001057600080fd5b50600436106100935760003560e01c8063313ce56711610066578063313ce5671461013457806370a082311461015257806395d89b4114610182578063a9059cbb146101a0578063dd62ed3e146101d057610093565b806306fdde0314610098578063095ea7b3146100b657806318160ddd146100e657806323b872dd14610104575b600080fd5b6100a0610200565b6040516100ad9190610b27565b60405180910390f35b6100d060048036038101906100cb9190610a66565b610239565b6040516100dd9190610b0c565b60405180910390f35b6100ee61032b565b6040516100fb9190610b49565b60405180910390f35b61011e60048036038101906101199190610a13565b610335565b60405161012b9190610b0c565b60405180910390f35b61013c61069b565b6040516101499190610b64565b60405180910390f35b61016c600480360381019061016791906109a6565b6106a0565b6040516101799190610b49565b60405180910390f35b61018a6106e8565b6040516101979190610b27565b60405180910390f35b6101ba60048036038101906101b59190610a66565b610721565b6040516101c79190610b0c565b60405180910390f35b6101ea60048036038101906101e591906109d3565b6108f5565b6040516101f79190610b49565b60405180910390f35b6040518060400160405280600f81526020017f54657272616e6f7661204552433230000000000000000000000000000000000081525081565b600081600160003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020819055508273ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff167f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925846040516103199190610b49565b60405180910390a36001905092915050565b6000600254905090565b60008060008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000205482111561038257600080fd5b600160008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000205482111561040b57600080fd5b816000808673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020546104559190610bf1565b6000808673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000208190555081600160008673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000205461051f9190610bf1565b600160008673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002081905550816000808573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020546105e99190610b9b565b6000808573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020819055508273ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef846040516106889190610b49565b60405180910390a3600190509392505050565b601281565b60008060008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020549050919050565b6040518060400160405280600481526020017f4e4f56410000000000000000000000000000000000000000000000000000000081525081565b60008060003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000205482111561076e57600080fd5b816000803373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020546107b89190610bf1565b6000803373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002081905550816000808573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020546108449190610b9b565b6000808573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020819055508273ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef846040516108e39190610b49565b60405180910390a36001905092915050565b6000600160008473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002054905092915050565b60008135905061098b81610cf2565b92915050565b6000813590506109a081610d09565b92915050565b6000602082840312156109bc576109bb610cdc565b5b60006109ca8482850161097c565b91505092915050565b600080604083850312156109ea576109e9610cdc565b5b60006109f88582860161097c565b9250506020610a098582860161097c565b9150509250929050565b600080600060608486031215610a2c57610a2b610cdc565b5b6000610a3a8682870161097c565b9350506020610a4b8682870161097c565b9250506040610a5c86828701610991565b9150509250925092565b60008060408385031215610a7d57610a7c610cdc565b5b6000610a8b8582860161097c565b9250506020610a9c85828601610991565b9150509250929050565b610aaf81610c37565b82525050565b6000610ac082610b7f565b610aca8185610b8a565b9350610ada818560208601610c7a565b610ae381610ce1565b840191505092915050565b610af781610c63565b82525050565b610b0681610c6d565b82525050565b6000602082019050610b216000830184610aa6565b92915050565b60006020820190508181036000830152610b418184610ab5565b905092915050565b6000602082019050610b5e6000830184610aee565b92915050565b6000602082019050610b796000830184610afd565b92915050565b600081519050919050565b600082825260208201905092915050565b6000610ba682610c63565b9150610bb183610c63565b9250827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff03821115610be657610be5610cad565b5b828201905092915050565b6000610bfc82610c63565b9150610c0783610c63565b925082821015610c1a57610c19610cad565b5b828203905092915050565b6000610c3082610c43565b9050919050565b60008115159050919050565b600073ffffffffffffffffffffffffffffffffffffffff82169050919050565b6000819050919050565b600060ff82169050919050565b60005b83811015610c98578082015181840152602081019050610c7d565b83811115610ca7576000848401525b50505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b600080fd5b6000601f19601f8301169050919050565b610cfb81610c25565b8114610d0657600080fd5b50565b610d1281610c63565b8114610d1d57600080fd5b5056fea2646970667358221220fd73d39b0f9762fc4ab42cde5b96e2b9cca69e87e5b45403fcc50bc76d377b9b64736f6c63430008070033";
    let erc20_tx = parse_hex(&erc20_tx_hex);
    let msg = ExecuteMsg::ExecuteRawEthereumTx { 
        caller_evm_address: depositor_addr.to_fixed_bytes(),
        unsigned_tx: erc20_tx
    };
    let res = execute(deps.as_mut(), mock_env(), relayer.clone(), msg).unwrap();
    assert_eq!("0xff3b783539a1a7a53ecacfb1c0778274c670f35b", res.attributes[2].value); // NOVA token address


    // NOVA token address
    // 0xff3b783539a1a7a53ecacfb1c0778274c670f35b
    let nova_token_address: H160 = parse_h160(&res.attributes[2].value);


    // Deploy the Uniswap factory and exchange contracts
    let uniswap_factory_deploy_hex = "0xf904138001830f42408080b904086103f056600035601c52740100000000000000000000000000000000000000006020526f7fffffffffffffffffffffffffffffff6040527fffffffffffffffffffffffffffffffff8000000000000000000000000000000060605274012a05f1fffffffffffffffffffffffffdabf41c006080527ffffffffffffffffffffffffed5fa0e000000000000000000000000000000000060a05263538a3f0e60005114156100ed57602060046101403734156100b457600080fd5b60043560205181106100c557600080fd5b50600054156100d357600080fd5b60006101405114156100e457600080fd5b61014051600055005b631648f38e60005114156102bf576020600461014037341561010e57600080fd5b600435602051811061011f57600080fd5b50600061014051141561013157600080fd5b6000600054141561014157600080fd5b60026101405160e05260c052604060c020541561015d57600080fd5b7f602e600c600039602e6000f33660006000376110006000366000730000000000610180526c010000000000000000000000006000540261019b527f5af41558576110006000f30000000000000000000000000000000000000000006101af5260406101806000f0806101cf57600080fd5b61016052610160513b6101e157600080fd5b610160513014156101f157600080fd5b6000600060246366d3820361022052610140516102405261023c6000610160515af161021c57600080fd5b6101605160026101405160e05260c052604060c020556101405160036101605160e05260c052604060c02055600154600160015401101561025c57600080fd5b6001600154016102a0526102a0516001556101405160046102a05160e05260c052604060c0205561016051610140517f9d42cb017eb05bd8944ab536a8b35bc68085931dd5f4356489801453923953f960006000a36101605160005260206000f3005b6306f2bf62600051141561030e57602060046101403734156102e057600080fd5b60043560205181106102f157600080fd5b5060026101405160e05260c052604060c0205460005260206000f3005b6359770438600051141561035d576020600461014037341561032f57600080fd5b600435602051811061034057600080fd5b5060036101405160e05260c052604060c0205460005260206000f3005b63aa65a6c0600051141561039a576020600461014037341561037e57600080fd5b60046101405160e05260c052604060c0205460005260206000f3005b631c2bbd1860005114156103c05734156103b357600080fd5b60005460005260206000f3005b639f181b5e60005114156103e65734156103d957600080fd5b60015460005260206000f3005b60006000fd5b6100046103f0036100046000396100046103f0036000f3";
    let uniswap_factory_deploy_tx = parse_hex(&uniswap_factory_deploy_hex);
    let msg = ExecuteMsg::ExecuteRawEthereumTx { 
        caller_evm_address: depositor_addr.to_fixed_bytes(),
        unsigned_tx: uniswap_factory_deploy_tx
    };
    let res_factory = execute(deps.as_mut(), mock_env(), relayer.clone(), msg).unwrap();
    assert_eq!("0x8c306b6fbaf1fbe40163d9e1fbb13a9f4d45581f", res_factory.attributes[2].value);

    let uniswap_exchange_deploy_hex = "0xf930bf8001830f42408080b930b461309c56600035601c52740100000000000000000000000000000000000000006020526f7fffffffffffffffffffffffffffffff6040527fffffffffffffffffffffffffffffffff8000000000000000000000000000000060605274012a05f1fffffffffffffffffffffffffdabf41c006080527ffffffffffffffffffffffffed5fa0e000000000000000000000000000000000060a0526366d38203600051141561013b57602060046101403734156100b457600080fd5b60043560205181106100c557600080fd5b506000610140511415600654156007541516166100e157600080fd5b33600755610140516006557f556e6973776170205631000000000000000000000000000000000000000000006000557f554e492d563100000000000000000000000000000000000000000000000000006001556012600255005b63422f104360005114156105ab5760606004610140376000341160006101605111164261018051111661016d57600080fd5b6003546101a05260006101a051111561043e576000610140511161019057600080fd5b343031101561019e57600080fd5b343031036103a0526006543b6101b357600080fd5b6006543014156101c257600080fd5b602061046060246370a082316103e05230610400526103fc6006545afa6101e857600080fd5b600050610460516103c0526103a05161020057600080fd5b6103a05134151561021257600061022f565b6103c051346103c0513402041461022857600080fd5b6103c05134025b0460016103a05161023f57600080fd5b6103a05134151561025157600061026e565b6103c051346103c0513402041461026757600080fd5b6103c05134025b0401101561027b57600080fd5b60016103a05161028a57600080fd5b6103a05134151561029c5760006102b9565b6103c051346103c051340204146102b257600080fd5b6103c05134025b0401610480526103a0516102cc57600080fd5b6103a0513415156102de5760006102fb565b6101a051346101a051340204146102f457600080fd5b6101a05134025b046104a052610140516104a0511015610480516101605110151661031e57600080fd5b60043360e05260c052604060c02080546104a051825401101561034057600080fd5b6104a0518154018155506101a0516104a0516101a05101101561036257600080fd5b6104a0516101a051016003556006543b61037b57600080fd5b60065430141561038a57600080fd5b602061058060646323b872dd6104c052336104e052306105005261048051610520526104dc60006006545af16103bf57600080fd5b600050610580516103cf57600080fd5b6104805134337f06239653922ac7bea6aa2b19dc486b9361821d37712eb796adfd38d81de278ca60006000a46104a0516105a0523360007fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60206105a0a36104a05160005260206000f36105a9565b633b9aca003410156000600654141560006007541415161661045f57600080fd5b306007543b61046d57600080fd5b60075430141561047c57600080fd5b602061024060246306f2bf626101c0526006546101e0526101dc6007545afa6104a457600080fd5b60005061024051146104b557600080fd5b6101605161026052303161028052610280516003556102805160043360e05260c052604060c020556006543b6104ea57600080fd5b6006543014156104f957600080fd5b602061036060646323b872dd6102a052336102c052306102e05261026051610300526102bc60006006545af161052e57600080fd5b6000506103605161053e57600080fd5b6102605134337f06239653922ac7bea6aa2b19dc486b9361821d37712eb796adfd38d81de278ca60006000a461028051610380523360007fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef6020610380a36102805160005260206000f35b005b63f88bf15a600051141561084a57608060046101403734156105cc57600080fd5b600061018051116000610160511116426101a051116000610140511116166105f357600080fd5b6003546101c05260006101c0511161060a57600080fd5b6006543b61061757600080fd5b60065430141561062657600080fd5b602061028060246370a0823161020052306102205261021c6006545afa61064c57600080fd5b600050610280516101e0526101c05161066457600080fd5b6101c051610140511515610679576000610699565b30316101405130316101405102041461069157600080fd5b303161014051025b046102a0526101c0516106ab57600080fd5b6101c0516101405115156106c05760006106e6565b6101e051610140516101e051610140510204146106dc57600080fd5b6101e05161014051025b046102c052610180516102c0511015610160516102a05110151661070957600080fd5b60043360e05260c052604060c020610140518154101561072857600080fd5b61014051815403815550610140516101c051101561074557600080fd5b610140516101c0510360035560006000600060006102a051336000f161076a57600080fd5b6006543b61077757600080fd5b60065430141561078657600080fd5b6020610380604463a9059cbb6102e05233610300526102c051610320526102fc60006006545af16107b657600080fd5b600050610380516107c657600080fd5b6102c0516102a051337f0fbf06c058b90cb038a618f8c2acbf6145f8b3570fd1fa56abb8f0f3f05b36e860006000a4610140516103a0526000337fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60206103a0a360406103c0526103e06102a05181526102c0518160200152506103c0516103e0f3005b6000156109c6575b6101a05261014052610160526101805260006101805111600061016051111661087a57600080fd5b61014051151561088b5760006108ae565b6103e5610140516103e5610140510204146108a557600080fd5b6103e561014051025b6101c0526101c05115156108c35760006108e9565b610180516101c051610180516101c0510204146108df57600080fd5b610180516101c051025b6101e0526101605115156108fe576000610921565b6103e8610160516103e86101605102041461091857600080fd5b6103e861016051025b6101c051610160511515610936576000610959565b6103e8610160516103e86101605102041461095057600080fd5b6103e861016051025b01101561096557600080fd5b6101c05161016051151561097a57600061099d565b6103e8610160516103e86101605102041461099457600080fd5b6103e861016051025b0161020052610200516109af57600080fd5b610200516101e051046000526000516101a0515650005b600015610bf3575b6101a0526101405261016052610180526000610180511160006101605111166109f657600080fd5b610160511515610a07576000610a2d565b61014051610160516101405161016051020414610a2357600080fd5b6101405161016051025b1515610a3a576000610af6565b6103e8610160511515610a4e576000610a74565b61014051610160516101405161016051020414610a6a57600080fd5b6101405161016051025b6103e8610160511515610a88576000610aae565b61014051610160516101405161016051020414610aa457600080fd5b6101405161016051025b020414610aba57600080fd5b6103e8610160511515610ace576000610af4565b61014051610160516101405161016051020414610aea57600080fd5b6101405161016051025b025b6101c05261014051610180511015610b0d57600080fd5b6101405161018051031515610b23576000610b8e565b6103e561014051610180511015610b3957600080fd5b6101405161018051036103e561014051610180511015610b5857600080fd5b610140516101805103020414610b6d57600080fd5b6103e561014051610180511015610b8357600080fd5b610140516101805103025b6101e0526101e051610b9f57600080fd5b6101e0516101c0510460016101e051610bb757600080fd5b6101e0516101c05104011015610bcc57600080fd5b60016101e051610bdb57600080fd5b6101e0516101c05104016000526000516101a0515650005b600015610df4575b6101e0526101405261016052610180526101a0526101c0526000610160511160006101405111164261018051101516610c3357600080fd5b6006543b610c4057600080fd5b600654301415610c4f57600080fd5b60206102a060246370a0823161022052306102405261023c6006545afa610c7557600080fd5b6000506102a051610200526101405161016051610180516101a0516101c0516101e05161020051610220516102405161026051610280516102a0516102c0516389f2a8716102e05261014051610300526101405130311015610cd657600080fd5b6101405130310361032052610200516103405261034051610320516103005160065801610852565b6103a0526102c0526102a05261028052610260526102405261022052610200526101e0526101c0526101a0526101805261016052610140526103a0516102c052610160516102c0511015610d5157600080fd5b6006543b610d5e57600080fd5b600654301415610d6d57600080fd5b6020610460604463a9059cbb6103c0526101c0516103e0526102c051610400526103dc60006006545af1610da057600080fd5b60005061046051610db057600080fd5b6102c051610140516101a0517fcd60aa75dea3072fbc07ae6d7d856b5dc5f4eee88854f5b4abf7b680ef8bc50f60006000a46102c0516000526000516101e0515650005b63f39b5b9b6000511415610e715760406004610140376101405161016051638c717a3361018052346101a052610140516101c052610160516101e0523361020052336102205261022051610200516101e0516101c0516101a05160065801610bfb565b6102805261016052610140526102805160005260206000f3005b63ad65d76d6000511415610f245760606004610140376044356020518110610e9857600080fd5b5060006101805114153061018051141516610eb257600080fd5b610140516101605161018051638c717a336101a052346101c052610140516101e0526101605161020052336102205261018051610240526102405161022051610200516101e0516101c05160065801610bfb565b6102a0526101805261016052610140526102a05160005260206000f3005b60001561116c575b6101e0526101405261016052610180526101a0526101c0526000610160511160006101405111164261018051101516610f6457600080fd5b6006543b610f7157600080fd5b600654301415610f8057600080fd5b60206102a060246370a0823161022052306102405261023c6006545afa610fa657600080fd5b6000506102a051610200526101405161016051610180516101a0516101c0516101e05161020051610220516102405161026051610280516102a0516102c05163fd11c2236102e0526101405161030052610160513031101561100757600080fd5b61016051303103610320526102005161034052610340516103205161030051600658016109ce565b6103a0526102c0526102a05261028052610260526102405261022052610200526101e0526101c0526101a0526101805261016052610140526103a0516102c05260016102c051026103e0526103e05161016051101561108d57600080fd5b6103e05161016051036103c05260006103c05111156110c35760006000600060006103c0516101a0516000f16110c257600080fd5b5b6006543b6110d057600080fd5b6006543014156110df57600080fd5b60206104a0604463a9059cbb610400526101c05161042052610140516104405261041c60006006545af161111257600080fd5b6000506104a05161112257600080fd5b6101405160016102c051026101a0517fcd60aa75dea3072fbc07ae6d7d856b5dc5f4eee88854f5b4abf7b680ef8bc50f60006000a460016102c051026000526000516101e0515650005b636b1d4db760005114156111e95760406004610140376101405161016051632dff394e61018052610140516101a052346101c052610160516101e0523361020052336102205261022051610200516101e0516101c0516101a05160065801610f2c565b6102805261016052610140526102805160005260206000f3005b630b573638600051141561129c576060600461014037604435602051811061121057600080fd5b506000610180511415306101805114151661122a57600080fd5b610140516101605161018051632dff394e6101a052610140516101c052346101e0526101605161020052336102205261018051610240526102405161022051610200516101e0516101c05160065801610f2c565b6102a0526101805261016052610140526102a05160005260206000f3005b6000156114b3575b6101e0526101405261016052610180526101a0526101c05260006101605111600061014051111642610180511015166112dc57600080fd5b6006543b6112e957600080fd5b6006543014156112f857600080fd5b60206102a060246370a0823161022052306102405261023c6006545afa61131e57600080fd5b6000506102a051610200526101405161016051610180516101a0516101c0516101e05161020051610220516102405161026051610280516102a0516102c0516389f2a8716102e0526101405161030052610200516103205230316103405261034051610320516103005160065801610852565b6103a0526102c0526102a05261028052610260526102405261022052610200526101e0526101c0526101a0526101805261016052610140526103a0516102c05260016102c051026103c052610160516103c05110156113ef57600080fd5b60006000600060006103c0516101c0516000f161140b57600080fd5b6006543b61141857600080fd5b60065430141561142757600080fd5b60206104a060646323b872dd6103e0526101a05161040052306104205261014051610440526103fc60006006545af161145f57600080fd5b6000506104a05161146f57600080fd5b6103c051610140516101a0517f7f4091b46c33e918a0f3aa42307641d17bb67029427a5369e54b35398423870560006000a46103c0516000526000516101e0515650005b6395e3c50b600051141561154657606060046101403734156114d457600080fd5b61014051610160516101805163fa1bb7be6101a052610140516101c052610160516101e0526101805161020052336102205233610240526102405161022051610200516101e0516101c051600658016112a4565b6102a0526101805261016052610140526102a05160005260206000f3005b637237e031600051141561160f576080600461014037341561156757600080fd5b606435602051811061157857600080fd5b5060006101a0511415306101a05114151661159257600080fd5b6101405161016051610180516101a05163fa1bb7be6101c052610140516101e0526101605161020052610180516102205233610240526101a05161026052610260516102405161022051610200516101e051600658016112a4565b6102c0526101a0526101805261016052610140526102c05160005260206000f3005b600015611813575b6101e0526101405261016052610180526101a0526101c05260006101405111426101805110151661164757600080fd5b6006543b61165457600080fd5b60065430141561166357600080fd5b60206102a060246370a0823161022052306102405261023c6006545afa61168957600080fd5b6000506102a051610200526101405161016051610180516101a0516101c0516101e05161020051610220516102405161026051610280516102a0516102c05163fd11c2236102e05261014051610300526102005161032052303161034052610340516103205161030051600658016109ce565b6103a0526102c0526102a05261028052610260526102405261022052610200526101e0526101c0526101a0526101805261016052610140526103a0516102c0526102c05161016051101561174f57600080fd5b6000600060006000610140516101c0516000f161176b57600080fd5b6006543b61177857600080fd5b60065430141561178757600080fd5b602061048060646323b872dd6103c0526101a0516103e05230610400526102c051610420526103dc60006006545af16117bf57600080fd5b600050610480516117cf57600080fd5b610140516102c0516101a0517f7f4091b46c33e918a0f3aa42307641d17bb67029427a5369e54b35398423870560006000a46102c0516000526000516101e0515650005b63013efd8b60005114156118a6576060600461014037341561183457600080fd5b61014051610160516101805163984fe8f66101a052610140516101c052610160516101e0526101805161020052336102205233610240526102405161022051610200516101e0516101c05160065801611617565b6102a0526101805261016052610140526102a05160005260206000f3005b63d4e4841d600051141561196f57608060046101403734156118c757600080fd5b60643560205181106118d857600080fd5b5060006101a0511415306101a0511415166118f257600080fd5b6101405161016051610180516101a05163984fe8f66101c052610140516101e0526101605161020052610180516102205233610240526101a05161026052610260516102405161022051610200516101e05160065801611617565b6102c0526101a0526101805261016052610140526102c05160005260206000f3005b600015611c0a575b610220526101405261016052610180526101a0526101c0526101e0526102005260006101805111600061016051111660006101405111426101a051101516166119bf57600080fd5b600061020051141530610200511415166119d857600080fd5b6006543b6119e557600080fd5b6006543014156119f457600080fd5b60206102e060246370a0823161026052306102805261027c6006545afa611a1a57600080fd5b6000506102e051610240526101405161016051610180516101a0516101c0516101e05161020051610220516102405161026051610280516102a0516102c0516102e051610300516389f2a871610320526101405161034052610240516103605230316103805261038051610360516103405160065801610852565b6103e052610300526102e0526102c0526102a05261028052610260526102405261022052610200526101e0526101c0526101a0526101805261016052610140526103e05161030052600161030051026104005261018051610400511015611afb57600080fd5b6006543b611b0857600080fd5b600654301415611b1757600080fd5b60206104e060646323b872dd610420526101c051610440523061046052610140516104805261043c60006006545af1611b4f57600080fd5b6000506104e051611b5f57600080fd5b610200513b611b6d57600080fd5b61020051301415611b7d57600080fd5b60206105e0606463ad65d76d6105205261016051610540526101a051610560526101e0516105805261053c61040051610200515af1611bbb57600080fd5b6000506105e0516105005261040051610140516101c0517f7f4091b46c33e918a0f3aa42307641d17bb67029427a5369e54b35398423870560006000a461050051600052600051610220515650005b63ddf7e1a76000511415611d575760a06004610140373415611c2b57600080fd5b6084356020518110611c3c57600080fd5b506007543b611c4a57600080fd5b600754301415611c5957600080fd5b602061028060246306f2bf62610200526101c0516102205261021c6007545afa611c8257600080fd5b600050610280516101e0526101405161016051610180516101a0516101c0516101e051610200516102205161024051610260516102805163204ea33b6102a052610140516102c052610160516102e05261018051610300526101a05161032052336103405233610360526101e0516103805261038051610360516103405161032051610300516102e0516102c05160065801611977565b6103e05261028052610260526102405261022052610200526101e0526101c0526101a0526101805261016052610140526103e05160005260206000f3005b63f552d91b6000511415611ec15760c06004610140373415611d7857600080fd5b6084356020518110611d8957600080fd5b5060a4356020518110611d9b57600080fd5b506007543b611da957600080fd5b600754301415611db857600080fd5b60206102a060246306f2bf62610220526101e0516102405261023c6007545afa611de157600080fd5b6000506102a051610200526101405161016051610180516101a0516101c0516101e05161020051610220516102405161026051610280516102a05163204ea33b6102c052610140516102e052610160516103005261018051610320526101a0516103405233610360526101c05161038052610200516103a0526103a05161038051610360516103405161032051610300516102e05160065801611977565b610400526102a05261028052610260526102405261022052610200526101e0526101c0526101a0526101805261016052610140526104005160005260206000f3005b6000156121d7575b610220526101405261016052610180526101a0526101c0526101e05261020052600061018051116000610140511116426101a051101516611f0957600080fd5b60006102005114153061020051141516611f2257600080fd5b610200513b611f3057600080fd5b61020051301415611f4057600080fd5b60206102e060246359e9486261026052610140516102805261027c610200515afa611f6a57600080fd5b6000506102e051610240526006543b611f8257600080fd5b600654301415611f9157600080fd5b60206103a060246370a0823161032052306103405261033c6006545afa611fb757600080fd5b6000506103a051610300526101405161016051610180516101a0516101c0516101e05161020051610220516102405161026051610280516102a0516102c0516102e05161030051610320516103405161036051610380516103a0516103c05163fd11c2236103e05261024051610400526103005161042052303161044052610440516104205161040051600658016109ce565b6104a0526103c0526103a05261038052610360526103405261032052610300526102e0526102c0526102a05261028052610260526102405261022052610200526101e0526101c0526101a0526101805261016052610140526104a0516103c052610240516101805110156103c051610160511015166120c857600080fd5b6006543b6120d557600080fd5b6006543014156120e457600080fd5b602061058060646323b872dd6104c0526101c0516104e05230610500526103c051610520526104dc60006006545af161211c57600080fd5b6000506105805161212c57600080fd5b610200513b61213a57600080fd5b6102005130141561214a57600080fd5b60206106806064630b5736386105c052610140516105e0526101a051610600526101e051610620526105dc61024051610200515af161218857600080fd5b600050610680516105a052610240516103c0516101c0517f7f4091b46c33e918a0f3aa42307641d17bb67029427a5369e54b35398423870560006000a46103c051600052600051610220515650005b63b040d54560005114156123245760a060046101403734156121f857600080fd5b608435602051811061220957600080fd5b506007543b61221757600080fd5b60075430141561222657600080fd5b602061028060246306f2bf62610200526101c0516102205261021c6007545afa61224f57600080fd5b600050610280516101e0526101405161016051610180516101a0516101c0516101e0516102005161022051610240516102605161028051631a7b28f26102a052610140516102c052610160516102e05261018051610300526101a05161032052336103405233610360526101e0516103805261038051610360516103405161032051610300516102e0516102c05160065801611ec9565b6103e05261028052610260526102405261022052610200526101e0526101c0526101a0526101805261016052610140526103e05160005260206000f3005b63f3c0efe9600051141561248e5760c0600461014037341561234557600080fd5b608435602051811061235657600080fd5b5060a435602051811061236857600080fd5b506007543b61237657600080fd5b60075430141561238557600080fd5b60206102a060246306f2bf62610220526101e0516102405261023c6007545afa6123ae57600080fd5b6000506102a051610200526101405161016051610180516101a0516101c0516101e05161020051610220516102405161026051610280516102a051631a7b28f26102c052610140516102e052610160516103005261018051610320526101a0516103405233610360526101c05161038052610200516103a0526103a05161038051610360516103405161032051610300516102e05160065801611ec9565b610400526102a05261028052610260526102405261022052610200526101e0526101c0526101a0526101805261016052610140526104005160005260206000f3005b63b1cb43bf600051141561255b5760a060046101403734156124af57600080fd5b60843560205181106124c057600080fd5b506101405161016051610180516101a0516101c05163204ea33b6101e0526101405161020052610160516102205261018051610240526101a051610260523361028052336102a0526101c0516102c0526102c0516102a051610280516102605161024051610220516102005160065801611977565b610320526101c0526101a0526101805261016052610140526103205160005260206000f3005b63ec384a3e60005114156126555760c0600461014037341561257c57600080fd5b608435602051811061258d57600080fd5b5060a435602051811061259f57600080fd5b50306101c05114156125b057600080fd5b6101405161016051610180516101a0516101c0516101e05163204ea33b610200526101405161022052610160516102405261018051610260526101a05161028052336102a0526101c0516102c0526101e0516102e0526102e0516102c0516102a0516102805161026051610240516102205160065801611977565b610340526101e0526101c0526101a0526101805261016052610140526103405160005260206000f3005b63ea650c7d60005114156127225760a0600461014037341561267657600080fd5b608435602051811061268757600080fd5b506101405161016051610180516101a0516101c051631a7b28f26101e0526101405161020052610160516102205261018051610240526101a051610260523361028052336102a0526101c0516102c0526102c0516102a051610280516102605161024051610220516102005160065801611ec9565b610320526101c0526101a0526101805261016052610140526103205160005260206000f3005b63981a1327600051141561281c5760c0600461014037341561274357600080fd5b608435602051811061275457600080fd5b5060a435602051811061276657600080fd5b50306101c051141561277757600080fd5b6101405161016051610180516101a0516101c0516101e051631a7b28f2610200526101405161022052610160516102405261018051610260526101a05161028052336102a0526101c0516102c0526101e0516102e0526102e0516102c0516102a0516102805161026051610240516102205160065801611ec9565b610340526101e0526101c0526101a0526101805261016052610140526103405160005260206000f3005b63cd7724c36000511415612918576020600461014037341561283d57600080fd5b6000610140511161284d57600080fd5b6006543b61285a57600080fd5b60065430141561286957600080fd5b602061020060246370a0823161018052306101a05261019c6006545afa61288f57600080fd5b60005061020051610160526101405161016051610180516101a0516101c0516101e051610200516389f2a871610220526101405161024052303161026052610160516102805261028051610260516102405160065801610852565b6102e052610200526101e0526101c0526101a0526101805261016052610140526102e05160005260206000f3005b6359e948626000511415612a27576020600461014037341561293957600080fd5b6000610140511161294957600080fd5b6006543b61295657600080fd5b60065430141561296557600080fd5b602061020060246370a0823161018052306101a05261019c6006545afa61298b57600080fd5b60005061020051610160526101405161016051610180516101a0516101c0516101e051610200516102205163fd11c223610240526101405161026052303161028052610160516102a0526102a0516102805161026051600658016109ce565b6103005261022052610200526101e0526101c0526101a05261018052610160526101405261030051610220526001610220510260005260206000f3005b6395b68fe76000511415612b365760206004610140373415612a4857600080fd5b60006101405111612a5857600080fd5b6006543b612a6557600080fd5b600654301415612a7457600080fd5b602061020060246370a0823161018052306101a05261019c6006545afa612a9a57600080fd5b60005061020051610160526101405161016051610180516101a0516101c0516101e05161020051610220516389f2a871610240526101405161026052610160516102805230316102a0526102a051610280516102605160065801610852565b6103005261022052610200526101e0526101c0526101a05261018052610160526101405261030051610220526001610220510260005260206000f3005b632640f62c6000511415612c325760206004610140373415612b5757600080fd5b60006101405111612b6757600080fd5b6006543b612b7457600080fd5b600654301415612b8357600080fd5b602061020060246370a0823161018052306101a05261019c6006545afa612ba957600080fd5b60005061020051610160526101405161016051610180516101a0516101c0516101e0516102005163fd11c2236102205261014051610240526101605161026052303161028052610280516102605161024051600658016109ce565b6102e052610200526101e0526101c0526101a0526101805261016052610140526102e05160005260206000f3005b639d76ea586000511415612c58573415612c4b57600080fd5b60065460005260206000f3005b63966dae0e6000511415612c7e573415612c7157600080fd5b60075460005260206000f3005b6370a082316000511415612ccd5760206004610140373415612c9f57600080fd5b6004356020518110612cb057600080fd5b5060046101405160e05260c052604060c0205460005260206000f3005b63a9059cbb6000511415612d985760406004610140373415612cee57600080fd5b6004356020518110612cff57600080fd5b5060043360e05260c052604060c0206101605181541015612d1f57600080fd5b6101605181540381555060046101405160e05260c052604060c0208054610160518254011015612d4e57600080fd5b61016051815401815550610160516101805261014051337fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef6020610180a3600160005260206000f3005b6323b872dd6000511415612eb35760606004610140373415612db957600080fd5b6004356020518110612dca57600080fd5b506024356020518110612ddc57600080fd5b5060046101405160e05260c052604060c0206101805181541015612dff57600080fd5b6101805181540381555060046101605160e05260c052604060c0208054610180518254011015612e2e57600080fd5b6101805181540181555060056101405160e05260c052604060c0203360e05260c052604060c0206101805181541015612e6657600080fd5b61018051815403815550610180516101a05261016051610140517fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60206101a0a3600160005260206000f3005b63095ea7b36000511415612f485760406004610140373415612ed457600080fd5b6004356020518110612ee557600080fd5b506101605160053360e05260c052604060c0206101405160e05260c052604060c02055610160516101805261014051337f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b9256020610180a3600160005260206000f3005b63dd62ed3e6000511415612fb85760406004610140373415612f6957600080fd5b6004356020518110612f7a57600080fd5b506024356020518110612f8c57600080fd5b5060056101405160e05260c052604060c0206101605160e05260c052604060c0205460005260206000f3005b6306fdde036000511415612fde573415612fd157600080fd5b60005460005260206000f3005b6395d89b416000511415613004573415612ff757600080fd5b60015460005260206000f3005b63313ce567600051141561302a57341561301d57600080fd5b60025460005260206000f3005b6318160ddd600051141561305057341561304357600080fd5b60035460005260206000f3005b638c717a33610140523461016052600161018052426101a052336101c052336101e0526101e0516101c0516101a051610180516101605160065801610bfb565b610240526102405b61000461309c0361000460003961000461309c036000f3";
    let uniswap_exchange_deploy_tx = parse_hex(&uniswap_exchange_deploy_hex); 
    let msg = ExecuteMsg::ExecuteRawEthereumTx { 
        caller_evm_address: depositor_addr.to_fixed_bytes(),
        unsigned_tx: uniswap_exchange_deploy_tx
    };
    let res_exchange = execute(deps.as_mut(), mock_env(), relayer.clone(), msg).unwrap();
    assert_eq!("0x04a9bcdb32fec840042fdc389e6b4b16895b9465", res_exchange.attributes[2].value);

    
    // Uniswap contract addresses
    let uniswap_factory_addr: H160 = parse_h160(&res_factory.attributes[2].value);
    let uniswap_exchange_addr: H160 = parse_h160(&res_exchange.attributes[2].value);


    // Call initializeFactory of the uniswap factory contract, providing the exchange addr as the template addr
    let uniswap_initialize_factory_hex = "0xf8418001830f4240948c306b6fbaf1fbe40163d9e1fbb13a9f4d45581f80a4538a3f0e00000000000000000000000004a9bcdb32fec840042fdc389e6b4b16895b9465";
    let uniswap_initialize_factory_tx = parse_hex(&uniswap_initialize_factory_hex);
    let msg = ExecuteMsg::ExecuteRawEthereumTx { 
        caller_evm_address: depositor_addr.to_fixed_bytes(),
        unsigned_tx: uniswap_initialize_factory_tx
    };
    let res_initialize_factory = execute(deps.as_mut(), mock_env(), relayer.clone(), msg).unwrap();

    // Call createExchange of the uniswap factory contract, providing the NOVA token address
    let uniswap_create_exchange_hex = "0xf8418001830f4240948c306b6fbaf1fbe40163d9e1fbb13a9f4d45581f80a41648f38e000000000000000000000000ff3b783539a1a7a53ecacfb1c0778274c670f35b";
    let uniswap_create_exchange_tx = parse_hex(&uniswap_create_exchange_hex);
    let msg = ExecuteMsg::ExecuteRawEthereumTx { 
        caller_evm_address: depositor_addr.to_fixed_bytes(),
        unsigned_tx: uniswap_create_exchange_tx
    };
    let res_create_exchange = execute(deps.as_mut(), mock_env(), relayer.clone(), msg).unwrap();
    assert_eq!("0xbc5b88741392c16647e36b9a574050ed27996c17", "0x".to_string() + &res_create_exchange.attributes[3].value[24..]);

    // NOVA exchange address
    // 0xbc5b88741392c16647e36b9a574050ed27996c17
    let nova_exchange_address: H160 = parse_h160(&("0x".to_string() + &res_create_exchange.attributes[3].value[24..]));

    // Approve the NOVA exchange address to spend depositor's NOVA tokens
    let erc20_approve_hex = "0xf8628001830f424094ff3b783539a1a7a53ecacfb1c0778274c670f35b80b844095ea7b3000000000000000000000000bc5b88741392c16647e36b9a574050ed27996c1700ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
    let erc20_approve_tx = parse_hex(&erc20_approve_hex);
    let msg = ExecuteMsg::ExecuteRawEthereumTx { 
        caller_evm_address: depositor_addr.to_fixed_bytes(),
        unsigned_tx: erc20_approve_tx
    };
    let res_erc20_approve = execute(deps.as_mut(), mock_env(), relayer.clone(), msg).unwrap();
    // Assert that the function call returned true (succeeded)
    assert_eq!(U256::one(), U256::from_big_endian_fast(&parse_hex(&res_erc20_approve.attributes[3].value)));


    // // TEMP: try sending NOVA from depositor to swapper using the NOVA exchange address as the caller

    // let swapper_addr: H160 = parse_h160("0x2e36b2970ab7A4C955eADD836585c21A087Ab904");

    // let erc20_transfer_hex = "0xf8828001830f424094ff3b783539a1a7a53ecacfb1c0778274c670f35b80b86423b872dd000000000000000000000000b34e2213751c5d8e9a31355fca6f1b4fa5bb6be10000000000000000000000002e36b2970ab7a4c955eadd836585c21a087ab9040000000000000000000000000000000000000000000000000000000000000a00";
    // let erc20_transfer_tx = parse_hex(&erc20_transfer_hex);
    // let msg = ExecuteMsg::ExecuteRawEthereumTx { 
    //     caller_evm_address: nova_exchange_address.to_fixed_bytes(),
    //     unsigned_tx: erc20_transfer_tx
    // };
    // let res_erc20_transfer = execute(deps.as_mut(), mock_env(), relayer.clone(), msg).unwrap(); 
    // // println!("erc20 transfer res: {:?}", res_erc20_transfer);
    
    // // Check new balance of receiver
    // let trx_hex = "0xf84180018398968094ff3b783539a1a7a53ecacfb1c0778274c670f35b80a470a082310000000000000000000000002e36b2970ab7a4c955eadd836585c21a087ab904";
    // let trx = parse_hex(&trx_hex);
    // let msg = QueryMsg::RawEthereumQuery {
    //     caller_evm_address: swapper_addr.to_fixed_bytes(),
    //     unsigned_tx: trx
    // };

    // let res: RawEthereumQueryResponse = from_binary(&query(deps.as_ref(), mock_env(), msg).unwrap()).unwrap();
    // println!("Balance result: {:?}", res);

    // Deposit some NOVA and ETH from depositor address into the NOVA exchaange
    let nova_exchange_deposit_hex = "0xf8878001830f424094bc5b88741392c16647e36b9a574050ed27996c1785012a05f200b864422f10430000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002386f26fc1000000000000000000000000000000000000000000000000000000000000627aee3c";
    let nova_exchange_deposit_tx = parse_hex(&nova_exchange_deposit_hex);
    let msg = ExecuteMsg::ExecuteRawEthereumTx { 
        caller_evm_address: depositor_addr.to_fixed_bytes(),
        unsigned_tx: nova_exchange_deposit_tx
    };
    let res_nova_exchange_deposit = execute(deps.as_mut(), mock_env(), relayer.clone(), msg).unwrap();
    // println!("Deposit res: {:?}", res_nova_exchange_deposit);

    // Check new balance of NOVA exchange
    let trx_hex = "0xf84180018398968094ff3b783539a1a7a53ecacfb1c0778274c670f35b80a470a08231000000000000000000000000bc5b88741392c16647e36b9a574050ed27996c17";
    let trx = parse_hex(&trx_hex);
    let msg = QueryMsg::RawEthereumQuery {
        caller_evm_address: nova_exchange_address.to_fixed_bytes(),
        unsigned_tx: trx
    };

    let res: RawEthereumQueryResponse = from_binary(&query(deps.as_ref(), mock_env(), msg).unwrap()).unwrap();
    println!("NOVA balance of NOVA exchange after deposit: {:?}", res);

    let msg = QueryMsg::QueryEvmAccount { evm_address: nova_exchange_address.to_fixed_bytes() };
    let res: EvmAccountResponse = from_binary(&query(deps.as_ref(), mock_env(), msg).unwrap()).unwrap();
    println!("ETH balance of NOVA exchange after deposit: {:?}", res);

    
}