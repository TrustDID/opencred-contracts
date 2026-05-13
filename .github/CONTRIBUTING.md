# Contributing to OpenCred Contracts

## Project Overview

OpenCred is a decentralized credential registry built on Stellar using Soroban smart contracts. Credential documents live on IPFS; the blockchain stores only hashes, issuer/holder addresses, timestamps, and revocation status.

## Local Setup

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

## Soroban Tooling

- Use `stellar-cli` for contract deployment and invocation.
- Contracts compile to `wasm32-unknown-unknown`. Always build with `--release` for deployment.
- Keep each contract in its own directory under `contracts/`.

## Coding Standards

- Follow standard Rust formatting: `cargo fmt` before committing.
- Run `cargo clippy -- -D warnings` and fix all warnings.
- No `unwrap()` in contract code — use proper error types.
- Document public functions with `///` doc comments.
- All on-chain storage decisions must be explained in the contract's `lib.rs`.

## Branch Naming

| Type | Pattern | Example |
|------|---------|---------|
| Feature | `feat/<short-description>` | `feat/revocation-logic` |
| Bug fix | `fix/<short-description>` | `fix/hash-collision` |
| Docs | `docs/<short-description>` | `docs/architecture` |
| Chore | `chore/<short-description>` | `chore/update-deps` |

Always branch from `main`.

## PR Expectations

- Reference the issue your PR closes: `Closes #<number>`.
- Keep PRs focused — one concern per PR.
- Fill out the pull request template completely.
- All CI checks must pass before requesting review.
- At least one approving review is required to merge.

## Testing Expectations

- No business logic merged without passing tests.
- Add integration tests under `contracts/<name>/test/`.
- Test both happy paths and error/edge cases.
- Run `cargo test` locally before pushing.
