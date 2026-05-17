#![allow(unused)]

use soroban_sdk::{symbol_short, Address, Env};

/// Emitted when a new escrow job is created
pub fn escrow_created(env: &Env, job_id: u64, client: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("CREATED"), job_id),
        (client.clone(), amount),
    );
}

/// Emitted when a worker accepts a job
pub fn job_accepted(env: &Env, job_id: u64, worker: &Address) {
    env.events().publish(
        (symbol_short!("ACCEPTED"), job_id),
        worker.clone(),
    );
}

/// Emitted when payment is released to the worker
pub fn payment_released(env: &Env, job_id: u64, worker: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("RELEASED"), job_id),
        (worker.clone(), amount),
    );
}

/// Emitted when an escrow is cancelled by the client
pub fn escrow_cancelled(env: &Env, job_id: u64, client: &Address) {
    env.events().publish(
        (symbol_short!("CANCELD"), job_id),
        client.clone(),
    );
}

/// Emitted when a dispute is raised on an escrow
pub fn escrow_disputed(env: &Env, job_id: u64, caller: &Address) {
    env.events().publish(
        (symbol_short!("DISPUTE"), job_id),
        caller.clone(),
    );
}