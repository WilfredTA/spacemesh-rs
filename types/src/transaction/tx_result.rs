use crate::{block::BlockId, LayerId, address::Address};



pub struct TxResult {
    pub status: u8,
    pub msg: String,
    pub gas: u64,
    pub fee: u64,
    pub block: BlockId,
    pub layer: LayerId,
    pub addresses: Vec<Address>,
}