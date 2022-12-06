# Spacemesh-rs

This repository contains a collection of crates related to the Spacemesh protocol.

Below is a list & description of the crates:
- `./types`
    - **Crate name:** `spacemesh-types`
    - **Usage:** `spacemesh-types = {git = "https://github.com/spacemeshos/spacemesh-rs"}`
    - **Description**: This crate provides type definitions for types in the core Spacemesh protocol alongside their serialization/deserialization to scale-codec & JSON.


When running tests, use feature flag `testnet` so the corresponding address tests utilize the expected human readable part: `cargo test --features testnet`.