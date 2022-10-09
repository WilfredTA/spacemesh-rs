#![allow(unused)]
pub mod transaction;
pub mod account;
pub mod activation;
pub mod ballot;
pub mod block;
pub mod layer;
pub mod proposal;
pub mod hashes;
pub mod bytes;
pub mod address;
pub mod beacon;
pub mod db;
pub type PubKey = u8;
pub struct EpochId(pub u32);
pub struct BlockId(pub hashes::Hash20);
pub struct LayerId(pub u32);
pub struct TxId(pub hashes::Hash32);
pub struct AtxId(pub hashes::Hash32);

pub struct NodeId(pub bytes::Bytes32);

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
