use bytes::Buf;
use thiserror::Error;
use std::io::prelude::*;
use std::fmt::Error as FmtError;
use parity_scale_codec::{Error as ScaleError, WrapperTypeDecode, CompactAs};
use parity_scale_codec::{Decode, Encode};
use super::hashes::{Hash, Hashable};

#[derive(PartialEq, Clone, Encode, Decode)]
pub struct Bytes(Vec<u8>);

impl<T> From<T> for Bytes
where
    T: Into<Vec<u8>>
{
    fn from(o: T) -> Self {
        Self(o.into())
    }
}
// pub struct Bytes {
//     #[codec(encode_as = "")]
//     inner: bytes::Bytes
// }
#[derive(Debug, Error)]
pub enum BytesError {
    #[error(transparent)]
    Codec(#[from] ScaleError),
}

type Bytes64Result<T> = Result<T, BytesError>;

#[derive(PartialEq, Encode, Decode)]
pub struct Bytes64([u8; 64]);

#[derive(PartialEq, Encode, Decode)]
pub struct Bytes12([u8; 12]);

#[derive(PartialEq, Encode, Decode, Default)]
pub struct Bytes20([u8; 20]);

#[derive(PartialEq, Encode, Decode)]
pub struct Bytes32([u8; 32]);


impl From<Vec<u8>> for Bytes32 {
    fn from(bytes: Vec<u8>) -> Self {
        let mut buf = [0u8; 32];
        buf.copy_from_slice(bytes.as_slice());
        Self(buf)
    }
}
impl Hashable for Bytes64 {
    fn hash(&self) -> Hash {
        todo!()
    }
}

impl From<Bytes12> for Bytes64 {
    fn from(_: Bytes12) -> Self {
        todo!()
    }
}

impl From<Bytes20> for Bytes64 {
    fn from(_: Bytes20) -> Self {
        todo!()
    }
} 

impl From<Bytes32> for Bytes64 {
    fn from(_: Bytes32) -> Self {
        todo!()
    }
}


impl From<Bytes64> for Bytes32 {
    fn from(_: Bytes64) -> Self {
        todo!()
    }
}

impl From<Bytes64> for Bytes20 {
    fn from(_: Bytes64) -> Self {
        todo!()
    }
}

impl From<Bytes64> for Bytes12 {
    fn from(_: Bytes64) -> Self {
        todo!()
    }
}