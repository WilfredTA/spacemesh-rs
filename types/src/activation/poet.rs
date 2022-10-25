use crate::bytes::Bytes;
use parity_scale_codec::{Encode, Decode};

#[derive(PartialEq, Encode, Decode)]
pub struct PoetProofRef(pub Bytes);
#[derive(PartialEq, Encode, Decode)]
pub struct PoetProof {
    pub merkle: MerkleProof,
    pub members: Vec<Bytes>,
    pub leaf_count: u64,
}
#[derive(PartialEq, Encode, Decode)]
pub struct PoetProofMsg {
    pub poet_proof: PoetProof,
    pub poet_service_id: Bytes,
    pub round_id: String,
    pub sig: Bytes,
}
#[derive(PartialEq, Encode, Decode)]
pub struct PoetRound {
    pub id: String
}
#[derive(PartialEq, Encode, Decode)]
pub struct MerkleProof {
    pub(crate) root: Bytes,
    pub(crate) proven_leaves: Vec<Bytes>,
    pub(crate) proof_nodes: Vec<Bytes>,
}