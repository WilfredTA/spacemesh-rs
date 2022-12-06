use crate::bytes::Bytes;
use parity_scale_codec::{Decode, Encode};

use crate::{address::Address, hashes::Hash32, AtxId, NodeId};

use super::post::*;

#[derive(PartialEq, Encode, Decode)]
pub struct Atx {
    inner: InnerATX,
    sig: Bytes,
}

#[derive(PartialEq, Encode, Decode)]
pub struct InnerATX {
    nipost_challenge: (),
    coinbase: Address,
    num_units: u32,
    nipost: NiPost,
    initial_post: Post,
    id: AtxId,
    node: NodeId,
}
