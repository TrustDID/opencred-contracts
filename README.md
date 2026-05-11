# opencred-contracts

Rust/Soroban smart contracts for the OpenCred decentralized credential registry on Stellar.

## What is OpenCred?

OpenCred enables trustless, on-chain credential management:

- **Issuance** — authorized issuers register credential hashes on-chain
- **Verification** — anyone can verify a credential's authenticity and status
- **Revocation** — issuers can permanently revoke credentials

Credential documents are stored on IPFS. The blockchain stores only hashes, issuer/holder addresses, timestamps, and revocation status.

## Repository Structure

```text
contracts/
└── credential_registry/   # Core registry contract (scaffold)
    ├── src/lib.rs
    ├── Cargo.toml
    ├── README.md
    └── test/
Cargo.toml                 # Workspace root
```

## Stack

- [Soroban](https://soroban.stellar.org/) — Stellar smart contract platform
- Rust (edition 2021)
- Stellar SDK

## Local Development

### Prerequisites

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked stellar-cli
```

### Build

```bash
cargo build --target wasm32-unknown-unknown --release
```

### Test

```bash
cargo test
```

## Repository Philosophy

- One contract per directory under `contracts/`
- No business logic merged until it has passing tests
- All on-chain storage decisions documented in the contract's `lib.rs`
- IPFS integration is off-chain; contracts only store hashes
