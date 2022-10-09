use bytes::Bytes;


pub struct PoetProofRef(pub Bytes);

pub struct PoetProof {
    pub merkle: MerkleProof,
    pub members: Vec<Bytes>,
    pub leaf_count: u64,
}

pub struct PoetProofMsg {
    pub poet_proof: PoetProof,
    pub poet_service_id: Bytes,
    pub round_id: String,
    pub sig: Bytes,
}

pub struct PoetRound {
    pub id: String
}
pub struct MerkleProof {
    pub(crate) root: Bytes,
    pub(crate) proven_leaves: Vec<Bytes>,
    pub(crate) proof_nodes: Vec<Bytes>,
}