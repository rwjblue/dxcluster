# dxcluster

A Rust workspace for experimenting with DX cluster-style radio spot distribution. The project is split into small crates so the low-level primitives stay dependency-light while higher layers add networking and runtime concerns.

## Workspace layout
All crates live under the [`crates/`](crates) directory:

- `dxcluster-types`: primitives such as callsigns, bands, frequencies, and identifiers.
- `dxcluster-model`: domain models and pure business logic for spots, caching, and filtering.
- `dxcluster-wire`: parsing and formatting for user- and peer-facing protocols.
- `dxcluster-client`: async clients for telnet-style and peer connections.
- `dxcluster-node`: the embeddable server engine and runtime plumbing.
- `dxcluster-node-bin`: binary entrypoint to run a node with sensible defaults.
- `dxcluster-cli`: a thin client binary that exercises the user protocol.

## Development

- Rust edition: 2024 for every crate (set via workspace defaults).
- Error handling: library crates use [`thiserror`](https://docs.rs/thiserror); binaries use [`anyhow`](https://docs.rs/anyhow).
- Dependencies are unified through the root `Cargo.toml` under `[workspace.dependencies]`.
- Common checks:
  - `cargo check` to verify the workspace builds.
  - `cargo fmt` before submitting substantial Rust changes.

## Next steps

The current code provides a skeleton that matches the proposed API surface. Future work includes fleshing out the parsing/formatting logic, implementing runtime behaviors in the node, and expanding CLI commands beyond the basic connectivity stubs.
