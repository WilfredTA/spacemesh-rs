use bytes::{Bytes, BytesMut, Buf};
use super::bytes::{Bytes12, Bytes20, Bytes32};
use sha2::{Sha256, Digest};

pub fn hash_256(data: impl AsRef<[u8]>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data.as_ref());
    hasher.finalize().to_vec()
}
pub struct Hash12(Bytes12);

pub struct Hash32(Bytes32);

impl<A> From<A> for Hash32 
where A: Into<Bytes32>
{
    fn from(bytes: A) -> Self {
        Self(bytes.into())
    }
}

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
