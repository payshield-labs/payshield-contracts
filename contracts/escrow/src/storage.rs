#![allow(unused)]

use soroban_sdk::{contracttype, Env};

use crate::types::EscrowState;

/// Storage key types for the contract
#[contracttype]
pub enum StorageKey {
    /// Stores EscrowState, keyed by job_id
    Escrow(u64),
    /// Stores the next available job ID counter
    JobCounter,
}

/// Save an escrow state to contract storage
pub fn save_escrow(env: &Env, escrow: &EscrowState) {
    env.storage()
        .persistent()
        .set(&StorageKey::Escrow(escrow.job_id), escrow);
}

/// Load an escrow state from contract storage
///
/// Panics if the job ID does not exist
pub fn load_escrow(env: &Env, job_id: u64) -> EscrowState {
    env.storage()
        .persistent()
        .get(&StorageKey::Escrow(job_id))
        .expect("Escrow not found for given job ID")
}

/// Check whether an escrow exists for a given job ID
pub fn escrow_exists(env: &Env, job_id: u64) -> bool {
    env.storage()
        .persistent()
        .has(&StorageKey::Escrow(job_id))
}

/// Get the current job counter and increment it
///
/// Returns the next available job ID
pub fn next_job_id(env: &Env) -> u64 {
    let current: u64 = env
        .storage()
        .instance()
        .get(&StorageKey::JobCounter)
        .unwrap_or(0);

    let next = current + 1;

    env.storage()
        .instance()
        .set(&StorageKey::JobCounter, &next);

    next
}