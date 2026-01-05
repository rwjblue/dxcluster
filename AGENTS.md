# Repository Guide for dxcluster

- All crates live under `crates/`. When adding new packages, keep them in that directory and update the workspace members list.
- Library crates should use `thiserror` for error types. Binary crates should use `anyhow` for fallible entrypoints.
- Prefer workspace dependencies declared in the root `Cargo.toml` so versions stay unified.
- Run `cargo fmt` on Rust changes when adding substantial code.
- Keep README files up to date when altering the workspace layout or developer workflows.
