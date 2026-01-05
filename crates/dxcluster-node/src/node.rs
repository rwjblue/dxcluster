use dxcluster_model::Spot;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;

use crate::config::{NodeConfig, UpstreamConfig};
use crate::error::NodeError;
use crate::session::UserSession;
use crate::state::NodeState;
use crate::upstream::UpstreamHandle;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Node {
    state: NodeState,
    upstreams: Vec<UpstreamConfig>,
}

#[derive(Debug)]
pub struct NodeHandle {
    state: NodeState,
    shutdown: broadcast::Sender<()>,
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

        let (shutdown, shutdown_rx) = broadcast::channel(8);
        let user_task =
            spawn_user_listener(self.config.user_listen, state.clone(), shutdown_rx).await?;

        Ok(NodeHandle {
            state,
            shutdown,
            tasks: vec![user_task],
        })
    }
}

impl NodeHandle {
    pub async fn shutdown(self) {
        let _ = self.shutdown.send(());
        for task in self.tasks {
            let _ = task.await;
        }
    }

    pub async fn inject_spot(&self, spot: Spot) {
        self.state.insert(spot).await;
    }

    pub async fn recent_spots(&self, n: usize) -> Vec<Spot> {
        self.state.recent(n).await
    }
}

async fn spawn_user_listener(
    addr: std::net::SocketAddr,
    state: NodeState,
    mut shutdown: broadcast::Receiver<()>,
) -> Result<JoinHandle<()>, NodeError> {
    let listener = TcpListener::bind(addr)
        .await
        .map_err(|_| NodeError::Listener)?;
    Ok(tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = shutdown.recv() => {
                    break;
                }
                accept_res = listener.accept() => {
                    match accept_res {
                        Ok((stream, _addr)) => {
                            let session = UserSession::new(stream, state.clone());
                            tokio::spawn(async move {
                                if let Err(err) = session.run().await {
                                    tracing::warn!(?err, "user session terminated with error");
                                }
                            });
                        }
                        Err(err) => {
                            tracing::error!(?err, "failed to accept user connection");
                            break;
                        }
                    }
                }
            }
        }
    }))
}
