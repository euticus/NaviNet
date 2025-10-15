# NaviNet

# NaviNet (Substrate)

Sovereign chain for user-instantiated Networks with their own coins, stake-to-unlock access, and a Service Basket of external services.

## Quickstart
- Install Rust + Wasm toolchain:
  ```bash
  rustup default stable
  rustup component add rustfmt clippy
  rustup target add wasm32-unknown-unknown
Build & test:

cargo build --workspace
cargo test --workspace --all-features
