use bytes::Bytes;
use super::LayerId;
use super::address::Address;

pub struct Account {
    pub layer: LayerId,
    pub address: Address,
    pub initialized: bool,
    pub next_nonce: u64,
    pub balance: u64,
    pub template: Address,
    pub state: Bytes,
}