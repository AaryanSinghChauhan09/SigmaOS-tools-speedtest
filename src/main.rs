//! speedtest — network speed test CLI.

use sigmaos_tools_speedtest::{LatencyStats, PingSample, download_bytes, format_throughput, ping_stats, upload_bytes};

fn main() {
    // Demo mode: simulate a test with canned measurements (real network
    // I/O against speed.omarchy.org is out of scope for this port).
    let samples: Vec<PingSample> = (0..5).map(|i| PingSample { rtt_ms: 12.0 + i as f64 * 2.0 }).collect();
    let latency = ping_stats(&samples, 5, 5);
    let dl = download_bytes(25_000_000, 2.5);
    let ul = upload_bytes(5_000_000, 2.0);
    println!("ping: {:.1} ms (min {:.1} / max {:.1}, loss {:.0}%)",
        latency.avg_ms, latency.min_ms, latency.max_ms, latency.loss_pct);
    println!("download: {}", format_throughput(dl));
    println!("upload:   {}", format_throughput(ul));
    let _ = LatencyStats::default();
}
