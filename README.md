# dxcluster

A Rust workspace for experimenting with DX cluster-style radio spot distribution. The project is split into small crates so the
low-level primitives stay dependency-light while higher layers add networking and runtime concerns.

## Workspace layout
All crates live under the [`crates/`](crates) directory:

- `dxcluster-types`: primitives such as callsigns, bands, frequencies, and identifiers (including band mappings from 160m through 70cm).
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

## How this project compares to classic DX Cluster systems

Classic DX Cluster software (e.g., DXSpider, AR-Cluster, and related implementations described at [dxcluster.org](https://www.dxcluster.org)) focuses on telnet-based interactive shells that exchange DX spots, private messages, and node-to-node traffic. Our goal is protocol compatibility with those ecosystems while modernizing the stack for easier deployment and scaling:

- **Protocol compatibility first**: We target telnet-style user sessions, peer links, and filtering semantics that match the behavior described in the DX Cluster user and admin manuals. Existing clients should be able to connect without change.
- **Modern runtime**: The Rust workspace is designed for container-friendly builds, horizontal scaling, and observability, so operators can run clusters with cloud tooling instead of bespoke host setups.
- **Composable crates**: Core types, protocol framing, and runtime layers are separated so downstream projects can embed only the pieces they need (e.g., building a custom node, a relay, or a specialized client).
- **Security and resilience**: While legacy clusters often rely on trust and static ACLs, we plan to add authentication hooks, auditing, and rate limiting to withstand untrusted networks.

## Primary usage scenarios we support

- **Casual operators**: Connect via telnet or a simple CLI, authenticate with a callsign, and issue familiar commands (e.g., `sh/dx`, `set/filter`, private messages). Filtering must mirror DXSpider-style syntax so users can constrain by band, mode, prefix, and geographic hints.
- **Cluster sysops**: Run a node that accepts inbound user sessions, peer links, and console access. Sysops should be able to configure filters, user accounts, forwarding rules, and spot validity windows through declarative configuration files or admin commands.
- **Upstream/downstream peers**: Maintain peer-to-peer links that exchange DX spots, announcements, and node heartbeats. The wire format and throttling behavior should interoperate with existing nodes so this implementation can join established networks.
- **Application developers**: Consume the underlying crates to embed DX spot ingestion, caching, or translation into new tools (e.g., web dashboards or alerting systems) without running a full node.

## Roadmap overview

See [ROADMAP.md](ROADMAP.md) for the prioritized compatibility milestones and modernization work that map to these usage scenarios.

## Next steps

The node now accepts TCP user sessions with banner/prompt framing and can ingest or query spots via the `dxcluster-wire` user protocol. Upcoming work focuses on peer links, authentication, and richer filtering plus history management across restarts.
