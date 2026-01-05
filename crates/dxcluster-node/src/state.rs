use std::sync::Arc;

use dxcluster_model::{Spot, SpotCache};
use dxcluster_types::NodeId;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct NodeState {
    node_id: NodeId,
    cache: Arc<Mutex<SpotCache>>,
}

impl NodeState {
    pub fn new(node_id: NodeId) -> Self {
        Self {
            node_id,
            cache: Arc::new(Mutex::new(SpotCache::new(256))),
        }
    }

    pub async fn insert(&self, spot: Spot) {
        let mut cache = self.cache.lock().await;
        cache.push(spot);
    }

    pub async fn recent(&self, n: usize) -> Vec<Spot> {
        let cache = self.cache.lock().await;
        cache.recent(n).cloned().collect()
    }

    pub fn node_id(&self) -> &NodeId {
        &self.node_id
    }
}
