use dxcluster_model::Spot;
use dxcluster_types::{Callsign, FrequencyHz, NodeId, SpotId};
use time::OffsetDateTime;

use crate::error::PeerParseError;

#[derive(Debug, Clone, PartialEq)]
pub enum PeerFrame {
    Hello { node_id: NodeId, version: String },
    Capabilities { values: Vec<String> },
    Auth { token: String },
    Spot { spot: Spot },
    Heartbeat { nonce: String },
    Ping { nonce: String },
    Pong { nonce: String },
}

impl PeerFrame {
    pub fn parse(line: &str) -> Result<Self, PeerParseError> {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return Err(PeerParseError::Empty);
        }

        let mut parts = trimmed.split('|');
        match parts.next() {
            Some("HELLO") => {
                let Some(node_id) = parts.next() else {
                    return Err(PeerParseError::Missing("node id"));
                };
                let version = parts.next().unwrap_or("1").to_string();
                Ok(PeerFrame::Hello {
                    node_id: NodeId(node_id.to_string()),
                    version,
                })
            }
            Some("CAPS") => {
                let values = parts
                    .next()
                    .unwrap_or_default()
                    .split(',')
                    .filter(|value| !value.trim().is_empty())
                    .map(|value| value.trim().to_string())
                    .collect();
                Ok(PeerFrame::Capabilities { values })
            }
            Some("AUTH") => {
                let token = parts.next().unwrap_or_default().to_string();
                if token.is_empty() {
                    return Err(PeerParseError::Missing("auth token"));
                }
                Ok(PeerFrame::Auth { token })
            }
            Some("HEARTBEAT") => {
                let nonce = parts.next().unwrap_or_default().to_string();
                Ok(PeerFrame::Heartbeat { nonce })
            }
            Some("SPOT") => {
                let Some(spot_id) = parts.next() else {
                    return Err(PeerParseError::Missing("spot id"));
                };
                let Some(ts) = parts.next() else {
                    return Err(PeerParseError::Missing("timestamp"));
                };
                let Some(freq) = parts.next() else {
                    return Err(PeerParseError::Missing("frequency"));
                };
                let Some(dx) = parts.next() else {
                    return Err(PeerParseError::Missing("dx callsign"));
                };
                let Some(spotter) = parts.next() else {
                    return Err(PeerParseError::Missing("spotter callsign"));
                };
                let comment = parts.next().unwrap_or_default();
                let origin = parts.next().unwrap_or_default();
                let hop = parts.next().unwrap_or_default();

                let spot_id = parse_spot_id(spot_id)?;
                let timestamp = ts
                    .parse::<i64>()
                    .map_err(|_| PeerParseError::Invalid("timestamp"))?;
                let ts = OffsetDateTime::from_unix_timestamp(timestamp)
                    .map_err(|_| PeerParseError::Invalid("timestamp"))?;
                let freq = freq
                    .parse::<u64>()
                    .map(FrequencyHz)
                    .map_err(|_| PeerParseError::Invalid("frequency"))?;
                let dx = Callsign::parse_loose(dx)
                    .map_err(|_| PeerParseError::Invalid("dx callsign"))?;
                let spotter = Callsign::parse_loose(spotter)
                    .map_err(|_| PeerParseError::Invalid("spotter callsign"))?;
                let origin = if origin.is_empty() {
                    None
                } else {
                    Some(NodeId(origin.to_string()))
                };
                let hop = hop
                    .parse::<u32>()
                    .map_err(|_| PeerParseError::Invalid("hop"))?;

                Ok(PeerFrame::Spot {
                    spot: Spot {
                        spot_id,
                        ts,
                        freq,
                        dx,
                        spotter,
                        comment: comment.to_string(),
                        origin,
                        hop,
                    },
                })
            }
            Some("PING") => {
                let nonce = parts.next().unwrap_or_default().to_string();
                Ok(PeerFrame::Ping { nonce })
            }
            Some("PONG") => {
                let nonce = parts.next().unwrap_or_default().to_string();
                Ok(PeerFrame::Pong { nonce })
            }
            _ => Err(PeerParseError::Unknown),
        }
    }

    pub fn to_line(&self) -> String {
        match self {
            PeerFrame::Hello { node_id, version } => {
                format!("HELLO|{}|{}", node_id.0, version)
            }
            PeerFrame::Capabilities { values } => {
                format!("CAPS|{}", values.join(","))
            }
            PeerFrame::Auth { token } => format!("AUTH|{token}"),
            PeerFrame::Spot { spot } => format!(
                "SPOT|{}|{}|{}|{}|{}|{}|{}|{}",
                format_spot_id(&spot.spot_id),
                spot.ts.unix_timestamp(),
                spot.freq.0,
                spot.dx.as_str(),
                spot.spotter.as_str(),
                spot.comment,
                spot.origin.as_ref().map(|id| id.0.as_str()).unwrap_or(""),
                spot.hop,
            ),
            PeerFrame::Heartbeat { nonce } => format!("HEARTBEAT|{}", nonce),
            PeerFrame::Ping { nonce } => format!("PING|{}", nonce),
            PeerFrame::Pong { nonce } => format!("PONG|{}", nonce),
        }
    }
}

fn format_spot_id(spot_id: &SpotId) -> String {
    spot_id.0.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn parse_spot_id(input: &str) -> Result<SpotId, PeerParseError> {
    if input.len() != 64 {
        return Err(PeerParseError::Invalid("spot id"));
    }

    let mut bytes = [0u8; 32];
    for (idx, chunk) in input.as_bytes().chunks(2).enumerate() {
        let hex = std::str::from_utf8(chunk).map_err(|_| PeerParseError::Invalid("spot id"))?;
        bytes[idx] = u8::from_str_radix(hex, 16).map_err(|_| PeerParseError::Invalid("spot id"))?;
    }
    Ok(SpotId(bytes))
}
