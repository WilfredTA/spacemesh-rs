use spacemesh_types::{
    bytes::{Bytes as SmeshBytes, Bytes12, Bytes20, Bytes32, BytesError},
    hashes::{hash_256, Hash, Hash12, Hash20, Hash32, Hashable},
    address::{Address, AddressError, AddressResult, ADDR_RESERVED_SPACE}
};

mod common;
mod test_data;
use test_data::addresses::*;
use common::TestSigner;
#[test]
fn test_new_addr_correct() {
    let addr_string = "stest1qqqqqqy0dd83jemjmfj3ghjndxm0ndh0z2rymaqyp0gu3";
    let addr_result = Address::from_string(addr_string);
    eprintln!("{:#?}", addr_result);
    assert!(addr_result.is_ok())
}

#[test]
fn test_fails_when_addr_space_is_reserved() {
    let addr_string = "stest1fejq2x3d79ukpkw06t7h6lndjuwzxdnj59npghsg43mh4";
    let addr_result = Address::from_string(addr_string);
    eprintln!("{:#?}", addr_result);
    assert!(addr_result.is_err());
    assert_eq!(addr_result.unwrap_err(), AddressError::UseOfReservedAddressSpace(ADDR_RESERVED_SPACE, "stest1fejq2x3d79ukpkw06t7h6lndjuwzxdnj59npghsg43mh4".to_string()));
   
}

#[test]
fn test_fails_when_incorrect_charset() {
    let addr_string = "stest1fejq2x3d79ukpkw06t7h6lniobuwzxdnj59nphsywyusj";
    let addr_result = Address::from_string(addr_string);
    eprintln!("{:#?}", addr_result);
    assert_eq!(addr_result.unwrap_err(), AddressError::Bech32Error(bech32::Error::InvalidChar('i')))
}


#[test]
fn test_fails_when_unsupported_network() {
    let addr_string = "sut1fejq2x3d79ukpkw06t7h6lndjuwzxdnj59npghsldfkky";
    let addr_result = Address::from_string(addr_string);
    eprintln!("{:#?}", addr_result);
    assert_eq!(addr_result.unwrap_err(), AddressError::WrongNetwork("sut".to_string(), "stest".to_string()));
}


