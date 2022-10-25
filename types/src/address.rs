use crate::{bytes::Bytes, hashes::{Hashable, hash_256}};
use parity_scale_codec::{Decode, Encode};
// TO DO: Impl Default, Format, Debug
#[derive(PartialEq, Encode, Decode)]
pub struct Address {
    len: u32,
    pub content: Bytes
}


impl TryFrom<Bytes> for Address {
    type Error = parity_scale_codec::Error;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        Address::decode(&mut value.as_ref())
    }
}

impl From<Address> for Vec<u8> {
    fn from(a: Address) -> Vec<u8> {
        a.encode()
    }
}

