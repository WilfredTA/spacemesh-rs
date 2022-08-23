#![allow(unused)]
pub mod transaction;
pub mod account;
pub mod activation;
pub mod ballot;
pub mod block;
pub mod layer;
pub mod proposal;
pub mod hashes;
pub mod bytes;

pub trait Hex {
    fn to_hex(&self) -> String;

    fn from_hex(data: impl AsRef<u8>) -> Self;
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
