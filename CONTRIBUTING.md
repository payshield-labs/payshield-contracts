# Contributing to PayShield Contracts

Thank you for your interest in contributing to PayShield! This repository contains the Soroban smart contract implementation for trustless escrow payments on Stellar.

## Repository Overview

The project currently contains a single Soroban contract:

- `contracts/escrow/` — the escrow smart contract implementation
  - `src/lib.rs` — contract entrypoint and exported contract methods
  - `src/escrow.rs` — core escrow business logic
  - `src/storage.rs` — on-chain storage definitions and helpers
  - `src/events.rs` — event emission helpers
  - `src/types.rs` — contract-specific data types and enums

## What We’re Looking For

We welcome contributions in the following areas:

- Implementing missing contract logic and completing TODOs in `contracts/escrow/src/lib.rs`
- Adding tests and validation for escrow workflows
- Improving storage, event handling, and contract safety
- Enhancing documentation and developer onboarding
- Fixing bugs and refining contract invariants

## Before You Begin

### 1. Pick an issue or open a new one

- Search existing issues first.
- If you want to propose a change or need clarification, open an issue describing the problem and your proposed solution.

### 2. Fork the repository

- Fork this repo to your GitHub account.
- Create a feature branch using a descriptive name:

```bash
git checkout -b feature/escrow-logic
```

## Development Setup

### Prerequisites

- Install Rust via `rustup`
- Install Soroban CLI:

```bash
cargo install --locked soroban-cli
```

### Build and test

From the root of the repository, run:

```bash
cd contracts/escrow
cargo build
cargo test
```

If you want to target the manifest explicitly from root:

```bash
cargo build --manifest-path contracts/escrow/Cargo.toml
cargo test --manifest-path contracts/escrow/Cargo.toml
```

### Formatting and linting

Use standard Rust formatting and linting:

```bash
cargo fmt
cargo clippy -- -D warnings
```

If you add new code, please keep it formatted and idiomatic.

## Coding Guidelines

- Keep contract execution deterministic and gas-efficient.
- Use `soroban_sdk` primitives and contract patterns consistently.
- Preserve the `#[no_std]` contract environment.
- Do not add unnecessary runtime allocations.
- Make authentication explicit using `.require_auth()` for actor addresses.
- Validate all inputs clearly, including amounts and job state transitions.

## Contract Implementation Notes

The current smart contract skeleton exposes core methods such as:

- `create_escrow`
- `accept_job`
- `release_payment`
- `cancel_escrow`

The implementation is expected to:

- use on-chain storage to persist escrow state
- emit meaningful events for state changes
- ensure only authorized parties can perform sensitive actions
- handle job lifecycle transitions safely

## Testing

Tests are highly encouraged for every meaningful change.

- Add unit tests directly in the contract crate as `#[cfg(test)]` modules.
- Test valid flows and error conditions.
- Verify state updates and expected event emissions.

## Submitting a Pull Request

When your change is ready:

1. Push your branch to your fork.
2. Open a pull request against the upstream `main` branch.
3. In your PR description, include:
   - what you changed
   - why you changed it
   - how to test it

## PR Checklist

- [ ] I have read and followed this `CONTRIBUTING.md`
- [ ] My code builds successfully
- [ ] I added or updated tests for my changes
- [ ] I ran `cargo fmt`
- [ ] I included a clear PR description

## Code Review and Feedback

- All contributions are reviewed before merging.
- Please be responsive to review comments.
- Maintain a collaborative and respectful tone.

## License

By contributing, you agree that your contributions will be made under the repository's existing MIT license.

---

Thanks for helping make PayShield better. Your contributions power safer, fairer escrow payments for gig workers and clients worldwide.