use sigmaos_tools_speedtest::{SpeedSample, SpeedTestEngine};

#[test]
fn test_speed_sample_mbps_calculation_aaa() {
    // Arrange: 12.5 MB in 1000 ms = 100 Mbps
    let sample = SpeedSample::new(12_500_000, 1000);
    // Act
    let mbps = sample.mbps();
    // Assert
    assert!((mbps - 100.0).abs() < 0.1);
}

#[test]
fn test_average_mbps_aaa() {
    // Arrange
    let mut engine = SpeedTestEngine::new();
    engine.add_sample(SpeedSample::new(12_500_000, 1000)); // 100 Mbps
    engine.add_sample(SpeedSample::new(25_000_000, 1000)); // 200 Mbps
                                                           // Act & Assert
    assert!((engine.average_mbps() - 150.0).abs() < 0.1);
    assert!((engine.peak_mbps() - 200.0).abs() < 0.1);
}
