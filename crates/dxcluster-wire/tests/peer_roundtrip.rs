use dxcluster_types::NodeId;
use dxcluster_wire::PeerFrame;

#[test]
fn hello_roundtrips() {
    let frame = PeerFrame::Hello {
        node_id: NodeId(String::from("DXNODE")),
        version: String::from("1.2.3"),
    };

    let formatted = frame.to_line();
    let parsed = PeerFrame::parse(&formatted).expect("hello frame should parse");
    assert_eq!(parsed, frame);
}

#[test]
fn ping_roundtrips() {
    let frame = PeerFrame::Ping {
        nonce: String::from("abc123"),
    };

    let formatted = frame.to_line();
    let parsed = PeerFrame::parse(&formatted).expect("ping frame should parse");
    assert_eq!(parsed, frame);
}

#[test]
fn pong_roundtrips() {
    let frame = PeerFrame::Pong {
        nonce: String::from("xyz789"),
    };

    let formatted = frame.to_line();
    let parsed = PeerFrame::parse(&formatted).expect("pong frame should parse");
    assert_eq!(parsed, frame);
}
