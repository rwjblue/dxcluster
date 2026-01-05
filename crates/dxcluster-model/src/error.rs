#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[error("{reason}")]
pub struct PolicyReject {
    pub reason: String,
}

impl PolicyReject {
    pub fn new(reason: impl Into<String>) -> Self {
        PolicyReject {
            reason: reason.into(),
        }
    }
}
