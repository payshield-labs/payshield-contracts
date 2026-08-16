#![allow(unused, deprecated)]

use soroban_sdk::{contractevent, Address, Env};

/// Emitted when a new escrow job is created
#[contractevent]
pub struct EscrowCreated {
    pub job_id: u64,
    pub client: Address,
    pub amount: i128,
}

/// Emitted when a worker accepts a job
#[contractevent]
pub struct JobAccepted {
    pub job_id: u64,
    pub worker: Address,
}

/// Emitted when payment is released to the worker
#[contractevent]
pub struct PaymentReleased {
    pub job_id: u64,
    pub worker: Address,
    pub amount: i128,
}

/// Emitted when an escrow is cancelled by the client
#[contractevent]
pub struct EscrowCancelled {
    pub job_id: u64,
    pub client: Address,
}

/// Emitted when a dispute is raised on an escrow
#[contractevent]
pub struct EscrowDisputed {
    pub job_id: u64,
    pub caller: Address,
}
