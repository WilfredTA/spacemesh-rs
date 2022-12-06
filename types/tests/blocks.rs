use spacemesh_types::{block::*, bytes::Bytes as SmeshBytes, hashes::*, transaction::*, LayerId};

use hex::{decode, encode, FromHex, ToHex};
use parity_scale_codec::{Decode, Encode};
mod common;
use common::*;

#[test]
fn test_certify_msg() {
    let layer_id = LayerId(11);
    let block_id = random_block_id();
    let msg_content = CertifyContent {
        layerid: layer_id,
        blockid: block_id.into(),
        eligibility_count: 2,
        proof: "not a fraud".as_bytes().into(),
    };
    let signer = TestSigner::new();
    let sig = signer.sign(msg_content.encode().as_ref());

    let msg = CertifyMsg {
        content: msg_content,
        sig: sig.to_bytes().into(),
    };
    // encode then decode msg
    // assert that decoded == original
    // extract pubkey from signature
    // assert extracted pubkey matches signer's
}

#[test]
fn test_blockids_to_hashes() {}

#[test]
fn test_new_existing_block() {}

#[test]
fn test_block_init() {}

#[test]
fn test_block_bytes() {}

#[test]
fn test_block_field_string() {}

#[test]
fn test_block_id_cmp() {}

#[test]
fn test_reward_codec() {}
