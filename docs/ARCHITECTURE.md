# Architecture

## Overview

OpenCred is a decentralized credential registry. Issuers register credential hashes on-chain; verifiers check authenticity and revocation status without trusting a central authority.

---

## IPFS Storage Rationale

Credential documents (e.g., JSON-LD VCs) are stored on IPFS, not on-chain. Reasons:

- **Cost** — storing large documents on Stellar is expensive and unnecessary.
- **Content addressing** — IPFS CIDs are cryptographic hashes of the content, so the on-chain hash *is* the IPFS CID. Tampering with the document breaks the CID.
- **Separation of concerns** — the blockchain enforces issuance, ownership, and revocation rules; IPFS handles document availability.

The contract stores only the IPFS CID (as a `Bytes` value), not the document itself.

---

## On-Chain Data Model

Each credential entry in `credential_registry` stores:

| Field | Type | Description |
|-------|------|-------------|
| `credential_id` | `Bytes` | Unique identifier (e.g., UUID or hash) |
| `issuer` | `Address` | Stellar address of the issuer |
| `holder` | `Address` | Stellar address of the credential holder |
| `ipfs_cid` | `Bytes` | IPFS CID of the credential document |
| `issued_at` | `u64` | Ledger timestamp of issuance |
| `revoked` | `bool` | Whether the credential has been revoked |

Storage is keyed by `credential_id` in Soroban's persistent storage.

---

## Decentralization Goals

- **No admin key** — there is no privileged account that can alter or delete credentials belonging to other issuers.
- **Permissionless verification** — anyone can call `verify_credential` without authentication.
- **Issuer sovereignty** — only the original issuer can revoke their own credentials.
- **Upgradability** — contract upgrades follow Soroban's upgrade mechanism and require explicit governance (to be defined).

---

## Security Expectations

- All state-mutating functions authenticate the caller via `Address::require_auth()`.
- Revocation is permanent and irreversible by design.
- The contract does not validate IPFS CID format on-chain; off-chain tooling is responsible for CID correctness.
- No cross-contract calls in the initial version to minimize attack surface.
- Audits are required before mainnet deployment.

---

## Interoperability

- Credential documents follow the [W3C Verifiable Credentials Data Model](https://www.w3.org/TR/vc-data-model/).
- IPFS CIDs use CIDv1 with SHA-256 by default, compatible with standard IPFS tooling.
- The on-chain interface is designed to be callable from any Stellar SDK (JavaScript, Python, Rust).
- Future versions may support DID-based issuer identifiers to align with the broader decentralized identity ecosystem.
