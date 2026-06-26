#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, Symbol};

#[test]
fn test_register_batch_success() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, AgroChainContract);
    let client = AgroChainContractClient::new(&env, &contract_id);

    let farmer = Address::generate(&env);
    let crop = Symbol::new(&env, "Coffee");
    
    client.register_batch(&farmer, &101, &crop, &1719416191);
    
    let batch = client.get_batch(&101);
    assert_eq!(batch.farmer, farmer);
    assert_eq!(batch.crop_type, crop);
}

#[test]
#[should_panic(expected = "Batch ID already exists")]
fn test_register_batch_duplicate_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, AgroChainContract);
    let client = AgroChainContractClient::new(&env, &contract_id);

    let farmer = Address::generate(&env);
    let crop = Symbol::new(&env, "Coffee");

    client.register_batch(&farmer, &102, &crop, &1719416191);
    client.register_batch(&farmer, &102, &crop, &1719416191);
}

#[test]
fn test_state_verification() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, AgroChainContract);
    let client = AgroChainContractClient::new(&env, &contract_id);

    let farmer = Address::generate(&env);
    let crop = Symbol::new(&env, "Cacao");

    client.register_batch(&farmer, &103, &crop, &1719416195);
    
    let fetched = client.get_batch(&103);
    assert_eq!(fetched.is_verified, false);
}

#[test]
fn test_verify_batch_success() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, AgroChainContract);
    let client = AgroChainContractClient::new(&env, &contract_id);

    let farmer = Address::generate(&env);
    let verifier = Address::generate(&env);
    let crop = Symbol::new(&env, "Coffee");

    client.register_batch(&farmer, &104, &crop, &1719416191);
    client.verify_batch(&verifier, &104);

    let batch = client.get_batch(&104);
    assert_eq!(batch.is_verified, true);
}

#[test]
#[should_panic(expected = "Batch not found")]
fn test_verify_non_existent_batch_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, AgroChainContract);
    let client = AgroChainContractClient::new(&env, &contract_id);

    let verifier = Address::generate(&env);
    client.verify_batch(&verifier, &999);
}