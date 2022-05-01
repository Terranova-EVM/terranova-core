use std::convert::TryInto;

use cw_storage_plus::{Map};
use evm::{H160, U256, H256};

use crate::account::{EvmAccount, EvmContract};
use crate::storage::{CwStorageInterface, StorageInterface};

/// TODO: Implement PrimaryKey for evm::H160 so that it can be used directly as the key instead of having to convert it
/// 
/// A component of the underlying backend to the persistent state accessible through CwStorageInterface
/// 
/// Key: an H160 address in the form of a byte array slice \ 
/// Value: an EvmAccount struct, see its documentation
pub const ACCOUNTS: Map<&[u8], EvmAccount> = Map::new("accounts");

/// TODO: Implement PrimaryKey for evm::H160 so that it can be used directly as the key instead of having to convert it
///  
/// A component of the underlying backend to the persistent state accessible through CwStorageInterface
///
/// This map should not be accessed unless you first use the ACCOUNTS map to verify that the provided H160 address 
/// is a contract account and thus is known to have an associated entry here. However, please still implement clean 
/// Result and Option handling when using it. <-- actually fk this nvm just directly keying in and just handling 
/// the None value if it doesn't exist makes more sense
/// 
/// Key: an H160 address in the form of a byte array slice \ 
/// Value: an EvmContract struct, see its documentation
pub const CONTRACTS: Map<&[u8], EvmContract> = Map::new("contracts");

impl StorageInterface for CwStorageInterface<'_> {
    fn token_mint(&self) -> &cosmwasm_std::Addr {
        &self.token_mint
    }

    fn block_number(&self) -> evm::U256 {
        self.cw_env.block.height.into()
    }

    /// Timestamp of the current block in seconds since the epoch.
    ///
    /// Converted from the Cosmwasm block time, which is in nanoseconds by default
    fn block_timestamp(&self) -> evm::U256 {
        self.cw_env.block.time.seconds().into()
    }

    /// TODO: Come back to this
    /// Cosmwasm currently does not directly expose the block hash through Env
    /// What to do here? Create own hash using block number + block time or something?
    /// Need to find out the specific purpose/uses of this in EVM smart contracts, and what the requirements of this hash are
    /// Does it only need to be unique? Or do we need other properties...?
    fn block_hash(&self, number: evm::U256) -> evm::H256 {
        todo!()
    }

    /// TODO: Come back to this
    /// Would it be better to use some hash of the cw_env chain_id, which is a String,
    /// or would it be better to just use arbitrary chain_id numbers as in Neon?
    fn chain_id(&self) -> u64 {
        todo!()
    }

    fn exists(&self, address: &H160) -> bool {
        ACCOUNTS.has(self.cw_deps.storage, address.as_bytes())
    }

    fn nonce(&self, address: &H160) -> evm::U256 {
        ACCOUNTS
            .may_load(self.cw_deps.storage, address.as_bytes())
            .unwrap_or(None)
            .map_or(0_u64, |acc| acc.trx_count)
            .into()
    }

    fn balance(&self, address: &H160) -> U256 {
        ACCOUNTS
            .may_load(self.cw_deps.storage, address.as_bytes())
            .unwrap_or(None)
            .map_or_else(U256::zero, |acc| acc.balance)
    }

    /// Possible performance consideration for this and code_hash/code/valids:
    /// Is it better to check that this is a contract first in ACCOUNTS, or just use may_load
    /// This method can panic, but only because of architecture related reasons, should (hopefully) be fine since
    /// this is compiled to cosmwasm anyways.
    fn code_size(&self, address: &H160) -> usize {
        CONTRACTS
            .may_load(self.cw_deps.storage, address.as_bytes())
            .unwrap_or(None)
            .map_or(0_u32, |contract| contract.code_size)
            .try_into()
            .expect("usize is 8 bytes")
    }

    /// THIS CURRENTLY USES A DUMMY HASHING FUNCTION, IT'S NOT REAL
    /// TODO: Implement the hashing shit in utils.rs
    /// TODO: Reimplement this when EvmContract.code type is changed to RefMut<[u8]>
    fn code_hash(&self, address: &H160) -> H256 {
        CONTRACTS
            .may_load(self.cw_deps.storage, address.as_bytes())
            .unwrap_or(None)
            .map(|contract| contract.code)
            .map_or_else(H256::zero, |code| {
                crate::utils::keccak256_h256(code.as_slice())
            })
    }

    fn code(&self, address: &H160) -> Vec<u8> {
        CONTRACTS
            .may_load(self.cw_deps.storage, address.as_bytes())
            .unwrap_or(None)
            .map(|contract| contract.code)
            .map_or_else(Vec::new, |code| code)
    }

    /// TODO: Reimplement this when EvmContract.valids type is changed to RefMut<[u8]>
    fn valids(&self, address: &H160) -> Vec<u8> {
        CONTRACTS
            .may_load(self.cw_deps.storage, address.as_bytes())
            .unwrap_or(None)
            .map_or_else(Vec::new, |contract| contract.valids)
    }

    /// TODO: Reimplement this if/when contract.storage is refactored as a different data structure
    fn storage(&self, address: &H160, index: &U256) -> U256 {
        CONTRACTS
            .may_load(self.cw_deps.storage, address.as_bytes())
            .unwrap_or(None)
            .map(|contract| contract.storage)
            .and_then(|storage| {
                storage.get(index).map(|value| *value)
            })
            .unwrap_or_else(U256::zero)
    }
}