#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Batch {
    pub farmer: Address,
    pub crop_type: Symbol,
    pub timestamp: u64,
    pub is_verified: bool,
}

#[contracttype]
pub enum DataKey {
    Batch(u64),
}

#[contract]
pub struct AgroChainContract;

#[contractimpl]
impl AgroChainContract {
    /// Registers a new agricultural harvest batch onto the immutable ledger.
    pub fn register_batch(env: Env, farmer: Address, id: u64, crop_type: Symbol, timestamp: u64) {
        farmer.require_auth();
        
        let key = DataKey::Batch(id);
        if env.storage().persistent().has(&key) {
            panic!("Batch ID already exists");
        }

        let batch = Batch {
            farmer: farmer.clone(),
            crop_type,
            timestamp,
            is_verified: false,
        };

        env.storage().persistent().set(&key, &batch);
    }

    /// Allows an authorized quality auditor or cooperative head to verify batch authenticity.
    pub fn verify_batch(env: Env, verifier: Address, id: u64) {
        verifier.require_auth();
        
        let key = DataKey::Batch(id);
        let mut batch: Batch = env.storage().persistent().get(&key).unwrap_or_else(|| {
            panic!("Batch not found")
        });

        batch.is_verified = true;
        env.storage().persistent().set(&key, &batch);
    }

    /// Fetches details of a specific batch for QR code lookups.
    pub fn get_batch(env: Env, id: u64) -> Batch {
        let key = DataKey::Batch(id);
        env.storage().persistent().get(&key).unwrap_or_else(|| {
            panic!("Batch not found")
        })
    }
}