use tiny_keccak::{keccak256};

use evm::{U256, H256};

/// Get Keccak256 hash as `H256`
#[must_use]
pub fn keccak256_h256(data: &[u8]) -> H256 {
    H256::from_slice(&keccak256(data))
}

/// Get Keccak256 hash as `H256` from several slices/ 
/// This function is only required for V2 address derivation/ 
/// Not needed by Terranova (for now) since our EVM is using the Legacy schema for address derivation
#[must_use]
pub fn keccak256_h256_v(data: &[&[u8]]) -> H256 {
    unimplemented!();
}

/// Get Keccak256 hash as Vec<u8>
#[must_use]
pub fn keccak256_digest(data: &[u8]) -> Vec<u8> {
    keccak256(data).to_vec()
}

/// Convert U256 to H256
#[must_use]
pub fn u256_to_h256(value: U256) -> H256 {
    let mut v = vec![0_u8; 32];
    value.to_big_endian(&mut v);
    H256::from_slice(&v)
}
