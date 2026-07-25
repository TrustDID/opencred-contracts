//! Integration tests for the Credential Registry contract.
//!
//! Run with:
//!   cargo test --package credential_registry

#[cfg(test)]
mod tests {
    use credential_registry::{CredentialRegistry, CredentialRegistryClient};
    use soroban_sdk::{
        testutils::{Address as _, Ledger},
        Address, Bytes, Env,
    };

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    /// Deploy the contract and return a client wired to the test environment.
    fn setup() -> (Env, CredentialRegistryClient<'static>) {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(CredentialRegistry, ());
        let client = CredentialRegistryClient::new(&env, &contract_id);
        (env, client)
    }

    /// Create a short `Bytes` value from a UTF-8 string slice.
    fn bytes(env: &Env, s: &str) -> Bytes {
        Bytes::from_slice(env, s.as_bytes())
    }

    // -----------------------------------------------------------------------
    // issue_credential — happy path
    // -----------------------------------------------------------------------

    #[test]
    fn test_issue_credential_stores_record() {
        let (env, client) = setup();

        let issuer = Address::generate(&env);
        let holder = Address::generate(&env);
        let cred_id = bytes(&env, "cred-001");
        let ipfs_cid = bytes(&env, "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi");

        // Set a known ledger timestamp so we can assert on it later.
        env.ledger().set_timestamp(1_000_000);

        client.issue_credential(&cred_id, &issuer, &holder, &ipfs_cid);

        let record = client.get_credential(&cred_id);

        assert_eq!(record.issuer, issuer);
        assert_eq!(record.holder, holder);
        assert_eq!(record.ipfs_cid, ipfs_cid);
        assert_eq!(record.issued_at, 1_000_000);
        assert!(!record.revoked);
    }

    #[test]
    fn test_issue_credential_different_ids_are_independent() {
        let (env, client) = setup();

        let issuer = Address::generate(&env);
        let holder_a = Address::generate(&env);
        let holder_b = Address::generate(&env);

        client.issue_credential(
            &bytes(&env, "cred-a"),
            &issuer,
            &holder_a,
            &bytes(&env, "cid-a"),
        );
        client.issue_credential(
            &bytes(&env, "cred-b"),
            &issuer,
            &holder_b,
            &bytes(&env, "cid-b"),
        );

        let a = client.get_credential(&bytes(&env, "cred-a"));
        let b = client.get_credential(&bytes(&env, "cred-b"));

        assert_eq!(a.holder, holder_a);
        assert_eq!(b.holder, holder_b);
    }

    // -----------------------------------------------------------------------
    // issue_credential — duplicate prevention
    // -----------------------------------------------------------------------

    #[test]
    #[should_panic(expected = "credential already exists")]
    fn test_issue_credential_duplicate_panics() {
        let (env, client) = setup();

        let issuer = Address::generate(&env);
        let holder = Address::generate(&env);
        let cred_id = bytes(&env, "cred-dup");
        let ipfs_cid = bytes(&env, "cid-dup");

        client.issue_credential(&cred_id, &issuer, &holder, &ipfs_cid);
        // Second call with the same credential_id must panic.
        client.issue_credential(&cred_id, &issuer, &holder, &ipfs_cid);
    }

    // -----------------------------------------------------------------------
    // revoke_credential — happy path
    // -----------------------------------------------------------------------

    #[test]
    fn test_revoke_credential_marks_revoked() {
        let (env, client) = setup();

        let issuer = Address::generate(&env);
        let holder = Address::generate(&env);
        let cred_id = bytes(&env, "cred-rev");

        client.issue_credential(&cred_id, &issuer, &holder, &bytes(&env, "cid-rev"));

        // Credential should not be revoked yet.
        assert!(!client.get_credential(&cred_id).revoked);

        client.revoke_credential(&cred_id, &issuer);

        assert!(client.get_credential(&cred_id).revoked);
    }

    // -----------------------------------------------------------------------
    // revoke_credential — error cases
    // -----------------------------------------------------------------------

    #[test]
    #[should_panic(expected = "credential not found")]
    fn test_revoke_nonexistent_credential_panics() {
        let (env, client) = setup();
        let issuer = Address::generate(&env);
        client.revoke_credential(&bytes(&env, "no-such-cred"), &issuer);
    }

    #[test]
    #[should_panic(expected = "unauthorized issuer")]
    fn test_revoke_wrong_issuer_panics() {
        let (env, client) = setup();

        let issuer = Address::generate(&env);
        let attacker = Address::generate(&env);
        let holder = Address::generate(&env);
        let cred_id = bytes(&env, "cred-auth");

        client.issue_credential(&cred_id, &issuer, &holder, &bytes(&env, "cid-auth"));
        // A different address tries to revoke — must panic.
        client.revoke_credential(&cred_id, &attacker);
    }

    #[test]
    #[should_panic(expected = "credential already revoked")]
    fn test_revoke_already_revoked_panics() {
        let (env, client) = setup();

        let issuer = Address::generate(&env);
        let holder = Address::generate(&env);
        let cred_id = bytes(&env, "cred-double-rev");

        client.issue_credential(&cred_id, &issuer, &holder, &bytes(&env, "cid-dr"));
        client.revoke_credential(&cred_id, &issuer);
        // Revoking a second time must panic.
        client.revoke_credential(&cred_id, &issuer);
    }

    // -----------------------------------------------------------------------
    // get_credential — error cases
    // -----------------------------------------------------------------------

    #[test]
    #[should_panic(expected = "credential not found")]
    fn test_get_nonexistent_credential_panics() {
        let (env, client) = setup();
        client.get_credential(&bytes(&env, "ghost-cred"));
    }
}
