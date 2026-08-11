//! # Credential Registry Contract
//!
//! This Soroban smart contract is the on-chain component of the OpenCred
//! decentralized credential system on Stellar.
//!
//! ## Credential Lifecycle
//!
//! 1. **Issuance** — An issuer submits a credential hash along with the
//!    holder's address. The contract records the hash, issuer, holder, and a
//!    ledger timestamp. The full credential document is stored off-chain on IPFS.
//!
//! 2. **Verification** — Any caller can query the registry by credential hash to
//!    confirm it was issued by a known issuer, retrieve the holder address, and
//!    check that it has not been revoked.
//!
//! 3. **Revocation** — The original issuer can mark a credential hash as
//!    revoked. Revocation is permanent and on-chain.
//!
//! ## Storage Design
//!
//! Each credential is stored in Soroban persistent storage keyed by its hash:
//!
//! ```text
//! CredentialKey { hash } → CredentialRecord { hash, issuer, holder, issued_at, revoked }
//! ```
//!
//! The hash is the content hash of the credential document (e.g. an IPFS CID),
//! so tampering with the document breaks the link. See `docs/ARCHITECTURE.md`.
//!
//! ## Authorization
//!
//! - `register_credential` and `revoke_credential` authenticate the caller via
//!   `Address::require_auth`.
//! - Only the issuer that registered a credential can revoke it. There is no
//!   privileged admin key — the registry is decentralized by design.

#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Bytes, Env};

/// Errors returned by the Credential Registry contract.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum CredentialError {
    /// A credential with the same hash is already registered.
    AlreadyRegistered = 1,
    /// No credential exists for the given hash.
    NotFound = 2,
    /// The caller is not authorized to perform the operation.
    Unauthorized = 3,
    /// The credential is already revoked; revocation is permanent.
    AlreadyRevoked = 4,
}

/// Storage key for a credential, keyed by its content hash.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CredentialKey {
    pub hash: Bytes,
}

/// On-chain record for a single credential.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CredentialRecord {
    /// Content hash of the credential document (e.g. an IPFS CID).
    pub hash: Bytes,
    /// Stellar address of the issuing organization.
    pub issuer: Address,
    /// Stellar address of the credential subject.
    pub holder: Address,
    /// Ledger timestamp at issuance.
    pub issued_at: u64,
    /// Whether the credential has been revoked. Revocation is permanent.
    pub revoked: bool,
}

/// The OpenCred Credential Registry contract.
#[contract]
pub struct CredentialRegistry;

#[contractimpl]
impl CredentialRegistry {
    /// Register a new credential on behalf of the authenticated issuer.
    ///
    /// # Arguments
    ///
    /// * `issuer` — Stellar address of the issuing organization. Must sign the
    ///   transaction.
    /// * `holder` — Stellar address of the credential subject.
    /// * `hash` — Content hash of the credential document.
    ///
    /// # Errors
    ///
    /// Returns [`CredentialError::AlreadyRegistered`] if a credential with the
    /// same hash already exists.
    pub fn register_credential(
        env: Env,
        issuer: Address,
        holder: Address,
        hash: Bytes,
    ) -> Result<CredentialRecord, CredentialError> {
        issuer.require_auth();

        let key = CredentialKey { hash: hash.clone() };

        if env.storage().persistent().has(&key) {
            return Err(CredentialError::AlreadyRegistered);
        }

        let record = CredentialRecord {
            hash,
            issuer: issuer.clone(),
            holder,
            issued_at: env.ledger().timestamp(),
            revoked: false,
        };

        env.storage().persistent().set(&key, &record);

        Ok(record)
    }

    /// Look up a credential by its content hash.
    ///
    /// Returns `None` when no credential with the given hash is registered.
    pub fn get_credential(env: Env, hash: Bytes) -> Option<CredentialRecord> {
        env.storage().persistent().get(&CredentialKey { hash })
    }

    /// Permanently revoke a credential. Only the issuer that registered it may
    /// revoke it.
    ///
    /// # Errors
    ///
    /// * [`CredentialError::NotFound`] if the hash is not registered.
    /// * [`CredentialError::Unauthorized`] if the caller is not the original
    ///   issuer.
    /// * [`CredentialError::AlreadyRevoked`] if the credential is already
    ///   revoked.
    pub fn revoke_credential(
        env: Env,
        issuer: Address,
        hash: Bytes,
    ) -> Result<CredentialRecord, CredentialError> {
        issuer.require_auth();

        let key = CredentialKey { hash };

        let mut record: CredentialRecord = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(CredentialError::NotFound)?;

        if record.issuer != issuer {
            return Err(CredentialError::Unauthorized);
        }

        if record.revoked {
            return Err(CredentialError::AlreadyRevoked);
        }

        record.revoked = true;
        env.storage().persistent().set(&key, &record);

        Ok(record)
    }
}
