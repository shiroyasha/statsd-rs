use std::collections::{HashMap, HashSet};
use metric::{self, Metric, MetricType};

pub struct Stats {
    pub counters: HashMap<String, i64>,
    pub timings: HashMap<String, Vec<i64>>,
    pub sets: HashMap<String, HashSet<i64>>,
    pub gauges: HashMap<String, i64>
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            counters: HashMap::new(),
            timings: HashMap::new(),
            sets: HashMap::new(),
            gauges: HashMap::new()
        }
    }

    pub fn process_metric(&mut self, metric : &Metric) {
        match metric.metric_type {
            MetricType::Gauge => {
                self.gauges.insert(metric.name.to_string(), metric.value);
            },
            MetricType::Timing => {
                self.timings.entry(metric.name.to_string()).or_insert_with(|| Vec::new()).push(metric.value);
            }
            MetricType::Counter => {
                *self.counters.entry(metric.name.to_string()).or_insert(0) += metric.value;
            },
            MetricType::Set => {
                self.sets.entry(metric.name.to_string()).or_insert_with(|| HashSet::new()).insert(metric.value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn processing_counters_test() {
        let mut stats = Stats::new();

        for _ in 0..10 {
            stats.process_metric(&metric::parse("test:1|c"));
        }

        assert_eq!(stats.counters.get("test").unwrap(), &10);
    }

    #[test]
    fn processing_gauges_test() {
        let mut stats = Stats::new();

        for _ in 0..10 {
            stats.process_metric(&metric::parse("test:10|g"));
        }

        assert_eq!(stats.gauges.get("test").unwrap(), &10);
    }

    #[test]
    fn processing_timing_test() {
        let mut stats = Stats::new();

        for _ in 0..3 {
            stats.process_metric(&metric::parse("test:10|ms"));
        }

        assert_eq!(stats.timings.get("test").unwrap(), &vec![10, 10, 10]);
    }

}
