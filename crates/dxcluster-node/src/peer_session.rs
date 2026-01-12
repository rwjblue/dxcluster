use std::io;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use dxcluster_types::NodeId;
use dxcluster_wire::PeerFrame;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::{RwLock, broadcast, mpsc};
use tokio::time::{Duration, interval};

use crate::config::PeerOptions;
use crate::state::{NodeState, SpotAnnouncement};

#[derive(Debug)]
pub struct PeerSession {
    state: NodeState,
    options: PeerOptions,
    auth_token: Option<String>,
}

impl PeerSession {
    pub fn new(state: NodeState, options: PeerOptions, auth_token: Option<String>) -> Self {
        Self {
            state,
            options,
            auth_token,
        }
    }

    pub async fn run(
        self,
        stream: TcpStream,
        mut shutdown: broadcast::Receiver<()>,
    ) -> io::Result<()> {
        let (reader, writer) = tokio::io::split(stream);
        let mut reader = BufReader::new(reader);
        let (tx, mut rx) = mpsc::unbounded_channel::<PeerFrame>();
        let remote_id = Arc::new(RwLock::new(None::<NodeId>));
        let auth_ok = Arc::new(AtomicBool::new(self.options.expected_auth_token.is_none()));

        let writer_task = tokio::spawn(async move {
            let mut writer = writer;
            while let Some(frame) = rx.recv().await {
                let line = frame.to_line();
                if writer.write_all(line.as_bytes()).await.is_err() {
                    break;
                }
                if writer.write_all(b"\n").await.is_err() {
                    break;
                }
            }
        });

        tx.send(PeerFrame::Hello {
            node_id: self.state.node_id().clone(),
            version: self.options.version.clone(),
        })
        .ok();
        tx.send(PeerFrame::Capabilities {
            values: self.options.capabilities.clone(),
        })
        .ok();
        if let Some(token) = self.auth_token.clone() {
            tx.send(PeerFrame::Auth { token }).ok();
        }

        let recent_spots = self.state.recent(50).await;
        for spot in recent_spots {
            let mut spot = spot.clone();
            spot.hop = spot.hop.saturating_add(1);
            tx.send(PeerFrame::Spot { spot }).ok();
        }

        let heartbeat_interval = self.options.heartbeat_interval.max(Duration::from_secs(1));
        let heartbeat_tx = tx.clone();
        let mut heartbeat_shutdown = shutdown.resubscribe();
        tokio::spawn(async move {
            let mut ticker = interval(heartbeat_interval);
            let mut counter = 0u64;
            loop {
                tokio::select! {
                    _ = heartbeat_shutdown.recv() => break,
                    _ = ticker.tick() => {
                        counter += 1;
                        if heartbeat_tx.send(PeerFrame::Heartbeat { nonce: counter.to_string() }).is_err() {
                            break;
                        }
                    }
                }
            }
        });

        let forward_tx = tx.clone();
        let mut spot_rx = self.state.subscribe_spots();
        let forward_remote = remote_id.clone();
        let forward_auth = auth_ok.clone();
        let mut forward_shutdown = shutdown.resubscribe();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = forward_shutdown.recv() => break,
                    received = spot_rx.recv() => match received {
                        Ok(announcement) => {
                            if !forward_auth.load(Ordering::Relaxed) {
                                continue;
                            }
                            if should_forward(&forward_remote, &announcement).await {
                                let mut spot = announcement.spot.clone();
                                spot.hop = spot.hop.saturating_add(1);
                                if forward_tx.send(PeerFrame::Spot { spot }).is_err() {
                                    break;
                                }
                            }
                        }
                        Err(broadcast::error::RecvError::Closed) => break,
                        Err(broadcast::error::RecvError::Lagged(_)) => continue,
                    }
                }
            }
        });

        let state = self.state.clone();
        let mut line = String::new();
        loop {
            line.clear();
            tokio::select! {
                _ = shutdown.recv() => {
                    break;
                }
                read = reader.read_line(&mut line) => {
                    let read = read?;
                    if read == 0 {
                        break;
                    }
                    if let Ok(frame) = PeerFrame::parse(&line) {
                        handle_frame(
                            frame,
                            &state,
                            &remote_id,
                            &auth_ok,
                            &tx,
                            self.options.expected_auth_token.as_deref(),
                        ).await?;
                    }
                }
            }
        }

        drop(tx);
        let _ = writer_task.await;
        Ok(())
    }
}

async fn should_forward(
    remote_id: &Arc<RwLock<Option<NodeId>>>,
    announcement: &SpotAnnouncement,
) -> bool {
    if let Some(source) = &announcement.source
        && let Some(remote) = remote_id.read().await.as_ref()
    {
        return source != remote;
    }
    true
}

async fn handle_frame(
    frame: PeerFrame,
    state: &NodeState,
    remote_id: &Arc<RwLock<Option<NodeId>>>,
    auth_ok: &Arc<AtomicBool>,
    tx: &mpsc::UnboundedSender<PeerFrame>,
    expected_auth_token: Option<&str>,
) -> io::Result<()> {
    match frame {
        PeerFrame::Hello { node_id, .. } => {
            let mut guard = remote_id.write().await;
            *guard = Some(node_id);
        }
        PeerFrame::Capabilities { .. } => {}
        PeerFrame::Auth { token } => {
            if let Some(expected) = expected_auth_token
                && token != expected
            {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "auth failed",
                ));
            }
            auth_ok.store(true, Ordering::Relaxed);
        }
        PeerFrame::Spot { mut spot } => {
            if !auth_ok.load(Ordering::Relaxed) {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "auth required",
                ));
            }
            let origin = if spot.origin.is_none() {
                remote_id.read().await.clone()
            } else {
                None
            };
            if let Some(origin) = origin {
                spot.origin = Some(origin);
            }
            let source = remote_id.read().await.clone();
            state.insert_with_source(spot, source).await;
        }
        PeerFrame::Heartbeat { .. } => {}
        PeerFrame::Ping { nonce } => {
            let _ = tx.send(PeerFrame::Pong { nonce });
        }
        PeerFrame::Pong { .. } => {}
    }
    Ok(())
}
