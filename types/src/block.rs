use crate::{LayerId, address::Address, hashes::Hash32};

pub type RatNum = u64;

pub type BlockId = super::hashes::Hash20;
pub struct Block {
    id: BlockId,
    inner: InnerBlock,
}


pub struct AnyReward {
    coinbase: Address,
    weight: RatNum
}
struct InnerBlock {
    layer_idx: LayerId,
    tick_height: u64,
    rewards: Vec<AnyReward>,
    tx_ids: Vec<Hash32>,
}