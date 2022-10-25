use parity_scale_codec::{Decode, Encode};

use crate::bytes::Bytes;
use super::LayerId;
use super::address::Address;

#[derive(Encode, Decode)]
pub struct Account {
    pub layer: LayerId,
    pub address: Address,
    pub initialized: bool,
    pub next_nonce: u64,
    pub balance: u64,
    pub template: Address,
    pub state: Bytes,
}

impl From<Account> for Vec<u8> {
    fn from(a: Account) -> Self {
        a.encode()
    }
}