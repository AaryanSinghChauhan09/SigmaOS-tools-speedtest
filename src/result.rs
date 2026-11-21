//! Result model — what a speed test reports.

/// Throughput unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThroughputUnit {
    /// Bits per second.
    Bps,
    /// Kilobits per second.
    Kbps,
    /// Megabits per second.
    Mbps,
    /// Gigabits per second.
    Gbps,
}

/// A completed speed test result.
#[derive(Debug, Clone)]
pub struct SpeedResult {
    /// Ping latency stats (min/avg/max ms).
    pub latency: crate::test::LatencyStats,
    /// Download throughput in Mbps.
    pub download_mbps: f64,
    /// Upload throughput in Mbps.
    pub upload_mbps: f64,
    /// Server used.
    pub server: String,
    /// Timestamp of test.
    pub timestamp: i64,
}

impl SpeedResult {
    /// Format a human-readable report.
    pub fn report(&self) -> String {
        format!(
            "server: {}\nping: {:.1} ms (min {:.1} / max {:.1})\ndownload: {:.1} Mbps\nupload: {:.1} Mbps",
            self.server,
            self.latency.avg_ms,
            self.latency.min_ms,
            self.latency.max_ms,
            self.download_mbps,
            self.upload_mbps
        )
    }
}
