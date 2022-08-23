use bytes::{Bytes, BytesMut, Buf};
use super::bytes::{Bytes12, Bytes20, Bytes32};

pub struct Hash12(Bytes12);

pub struct Hash32(Bytes32);

pub struct Hash20(Bytes20);

pub struct Hash {
    size: usize,
    inner_data: Bytes
}

impl Hash {
    pub fn as_hash12(&self) -> Hash12 {
        todo!()
    }

    pub fn as_hash20(&self) -> Hash20 {
        todo!()
    }

    pub fn as_hash32(&self) -> Hash32 {
        todo!()
    }
}

pub trait Hashable {
    fn hash(&self) -> Hash;
}
