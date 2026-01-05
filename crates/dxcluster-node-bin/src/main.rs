use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use dxcluster_node::{Node, NodeConfig};
use dxcluster_types::NodeId;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = NodeConfig {
        user_listen: SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 7300),
        peer_listen: None,
        node_id: NodeId("local".into()),
    };

    let _handle = Node::builder(config).spawn().await?;
    Ok(())
}
