use std::fmt::Display;
use thiserror::Error;
use crate::{bytes::Bytes, hashes::{Hashable, hash_256}};
use parity_scale_codec::{Decode, Encode};
use bech32::{Error as Bech32Error, encode, decode};

#[derive(Debug, Error)]
pub enum AddressError {
    #[error(transparent)]
    Bech32Error(#[from] bech32::Error),
}

pub type AddressResult<T> = Result<T, AddressError>;

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

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}


impl Address {
    // from common/types/address/StringToAddress in go-spacemesh
    pub fn from_string(s: impl AsRef<str>) -> AddressResult<Address> {
        let s = s.as_ref();
        let r = bech32::decode(s)?;
        todo!()
    }

    pub fn new(pubkey: &[u8]) -> Self {
        todo!()
    }

    pub fn hrp_network(&self) -> String {
        todo!()
    }
}



#[cfg(test)]
mod address_test {
    use super::*;
    
}