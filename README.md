# ![iqkms](https://raw.githubusercontent.com/iqlusioninc/iqkms/main/.img/iqkms.svg)

Cryptographic key management service providing a gRPC API and support
for a variety of key storage methods including [YubiHSM2] devices.

## Status

iqkms is currently in an early stage of development and is not ready to use.

Please check back later.

## Features

- [ ] Cosmos
  - [ ] Cosmos SDK-compatible transaction signatures
- [ ] Ethereum
  - [x] EIP-155 signatures
  - [x] EIP-712 signatures
  - [ ] Transaction signing
- [ ] Tendermint
  - [ ] Consensus signatures (i.e. "privval" support)

## Relationship to Tendermint KMS (a.k.a. tmkms)

Tendermint KMS is another KMS service developed by iqlusion, aimed specifically
at Tendermint applications.

We recommend all current users of Tendermint KMS continue to do so. While we
aim for *iqkms* to eventually be able to function as a gRPC-native alternative
to Tendermint KMS, it will be quite some time before it has the necessary
functionality to do so.

We have no plans to retire Tendermint KMS and will continue supporting it
indefinitely.

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

## Contributing

Please open or discuss on an issue to discuss any potential changes you'd like
to make prior to opening a PR.

Please read [CODE_OF_CONDUCT.md] and [CONTRIBUTING.md] for more information.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.

[//]: # (links)

[YubiHSM2]: https://developers.yubico.com/YubiHSM2/
[CODE_OF_CONDUCT.md]: https://github.com/iqlusioninc/iqkms/blob/main/CODE_OF_CONDUCT.md
[CONTRIBUTING.md]: https://github.com/iqlusioninc/iqkms/blob/main/CONTRIBUTING.md
