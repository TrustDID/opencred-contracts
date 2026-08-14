# credential_registry

Soroban smart contract that maintains the on-chain registry of verifiable credentials for the OpenCred system.

## Purpose

This contract is the trust anchor for OpenCred. It records:

| Field | Description |
|---|---|
| `hash` | Content hash (e.g. SHA-256 or IPFS CID) of the credential document |
| `issuer` | Stellar address of the entity that issued the credential |
| `holder` | Stellar address of the credential subject |
| `issued_at` | Ledger timestamp at issuance |
| `revoked` | Boolean revocation flag |

The full credential document (JSON-LD / W3C VC) lives on IPFS. Only its hash is stored on-chain, keeping costs low while preserving verifiability.

## Functions

| Function | Description |
|---|---|
| `register_credential(hash, holder)` | Register a new credential; the caller is recorded as the issuer |
| `get_credential(hash)` | Return the credential record, or `None` if not found |
| `revoke_credential(hash)` | Mark a credential as revoked; only the original issuer may do so |

Authorization uses `Address::require_auth`, so every mutation requires the caller's signature. Revocation is permanent.

## Status

**Implemented** — the credential lifecycle (register / get / revoke) is complete and covered by the integration tests in `test/integration_tests.rs`.

## Development

```bash
# Build
cargo build --package credential_registry --target wasm32-unknown-unknown --release

# Test
cargo test --package credential_registry

# Lint
cargo clippy --all-targets --all-features -- -D warnings
```
