use std::net::{SocketAddr, TcpListener};

use dxcluster_node::{Node, NodeConfig};
use dxcluster_types::NodeId;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

fn ephemeral_addr() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind temp port");
    let addr = listener.local_addr().expect("addr");
    drop(listener);
    addr
}

async fn connect_client(
    addr: SocketAddr,
) -> (
    BufReader<tokio::net::tcp::OwnedReadHalf>,
    tokio::net::tcp::OwnedWriteHalf,
) {
    let stream = TcpStream::connect(addr).await.expect("connect client");
    let (read_half, write_half) = stream.into_split();
    (BufReader::new(read_half), write_half)
}

async fn read_line(reader: &mut BufReader<tokio::net::tcp::OwnedReadHalf>) -> String {
    let mut buf = String::new();
    reader.read_line(&mut buf).await.expect("read line");
    buf
}

#[tokio::test]
async fn user_can_submit_and_query_spot() {
    let addr = ephemeral_addr();
    let config = NodeConfig {
        user_listen: addr,
        peer_listen: None,
        node_id: NodeId("test-node".into()),
    };

    let handle = Node::builder(config).spawn().await.expect("spawn node");

    let (mut reader1, mut writer1) = connect_client(addr).await;
    read_line(&mut reader1).await; // banner
    read_line(&mut reader1).await; // prompt

    writer1
        .write_all(b"DX K1ABC 14074 first spot\n")
        .await
        .expect("write dx command");

    let spot_line = read_line(&mut reader1).await;
    assert!(spot_line.contains("DX de ANON"));
    assert!(spot_line.contains("K1ABC"));
    read_line(&mut reader1).await; // prompt

    let (mut reader2, mut writer2) = connect_client(addr).await;
    read_line(&mut reader2).await; // banner
    read_line(&mut reader2).await; // prompt

    writer2.write_all(b"SH/DX\n").await.expect("write sh/dx");

    let show_line = read_line(&mut reader2).await;
    assert!(show_line.contains("K1ABC"));
    assert!(show_line.contains("first spot"));
    read_line(&mut reader2).await; // prompt

    handle.shutdown().await;
}

#[tokio::test]
async fn heartbeat_and_filters_reported() {
    let addr = ephemeral_addr();
    let config = NodeConfig {
        user_listen: addr,
        peer_listen: None,
        node_id: NodeId("test-node".into()),
    };

    let handle = Node::builder(config).spawn().await.expect("spawn node");
    let (mut reader, mut writer) = connect_client(addr).await;

    read_line(&mut reader).await; // banner
    read_line(&mut reader).await; // prompt

    writer.write_all(b"PING\n").await.expect("write heartbeat");

    let pong = read_line(&mut reader).await;
    assert!(pong.contains("PONG"));
    read_line(&mut reader).await; // prompt

    writer
        .write_all(b"SH/FILTERS\n")
        .await
        .expect("write filters");

    let filters = read_line(&mut reader).await;
    assert!(filters.contains("Filters"));
    read_line(&mut reader).await; // prompt

    handle.shutdown().await;
}
