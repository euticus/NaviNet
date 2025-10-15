# NaviNet

Sovereign chain for user-instantiated Networks with their own coins, stake-to-unlock access, and a Service Basket of external services.

## Overview

NaviNet is a Substrate-based blockchain that enables users to create and manage their own networks with custom configurations. The chain includes four core pallets:

- **Identity**: NaviID decentralized identity management
- **Network Factory**: Create user-owned networks with custom coins
- **Access Gate**: Stake-to-unlock resource access with tiered pricing
- **Service Basket**: Manage weighted baskets of external services

## Quickstart

### Prerequisites

Install Rust and the Wasm toolchain:

```bash
rustup default stable
rustup component add rustfmt clippy
rustup target add wasm32-unknown-unknown
```

The project uses Rust 1.79.0 as specified in `chain/rust-toolchain.toml`.

### Build

Build the entire workspace:

```bash
cd chain
cargo build --workspace
```

### Test

Run all tests:

```bash
cd chain
cargo test --workspace --all-features
```

### Code Quality

Ensure code formatting and linting:

```bash
cd chain
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
```

## Project Structure

```
chain/
├── node/           # Node implementation
├── runtime/        # Runtime configuration
├── pallets/        # Custom pallets
│   ├── identity/           # NaviID pallet
│   ├── network-factory/    # Network creation pallet
│   ├── access-gate/        # Access control pallet
│   └── service-basket/     # Service management pallet
└── Cargo.toml      # Workspace configuration
```

## Development

The chain is built using the Polkadot SDK solochain template. Each pallet includes:
- Core logic in `src/lib.rs`
- Mock runtime for testing in `src/mock.rs`
- Unit tests in `src/tests.rs`

## License

MIT-0
