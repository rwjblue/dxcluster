use dxcluster_types::{CallsignError, FrequencyError};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum UserParseError {
    #[error("line was empty")]
    Empty,
    #[error("unrecognized command")]
    Unknown,
    #[error("DX command missing callsign")]
    MissingCallsign,
    #[error("DX command missing frequency")]
    MissingFrequency,
    #[error("invalid callsign: {0}")]
    InvalidCallsign(#[source] CallsignError),
    #[error("invalid frequency: {0}")]
    InvalidFrequency(#[source] FrequencyError),
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PeerParseError {
    #[error("frame was empty")]
    Empty,
    #[error("frame type was not recognized")]
    Unknown,
    #[error("frame missing {0}")]
    Missing(&'static str),
    #[error("frame invalid: {0}")]
    Invalid(&'static str),
}
