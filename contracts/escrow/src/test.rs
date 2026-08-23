use crate::{errors::Error, types::JobStatus, EscrowContract, EscrowContractClient};
use soroban_sdk::{testutils::Address as _, token, Address, Env};

/// Deploys a Stellar Asset Contract to stand in for USDC in tests
/// and returns (token_address, admin_client, token_client).
fn create_token<'a>(
    env: &Env,
    admin: &Address,
) -> (Address, token::StellarAssetClient<'a>, token::Client<'a>) {
    let sac = env.register_stellar_asset_contract_v2(admin.clone());
    let address = sac.address();
    (
        address.clone(),
        token::StellarAssetClient::new(env, &address),
        token::Client::new(env, &address),
    )
}

fn setup<'a>() -> (
    Env,
    EscrowContractClient<'a>,
    Address, // client
    Address, // worker
    token::StellarAssetClient<'a>,
    token::Client<'a>,
    Address, // token address
) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(EscrowContract, ());
    let client_contract = EscrowContractClient::new(&env, &contract_id);

    let client = Address::generate(&env);
    let worker = Address::generate(&env);
    let token_admin = Address::generate(&env);

    let (token_address, token_admin_client, token_client) = create_token(&env, &token_admin);
    token_admin_client.mint(&client, &1_000_000_000);

    (
        env,
        client_contract,
        client,
        worker,
        token_admin_client,
        token_client,
        token_address,
    )
}

#[test]
fn test_create_escrow_locks_funds() {
    let (_env, contract, client, _worker, _admin, token, token_address) = setup();

    let job_id = contract.create_escrow(&client, &100_000_000, &token_address);

    assert_eq!(job_id, 1);
    assert_eq!(token.balance(&client), 900_000_000);
    assert_eq!(token.balance(&contract.address), 100_000_000);

    let state = contract.get_escrow(&job_id);
    assert_eq!(state.status, JobStatus::Open);
    assert_eq!(state.amount, 100_000_000);
    assert_eq!(state.client, client);
    assert!(state.worker.is_none());
}

#[test]
fn test_create_escrow_rejects_zero_amount() {
    let (_env, contract, client, _worker, _admin, _token, token_address) = setup();

    let result = contract.try_create_escrow(&client, &0, &token_address);
    assert_eq!(result, Err(Ok(Error::InvalidAmount)));
}

#[test]
fn test_full_happy_path_releases_payment_to_worker() {
    let (_env, contract, client, worker, _admin, token, token_address) = setup();

    let job_id = contract.create_escrow(&client, &100_000_000, &token_address);
    contract.accept_job(&worker, &job_id);

    let state = contract.get_escrow(&job_id);
    assert_eq!(state.status, JobStatus::InProgress);
    assert_eq!(state.worker, Some(worker.clone()));

    contract.release_payment(&client, &job_id);

    let state = contract.get_escrow(&job_id);
    assert_eq!(state.status, JobStatus::Completed);
    assert_eq!(token.balance(&worker), 100_000_000);
    assert_eq!(token.balance(&contract.address), 0);
}

#[test]
fn test_cancel_escrow_refunds_client() {
    let (_env, contract, client, _worker, _admin, token, token_address) = setup();

    let job_id = contract.create_escrow(&client, &100_000_000, &token_address);
    contract.cancel_escrow(&client, &job_id);

    let state = contract.get_escrow(&job_id);
    assert_eq!(state.status, JobStatus::Cancelled);
    assert_eq!(token.balance(&client), 1_000_000_000);
    assert_eq!(token.balance(&contract.address), 0);
}

#[test]
fn test_cannot_cancel_after_accepted() {
    let (_env, contract, client, worker, _admin, _token, token_address) = setup();

    let job_id = contract.create_escrow(&client, &100_000_000, &token_address);
    contract.accept_job(&worker, &job_id);

    let result = contract.try_cancel_escrow(&client, &job_id);
    assert_eq!(result, Err(Ok(Error::EscrowNotOpen)));
}

#[test]
fn test_cannot_accept_already_accepted_job() {
    let (_env, contract, client, worker, _admin, _token, token_address) = setup();

    let job_id = contract.create_escrow(&client, &100_000_000, &token_address);
    contract.accept_job(&worker, &job_id);

    let other_worker = Address::generate(&_env);
    let result = contract.try_accept_job(&other_worker, &job_id);
    assert_eq!(result, Err(Ok(Error::EscrowNotOpen)));
}

#[test]
fn test_dispute_by_worker_flags_escrow() {
    let (_env, contract, client, worker, _admin, _token, token_address) = setup();

    let job_id = contract.create_escrow(&client, &100_000_000, &token_address);
    contract.accept_job(&worker, &job_id);
    contract.dispute_escrow(&worker, &job_id);

    let state = contract.get_escrow(&job_id);
    assert_eq!(state.status, JobStatus::Disputed);
}

#[test]
fn test_dispute_rejects_unrelated_caller() {
    let (env, contract, client, worker, _admin, _token, token_address) = setup();

    let job_id = contract.create_escrow(&client, &100_000_000, &token_address);
    contract.accept_job(&worker, &job_id);

    let stranger = Address::generate(&env);
    let result = contract.try_dispute_escrow(&stranger, &job_id);
    assert_eq!(result, Err(Ok(Error::NotParty)));
}

#[test]
fn test_get_escrow_not_found() {
    let (_env, contract, ..) = setup();
    let result = contract.try_get_escrow(&999);
    assert_eq!(result, Err(Ok(Error::EscrowNotFound)));
}
