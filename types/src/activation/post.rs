use crate::bytes::Bytes;
use parity_scale_codec::{Decode, Encode};

use crate::hashes::Hash32;
#[derive(PartialEq, Encode, Decode)]
pub struct Post {
    nonce: u32,
    indices: Vec<u8>,
}
#[derive(PartialEq, Encode, Decode)]
pub struct PostMetadata {
    challenge: Bytes,
    bits_per_lbl: u8,
    lbls_per_unit: u64,
    k1: u32,
    k2: u32,
}
#[derive(PartialEq, Encode, Decode)]
pub struct NiPost {
    challenge: Hash32,
    post: Post,
    metadata: PostMetadata,
}
