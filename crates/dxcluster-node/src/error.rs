#[derive(Debug, thiserror::Error)]
pub enum NodeError {
    #[error("failed to start listener")]
    Listener,
    #[error("task join error")]
    Join,
}
