use std::sync::Arc;

use dxcluster_model::{Spot, SpotCache};
use dxcluster_types::NodeId;
use tokio::sync::{Mutex, broadcast};

#[derive(Debug, Clone)]
pub struct SpotAnnouncement {
    pub spot: Spot,
    pub source: Option<NodeId>,
}

#[derive(Debug, Clone)]
pub struct NodeState {
    node_id: NodeId,
    cache: Arc<Mutex<SpotCache>>,
    spot_tx: broadcast::Sender<SpotAnnouncement>,
}

impl NodeState {
    pub fn new(node_id: NodeId) -> Self {
        let (spot_tx, _) = broadcast::channel(256);
        Self {
            node_id,
            cache: Arc::new(Mutex::new(SpotCache::new(256))),
            spot_tx,
        }
    }

    pub async fn insert(&self, spot: Spot) {
        self.insert_with_source(spot, None).await;
    }

    pub async fn insert_with_source(&self, spot: Spot, source: Option<NodeId>) {
        let mut cache = self.cache.lock().await;
        cache.push(spot.clone());
        let _ = self.spot_tx.send(SpotAnnouncement { spot, source });
    }

    pub async fn recent(&self, n: usize) -> Vec<Spot> {
        let cache = self.cache.lock().await;
        cache.recent(n).cloned().collect()
    }

    pub fn node_id(&self) -> &NodeId {
        &self.node_id
    }

    pub fn subscribe_spots(&self) -> broadcast::Receiver<SpotAnnouncement> {
        self.spot_tx.subscribe()
    }
}
