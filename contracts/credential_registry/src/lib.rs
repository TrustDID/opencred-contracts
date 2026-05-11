//! # Credential Registry Contract
//!
//! This Soroban smart contract is the on-chain component of the OpenCred
//! decentralized credential system on Stellar.
//!
//! ## Credential Lifecycle (future implementation)
//!
//! 1. **Issuance** — An authorized issuer submits a credential hash along with
//!    the holder's address. The contract records the hash, issuer, holder, and
//!    a block timestamp. The full credential document is stored off-chain on IPFS.
//!
//! 2. **Verification** — Any caller can query the registry by credential hash to
//!    confirm it was issued by a known issuer, retrieve the holder address, and
//!    check that it has not been revoked.
//!
//! 3. **Revocation** — The original issuer (or an authorized admin) can mark a
//!    credential hash as revoked. Revocation is permanent and on-chain.
//!
//! ## Storage Design (future implementation)
//!
//! Planned storage keys (Soroban persistent storage):
//! - `CredentialEntry { hash }` → `CredentialRecord { issuer, holder, timestamp, revoked }`
//! - `IssuerAllowlist { address }` → `bool`  (optional access control)
//!
//! ## Extensibility
//!
//! - Additional credential types can be supported by versioning the record struct.
//! - Delegation / sub-issuer patterns can be layered on top of the allowlist.
//! - Cross-contract calls may be added for on-chain verification by other dApps.

#![no_std]

use soroban_sdk::{contract, contractimpl, Env};

/// Placeholder contract struct.
/// Business logic will be added in subsequent issues.
#[contract]
pub struct CredentialRegistry;

#[contractimpl]
impl CredentialRegistry {
    /// Placeholder initializer.
    ///
    /// Future: accept an admin address and persist it to storage so that
    /// only the admin can manage the issuer allowlist.
    pub fn init(_env: Env) {}
}
