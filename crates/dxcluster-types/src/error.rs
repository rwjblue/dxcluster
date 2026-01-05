#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CallsignError {
    #[error("callsign is empty")]
    Empty,
    #[error("callsign has invalid format")]
    InvalidFormat,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum FrequencyError {
    #[error("frequency value is missing")]
    Missing,
    #[error("frequency string is invalid")]
    Invalid,
}
