use std::time::Duration;

use tokio::net::TcpStream;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use tokio::time::sleep;

use crate::config::{PeerOptions, PeerRetryPolicy, UpstreamConfig, UpstreamMode};
use crate::peer_session::PeerSession;
use crate::state::NodeState;

#[derive(Debug)]
pub struct UpstreamHandle {
    pub(crate) task: JoinHandle<()>,
}

impl UpstreamHandle {
    pub fn spawn_all(
        configs: &[UpstreamConfig],
        state: NodeState,
        peer_options: PeerOptions,
        retry: PeerRetryPolicy,
        shutdown: broadcast::Sender<()>,
    ) -> Vec<Self> {
        configs
            .iter()
            .filter(|config| matches!(config.mode, UpstreamMode::Peer))
            .map(|config| {
                let addr = config.addr.clone();
                let auth_token = config.auth_token.clone();
                let state = state.clone();
                let options = peer_options.clone();
                let retry = retry.clone();
                let mut shutdown_rx = shutdown.subscribe();
                let task = tokio::spawn(async move {
                    run_peer_connector(addr, state, options, auth_token, retry, &mut shutdown_rx)
                        .await;
                });
                UpstreamHandle { task }
            })
            .collect()
    }
}

async fn run_peer_connector(
    addr: String,
    state: NodeState,
    options: PeerOptions,
    auth_token: Option<String>,
    retry: PeerRetryPolicy,
    shutdown: &mut broadcast::Receiver<()>,
) {
    let mut attempt = 0usize;
    loop {
        let connect = tokio::select! {
            _ = shutdown.recv() => return,
            result = TcpStream::connect(&addr) => result,
        };
        match connect {
            Ok(stream) => {
                attempt = 0;
                let session = PeerSession::new(state.clone(), options.clone(), auth_token.clone());
                if let Err(err) = session.run(stream, shutdown.resubscribe()).await {
                    tracing::warn!(?err, addr, "peer session ended");
                }
            }
            Err(err) => {
                attempt += 1;
                tracing::warn!(?err, addr, attempt, "peer connect failed");
            }
        }

        let delay = backoff_delay(&retry, attempt);
        tokio::select! {
            _ = shutdown.recv() => return,
            _ = sleep(delay) => {}
        };
    }
}

fn backoff_delay(retry: &PeerRetryPolicy, attempt: usize) -> Duration {
    if attempt == 0 {
        return retry.base_delay;
    }
    let exp = 2u32.saturating_pow(attempt.saturating_sub(1) as u32);
    let delay = retry.base_delay.saturating_mul(exp);
    delay.min(retry.max_delay)
}
