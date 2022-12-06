mod tx;
mod tx_header;
mod tx_result;
use parity_scale_codec::{Decode, Encode};
pub use tx::*;
pub use tx_header::*;
pub use tx_result::*;
#[derive(PartialEq, Encode, Decode)]
pub struct Nonce {
    pub counter: u64,
    pub bitfield: u8,
}
#[derive(PartialEq, Encode, Decode)]
pub struct LayerLimits {
    min: u32,
    max: u32,
}
