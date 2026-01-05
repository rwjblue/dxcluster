//! Wire formats and protocol helpers.
//!
//! The user-facing protocol mirrors traditional DX cluster telnet commands:
//! - `DX <call> <frequency_khz> <comment>` publishes a new spot.
//! - `SH/DX` returns recent spots, while `SH/FILTERS` reports the active
//!   filter configuration.
//! - `PING`/`HEARTBEAT` is a keep-alive with no payload.
//!
//! Peer-to-peer frames use pipe-separated fields prefixed by a keyword, for
//! example `HELLO|<node_id>|<version>` or `PING|<nonce>`. Formatting helpers
//! round-trip with the parsers to make it easy to test protocol compliance.

pub mod error;
pub mod format;
pub mod parse;
pub mod peer;
pub mod user;

pub use error::{PeerParseError, UserParseError};
pub use peer::PeerFrame;
pub use user::{ServerLine, UserCommand};
