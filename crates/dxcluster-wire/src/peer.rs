use dxcluster_model::Spot;
use dxcluster_types::NodeId;

use crate::error::PeerParseError;

#[derive(Debug, Clone, PartialEq)]
pub enum PeerFrame {
    Hello { node_id: NodeId, version: String },
    Spot { spot: Spot },
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
            Some("PING") => {
                let nonce = parts.next().unwrap_or_default().to_string();
                Ok(PeerFrame::Ping { nonce })
            }
            Some("PONG") => {
                let nonce = parts.next().unwrap_or_default().to_string();
                Ok(PeerFrame::Pong { nonce })
            }
            Some("SPOT") => Err(PeerParseError::Unknown),
            _ => Err(PeerParseError::Unknown),
        }
    }

    pub fn to_line(&self) -> String {
        match self {
            PeerFrame::Hello { node_id, version } => {
                format!("HELLO|{}|{}", node_id.0, version)
            }
            PeerFrame::Spot { spot: _ } => String::from("SPOT|"),
            PeerFrame::Ping { nonce } => format!("PING|{}", nonce),
            PeerFrame::Pong { nonce } => format!("PONG|{}", nonce),
        }
    }
}
