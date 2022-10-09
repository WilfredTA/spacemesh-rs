use bytes::Bytes;

use crate::hashes::Hash32;

pub struct Post {
    nonce: u32,
    indices: Vec<u8>,
}

pub struct PostMetadata {
    challenge: Bytes,
    bits_per_lbl: u8,
    lbls_per_unit: u64,
    k1: u32,
    k2: u32,
}

pub struct NiPost {
    challenge: Hash32,
    post: Post,
    metadata: PostMetadata,
}
