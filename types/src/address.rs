use crate::{
    bytes::Bytes,
    hashes::{hash_256, Hashable},
};
use bech32::{decode, encode, u5, Error as Bech32Error};
use hex::encode as hexcode;
use parity_scale_codec::{Decode, Encode};
use std::fmt::Display;
use thiserror::Error;
pub const ADDR_LENGTH: usize = 24;
pub const ADDR_RESERVED_SPACE: usize = 4;
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AddressError {
    #[error(transparent)]
    Bech32Error(#[from] bech32::Error),
    #[error("Incorrect HRP: expected {0} but got {1}")]
    WrongNetwork(String, String),
    #[error("First {0} bytes in address are reserved & should be 0.\nAddress: {1}")]
    UseOfReservedAddressSpace(usize, String),
}

pub struct Hrp;

impl Hrp {
    pub fn testnet() -> String {
        "stest".to_string()
    }

    #[cfg(feature = "testnet")]
    pub fn default() -> String {
        Hrp::testnet()
    }

    #[cfg(not(feature = "testnet"))]
    pub fn default() -> String {
        "sm".to_string()
    }
}
pub type AddressResult<T> = Result<T, AddressError>;

// TO DO: Impl Default, Format, Debug
#[derive(PartialEq, Encode, Decode, Debug)]
pub struct Address {
    len: u32,
    pub content: Bytes,
    hrp: String,
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
        let (hrp, data, variant) = bech32::decode(s)?;

        if hrp != Hrp::default() {
            Err(AddressError::WrongNetwork(hrp, Hrp::default()))
        } else {
            let data = data.into_iter().map(|d| d.to_u8()).collect::<Vec<_>>();
            for i in 0..ADDR_RESERVED_SPACE {
                if *data.get(i).unwrap() != 0 {
                    return Err(AddressError::UseOfReservedAddressSpace(
                        ADDR_RESERVED_SPACE,
                        bech32::encode(
                            hrp.as_str(),
                            data.iter()
                                .map(|d| u5::try_from_u8(*d).unwrap())
                                .collect::<Vec<_>>(),
                            bech32::Variant::Bech32,
                        )?,
                    ));
                }
            }
            Ok(Address {
                hrp,
                content: data.into(),
                len: ADDR_LENGTH as u32,
            })
        }
    }

    // from common/types/address/GenerateAddress in go-spacemesh
    pub fn from_pubkey(pubkey: &[u8]) -> Self {
        let mut addr: [u8; 24] = [0u8; 24];
        let addr_bytes = {
            if pubkey.len() > ADDR_LENGTH - ADDR_RESERVED_SPACE {
                &pubkey[pubkey.len() - (ADDR_LENGTH - ADDR_RESERVED_SPACE)..]
            } else {
                pubkey
            }
        };
        let mut addr_slice = &mut addr[ADDR_RESERVED_SPACE..];
        addr_slice.copy_from_slice(addr_bytes);
        Self {
            len: ADDR_LENGTH as u32,
            content: addr.into(),
            hrp: Hrp::default(),
        }
    }

    pub fn hrp_network(&self) -> String {
        self.hrp.clone()
    }
}

#[cfg(test)]
mod address_test {
    use super::*;
}
