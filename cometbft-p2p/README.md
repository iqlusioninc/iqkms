# `cometbft-p2p`

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![MSRV][rustc-image]

Pure Rust implementation of the Secret Connection transport encryption protocol used by
[CometBFT] for peer-to-peer (P2P) connections.

## Features

- Synchronous `SecretConnection` implementation for `std::io` (e.g. `TcpStream`)
- `AsyncSecretConnection` implementation based on `tokio`
- Protobuf `Message`-oriented interface which abstracts away low-level buffering/framing
- Support for splitting connections into separate readers/writers that can be used concurrently
- Self-contained with no dependencies on legacy `tendermint-rs`

## Compatibility

Tested in production with CometBFT v0.37, v0.38, and v1.0.

Should also be compatible with legacy Tendermint versions v0.34+, as well as earlier versions of
CometBFT.

## Cryptographic algorithms

The Secret Connection protocol uses the following cryptographic algorithms:

- Identity: Ed25519
- Key exchange: X25519
- Packet encryption: ChaCha20Poly1305

## License

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

<https://www.apache.org/licenses/LICENSE-2.0>

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/cometbft-p2p.svg?logo=rust
[crate-link]: https://crates.io/crates/cometbft-p2p
[docs-image]: https://docs.rs/cometbft-p2p/badge.svg
[docs-link]: https://docs.rs/cometbft-p2p/
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusioninc/iqkms/blob/main/cometbft-p2p/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-1.85+-blue.svg

[//]: # (links)

[CometBFT]: https://cometbft.com/
