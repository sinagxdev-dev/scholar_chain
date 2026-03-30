#[cfg(test)]
mod tests {
    use soroban_sdk::{
        testutils::Address as _,
        Address, BytesN, Env, Symbol,
    };

    use crate::{ScholarChainContract, ScholarChainContractClient};

    fn mock_hash(env: &Env, seed: u8) -> BytesN<32> {
        BytesN::from_array(env, &[seed; 32])
    }

    // Test 1 — Happy path: certificate is registered successfully
    #[test]
    fn test_register_certificate_success() {
        let env = Env::default();
        env.mock_all_auths();

        let registrar = Address::generate(&env);
        let student = Address::generate(&env);
        let doc_hash = mock_hash(&env, 1u8);
        let term = Symbol::new(&env, "2025_2026_1S");

        let contract_id = env.register_contract(None, ScholarChainContract);
        let client = ScholarChainContractClient::new(&env, &contract_id);

        // Initialize and register
        client.initialize(&registrar);
        client.register_certificate(&student, &doc_hash, &term);

        // Record should exist in storage
        let record = client.get_enrollment(&doc_hash).unwrap();
        assert_eq!(record.student_wallet, student);
    }

    // Test 2 — Edge case: duplicate registration is rejected
    #[test]
    #[should_panic(expected = "certificate already registered")]
    fn test_duplicate_registration_rejected() {
        let env = Env::default();
        env.mock_all_auths();

        let registrar = Address::generate(&env);
        let student = Address::generate(&env);
        let doc_hash = mock_hash(&env, 2u8);
        let term = Symbol::new(&env, "2025_2026_1S");

        let contract_id = env.register_contract(None, ScholarChainContract);
        let client = ScholarChainContractClient::new(&env, &contract_id);

        client.initialize(&registrar);
        client.register_certificate(&student, &doc_hash, &term);

        // Second registration of same hash must panic
        client.register_certificate(&student, &doc_hash, &term);
    }

    // Test 3 — State verification: verify_certificate returns correct result
    #[test]
    fn test_verify_certificate_correct_state() {
        let env = Env::default();
        env.mock_all_auths();

        let registrar = Address::generate(&env);
        let student = Address::generate(&env);
        let doc_hash = mock_hash(&env, 3u8);
        let term = Symbol::new(&env, "2025_2026_2S");

        let contract_id = env.register_contract(None, ScholarChainContract);
        let client = ScholarChainContractClient::new(&env, &contract_id);

        client.initialize(&registrar);
        client.register_certificate(&student, &doc_hash, &term);

        // Correct wallet → should return true
        let valid = client.verify_certificate(&student, &doc_hash);
        assert!(valid, "should be valid for registered student");

        // Wrong wallet → should return false
        let attacker = Address::generate(&env);
        let invalid = client.verify_certificate(&attacker, &doc_hash);
        assert!(!invalid, "should be invalid for wrong wallet");
    }
}