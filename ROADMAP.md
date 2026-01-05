# Roadmap

This roadmap focuses on interoperability with existing DX Cluster ecosystems (per the user and admin manuals at [dxcluster.org](https://www.dxcluster.org/usermanual_en.html)) while adopting a modern, container-friendly stack. Milestones are organized by the primary usage scenarios: casual operators, cluster sysops, peer links, and application developers.

## 1. Protocol compatibility foundation
- **Telnet user shell parity**: Implement login banner, callsign validation, and classic commands (`sh/dx`, `sh/ann`, `set/filter`, `dir`, `help`, private messages). Parsing/formatting must mirror DXSpider-style quirks so legacy clients behave identically.
- **DX spot model**: Represent bands, modes, SNR/remarks, timestamps, and optional origin metadata exactly as legacy clusters forward them.
- **Peer link framing**: Encode/decode node-to-node spot and announce frames, including flood control, hop counts, and duplicate suppression to avoid loops.
- **Filtering semantics**: Support inclusion/exclusion rules by band, prefix, continent, callsign regexes, and node origin as described in the filtering reference. Ensure filters apply consistently across user sessions and peer forwarding paths.

## 2. Operator experience and administration
- **Accounts and ACLs**: Add account persistence with permissions for users vs. sysops. Support per-user filter defaults and login scripts.
- **Admin console**: Provide telnet or CLI-based admin commands for user management, link status, queues, and runtime metrics. Keep command names aligned with established manuals to reduce re-training.
- **Configuration**: Ship declarative configuration (files/env) for ports, peer definitions, motd/help text, rate limits, and logging destinations.
- **Observability**: Integrate structured logging, metrics, and tracing so operators can troubleshoot spot propagation and peer health.

## 3. Networking and scalability
- **Connection handling**: Harden TCP handling (timeouts, keepalives, backpressure) for user and peer sessions. Add TLS and proxy support for modern deployments.
- **Horizontal scaling**: Allow multiple node instances to share state via pluggable backends (e.g., Redis/PostgreSQL) for user sessions, spot caches, and rate limits.
- **Replay and buffering**: Implement bounded queues and replay windows so newly connected peers/users receive recent spots without overwhelming the network.

## 4. Developer enablement
- **Library boundaries**: Stabilize crates for types, parsing, and models with semantic versioning to support downstream tooling.
- **SDKs and examples**: Provide examples for embedding the node engine, building a custom peer bridge, or consuming spots via an async stream.
- **Testing harness**: Add golden tests for parsing/formatting against captured DXSpider transcripts and fuzz tests for filter expressions.

## 5. Compatibility validation and rollout
- **Interop matrix**: Test against common clients (DXTelnet-like telnet shells) and peer nodes to verify command acceptance, spot forwarding, and filter behavior.
- **Documentation**: Maintain operator and developer guides that map features back to the corresponding sections of the classic DX Cluster manuals.
- **Migration aids**: Offer conversion scripts for existing configuration files (nodes, filters, user databases) to ease adoption.
