use bytes::Bytes;

use crate::{TxId, hashes::{Hash32, hash_256}, block::BlockId, LayerId, address::Address};

use super::TxHeader;
use chrono::{Utc, DateTime};
use sha2::Sha256;
#[repr(u32)]
pub enum TxState {
    Pending = 0,
    Mempool,
    Proposal,
    Block,
    Applied,
    Discarded
}

pub struct RawTx {
    id: TxId,
    raw: Bytes
}

pub struct Transaction {
    tx: RawTx,
    header: TxHeader,
}


pub struct MeshTransaction {
    tx: Transaction,
    layer: LayerId,
    block: BlockId,
    state: TxState,
    received: DateTime<Utc>,
}


pub struct Reward {
    layer_id: LayerId,
    total: u64,
    layer: u64,
    coinbase: Address,
}


impl RawTx {
    pub fn new(bytes: impl AsRef<[u8]>) -> Self {
        let raw = bytes.as_ref().to_vec();
        let id = hash_256(&raw);
        Self {
            id: TxId(id.into()),
            raw: raw.into()
        }
    }
}