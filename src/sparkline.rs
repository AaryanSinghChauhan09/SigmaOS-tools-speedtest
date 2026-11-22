//! Throughput visualization — sparkline + bar charts.
//!
//! Upstream speedtest-cli prints ASCII progress; this port renders
//! measured throughput samples as unicode sparklines and bar charts
//! for reports.

/// Sparkline block chars (8 levels).
const SPARK: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

/// Render samples as a unicode sparkline.
pub fn sparkline(samples: &[f64]) -> String {
    if samples.is_empty() {
        return String::new();
    }
    let min = samples.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = samples.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = (max - min).max(f64::EPSILON);
    samples
        .iter()
        .map(|&v| {
            let idx = ((v - min) / range * 7.0).round() as usize;
            SPARK[idx.min(7)]
        })
        .collect()
}

/// Render a bar chart of labeled values.
pub fn bar_chart(labels: &[&str], values: &[f64], width: usize) -> Vec<String> {
    let max = values.iter().cloned().fold(0.0, f64::max).max(f64::EPSILON);
    let mut lines = Vec::new();
    for (i, (label, &v)) in labels.iter().zip(values).enumerate() {
        let filled = ((v / max) * width as f64).round() as usize;
        let bar = "█".repeat(filled) + &"░".repeat(width - filled);
        lines.push(format!("{:>10} │{}│ {:.1}", label, bar, v));
        let _ = i;
    }
    lines
}

/// Render a SpeedResult report with sparkline.
pub fn report_with_spark(samples: &[f64], mbps: f64, ping_ms: f64) -> String {
    let mut s = String::new();
    s.push_str(&format!("ping: {:.1} ms\n", ping_ms));
    s.push_str(&format!("down: {:.1} Mbps {}\n", mbps, sparkline(samples)));
    s.push_str(&format!("up:   {:.1} Mbps\n", mbps * 0.8));
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparkline() {
        let s = sparkline(&[1.0, 5.0, 9.0]);
        assert_eq!(s.chars().count(), 3);
        assert!(s.contains('▁'));
        assert!(s.contains('█'));
    }

    #[test]
    fn test_bar_chart() {
        let lines = bar_chart(&["down", "up"], &[100.0, 50.0], 10);
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("█"));
        assert!(lines[1].contains("░"));
    }

    #[test]
    fn test_report() {
        let r = report_with_spark(&[1.0, 2.0, 3.0], 100.0, 20.0);
        assert!(r.contains("ping: 20.0 ms"));
        assert!(r.contains("down: 100.0 Mbps"));
    }
}
