use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateAccount {
        // The operator's cosmwasm Addr
        operator: String,

        // The serialized H160 address of the account to be created
        eth_address: [u8; 20],

        // The contract bytecode, empty if this is a user account
        contract_byte_code: Vec<u8>,
    },

    CallFromRawEthereumTX {
        
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
}

pub mod account_create;
pub mod transaction_execute_from_instruction;
