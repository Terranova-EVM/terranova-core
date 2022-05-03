/// These are all dummy implementations right now, implement later

//use sha3::Keccak256;
//use tiny_keccak::some_shit

use evm::{U256, H256};

/// Get Keccak256 hash as `H256`
#[must_use]
pub fn keccak256_h256(data: &[u8]) -> H256 {
    H256::zero()
}

/// Get Keccak256 hash as `H256` from several slices
#[must_use]
pub fn keccak256_h256_v(data: &[&[u8]]) -> H256 {
    H256::zero()
}

/// Get Keccak256 hash as Vec<u8>
#[must_use]
pub fn keccak256_digest(data: &[u8]) -> Vec<u8> {
    vec![]
}

/// Convert U256 to H256
#[must_use]
pub fn u256_to_h256(value: U256) -> H256 {
    H256::zero()
}
