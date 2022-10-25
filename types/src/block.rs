use parity_scale_codec::{Encode, Decode};

use crate::{LayerId, address::Address, hashes::Hash32};

pub type RatNum = u64;

pub type BlockId = super::hashes::Hash20;
#[derive(PartialEq, Encode, Decode)]
pub struct Block {
    id: BlockId,
    inner: InnerBlock,
}

#[derive(PartialEq, Encode, Decode)]
pub struct AnyReward {
    coinbase: Address,
    weight: RatNum
}
#[derive(PartialEq, Encode, Decode)]
struct InnerBlock {
    layer_idx: LayerId,
    tick_height: u64,
    rewards: Vec<AnyReward>,
    tx_ids: Vec<Hash32>,
}