use soroban_sdk::contracterror;

/// Errors returned by the PayShield escrow contract.
///
/// Exposed on-chain via `#[contracterror]` so callers (and the
/// payshield-api indexer) can match on a stable error code instead
/// of parsing panic strings.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// No escrow exists for the given job ID
    EscrowNotFound = 1,
    /// `amount` must be greater than zero
    InvalidAmount = 2,
    /// Escrow is not in the `Open` state (e.g. accept/cancel on a job already accepted)
    EscrowNotOpen = 3,
    /// Escrow is not in the `InProgress` state (e.g. release/dispute before a worker accepted)
    EscrowNotInProgress = 4,
    /// Caller is not the client who created this escrow
    NotClient = 5,
    /// Caller is neither the client nor the assigned worker
    NotParty = 6,
}
