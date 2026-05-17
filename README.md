# payshield-contracts

Soroban smart contracts powering trustless escrow for gig worker payments on Stellar.

---

## What is PayShield?

PayShield is an open-source escrow protocol built on the Stellar blockchain. It enables gig workers and clients anywhere in the world to transact with confidence — funds are locked in a Soroban smart contract and only released when work is verified and approved.

No middlemen. No chargebacks. No trust required.

---

## The Problem

Global gig workers face a fundamental payment problem:

- Clients can disappear after work is delivered
- Workers can vanish after payment is sent
- Traditional escrow services are expensive, slow, and inaccessible in many regions
- Cross-border payments carry high fees and long settlement times

Stellar's payment network and Soroban smart contracts make it possible to solve all of these problems in a single, lightweight protocol.

---

## How It Works

```
Client creates a job and deposits USDC into escrow
              ↓
Soroban contract locks the funds on-chain
              ↓
Gig worker accepts the job and completes the work
              ↓
Client approves the deliverable
              ↓
Contract releases USDC directly to the worker's Stellar wallet
```

If a dispute arises, the contract supports a cancellation and refund path back to the client.

---

## Contract Architecture

```
payshield-contracts/
├── contracts/
│   └── escrow/
│       ├── src/
│       │   ├── lib.rs          # Contract entry point
│       │   ├── escrow.rs       # Core escrow logic
│       │   ├── storage.rs      # On-chain state management
│       │   └── events.rs       # Contract event emissions
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

---

## Core Contract Functions

| Function | Description | Status |
|---|---|---|
| `create_escrow` | Client deposits USDC and initializes a job | 🔲 Open for contribution |
| `accept_job` | Worker accepts an open job | 🔲 Open for contribution |
| `release_payment` | Client approves work and releases USDC to worker | 🔲 Open for contribution |
| `cancel_escrow` | Client cancels and reclaims USDC before job is accepted | 🔲 Open for contribution |
| `dispute_escrow` | Either party flags a dispute for resolution | 🔲 Open for contribution |
| `get_escrow` | Read current state of an escrow | 🔲 Open for contribution |

---

## Tech Stack

- **Rust** — Smart contract language
- **Soroban SDK** — Stellar's native smart contract framework
- **Stellar Testnet** — Development and testing environment
- **USDC** — Payment token

---

## Getting Started

### Prerequisites

- Rust toolchain (`rustup`)
- Soroban CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Soroban CLI
cargo install --locked soroban-cli
```

### Setup

```bash
# Clone the repository
git clone https://github.com/payshield-labs/payshield-contracts.git
cd payshield-contracts

# Build the contracts
cargo build

# Run tests
cargo test
```

### Deploy to Testnet

```bash
# Configure Soroban CLI for testnet
soroban network add testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"

# Generate a testnet identity
soroban keys generate --global alice --network testnet

# Deploy the escrow contract
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/escrow.wasm \
  --network testnet \
  --source alice
```

---

## Current Status

This project is in **early development**. The contract architecture and interfaces are defined. Core functions are open for contributors to implement.

See the [Issues](https://github.com/payshield-labs/payshield-contracts/issues) tab for available tasks across all complexity levels.

---

## Contributing

Contributions are what make PayShield grow. We welcome developers at all levels.

1. Browse open issues and pick a task that matches your skill level
2. Fork the repository
3. Create a new branch: `git checkout -b feature/your-feature-name`
4. Implement your changes with clear, commented code
5. Write or update tests where applicable
6. Open a Pull Request with a clear description of what you built

Please read our `CONTRIBUTING.md` before submitting. All PRs are reviewed within 48 hours.

---

## Related Repositories

| Repo | Description |
|---|---|
| [payshield-api](https://github.com/payshield-labs/payshield-api) | NestJS backend — job orchestration and Stellar integration |
| [payshield-app](https://github.com/payshield-labs/payshield-app) | Next.js frontend — wallet connection and job dashboard |

---

## License

MIT — see [LICENSE](./LICENSE) for details.

---

> Built with ❤️ by [PayShield Labs](https://github.com/payshield-labs) — making gig work trustless, global, and fair.
