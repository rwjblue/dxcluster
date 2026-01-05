use std::collections::HashMap;
use std::time::Duration;

use dxcluster_types::SpotId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DedupeResult {
    Fresh,
    Duplicate,
    Expired,
}

pub struct DedupeTable {
    ttl: Duration,
    seen: HashMap<SpotId, u64>,
}

impl DedupeTable {
    pub fn new(ttl: Duration) -> Self {
        Self {
            ttl,
            seen: HashMap::new(),
        }
    }

    pub fn check_and_mark(&mut self, spot_id: SpotId, now: u64) -> DedupeResult {
        let mut result = DedupeResult::Fresh;
        if let Some(prev) = self.seen.get(&spot_id).copied() {
            if now.saturating_sub(prev) <= self.ttl.as_secs() {
                result = DedupeResult::Duplicate;
            } else {
                result = DedupeResult::Expired;
            }
        }

        self.seen.insert(spot_id, now);
        result
    }
}
