#[derive(Debug, Clone, Default)]
pub struct ReconnectPolicy {
    pub max_retries: usize,
}
