use super::bytes::{Bytes12, Bytes20, Bytes32};
use bytes::{Buf, Bytes, BytesMut};
use parity_scale_codec::{Decode, Encode};
use sha2::{Digest, Sha256};

pub fn hash_256(data: impl AsRef<[u8]>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data.as_ref());
    hasher.finalize().to_vec()
}
#[derive(PartialEq, Encode, Decode)]
pub struct Hash12(pub(crate) Bytes12);

#[derive(PartialEq, Encode, Decode)]
pub struct Hash32(pub(crate) Bytes32);

impl<A> From<A> for Hash32
where
    A: Into<Bytes32>,
{
    fn from(bytes: A) -> Self {
        Self(bytes.into())
    }
}
#[derive(PartialEq, Encode, Decode, Default, Clone)]
pub struct Hash20(pub Bytes20);

pub struct Hash {
    size: usize,
    inner_data: Bytes,
}

impl From<Bytes> for Hash {
    fn from(b: Bytes) -> Self {
        Self {
            size: b.len(),
            inner_data: b,
        }
    }
}

impl From<Hash12> for Hash {
    fn from(h: Hash12) -> Self {
        let mut inner_data = vec![];
        h.0.encode_to(&mut inner_data);
        Self::new(inner_data, 12)
    }
}

impl From<Hash20> for Hash {
    fn from(h: Hash20) -> Self {
        let mut inner_data = vec![];
        h.0.encode_to(&mut inner_data);
        Self::new(inner_data, 20)
    }
}

impl From<Hash32> for Hash {
    fn from(h: Hash32) -> Self {
        let mut inner_data = vec![];
        h.0.encode_to(&mut inner_data);
        Self::new(inner_data, 32)
    }
}
impl Hash {
    pub fn hash(data: impl AsRef<[u8]>) -> Self {
        let hashed = hash_256(data);
        Self {
            size: hashed.len(),
            inner_data: hashed.into(),
        }
    }
    pub(crate) fn new(bytes: impl Into<Bytes>, sz: usize) -> Self {
        Self {
            size: sz,
            inner_data: bytes.into(),
        }
    }

    pub fn as_hash12(&self) -> Hash12 {
        // Size should always be >= 12
        let offset = self.size - 12;
        let mut bytes = [0u8; 12];
        let sliced = &self.inner_data[offset..(offset + 12)];
        bytes.copy_from_slice(sliced);
        Hash12(bytes.into())
    }

    pub fn as_hash20(&self) -> Hash20 {
        let offset = self.size - 20;
        let mut bytes = [0u8; 20];
        let sliced = &self.inner_data[offset..(offset + 20)];
        bytes.copy_from_slice(sliced);
        Hash20(bytes.into())
    }

    pub fn as_hash32(&self) -> Hash32 {
        let offset = self.size - 32;
        let mut bytes = [0u8; 32];
        let sliced = &self.inner_data[offset..(offset + 32)];
        bytes.copy_from_slice(sliced);
        Hash32(bytes.into())
    }
}

pub trait Hashable {
    fn hash(&self) -> Hash;
}
