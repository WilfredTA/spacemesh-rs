use bytes::Bytes;

use crate::{address::Address, AtxId, hashes::Hash32, NodeId};

use super::post::*;


pub struct Atx {
    inner: InnerATX,
    sig: Bytes,
}


pub struct InnerATX {
    nipost_challenge: (),
    coinbase: Address,
    num_units: u32,
    nipost: NiPost,
    initial_post: Post,
    id: AtxId,
    node: NodeId,
}

