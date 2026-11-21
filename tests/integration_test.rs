use sigmaos_tools_speedtest::*;

#[test]
fn test_full_result_report() {
    let s = ping_stats(&[PingSample { rtt_ms: 15.0 }], 1, 1);
    let r = SpeedResult { latency: s, download_mbps: 100.0, upload_mbps: 20.0, server: "speed.omarchy.org".into(), timestamp: 0 };
    let rep = r.report();
    assert!(rep.contains("100.0 Mbps"));
    assert!(rep.contains("speed.omarchy.org"));
}

#[test]
fn test_measurements() {
    assert!(download_bytes(1_000_000, 1.0) > 0.0);
    assert!(format_throughput(0.4).contains("Kbps"));
}
