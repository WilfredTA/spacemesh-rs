use parity_scale_codec::{Decode, Encode};

use crate::{
    ballot::Ballot,
    bytes::Bytes,
    hashes::{hash_256, Hash20, Hash32},
    TxId,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProposalError {
    #[error("Proposal already initialized")]
    ProposalAlreadyInit,
}

pub type ProposalResult<T> = Result<T, ProposalError>;

#[derive(Default, Clone, Encode, Decode)]
pub struct ProposalId(pub Hash20);

impl From<ProposalId> for Bytes {
    fn from(id: ProposalId) -> Self {
        todo!()
    }
}

impl ProposalId {
    pub fn as_hash_32(&self) -> Hash32 {
        todo!()
    }

    pub fn as_hash_20(&self) -> Hash20 {
        todo!()
    }
}

pub struct Proposal {
    inner: InnerProposal,
    sig: Bytes,
    id: ProposalId,
}

pub struct InnerProposal {
    ballot: Ballot,
    tx_ids: Vec<TxId>,
    mesh_hash: Hash32,
}

impl Proposal {
    pub fn init(&mut self) -> ProposalResult<Proposal> {
        todo!()
    }

    pub fn id(&self) -> ProposalId {
        todo!()
    }

    pub fn set_id(&mut self, id: impl Into<ProposalId>) {}
}
