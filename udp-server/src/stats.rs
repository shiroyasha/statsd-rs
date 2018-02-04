use std::collections::{HashMap, HashSet};
use metric::{Metric, MetricType};

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
