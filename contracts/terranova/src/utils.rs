use tiny_keccak::{keccak256};

use evm::{U256, H256, H160};

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

/// Testing util
pub fn parse_hex(hex_asm: &str) -> Vec<u8> {
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

/// Testing util
pub fn parse_h160(address_str: &str) -> H160 {
    let vec_u8 = parse_hex(address_str);
    let mut array_u8 = [0_u8; 20];
    for (i, n_u8) in vec_u8.iter().enumerate() {
        array_u8[i] = *n_u8;
    }

    H160(array_u8)
}
