#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token, Address, Env, String,
};

fn setup() -> (Env, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let contract_id = env.register(MarketplaceContract, ());
    (env, admin, contract_id)
}

fn create_token(env: &Env, admin: &Address) -> (Address, token::StellarAssetClient) {
    let token_admin = Address::generate(env);
    let contract_address = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_client = token::StellarAssetClient::new(env, &contract_address.address());
    token_client.mint(admin, &10_000_000_000);
    (contract_address.address(), token_client)
}

#[test]
fn test_initialize() {
    let (env, admin, contract_id) = setup();
    let client = MarketplaceContractClient::new(&env, &contract_id);
    client.initialize(&admin);
    assert_eq!(client.asset_count(), 0);
}

#[test]
fn test_list_and_get_asset() {
    let (env, admin, contract_id) = setup();
    let client = MarketplaceContractClient::new(&env, &contract_id);
    client.initialize(&admin);

    let asset_id = client.list_asset(
        &admin,
        &String::from_str(&env, "GPT-4 Chain-of-Thought Prompt"),
        &String::from_str(&env, "Advanced reasoning prompt for complex analysis"),
        &AssetType::Prompt,
        &LicenseType::Perpetual,
        &5_000_000i128, // 0.5 XLM
    );

    assert_eq!(asset_id, 1);
    assert_eq!(client.asset_count(), 1);

    let asset = client.get_asset(&1).unwrap();
    assert_eq!(asset.id, 1);
    assert!(asset.is_active);
    assert_eq!(asset.price, 5_000_000);
}

#[test]
fn test_multiple_assets() {
    let (env, admin, contract_id) = setup();
    let client = MarketplaceContractClient::new(&env, &contract_id);
    client.initialize(&admin);

    for i in 0..5u32 {
        let name = if i == 0 {
            String::from_str(&env, "Asset One")
        } else if i == 1 {
            String::from_str(&env, "Asset Two")
        } else if i == 2 {
            String::from_str(&env, "Asset Three")
        } else if i == 3 {
            String::from_str(&env, "Asset Four")
        } else {
            String::from_str(&env, "Asset Five")
        };

        client.list_asset(
            &admin,
            &name,
            &String::from_str(&env, "A test intelligence asset"),
            &AssetType::Workflow,
            &LicenseType::UsageBased,
            &1_000_000i128,
        );
    }

    assert_eq!(client.asset_count(), 5);
}

#[test]
fn test_delist_asset() {
    let (env, admin, contract_id) = setup();
    let client = MarketplaceContractClient::new(&env, &contract_id);
    client.initialize(&admin);

    let asset_id = client.list_asset(
        &admin,
        &String::from_str(&env, "Deprecated Evaluator"),
        &String::from_str(&env, "Old evaluator being retired"),
        &AssetType::Evaluator,
        &LicenseType::Perpetual,
        &2_000_000i128,
    );

    client.delist_asset(&admin, &asset_id);

    let asset = client.get_asset(&asset_id).unwrap();
    assert!(!asset.is_active);
}

#[test]
fn test_update_price() {
    let (env, admin, contract_id) = setup();
    let client = MarketplaceContractClient::new(&env, &contract_id);
    client.initialize(&admin);

    let asset_id = client.list_asset(
        &admin,
        &String::from_str(&env, "Memory System v1"),
        &String::from_str(&env, "Persistent agent memory module"),
        &AssetType::MemorySystem,
        &LicenseType::Subscription,
        &10_000_000i128,
    );

    client.update_price(&admin, &asset_id, &15_000_000i128);

    let asset = client.get_asset(&asset_id).unwrap();
    assert_eq!(asset.price, 15_000_000);
}

#[test]
fn test_purchase_license() {
    let (env, admin, contract_id) = setup();
    let buyer = Address::generate(&env);
    let client = MarketplaceContractClient::new(&env, &contract_id);
    client.initialize(&admin);

    let (token_addr, token_sac) = create_token(&env, &buyer);
    token_sac.mint(&buyer, &50_000_000);

    let asset_id = client.list_asset(
        &admin,
        &String::from_str(&env, "Reasoning Chain Alpha"),
        &String::from_str(&env, "Multi-step reasoning for legal analysis"),
        &AssetType::ReasoningChain,
        &LicenseType::Perpetual,
        &10_000_000i128,
    );

    assert!(!client.has_license(&buyer, &asset_id));

    let license = client.purchase_license(&buyer, &asset_id, &token_addr);
    assert_eq!(license.asset_id, asset_id);
    assert!(client.has_license(&buyer, &asset_id));
}

#[test]
fn test_has_no_license_by_default() {
    let (env, admin, contract_id) = setup();
    let stranger = Address::generate(&env);
    let client = MarketplaceContractClient::new(&env, &contract_id);
    client.initialize(&admin);

    client.list_asset(
        &admin,
        &String::from_str(&env, "Tool Pack"),
        &String::from_str(&env, "Collection of agent tools"),
        &AssetType::Tool,
        &LicenseType::UsageBased,
        &3_000_000i128,
    );

    assert!(!client.has_license(&stranger, &1));
}
