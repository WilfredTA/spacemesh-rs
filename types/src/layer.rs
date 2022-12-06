use parity_scale_codec::{Decode, Encode};

use crate::{ballot::Ballot, block::Block, hashes::Hash32, LayerId};

#[derive(PartialEq, Encode, Decode)]
pub struct Layer {
    idx: LayerId,
    hash: Hash32,
    ballots: Vec<Ballot>,
    blocks: Vec<Block>,
}

impl Layer {
    pub fn new(idx: LayerId) -> Self {
        todo!()
    }
}
