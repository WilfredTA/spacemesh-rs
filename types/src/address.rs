use crate::bytes::Bytes;
use parity_scale_codec::{Decode, Encode};
// TO DO: Impl Default, Format, Debug
#[derive(PartialEq, Encode, Decode)]
pub struct Address {
    len: u32,
    pub content: Bytes
}

