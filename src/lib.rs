#![forbid(unsafe_code)]
//! speedtest - Network speed and latency test client for SigmaOS

#[derive(Debug, Clone)]
pub struct SpeedSample {
    pub bytes: u64,
    pub duration_ms: u64,
}

impl SpeedSample {
    pub fn new(bytes: u64, duration_ms: u64) -> Self {
        Self { bytes, duration_ms }
    }

    pub fn mbps(&self) -> f64 {
        if self.duration_ms == 0 {
            return 0.0;
        }
        (self.bytes as f64 * 8.0) / (self.duration_ms as f64 / 1000.0) / 1_000_000.0
    }
}

pub struct SpeedTestEngine {
    samples: Vec<SpeedSample>,
}

impl SpeedTestEngine {
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
        }
    }

    pub fn add_sample(&mut self, sample: SpeedSample) {
        self.samples.push(sample);
    }

    pub fn average_mbps(&self) -> f64 {
        if self.samples.is_empty() {
            return 0.0;
        }
        self.samples.iter().map(|s| s.mbps()).sum::<f64>() / self.samples.len() as f64
    }

    pub fn peak_mbps(&self) -> f64 {
        self.samples
            .iter()
            .map(|s| s.mbps())
            .fold(0.0_f64, f64::max)
    }

    pub fn sample_count(&self) -> usize {
        self.samples.len()
    }
}

impl Default for SpeedTestEngine {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod speedtest_extra_tests {
    use super::*;

    #[test]
    fn test_speedtest_zero_duration_returns_zero_mbps_aaa() {
        // Arrange
        let sample = SpeedSample::new(1000, 0);
        // Act
        let mbps = sample.mbps();
        // Assert
        assert_eq!(mbps, 0.0);
    }

    #[test]
    fn test_speedtest_empty_engine_averages_aaa() {
        // Arrange
        let engine = SpeedTestEngine::new();
        // Act & Assert
        assert_eq!(engine.average_mbps(), 0.0);
        assert_eq!(engine.peak_mbps(), 0.0);
    }

    #[test]
    fn test_speedtest_engine_default_aaa() {
        // Arrange & Act
        let engine = SpeedTestEngine::default();
        // Assert
        assert_eq!(engine.average_mbps(), 0.0);
    }
}
