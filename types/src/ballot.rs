use crate::{beacon::Beacon, AtxId, BallotId, LayerId, PubKey};
use crate::{
    block::{Block, BlockId},
    bytes::Bytes,
};
use parity_scale_codec::{Decode, Encode};

#[derive(PartialEq, Encode, Decode)]
pub struct Ballot {
    id: BallotId,
    inner: InnerBallot,
    sig: Bytes,
    smesher_id: PubKey,
    malicious: bool,
}
#[derive(PartialEq, Encode, Decode)]
pub struct InnerBallot {
    atx_id: AtxId,
    eligibility_proofs: Vec<VotingEligibilityProof>,
    votes: Votes,
    ref_ballot: BallotId,
    epoch_data: EpochData,
    layer_idx: LayerId,
}
#[derive(PartialEq, Encode, Decode)]
pub struct VotingEligibilityProof {
    j: u32,
    sig: Bytes,
}
#[derive(PartialEq, Encode, Decode)]
pub struct Votes {
    base: BallotId,
    support: Vec<BlockId>,
    against: Vec<BlockId>,
    abstain: Vec<LayerId>,
}
#[derive(PartialEq, Encode, Decode)]
pub struct EpochData {
    active: Vec<AtxId>,
    beacon: Beacon,
}

#[derive(PartialEq, Encode, Decode)]
pub struct Vote {
    id: BlockId,
    layer_id: LayerId,
    height: u64,
}

impl From<Block> for Vote {
    fn from(b: Block) -> Vote {
        todo!()
    }
}
