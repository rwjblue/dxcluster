use dxcluster_model::Spot;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;

use crate::config::{NodeConfig, UpstreamConfig};
use crate::error::NodeError;
use crate::session::UserSession;
use crate::state::NodeState;
use crate::upstream::UpstreamHandle;

/// Runtime entrypoint for embedding a DX Cluster node.
///
/// Constructed via [`Node::builder`], the node owns shared state and upstream
/// configurations and can be spawned to accept user sessions and peer links.
#[derive(Debug)]
#[allow(dead_code)]
pub struct Node {
    state: NodeState,
    upstreams: Vec<UpstreamConfig>,
}

/// Handle returned from [`NodeBuilder::spawn`] that allows callers to inject
/// new spots, query the in-memory cache, and trigger graceful shutdown.
#[derive(Debug)]
pub struct NodeHandle {
    state: NodeState,
    shutdown: broadcast::Sender<()>,
    #[allow(dead_code)]
    tasks: Vec<JoinHandle<()>>,
}

/// Builder for configuring and launching a node.
#[derive(Debug)]
pub struct NodeBuilder {
    config: NodeConfig,
    upstreams: Vec<UpstreamConfig>,
}

impl Node {
    /// Start building a node with the provided configuration.
    pub fn builder(config: NodeConfig) -> NodeBuilder {
        NodeBuilder {
            config,
            upstreams: Vec::new(),
        }
    }
}

impl NodeBuilder {
    /// Attach an upstream peer configuration to this node before spawning.
    pub fn with_upstream(mut self, upstream: UpstreamConfig) -> Self {
        self.upstreams.push(upstream);
        self
    }

    /// Spawn the node runtime and return a handle for control and inspection.
    pub async fn spawn(self) -> Result<NodeHandle, NodeError> {
        let state = NodeState::new(self.config.node_id.clone());

        let (shutdown, shutdown_rx) = broadcast::channel(8);
        let user_task =
            spawn_user_listener(self.config.user_listen, state.clone(), shutdown_rx).await?;

        let mut tasks = vec![user_task];

        if let Some(peer_addr) = self.config.peer_listen {
            let peer_task = spawn_peer_listener(
                peer_addr,
                state.clone(),
                self.config.peer_options.clone(),
                shutdown.subscribe(),
            )
            .await?;
            tasks.push(peer_task);
        }

        let upstream_tasks = UpstreamHandle::spawn_all(
            &self.upstreams,
            state.clone(),
            self.config.peer_options.clone(),
            self.config.peer_retry.clone(),
            shutdown.clone(),
        );
        tasks.extend(upstream_tasks.into_iter().map(|handle| handle.task));

        Ok(NodeHandle {
            state,
            shutdown,
            tasks,
        })
    }
}

impl NodeHandle {
    /// Initiate a graceful shutdown and wait for background tasks to finish.
    pub async fn shutdown(self) {
        let _ = self.shutdown.send(());
        for task in self.tasks {
            let _ = task.await;
        }
    }

    /// Insert a spot directly into the node state, useful for tests.
    pub async fn inject_spot(&self, spot: Spot) {
        self.state.insert(spot).await;
    }

    /// Fetch the `n` most recent spots currently stored in memory.
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

async fn spawn_peer_listener(
    addr: std::net::SocketAddr,
    state: NodeState,
    peer_options: crate::config::PeerOptions,
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
                            let session = crate::peer_session::PeerSession::new(
                                state.clone(),
                                peer_options.clone(),
                                None,
                            );
                            let shutdown_rx = shutdown.resubscribe();
                            tokio::spawn(async move {
                                if let Err(err) = session.run(stream, shutdown_rx).await {
                                    tracing::warn!(?err, "peer session terminated with error");
                                }
                            });
                        }
                        Err(err) => {
                            tracing::error!(?err, "failed to accept peer connection");
                            break;
                        }
                    }
                }
            }
        }
    }))
}
