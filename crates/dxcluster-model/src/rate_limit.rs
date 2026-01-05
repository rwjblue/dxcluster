use crate::spot::Spot;

#[derive(Debug, Clone, Default)]
pub struct RateLimiter;

impl RateLimiter {
    pub fn check(&self, _spot: &Spot) -> bool {
        true
    }
}
