# dxcluster-cli

`dxcluster-cli` is a small async command-line utility for interacting with DX
Cluster nodes over telnet using the `dxcluster-client` crate.

## Subcommands

- `login` – connect to a node and print the first server response.
- `spot` – submit a DX spot using the standard `DX` command.
- `list` – request the server's recent spots (via `SH/DX`).
- `watch` – stream user-facing lines until the connection closes.

The following flags apply to every subcommand:

- `--addr <host:port>`: address of the node (default `127.0.0.1:7300`).
- `--callsign <call>`: callsign to log in with.
- `--password <secret>`: optional password to send to the node.

## Examples

```bash
# Connect to a cluster node and print the greeting
dxcluster-cli --addr cluster.example:7300 --callsign N0CALL login

# Connect with a password to a private cluster
dxcluster-cli --addr cluster.example:7300 --callsign N0CALL --password hunter2 login

# Submit a new FT8 spot
dxcluster-cli --callsign N0CALL --password hunter2 spot K1ABC 14074 "ft8 cq dx"

# Show recent spots once
dxcluster-cli list

# Continuously watch user-visible output
dxcluster-cli --addr 192.0.2.10:7300 watch

# Spot followed by a one-shot listing
dxcluster-cli --callsign N0CALL spot K1ABC 14074 "ft8 cq"
dxcluster-cli --callsign N0CALL list
```
