use crate::config::UpstreamConfig;

#[derive(Debug)]
pub struct UpstreamHandle;

impl UpstreamHandle {
    pub fn spawn_all(configs: &[UpstreamConfig]) -> Vec<Self> {
        configs.iter().map(|_| UpstreamHandle).collect()
    }
}
