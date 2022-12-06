use parity_scale_codec::{Decode, Encode};

use super::address::Address;
use super::LayerId;
use crate::bytes::Bytes;

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
