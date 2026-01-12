use dxcluster_model::Spot;
use dxcluster_types::{Callsign, FrequencyHz, NodeId, SpotId};
use dxcluster_wire::PeerFrame;

fn sample_spot() -> Spot {
    Spot {
        spot_id: SpotId([1u8; 32]),
        ts: time::OffsetDateTime::from_unix_timestamp(1_700_000_000).expect("timestamp"),
        freq: FrequencyHz(14_074_000),
        dx: Callsign::parse_loose("K1ABC").expect("dx callsign"),
        spotter: Callsign::parse_loose("N0CALL").expect("spotter callsign"),
        comment: "test spot".to_string(),
        origin: Some(NodeId("node-a".to_string())),
        hop: 2,
    }
}

#[test]
fn spot_frame_round_trip() {
    let frame = PeerFrame::Spot {
        spot: sample_spot(),
    };
    let line = frame.to_line();
    let parsed = PeerFrame::parse(&line).expect("parse frame");
    assert_eq!(parsed, frame);
}

#[test]
fn capabilities_round_trip() {
    let frame = PeerFrame::Capabilities {
        values: vec!["spots-v1".to_string(), "heartbeat".to_string()],
    };
    let line = frame.to_line();
    let parsed = PeerFrame::parse(&line).expect("parse frame");
    assert_eq!(parsed, frame);
}

#[test]
fn spot_frame_comment_escapes_pipe() {
    let mut spot = sample_spot();
    spot.comment = "pipe | percent % ok".to_string();
    let frame = PeerFrame::Spot { spot };
    let line = frame.to_line();
    assert!(line.contains("%7C"));
    let parsed = PeerFrame::parse(&line).expect("parse frame");
    assert_eq!(parsed, frame);
}
