//! Measurement primitives — latency stats + throughput math.
//!
//! Actual network I/O (HTTP GET/POST to the speed server) is abstracted:
//! callers supply byte counts and durations; this module computes the
//! statistics upstream's Go client produces.

/// A single ping sample.
#[derive(Debug, Clone, Copy)]
pub struct PingSample {
    /// Round-trip time in milliseconds.
    pub rtt_ms: f64,
}

/// Latency statistics over ping samples.
#[derive(Debug, Clone, Copy, Default)]
pub struct LatencyStats {
    /// Min RTT.
    pub min_ms: f64,
    /// Average RTT.
    pub avg_ms: f64,
    /// Max RTT.
    pub max_ms: f64,
    /// Packet loss percentage (0-100).
    pub loss_pct: f64,
}

/// Compute latency stats from samples; `sent`/`received` for loss.
pub fn ping_stats(samples: &[PingSample], sent: u32, received: u32) -> LatencyStats {
    if samples.is_empty() {
        return LatencyStats {
            loss_pct: if sent > 0 { 100.0 } else { 0.0 },
            ..Default::default()
        };
    }
    let mut min = f64::INFINITY;
    let mut max = 0.0f64;
    let mut sum = 0.0;
    for s in samples {
        min = min.min(s.rtt_ms);
        max = max.max(s.rtt_ms);
        sum += s.rtt_ms;
    }
    LatencyStats {
        min_ms: min,
        avg_ms: sum / samples.len() as f64,
        max_ms: max,
        loss_pct: if sent > 0 {
            (sent.saturating_sub(received)) as f64 / sent as f64 * 100.0
        } else {
            0.0
        },
    }
}

/// Download throughput in Mbps given bytes downloaded and elapsed seconds.
pub fn download_bytes(bytes: u64, elapsed_secs: f64) -> f64 {
    throughput_mbps(bytes, elapsed_secs)
}

/// Upload throughput in Mbps given bytes uploaded and elapsed seconds.
pub fn upload_bytes(bytes: u64, elapsed_secs: f64) -> f64 {
    throughput_mbps(bytes, elapsed_secs)
}

fn throughput_mbps(bytes: u64, elapsed_secs: f64) -> f64 {
    if elapsed_secs <= 0.0 {
        return 0.0;
    }
    // bytes → bits → /1e6 for Mbps.
    (bytes as f64 * 8.0) / elapsed_secs / 1_000_000.0
}

/// Format Mbps value with unit suffix (Gbps/Mbps/Kbps).
pub fn format_throughput(mbps: f64) -> String {
    if mbps >= 1000.0 {
        format!("{:.2} Gbps", mbps / 1000.0)
    } else if mbps >= 1.0 {
        format!("{:.1} Mbps", mbps)
    } else {
        format!("{:.0} Kbps", mbps * 1000.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ping_stats() {
        let samples = vec![
            PingSample { rtt_ms: 10.0 },
            PingSample { rtt_ms: 20.0 },
            PingSample { rtt_ms: 30.0 },
        ];
        let s = ping_stats(&samples, 4, 3);
        assert_eq!(s.min_ms, 10.0);
        assert_eq!(s.max_ms, 30.0);
        assert!((s.avg_ms - 20.0).abs() < 0.001);
        assert!((s.loss_pct - 25.0).abs() < 0.001);
    }

    #[test]
    fn test_throughput() {
        // 10 MB in 1s = 80 Mbps.
        assert!((download_bytes(10_000_000, 1.0) - 80.0).abs() < 0.1);
        assert_eq!(format_throughput(1500.0), "1.50 Gbps");
        assert_eq!(format_throughput(50.0), "50.0 Mbps");
        assert_eq!(format_throughput(0.5), "500 Kbps");
    }

    #[test]
    fn test_zero_duration() {
        assert_eq!(download_bytes(1000, 0.0), 0.0);
    }
}
