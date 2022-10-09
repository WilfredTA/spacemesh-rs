use bytes::Bytes;

use crate::{BallotId, PubKey, AtxId, BlockId, LayerId, beacon::Beacon};


pub struct Ballot {
    id: BallotId,
    inner: InnerBallot,
    sig: Bytes,
    smesher_id: PubKey,
    malicious: bool,
}

pub struct InnerBallot {
    atx_id: AtxId,
    eligibility_proofs: Vec<VotingEligibilityProof>,
    votes: Votes,
    ref_ballot: BallotId,
    epoch_data: EpochData,
    layer_idx: LayerId,
}

pub struct VotingEligibilityProof {
    j: u32,
    sig: Bytes,
}

pub struct Votes {
    base: BallotId,
    support: Vec<BlockId>,
    against: Vec<BlockId>,
    abstain: Vec<LayerId>,
}

pub struct EpochData {
    active: Vec<AtxId>,
    beacon: Beacon,
}