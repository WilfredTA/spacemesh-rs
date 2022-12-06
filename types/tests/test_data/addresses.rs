pub const TEST_ADDRESSES: [(&str, &str); 7] = [
    (
        "stest1qqqqqqy0dd83jemjmfj3ghjndxm0ndh0z2rymaqyp0gu3",
        "correct",
    ),
    (
        "stest1fejq2x3d79ukpkw06t7h6lndjuwzxdnj59npghsg43mh4",
        "correct address, but no reserved space",
    ),
    (
        "stest1fejq2x3d79ukpkw06t7h6lniobuwzxdnj59nphsywyusj",
        "incorrect charset",
    ),
    (
        "1fejq2x3d79ukpkw06t7h6lniobuwzxdnj59nphsywyusj",
        "missing hrp",
    ),
    ("stest1qw508d6e1qejxtdg4y5r3zarvax8wucu", "too long"),
    ("stest1qw504y5r3zarva2v6vda", "too short"),
    (
        "sut1fejq2x3d79ukpkw06t7h6lndjuwzxdnj59npghsldfkky",
        "wrong network (hrp)",
    ),
];
