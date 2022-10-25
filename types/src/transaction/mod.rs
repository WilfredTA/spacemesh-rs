mod tx_header;
mod tx_result;
mod tx;
pub use tx_header::*;
pub use tx_result::*;
pub use tx::*;
use parity_scale_codec::{Decode, Encode};
#[derive(PartialEq, Encode, Decode)]
pub struct Nonce {
    pub counter: u64,
    pub bitfield: u8,
}
#[derive(PartialEq, Encode, Decode)]
pub struct LayerLimits {
    min: u32,
    max: u32
}