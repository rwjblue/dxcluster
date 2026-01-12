use std::net::SocketAddr;
use std::time::Duration;

use clap::Parser;
use dxcluster_node::{
    Node, NodeConfig, PeerOptions, PeerRetryPolicy, UpstreamConfig, UpstreamMode,
};
use dxcluster_types::NodeId;

#[derive(Debug, Parser)]
#[command(name = "dxcluster-node-bin", about = "Run a DX cluster node")]
struct Args {
    /// Address to listen for user telnet sessions.
    #[arg(long, default_value = "0.0.0.0:7300")]
    user_listen: SocketAddr,
    /// Address to listen for inbound peer links.
    #[arg(long)]
    peer_listen: Option<SocketAddr>,
    /// Outbound peer addresses to connect to (repeatable).
    #[arg(long = "peer", value_name = "ADDR")]
    peers: Vec<String>,
    /// Local node identifier used in peer handshakes.
    #[arg(long, default_value = "local")]
    node_id: String,
    /// Base retry delay for outbound peer reconnection attempts (ms).
    #[arg(long, default_value_t = 1_000)]
    peer_retry_base_ms: u64,
    /// Maximum retry delay for outbound peer reconnection attempts (ms).
    #[arg(long, default_value_t = 30_000)]
    peer_retry_max_ms: u64,
    /// Heartbeat interval for peer links (ms).
    #[arg(long, default_value_t = 10_000)]
    peer_heartbeat_ms: u64,
    /// Optional auth token to present to outbound peers.
    #[arg(long)]
    peer_auth_token: Option<String>,
    /// Optional auth token required from inbound peers.
    #[arg(long)]
    peer_expected_token: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    let peer_options = PeerOptions {
        heartbeat_interval: Duration::from_millis(args.peer_heartbeat_ms),
        expected_auth_token: args.peer_expected_token.clone(),
        ..PeerOptions::default()
    };
    let peer_retry = PeerRetryPolicy {
        base_delay: Duration::from_millis(args.peer_retry_base_ms),
        max_delay: Duration::from_millis(args.peer_retry_max_ms),
    };

    let config = NodeConfig {
        user_listen: args.user_listen,
        peer_listen: args.peer_listen,
        node_id: NodeId(args.node_id),
        peer_options,
        peer_retry,
    };

    let mut builder = Node::builder(config);
    for addr in args.peers {
        builder = builder.with_upstream(UpstreamConfig {
            addr,
            mode: UpstreamMode::Peer,
            login_callsign: None,
            auth_token: args.peer_auth_token.clone(),
        });
    }

    let _handle = builder.spawn().await?;
    Ok(())
}
