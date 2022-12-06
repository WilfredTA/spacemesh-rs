use parity_scale_codec::{Decode, Encode};

use crate::{address::Address, block::BlockId, LayerId};

#[derive(PartialEq, Encode, Decode)]
pub struct TxResult {
    pub status: u8,
    pub msg: String,
    pub gas: u64,
    pub fee: u64,
    pub block: BlockId,
    pub layer: LayerId,
    pub addresses: Vec<Address>,
}
