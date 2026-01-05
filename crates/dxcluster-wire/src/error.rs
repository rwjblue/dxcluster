#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum UserParseError {
    #[error("line was empty")]
    Empty,
    #[error("unrecognized command")]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PeerParseError {
    #[error("frame was empty")]
    Empty,
    #[error("frame was invalid")]
    Invalid,
}
