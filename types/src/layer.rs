use crate::{LayerId, hashes::Hash32, ballot::Ballot, block::Block};


pub struct Layer {
    idx: LayerId,
    hash: Hash32,
    ballots: Vec<Ballot>,
    blocks: Vec<Block>
}

impl Layer {
    pub fn new(idx: LayerId) -> Self {
        todo!()
    }
}