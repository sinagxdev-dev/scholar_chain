#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, BytesN, Env, Symbol,
};

// ─── Storage Keys ─────────────────────────────────────────────────────────────

#[contracttype]
pub enum DataKey {
    // Maps a certificate hash → EnrollmentRecord
    Enrollment(BytesN<32>),
    // The authorized registrar (university/CHED admin)
    Registrar,
}

// ─── Data Structures ──────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct EnrollmentRecord {
    // The student's Stellar wallet address
    pub student_wallet: Address,
    // SHA-256 hash of the enrollment document
    pub doc_hash: BytesN<32>,
    // Academic term e.g. "2025_2026_1S"
    pub term: Symbol,
    // Whether this record has been verified
    pub verified: bool,
}

// ─── Contract ─────────────────────────────────────────────────────────────────

#[contract]
pub struct ScholarChainContract;

#[contractimpl]
impl ScholarChainContract {

    // Initialize the contract with a registrar address
    pub fn initialize(env: Env, registrar: Address) {
        if env.storage().instance().has(&DataKey::Registrar) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Registrar, &registrar);
    }

    // Register a certificate on-chain
    // Only the registrar can call this
    // Rejects duplicate hashes (tamper detection)
    pub fn register_certificate(
        env: Env,
        student_wallet: Address,
        doc_hash: BytesN<32>,
        term: Symbol,
    ) {
        // Only registrar may call this
        let registrar: Address = env
            .storage()
            .instance()
            .get(&DataKey::Registrar)
            .unwrap();
        registrar.require_auth();

        // Reject duplicate hash
        let key = DataKey::Enrollment(doc_hash.clone());
        if env.storage().persistent().has(&key) {
            panic!("certificate already registered");
        }

        // Store the enrollment record
        let record = EnrollmentRecord {
            student_wallet: student_wallet.clone(),
            doc_hash: doc_hash.clone(),
            term: term.clone(),
            verified: false,
        };
        env.storage().persistent().set(&key, &record);

        // Emit event for on-chain auditability
        env.events().publish(
            (symbol_short!("REGISTER"),),
            (student_wallet, doc_hash, term),
        );
    }

    // Verify a certificate — returns true if hash matches the wallet
    // Emits a verification event on-chain
    pub fn verify_certificate(
        env: Env,
        student_wallet: Address,
        doc_hash: BytesN<32>,
    ) -> bool {
        let key = DataKey::Enrollment(doc_hash.clone());

        if !env.storage().persistent().has(&key) {
            env.events().publish(
                (symbol_short!("VERIFY"),),
                (student_wallet, doc_hash, false),
            );
            return false;
        }

        let record: EnrollmentRecord =
            env.storage().persistent().get(&key).unwrap();

        let valid = record.student_wallet == student_wallet;

        // Mark as verified if valid
        if valid {
            let mut updated = record.clone();
            updated.verified = true;
            env.storage().persistent().set(&key, &updated);
        }

        env.events().publish(
            (symbol_short!("VERIFY"),),
            (student_wallet, doc_hash, valid),
        );

        valid
    }

    // Read-only: get enrollment record by hash
    pub fn get_enrollment(
        env: Env,
        doc_hash: BytesN<32>,
    ) -> Option<EnrollmentRecord> {
        let key = DataKey::Enrollment(doc_hash);
        env.storage().persistent().get(&key)
    }

    // Read-only: get the registrar address
    pub fn get_registrar(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::Registrar)
            .unwrap()
    }
}