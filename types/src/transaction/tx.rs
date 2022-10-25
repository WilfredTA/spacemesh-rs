use crate::bytes::Bytes;
use parity_scale_codec::{Decode, Encode};

use crate::{TxId, hashes::{Hash32, hash_256}, block::BlockId, LayerId, address::Address};

use super::TxHeader;
use chrono::{Utc, DateTime};
use sha2::Sha256;
#[derive(PartialEq, Encode, Decode)]
#[repr(u32)]
pub enum TxState {
    Pending = 0,
    Mempool,
    Proposal,
    Block,
    Applied,
    Discarded
}
#[derive(PartialEq, Encode, Decode)]
pub struct RawTx {
    id: TxId,
    raw: Bytes
}
#[derive(PartialEq, Encode, Decode)]
pub struct Transaction {
    tx: RawTx,
    header: TxHeader,
}

#[derive(PartialEq)]
struct DateTimeInner(DateTime<Utc>);

impl From<String> for DateTimeInner {
    fn from(s: String) -> Self {
        Self(DateTime::parse_from_rfc3339(s.as_str()).expect("Invalid datetime string").into())
    }
}


#[derive(PartialEq,Decode)]
pub struct MeshTransaction {
    tx: Transaction,
    layer: LayerId,
    block: BlockId,
    state: TxState,
    #[codec(encoded_as = "String")]
    received: DateTimeInner,
}


#[derive(PartialEq, Encode, Decode)]
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