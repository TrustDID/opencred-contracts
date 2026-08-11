//! Integration tests for the Credential Registry contract.
//!
//! These tests exercise the full credential lifecycle against the contract
//! deployed in a local Soroban test environment: registration, lookup,
//! revocation, and the authorization rules that guard each operation.
//!
//! Run with:
//!   cargo test --package credential_registry

use soroban_sdk::{
    testutils::Address as _, // Address::generate
    Address,
    Bytes,
    Env,
};

use credential_registry::{
    CredentialError, CredentialRecord, CredentialRegistry, CredentialRegistryClient,
};

/// Deploy the contract and return `(env, contract_id, issuer, holder)`.
fn deploy() -> (Env, Address, Address, Address) {
    let env = Env::default();
    let contract_id = env.register(CredentialRegistry, ());
    let issuer = Address::generate(&env);
    let holder = Address::generate(&env);
    (env, contract_id, issuer, holder)
}

/// A client configured to mock all authorizations.
fn client<'a>(env: &'a Env, contract_id: &Address) -> CredentialRegistryClient<'a> {
    CredentialRegistryClient::new(env, contract_id).mock_all_auths()
}

fn sample_hash(env: &Env, tag: u8) -> Bytes {
    Bytes::from_slice(env, &[0x00, 0x11, 0x22, 0x33, 0x44, tag])
}

/// Registering a credential stores the issuer, holder, timestamp, and an
/// unrevoked status, and the record can be retrieved by hash.
#[test]
fn test_register_credential() {
    let (env, contract_id, issuer, holder) = deploy();
    let client = client(&env, &contract_id);
    let hash = sample_hash(&env, 1);

    let record = client.register_credential(&issuer, &holder, &hash);

    assert_eq!(record.hash, hash);
    assert_eq!(record.issuer, issuer);
    assert_eq!(record.holder, holder);
    assert_eq!(record.issued_at, env.ledger().timestamp());
    assert!(!record.revoked);

    let fetched = client
        .get_credential(&hash)
        .expect("registered credential should be found");
    assert_eq!(fetched, record);
}

/// A hash can only be registered once.
#[test]
fn test_duplicate_hash_is_rejected() {
    let (env, contract_id, issuer, holder) = deploy();
    let client = client(&env, &contract_id);
    let hash = sample_hash(&env, 2);

    client.register_credential(&issuer, &holder, &hash);

    assert!(matches!(
        client.try_register_credential(&issuer, &holder, &hash),
        Err(Ok(CredentialError::AlreadyRegistered))
    ));
}

/// Looking up an unknown hash returns `None`.
#[test]
fn test_get_unknown_hash_returns_none() {
    let (env, contract_id, _, _) = deploy();
    let client = client(&env, &contract_id);

    let fetched = client.get_credential(&sample_hash(&env, 99));

    assert!(fetched.is_none());
}

/// The original issuer can revoke a credential, after which the status is
/// visible to any caller.
#[test]
fn test_revoke_credential() {
    let (env, contract_id, issuer, holder) = deploy();
    let client = client(&env, &contract_id);
    let hash = sample_hash(&env, 3);

    client.register_credential(&issuer, &holder, &hash);

    let revoked = client.revoke_credential(&issuer, &hash);
    assert!(revoked.revoked);

    let fetched = client
        .get_credential(&hash)
        .expect("revoked credential should still be found");
    assert!(fetched.revoked);
}

/// Only the original issuer can revoke a credential.
#[test]
fn test_revoke_by_non_issuer_is_rejected() {
    let (env, contract_id, issuer, holder) = deploy();
    let client = client(&env, &contract_id);
    let hash = sample_hash(&env, 4);
    let stranger = Address::generate(&env);

    client.register_credential(&issuer, &holder, &hash);

    assert!(matches!(
        client.try_revoke_credential(&stranger, &hash),
        Err(Ok(CredentialError::Unauthorized))
    ));
}

/// Revoking an unknown hash returns `NotFound`.
#[test]
fn test_revoke_unknown_hash_returns_not_found() {
    let (env, contract_id, issuer, _) = deploy();
    let client = client(&env, &contract_id);

    assert!(matches!(
        client.try_revoke_credential(&issuer, &sample_hash(&env, 98)),
        Err(Ok(CredentialError::NotFound))
    ));
}

/// Revocation is permanent: a revoked credential cannot be revoked again.
#[test]
fn test_revocation_is_permanent() {
    let (env, contract_id, issuer, holder) = deploy();
    let client = client(&env, &contract_id);
    let hash = sample_hash(&env, 5);

    client.register_credential(&issuer, &holder, &hash);
    client.revoke_credential(&issuer, &hash);

    assert!(matches!(
        client.try_revoke_credential(&issuer, &hash),
        Err(Ok(CredentialError::AlreadyRevoked))
    ));
}

/// Registration requires the issuer's signature: without authorizations being
/// provided, `register_credential` panics.
#[test]
#[should_panic]
fn test_register_requires_auth() {
    let env = Env::default();
    let contract_id = env.register(CredentialRegistry, ());
    let client = CredentialRegistryClient::new(&env, &contract_id);

    let issuer = Address::generate(&env);
    let holder = Address::generate(&env);
    let hash = Bytes::from_slice(&env, &[0xAA, 0xBB, 0xCC]);

    client.register_credential(&issuer, &holder, &hash);
}

/// A credential registered by one issuer remains immutable for a second issuer
/// and cannot be revoked by it, demonstrating per-issuer sovereignty.
#[test]
fn test_issuer_sovereignty() {
    let (env, contract_id, issuer_a, holder) = deploy();
    let client = client(&env, &contract_id);
    let issuer_b = Address::generate(&env);
    let hash = sample_hash(&env, 6);

    client.register_credential(&issuer_a, &holder, &hash);

    let fetched: CredentialRecord = client
        .get_credential(&hash)
        .expect("credential should be retrievable");
    assert_eq!(fetched.issuer, issuer_a);

    assert!(matches!(
        client.try_revoke_credential(&issuer_b, &hash),
        Err(Ok(CredentialError::Unauthorized))
    ));
}
