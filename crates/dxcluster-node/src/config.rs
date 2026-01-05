use std::net::SocketAddr;

use dxcluster_types::NodeId;

#[derive(Debug, Clone)]
pub struct NodeConfig {
    pub user_listen: SocketAddr,
    pub peer_listen: Option<SocketAddr>,
    pub node_id: NodeId,
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
}
