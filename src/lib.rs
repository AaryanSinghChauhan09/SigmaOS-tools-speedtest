//! SigmaOS tools-speedtest component.
//!
//! Rust implementation of `omacom/omarchy-speedtest`: Omarchy's network
//! speed test client for speed.omarchy.org (upstream: Go single binary).
//!
//! This port implements the measurement model: latency ping samples,
//! download/upload throughput calculation from byte counts and timings,
//! and the result report — the logic upstream's Go client computes.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod error;
pub mod result;
pub mod test;

pub use error::SpeedtestError;
pub use result::{SpeedResult, ThroughputUnit};
pub use test::{LatencyStats, PingSample, download_bytes, format_throughput, ping_stats, upload_bytes};
