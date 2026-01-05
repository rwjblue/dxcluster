use dxcluster_types::{Callsign, FrequencyHz, NodeId, SpotId};

#[cfg(feature = "time")]
pub type Timestamp = time::OffsetDateTime;
#[cfg(not(feature = "time"))]
pub type Timestamp = i64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spot {
    pub spot_id: SpotId,
    pub ts: Timestamp,
    pub freq: FrequencyHz,
    pub dx: Callsign,
    pub spotter: Callsign,
    pub comment: String,
    pub origin: Option<NodeId>,
    pub hop: u32,
}

impl Spot {
    pub fn new_local(
        spot_id: SpotId,
        ts: Timestamp,
        freq: FrequencyHz,
        dx: Callsign,
        spotter: Callsign,
        comment: impl Into<String>,
        origin: Option<NodeId>,
    ) -> Self {
        Spot {
            spot_id,
            ts,
            freq,
            dx,
            spotter,
            comment: comment.into(),
            origin,
            hop: 0,
        }
    }
}
