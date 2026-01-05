//! Domain model types and deterministic business logic.

pub mod cache;
pub mod dedupe;
pub mod error;
pub mod filter;
pub mod policy;
#[cfg(feature = "rate_limit")]
pub mod rate_limit;
pub mod spot;

pub use cache::SpotCache;
pub use dedupe::{DedupeResult, DedupeTable};
pub use error::PolicyReject;
pub use filter::Filter;
pub use policy::Policy;
#[cfg(feature = "rate_limit")]
pub use rate_limit::RateLimiter;
pub use spot::Spot;
