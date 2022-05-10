use super::*;
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, from_binary, Order, Uint256};
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
    let msg = ExecuteMsg::ExecuteRawEthereumTx { 
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
    assert_eq!("0xff3b783539a1a7a53ecacfb1c0778274c670f35b", res.attributes[1].value);

    let contract_address = parse_h160(&res.attributes[1].value);

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
