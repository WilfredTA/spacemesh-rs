extern crate ed25519_dalek;
extern crate rand;


use rand::{prelude::*, rngs::OsRng};
use spacemesh_types::{address::Address, BlockId, proposal::ProposalId, bytes::Bytes20, hashes::Hash20};
use rand::rngs::ThreadRng;
use rand::CryptoRng;
use rand::rngs::mock::StepRng;

use ed25519_dalek::{Signature, Signer, Keypair, Verifier, PublicKey};


pub struct TestSigner {
    keypair: Keypair
}

impl TestSigner {
    pub fn new() -> Self {
        Self {
            keypair: gen_keypair()
        }
    }

    pub fn sign(&self, msg: &[u8]) -> Signature {
        sign_msg(msg, &self.keypair)
    }

    pub fn verify(&self, msg: &[u8], sig: &Signature) -> bool {
        verify_msg(msg, &self.keypair.public, sig)
    }
}


pub fn gen_keypair() -> Keypair {
    let mut rng = OsRng{};
    Keypair::generate(&mut rng)

}

pub fn sign_msg(msg: &[u8], keypair: &Keypair) -> Signature {
   keypair.sign(msg)
}

pub fn verify_msg(msg: &[u8], pubkey: &PublicKey, sig: &Signature) -> bool {
    pubkey.verify(msg, sig).is_ok()
}

pub fn random_block_id() -> Hash20 {
    let random_bytes: [u8; 20] = random();
    Hash20(Bytes20::from(random_bytes))
}