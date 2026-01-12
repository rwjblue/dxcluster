use std::net::SocketAddr;
use std::time::Duration;

use dxcluster_types::NodeId;

#[derive(Debug, Clone)]
pub struct NodeConfig {
    pub user_listen: SocketAddr,
    pub peer_listen: Option<SocketAddr>,
    pub node_id: NodeId,
    pub peer_options: PeerOptions,
    pub peer_retry: PeerRetryPolicy,
}

#[derive(Debug, Clone)]
pub struct PeerOptions {
    pub version: String,
    pub capabilities: Vec<String>,
    pub heartbeat_interval: Duration,
    pub expected_auth_token: Option<String>,
}

impl Default for PeerOptions {
    fn default() -> Self {
        Self {
            version: "1".to_string(),
            capabilities: vec!["spots-v1".to_string(), "heartbeat".to_string()],
            heartbeat_interval: Duration::from_secs(10),
            expected_auth_token: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PeerRetryPolicy {
    pub base_delay: Duration,
    pub max_delay: Duration,
}

impl Default for PeerRetryPolicy {
    fn default() -> Self {
        Self {
            base_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(30),
        }
    }
}

#[derive(Debug, Clone)]
pub enum UpstreamMode {
    Telnet,
    Peer,
}

#[derive(Debug, Clone)]
pub struct UpstreamConfig {
    pub addr: String,
    pub mode: UpstreamMode,
    pub login_callsign: Option<String>,
    pub auth_token: Option<String>,
}
