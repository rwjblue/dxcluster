#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("connection failed")]
    Connection,
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
