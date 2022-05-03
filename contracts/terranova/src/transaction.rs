use evm::{H160, U256};

use crate::ContractError;

#[derive(Debug)]
pub struct UnsignedTransaction {
    pub nonce: u64,
    pub gas_price: U256,
    pub gas_limit: U256,
    pub to: Option<H160>,
    pub value: U256,
    pub call_data: Vec<u8>,
    pub chain_id: Option<U256>,
    pub rlp_len: usize,
}

impl UnsignedTransaction {
    pub fn from_rlp(unsigned_msg: &[u8]) -> Result<Self, ContractError> {
        let trx = rlp::decode(unsigned_msg)
            .map_err(|e| E!(ContractError::InvalidTransactionData; "RLP DecoderError={}", e))?;

        Ok(trx)
    }
}

impl rlp::Decodable for UnsignedTransaction {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        let field_count = rlp.item_count()?;
        match field_count {
            6 | 9 => (),
            _ => return Err(rlp::DecoderError::RlpIncorrectListLen),
        }

        let info = rlp.payload_info()?;
        let payload_size = info.header_len + info.value_len;

        let tx = Self {
            nonce: rlp.val_at(0)?,
            gas_price: rlp.val_at(1)?,
            gas_limit: rlp.val_at(2)?,
            to: {
                let to = rlp.at(3)?;
                if to.is_empty() {
                    if to.is_data() {
                        None
                    } else {
                        return Err(rlp::DecoderError::RlpExpectedToBeData);
                    }
                } else {
                    Some(to.as_val()?)
                }
            },
            value: rlp.val_at(4)?,
            call_data: rlp.val_at(5)?,
            chain_id: if field_count == 6 {
                None
            } else {
                // Although v size is not limited by the specification, we don't expect it
                // to be higher, so make the code simpler:
                Some(rlp.val_at(6)?)
            },
            rlp_len: payload_size,
        };

        Ok(tx)
    }
}
