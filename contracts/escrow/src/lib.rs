#![no_std]

mod errors;
mod escrow;
mod events;
mod storage;
mod types;

use soroban_sdk::{contract, contractimpl, token, Address, Env};

use crate::errors::Error;
use crate::events::{EscrowCancelled, EscrowCreated, EscrowDisputed, JobAccepted, PaymentReleased};
use crate::storage::{load_escrow, next_job_id, save_escrow};
use crate::types::{EscrowState, JobStatus};

/// PayShield Escrow Contract
///
/// Enables trustless gig worker payments on Stellar.
/// Clients lock USDC into escrow; funds are released
/// to workers only when work is approved.
#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    /// Create a new escrow job.
    ///
    /// The client deposits USDC into the contract.
    /// A unique job ID is generated and the escrow
    /// state is initialized as `Open`.
    ///
    /// # Arguments
    /// * `client`  - The wallet address of the client posting the job
    /// * `amount`  - The USDC amount to lock in escrow (in stroops)
    /// * `token`   - The USDC token contract address
    ///
    /// # Returns
    /// * `u64` - The unique job ID for this escrow
    pub fn create_escrow(
        env: Env,
        client: Address,
        amount: i128,
        token: Address,
    ) -> Result<u64, Error> {
        client.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let job_id = next_job_id(&env);

        // Pull the USDC from the client into this contract.
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&client, env.current_contract_address(), &amount);

        let state = EscrowState {
            job_id,
            client: client.clone(),
            worker: None,
            token,
            amount,
            status: JobStatus::Open,
            created_at: env.ledger().timestamp(),
        };
        save_escrow(&env, &state);

        EscrowCreated {
            job_id,
            client,
            amount,
        }
        .publish(&env);

        Ok(job_id)
    }

    /// Worker accepts an open job.
    ///
    /// Marks the escrow as `InProgress` and assigns
    /// the worker's address to the job.
    ///
    /// # Arguments
    /// * `worker` - The wallet address of the worker accepting the job
    /// * `job_id` - The ID of the job to accept
    pub fn accept_job(env: Env, worker: Address, job_id: u64) -> Result<(), Error> {
        worker.require_auth();

        let mut state = load_escrow(&env, job_id)?;

        if state.status != JobStatus::Open {
            return Err(Error::EscrowNotOpen);
        }

        state.worker = Some(worker.clone());
        state.status = JobStatus::InProgress;
        save_escrow(&env, &state);

        JobAccepted { job_id, worker }.publish(&env);

        Ok(())
    }

    /// Client approves work and releases USDC to worker.
    ///
    /// Transfers the locked USDC from the contract
    /// to the worker's Stellar wallet and marks the
    /// escrow as `Completed`.
    ///
    /// # Arguments
    /// * `client` - The wallet address of the client (must match job creator)
    /// * `job_id` - The ID of the job to release payment for
    pub fn release_payment(env: Env, client: Address, job_id: u64) -> Result<(), Error> {
        client.require_auth();

        let mut state = load_escrow(&env, job_id)?;

        if state.client != client {
            return Err(Error::NotClient);
        }
        if state.status != JobStatus::InProgress {
            return Err(Error::EscrowNotInProgress);
        }

        // Worker is guaranteed to be set once status is InProgress
        // (accept_job is the only path that sets it).
        let worker = state.worker.clone().ok_or(Error::EscrowNotInProgress)?;

        let token_client = token::Client::new(&env, &state.token);
        token_client.transfer(&env.current_contract_address(), &worker, &state.amount);

        state.status = JobStatus::Completed;
        save_escrow(&env, &state);

        PaymentReleased {
            job_id,
            worker,
            amount: state.amount,
        }
        .publish(&env);

        Ok(())
    }

    /// Client cancels escrow and reclaims USDC.
    ///
    /// Only allowed before a worker has accepted the job.
    /// Returns the locked USDC back to the client.
    ///
    /// # Arguments
    /// * `client` - The wallet address of the client (must match job creator)
    /// * `job_id` - The ID of the job to cancel
    pub fn cancel_escrow(env: Env, client: Address, job_id: u64) -> Result<(), Error> {
        client.require_auth();

        let mut state = load_escrow(&env, job_id)?;

        if state.client != client {
            return Err(Error::NotClient);
        }
        if state.status != JobStatus::Open {
            return Err(Error::EscrowNotOpen);
        }

        let token_client = token::Client::new(&env, &state.token);
        token_client.transfer(&env.current_contract_address(), &client, &state.amount);

        state.status = JobStatus::Cancelled;
        save_escrow(&env, &state);

        EscrowCancelled { job_id, client }.publish(&env);

        Ok(())
    }

    /// Either party flags a dispute for resolution.
    ///
    /// Marks the escrow as `Disputed` so off-chain or
    /// arbitration logic can take over resolution.
    ///
    /// # Arguments
    /// * `caller` - The wallet address of the client or worker raising the dispute
    /// * `job_id` - The ID of the job being disputed
    pub fn dispute_escrow(env: Env, caller: Address, job_id: u64) -> Result<(), Error> {
        caller.require_auth();

        let mut state = load_escrow(&env, job_id)?;

        if state.status != JobStatus::InProgress {
            return Err(Error::EscrowNotInProgress);
        }

        let is_client = state.client == caller;
        let is_worker = state.worker.as_ref() == Some(&caller);
        if !is_client && !is_worker {
            return Err(Error::NotParty);
        }

        state.status = JobStatus::Disputed;
        save_escrow(&env, &state);

        EscrowDisputed { job_id, caller }.publish(&env);

        Ok(())
    }

    /// Read the current state of an escrow.
    ///
    /// # Arguments
    /// * `job_id` - The ID of the job to look up
    ///
    /// # Returns
    /// * `EscrowState` - The full on-chain state of the escrow
    pub fn get_escrow(env: Env, job_id: u64) -> Result<EscrowState, Error> {
        load_escrow(&env, job_id)
    }
}

#[cfg(test)]
mod test;
