use std::net::{SocketAddr, TcpListener};
use std::time::Duration;

use dxcluster_model::Spot;
use dxcluster_node::{
    Node, NodeConfig, PeerOptions, PeerRetryPolicy, UpstreamConfig, UpstreamMode,
};
use dxcluster_types::{Callsign, FrequencyHz, NodeId, SpotId};
use tokio::time::{sleep, timeout};

fn ephemeral_addr() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind temp port");
    let addr = listener.local_addr().expect("addr");
    drop(listener);
    addr
}

fn make_spot(origin: &str, dx: &str, comment: &str) -> Spot {
    let ts = time::OffsetDateTime::now_utc();
    let spot_id = SpotId::hash_components(&[
        origin.as_bytes(),
        dx.as_bytes(),
        comment.as_bytes(),
        &ts.unix_timestamp().to_be_bytes(),
    ]);
    Spot {
        spot_id,
        ts,
        freq: FrequencyHz(14_074_000),
        dx: Callsign::parse_loose(dx).expect("dx callsign"),
        spotter: Callsign::parse_loose("N0CALL").expect("spotter callsign"),
        comment: comment.to_string(),
        origin: Some(NodeId(origin.to_string())),
        hop: 0,
    }
}

async fn wait_for_dx(handle: &dxcluster_node::NodeHandle, dx: &str) {
    timeout(Duration::from_secs(3), async {
        loop {
            let spots = handle.recent_spots(20).await;
            if spots.iter().any(|spot| spot.dx.as_str() == dx) {
                break;
            }
            sleep(Duration::from_millis(50)).await;
        }
    })
    .await
    .expect("spot should propagate");
}

#[tokio::test]
async fn peer_links_exchange_spots() {
    let node_a_addr = ephemeral_addr();
    let node_b_addr = ephemeral_addr();
    let peer_listen_b = ephemeral_addr();

    let peer_options = PeerOptions {
        heartbeat_interval: Duration::from_millis(200),
        ..PeerOptions::default()
    };
    let peer_retry = PeerRetryPolicy {
        base_delay: Duration::from_millis(50),
        max_delay: Duration::from_millis(200),
    };

    let config_b = NodeConfig {
        user_listen: node_b_addr,
        peer_listen: Some(peer_listen_b),
        node_id: NodeId("node-b".into()),
        peer_options: peer_options.clone(),
        peer_retry: peer_retry.clone(),
    };
    let handle_b = Node::builder(config_b).spawn().await.expect("spawn B");

    let config_a = NodeConfig {
        user_listen: node_a_addr,
        peer_listen: None,
        node_id: NodeId("node-a".into()),
        peer_options: peer_options.clone(),
        peer_retry: peer_retry.clone(),
    };
    let handle_a = Node::builder(config_a)
        .with_upstream(UpstreamConfig {
            addr: peer_listen_b.to_string(),
            mode: UpstreamMode::Peer,
            login_callsign: None,
            auth_token: None,
        })
        .spawn()
        .await
        .expect("spawn A");

    handle_a
        .inject_spot(make_spot("node-a", "K1ABC", "from A"))
        .await;
    wait_for_dx(&handle_b, "K1ABC").await;

    handle_b
        .inject_spot(make_spot("node-b", "W1AW", "from B"))
        .await;
    wait_for_dx(&handle_a, "W1AW").await;

    handle_a.shutdown().await;
    handle_b.shutdown().await;
}
