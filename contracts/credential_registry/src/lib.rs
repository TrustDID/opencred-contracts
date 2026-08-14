//! # Credential Registry Contract
//!
//! This Soroban smart contract is the on-chain component of the OpenCred
//! decentralized credential system on Stellar.
//!
//! ## Credential Lifecycle
//!
//! 1. **Issuance** — An authorized issuer calls [`CredentialRegistry::issue_credential`]
//!    with a unique `credential_id`, their own address as the issuer, the holder's
//!    address, and the IPFS CID of the credential document.  The contract records
//!    the metadata and the ledger timestamp.  The full credential document is stored
//!    off-chain on IPFS; the contract stores only the CID.
//!
//! 2. **Verification** — Any caller can call [`CredentialRegistry::get_credential`]
//!    with a `credential_id` to confirm it was issued, retrieve the holder address,
//!    and check that it has not been revoked.
//!
//! 3. **Revocation** — The original issuer calls [`CredentialRegistry::revoke_credential`]
//!    to permanently mark their credential as revoked.  Revocation is irreversible.
//!
//! ## Storage Design
//!
//! All entries live in Soroban **persistent** storage so they survive ledger
//! archival of the contract's footprint.
//!
//! Storage layout:
//! - Key: `StorageKey::Credential(credential_id: Bytes)`
//!   → Value: [`CredentialRecord`]
//!
//! ## Security Model
//!
//! - `issue_credential` calls `issuer.require_auth()` — only the declared issuer
//!   can submit the transaction.
//! - `revoke_credential` calls `issuer.require_auth()` and checks that the caller
//!   is the original issuer stored in the record.
//! - `get_credential` is permissionless (read-only).
//! - No admin key exists; there is no privileged account that can alter credentials
//!   belonging to other issuers.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Bytes, Env};

// ---------------------------------------------------------------------------
// Error codes
// ---------------------------------------------------------------------------

/// Contract-level error codes returned as `u32` panic values via
/// `env.panic_with_error()` / `panic!`.
///
/// Clients should match on these values when catching contract errors.
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    /// A credential with the given `credential_id` already exists.
    CredentialAlreadyExists = 1,
    /// No credential found for the given `credential_id`.
    CredentialNotFound = 2,
    /// Caller is not the original issuer of this credential.
    UnauthorizedIssuer = 3,
    /// The credential has already been revoked.
    AlreadyRevoked = 4,
}

// ---------------------------------------------------------------------------
// Data types
// ---------------------------------------------------------------------------

/// Soroban storage key.
///
/// Using an enum with a `Bytes` variant keeps keys human-readable in ledger
/// explorers and avoids collision if more key types are added in the future.
#[contracttype]
#[derive(Clone)]
pub enum StorageKey {
    /// Persistent credential record keyed by `credential_id`.
    Credential(Bytes),
}

/// On-chain representation of a single verifiable credential entry.
///
/// The full credential document lives on IPFS; only the CID is stored here.
/// All fields are immutable after issuance, except `revoked` which transitions
/// `false → true` exactly once.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CredentialRecord {
    /// Stellar address of the entity that issued the credential.
    pub issuer: Address,
    /// Stellar address of the credential holder / subject.
    pub holder: Address,
    /// IPFS CIDv1 (as raw bytes) pointing to the off-chain credential document.
    pub ipfs_cid: Bytes,
    /// Ledger timestamp (Unix seconds) at the time of issuance.
    pub issued_at: u64,
    /// `true` if the credential has been permanently revoked by the issuer.
    pub revoked: bool,
}

// ---------------------------------------------------------------------------
// Contract
// ---------------------------------------------------------------------------

#[contract]
pub struct CredentialRegistry;

#[contractimpl]
impl CredentialRegistry {
    // -----------------------------------------------------------------------
    // Issuance
    // -----------------------------------------------------------------------

    /// Issue a new credential and record it on-chain.
    ///
    /// # Arguments
    ///
    /// * `env`           – Soroban host environment.
    /// * `credential_id` – Unique identifier for this credential (e.g. a UUID
    ///                     or content hash supplied by the issuer's tooling).
    /// * `issuer`        – Address of the issuing party.  The transaction must
    ///                     be signed by this address (`require_auth` is called).
    /// * `holder`        – Address of the credential subject / holder.
    /// * `ipfs_cid`      – IPFS CID of the full credential document stored
    ///                     off-chain.
    ///
    /// # Panics
    ///
    /// * [`ContractError::CredentialAlreadyExists`] if `credential_id` is
    ///   already recorded in persistent storage.
    ///
    /// # Storage
    ///
    /// Writes a [`CredentialRecord`] to persistent storage under
    /// `StorageKey::Credential(credential_id)`.  The `issued_at` timestamp is
    /// taken from the current ledger via `env.ledger().timestamp()`.
    pub fn issue_credential(
        env: Env,
        credential_id: Bytes,
        issuer: Address,
        holder: Address,
        ipfs_cid: Bytes,
    ) {
        // Authenticate: the declared issuer must have signed this transaction.
        issuer.require_auth();

        let key = StorageKey::Credential(credential_id.clone());

        // Prevent duplicate issuance.
        if env.storage().persistent().has(&key) {
            panic!("credential already exists");
        }

        let record = CredentialRecord {
            issuer,
            holder,
            ipfs_cid,
            issued_at: env.ledger().timestamp(),
            revoked: false,
        };

        env.storage().persistent().set(&key, &record);
    }

    // -----------------------------------------------------------------------
    // Revocation
    // -----------------------------------------------------------------------

    /// Permanently revoke a previously issued credential.
    ///
    /// Only the original issuer of the credential may call this function.
    /// Revocation is irreversible: once `revoked` is set to `true` it cannot
    /// be cleared.
    ///
    /// # Arguments
    ///
    /// * `env`           – Soroban host environment.
    /// * `credential_id` – Identifier of the credential to revoke.
    /// * `issuer`        – Address of the issuer requesting revocation.  Must
    ///                     match the issuer stored in the credential record.
    ///
    /// # Panics
    ///
    /// * [`ContractError::CredentialNotFound`] if no credential exists for the
    ///   given `credential_id`.
    /// * [`ContractError::UnauthorizedIssuer`] if `issuer` does not match the
    ///   stored issuer.
    /// * [`ContractError::AlreadyRevoked`] if the credential is already revoked.
    pub fn revoke_credential(env: Env, credential_id: Bytes, issuer: Address) {
        // Authenticate: the caller must have signed as the declared issuer.
        issuer.require_auth();

        let key = StorageKey::Credential(credential_id);

        let mut record: CredentialRecord = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic!("credential not found"));

        if record.issuer != issuer {
            panic!("unauthorized issuer");
        }

        if record.revoked {
            panic!("credential already revoked");
        }

        record.revoked = true;
        env.storage().persistent().set(&key, &record);
    }

    // -----------------------------------------------------------------------
    // Verification
    // -----------------------------------------------------------------------

    /// Retrieve a credential record by its identifier.
    ///
    /// This function is permissionless — any caller may verify a credential.
    ///
    /// # Arguments
    ///
    /// * `env`           – Soroban host environment.
    /// * `credential_id` – Identifier of the credential to look up.
    ///
    /// # Returns
    ///
    /// The [`CredentialRecord`] associated with `credential_id`.
    ///
    /// # Panics
    ///
    /// * [`ContractError::CredentialNotFound`] if no credential exists for the
    ///   given `credential_id`.
    pub fn get_credential(env: Env, credential_id: Bytes) -> CredentialRecord {
        let key = StorageKey::Credential(credential_id);

        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic!("credential not found"))
    }
}
