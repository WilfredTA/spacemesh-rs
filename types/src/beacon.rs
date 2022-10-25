use parity_scale_codec::{Encode, Decode};

#[derive(PartialEq, Encode, Decode)]
pub struct Beacon(Vec<u32>);