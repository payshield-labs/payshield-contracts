#![allow(unused)]

use soroban_sdk::{contracttype, Address};

/// Represents the lifecycle status of an escrow job
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum JobStatus {
    /// Job has been created, waiting for a worker to accept
    Open,
    /// A worker has accepted the job and work is underway
    InProgress,
    /// Client has approved work and payment has been released
    Completed,
    /// Client cancelled before a worker accepted
    Cancelled,
    /// A dispute has been raised by either party
    Disputed,
}

/// Full on-chain state of a single escrow job
#[contracttype]
#[derive(Clone, Debug)]
pub struct EscrowState {
    /// Unique job identifier
    pub job_id: u64,
    /// Wallet address of the client who created the escrow
    pub client: Address,
    /// Wallet address of the worker (set when job is accepted)
    pub worker: Option<Address>,
    /// USDC token contract address
    pub token: Address,
    /// Locked USDC amount in stroops
    pub amount: i128,
    /// Current lifecycle status of the escrow
    pub status: JobStatus,
    /// Ledger timestamp when the escrow was created
    pub created_at: u64,
}