use dxcluster_model::Spot;
use tokio::task::JoinHandle;

use crate::config::{NodeConfig, UpstreamConfig};
use crate::error::NodeError;
use crate::state::NodeState;
use crate::upstream::UpstreamHandle;

#[derive(Debug)]
pub struct Node {
    state: NodeState,
    upstreams: Vec<UpstreamConfig>,
}

#[derive(Debug)]
pub struct NodeHandle {
    state: NodeState,
    #[allow(dead_code)]
    tasks: Vec<JoinHandle<()>>,
}

#[derive(Debug)]
pub struct NodeBuilder {
    config: NodeConfig,
    upstreams: Vec<UpstreamConfig>,
}

impl Node {
    pub fn builder(config: NodeConfig) -> NodeBuilder {
        NodeBuilder {
            config,
            upstreams: Vec::new(),
        }
    }
}

impl NodeBuilder {
    pub fn with_upstream(mut self, upstream: UpstreamConfig) -> Self {
        self.upstreams.push(upstream);
        self
    }

    pub async fn spawn(self) -> Result<NodeHandle, NodeError> {
        let state = NodeState::new(self.config.node_id.clone());
        let _ = UpstreamHandle::spawn_all(&self.upstreams);
        Ok(NodeHandle {
            state,
            tasks: Vec::new(),
        })
    }
}

impl NodeHandle {
    pub async fn shutdown(self) {}

    pub async fn inject_spot(&self, spot: Spot) {
        self.state.insert(spot).await;
    }

    pub async fn recent_spots(&self, n: usize) -> Vec<Spot> {
        self.state.recent(n).await
    }
}
