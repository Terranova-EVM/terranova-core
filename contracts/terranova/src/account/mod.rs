use evm::{H160, U256};
use serde::{Serialize, Deserialize};

/// TODO: Document this
#[derive(Debug, Serialize, Deserialize)]
pub struct EvmAccount {
    /// Ethereum address
    /// Bytes of an evm::H160
    pub address: H160,

    pub bump_seed: u8,

    /// Ethereum account nonce
    pub trx_count: u64,

    /// TODO: Document this, explain why a contract account's H160 address can be used in 
    /// our wasm implementation of EVM as the key to its storage, rather than how it's done in Ethereum
    /// Option of bytes of an evm::H160
    pub contract_storage_key: Option<H160>,

    /// EVM native balance 
    /// Little-endian bytes of an evm::U256
    pub balance: U256,

    /// Read-write lock
    pub rw_blocked: bool,

    /// Read-only lock counter
    pub ro_blocked_count: u8,
}

impl EvmAccount {
    /// Create an initialiized user account with the given address
    /// NOTE: This is also used for instantiating contract accounts, even though the initial field for contract_storage_key is None
    /// On contract deployments if this account exists as a user account the contract_storage_key should be changed 
    pub fn new_user_account(address: &H160) -> Self {
        Self {
            // address: *address.as_fixed_bytes(),
            address: *address,
            bump_seed: 0_u8,
            trx_count: 0_u64,
            contract_storage_key: None,
            balance: U256::zero(),
            rw_blocked: false,
            ro_blocked_count: 0_u8,
        }
    }
}

/// For every EvmAccount struct that is a contract account, there will be a corresponding EvmContract
/// stored in the CONTRACTS map, with the same key, the account's H160 address, used. The H160 address is stored
/// as Some(H160_address) in the contract_storage_key field of EvmAccount to make this relationship clear.
#[derive(Debug, Serialize, Deserialize)]
pub struct EvmContract {
    /// Contract code size
    pub code_size: u32,

    pub code: Vec<u8>,

    pub valids: Vec<u8>,

    // Storage has its own dedicated cw-storage-plus Map with (H160, U256) key values
}
