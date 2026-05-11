# credential_registry

Soroban smart contract that maintains the on-chain registry of verifiable credentials for the OpenCred system.

## Purpose

This contract is the trust anchor for OpenCred. It records:

| Field | Description |
|---|---|
| `credential_hash` | SHA-256 (or similar) hash of the credential document stored on IPFS |
| `issuer` | Stellar address of the entity that issued the credential |
| `holder` | Stellar address of the credential subject |
| `timestamp` | Ledger timestamp at issuance |
| `revoked` | Boolean revocation flag |

The full credential document (JSON-LD / W3C VC) lives on IPFS. Only its hash is stored on-chain, keeping costs low while preserving verifiability.

## Planned Functions

| Function | Description |
|---|---|
| `init(admin)` | Initialize the contract with an admin address |
| `issue(hash, holder)` | Register a new credential (issuer = caller) |
| `revoke(hash)` | Mark a credential as revoked (issuer or admin only) |
| `get(hash)` | Return the full credential record |

## Status

> **Scaffold only.** No business logic is implemented yet.  
> See the project issue tracker for upcoming implementation tasks.

## Development

```bash
# Build
cargo build --package credential_registry --target wasm32-unknown-unknown --release

# Test (stubs only for now)
cargo test --package credential_registry
```
