//! Ethereum integration tests.
//!
//! Requires running `iqkms` instance. Use `cargo test -- --ignored` to run.
// TODO(tarcieri): start iqkms for proper integration test.

#![cfg(feature = "ethereum")]

use iqkms::{
    ethereum::{Address, SignerClient, H256},
    StdError,
};

#[ignore]
#[tokio::test]
async fn sign_digest_with_eip155() -> Result<(), StdError> {
    let mut client = SignerClient::connect("http://[::1]:27100").await?;

    // TODO(tarcieri): real signing key address
    let address = "0x27b1fdb04752bbc536007a920d24acb045561c26"
        .parse::<Address>()
        .unwrap();

    let chain_id = 2018;
    let digest = H256::from([
        0x6f, 0xd4, 0x3e, 0x7c, 0xff, 0xc3, 0x1b, 0xb5, 0x81, 0xd7, 0x42, 0x1c, 0x86, 0x98, 0xe2,
        0x9a, 0xa2, 0xbd, 0x8e, 0x71, 0x86, 0xa3, 0x94, 0xb8, 0x52, 0x99, 0x90, 0x8b, 0x4e, 0xb9,
        0xb1, 0x75,
    ]);

    let response = client
        .sign_digest_with_eip155(address, digest, chain_id)
        .await
        .unwrap();

    println!("RESPONSE={:?}", response);
    Ok(())
}
