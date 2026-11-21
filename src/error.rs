//! Error types for the speedtest component.

/// Errors running a speed test.
#[derive(Debug, thiserror::Error)]
pub enum SpeedtestError {
    /// Network/HTTP error.
    #[error("network error: {0}")]
    Network(String),

    /// Invalid measurement (zero duration, etc.).
    #[error("invalid measurement: {0}")]
    Invalid(String),
}
