use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    ExecuteRawEthereumTx {
        /// H160 address in the form of a byte array
        caller_evm_address: [u8; 20],

        /// THe RLP encoded unsigned transaction message
        unsigned_tx: Vec<u8>,
    },

    StoreTxChunk {
        /// H160 address in the form of a byte array
        caller_evm_address: [u8; 20],

        /// keccak_h256 hash of the full unsigned transaction as as byte array
        full_tx_hash: [u8; 32],

        chunk_index: u8,

        chunk_data: Vec<u8>,
    },

    ExecuteChunkedEthereumTx {
        caller_evm_address: [u8; 20],

        full_tx_hash: [u8; 32],

        chunk_count: u8,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
}

pub mod execute_simple_transaction;
pub mod store_transaction_chunk;
pub mod execute_chunked_transaction;
