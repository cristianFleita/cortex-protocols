#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, vec, Address, Env, String};

fn setup() -> (Env, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(AgentRegistryContract, ());
    (env, contract_id)
}

#[test]
fn test_register_agent() {
    let (env, contract_id) = setup();
    let owner = Address::generate(&env);
    let client = AgentRegistryContractClient::new(&env, &contract_id);

    let caps = vec![&env, Capability::Reasoning, Capability::TextGeneration];

    let agent_id = client.register_agent(
        &owner,
        &String::from_str(&env, "Cortex-Alpha"),
        &String::from_str(&env, "General-purpose reasoning agent"),
        &caps,
    );

    assert_eq!(agent_id, 1);
    assert_eq!(client.agent_count(), 1);

    let agent = client.get_agent(&1).unwrap();
    assert!(agent.is_active);
    assert_eq!(agent.reputation, 5_000);
}

#[test]
fn test_vote_reputation() {
    let (env, contract_id) = setup();
    let owner = Address::generate(&env);
    let voter = Address::generate(&env);
    let client = AgentRegistryContractClient::new(&env, &contract_id);

    client.register_agent(
        &owner,
        &String::from_str(&env, "DataBot"),
        &String::from_str(&env, "Data analysis specialist"),
        &vec![&env, Capability::DataAnalysis],
    );

    // Vote 80/100 → new_rep = (5000 * 9 + 8000) / 10 = 5300
    client.vote_reputation(&voter, &1, &80);
    assert_eq!(client.get_reputation(&1), 5_300);
}

#[test]
fn test_update_capabilities() {
    let (env, contract_id) = setup();
    let owner = Address::generate(&env);
    let client = AgentRegistryContractClient::new(&env, &contract_id);

    client.register_agent(
        &owner,
        &String::from_str(&env, "VisionAgent"),
        &String::from_str(&env, "Computer vision agent"),
        &vec![&env, Capability::VisionUnderstanding],
    );

    let new_caps = vec![
        &env,
        Capability::VisionUnderstanding,
        Capability::AudioProcessing,
        Capability::DataAnalysis,
    ];
    client.update_capabilities(&owner, &1, &new_caps);

    let agent = client.get_agent(&1).unwrap();
    assert_eq!(agent.capabilities.len(), 3);
}

#[test]
fn test_deactivate_agent() {
    let (env, contract_id) = setup();
    let owner = Address::generate(&env);
    let client = AgentRegistryContractClient::new(&env, &contract_id);

    client.register_agent(
        &owner,
        &String::from_str(&env, "DeprecatedAgent"),
        &String::from_str(&env, "Being retired"),
        &vec![&env, Capability::WebResearch],
    );

    client.deactivate_agent(&owner, &1);

    let agent = client.get_agent(&1).unwrap();
    assert!(!agent.is_active);
}
