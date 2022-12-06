#![allow(unused)]
use parity_scale_codec::{Decode, Encode};
pub mod account;
pub mod activation;
pub mod address;
pub mod ballot;
pub mod beacon;
pub mod block;
pub mod bytes;
pub mod db;
pub mod hashes;
pub mod layer;
pub mod proposal;
pub mod transaction;
pub type PubKey = u8;
#[derive(PartialEq, Encode, Decode)]
pub struct EpochId(pub u32);
#[derive(PartialEq, Encode, Decode)]
pub struct BlockId(pub hashes::Hash20);

#[derive(PartialEq, Encode, Decode)]
pub struct LayerId(pub u32);
#[derive(PartialEq, Encode, Decode)]
pub struct TxId(pub hashes::Hash32);
#[derive(PartialEq, Encode, Decode)]
pub struct AtxId(pub hashes::Hash32);
#[derive(PartialEq, Encode, Decode)]
pub struct NodeId(pub bytes::Bytes32);
#[derive(PartialEq, Encode, Decode)]
pub struct BallotId(pub hashes::Hash20);
pub trait Hex {
    fn to_hex(&self) -> String;

    fn from_hex(data: impl AsRef<u8>) -> Self;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
