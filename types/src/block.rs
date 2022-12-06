use parity_scale_codec::{Decode, Encode};

use crate::{
    address::Address,
    bytes::Bytes,
    hashes::{Hash, Hash20, Hash32},
    LayerId,
};

pub type RatNum = u64;

pub type BlockId = super::hashes::Hash20;
#[derive(PartialEq, Encode, Decode)]
pub struct Block {
    id: BlockId,
    inner: InnerBlock,
}
impl Block {
    pub fn init(&mut self) {
        let self_bytes = self.inner.encode();
        let hashed = Hash::hash(self_bytes);
        let self_hash = hashed.as_hash20();
        self.id = self_hash;
    }
}

#[derive(PartialEq, Encode, Decode)]
pub struct AnyReward {
    coinbase: Address,
    weight: RatNum,
}
#[derive(PartialEq, Encode, Decode)]
struct InnerBlock {
    layer_idx: LayerId,
    tick_height: u64,
    rewards: Vec<AnyReward>,
    tx_ids: Vec<Hash32>,
}
#[derive(PartialEq, Encode, Decode)]
pub struct Certificate {
    id: BlockId,
    sigs: Vec<CertifyMsg>,
}
#[derive(PartialEq, Encode, Decode)]
pub struct CertifyMsg {
    pub content: CertifyContent,
    pub sig: Bytes,
}
#[derive(PartialEq, Encode, Decode)]
pub struct CertifyContent {
    pub layerid: LayerId,
    pub blockid: BlockId,
    pub eligibility_count: u16,
    pub proof: Bytes,
}
