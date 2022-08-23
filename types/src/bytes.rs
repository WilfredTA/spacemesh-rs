use bytes::Buf;
use thiserror::Error;
use std::io::prelude::*;
use std::fmt::Error as FmtError;
use parity_scale_codec::Error as ScaleError;
use parity_scale_codec::{Decode, Encode};
use super::hashes::{Hash, Hashable};
#[derive(Debug, Error)]
pub enum BytesError {
    #[error(transparent)]
    Codec(#[from] ScaleError),
}

type Bytes64Result<T> = Result<T, BytesError>;

#[derive(Encode, Decode)]
pub struct Bytes64([u8; 64]);

#[derive(Encode, Decode)]
pub struct Bytes12([u8; 12]);

#[derive(Encode, Decode)]
pub struct Bytes20([u8; 20]);

#[derive(Encode, Decode)]
pub struct Bytes32([u8; 32]);


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