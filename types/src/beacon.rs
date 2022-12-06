use parity_scale_codec::{Decode, Encode};

#[derive(PartialEq, Encode, Decode)]
pub struct Beacon(Vec<u32>);
