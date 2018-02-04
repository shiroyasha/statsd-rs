#[derive(Debug)]
pub enum MetricType {
    Timing,
    Gauge,
    Counter,
    Set
}

pub struct Metric<'a> {
    pub metric_type: MetricType,
    pub value: i64,
    pub name: &'a str,
}

pub fn parse<'a>(buffer : &'a str) -> Metric<'a> {
    let mut fields = buffer.split("|");

    let mut name_and_value = fields.next().unwrap().split(":");
    let name = name_and_value.next().unwrap();
    let value = name_and_value.next().unwrap().parse().unwrap();

    let metric_type = match fields.next().unwrap() {
        "c"  => MetricType::Counter,
        "ms" => MetricType::Timing,
        "g"  => MetricType::Gauge,
        "s"  => MetricType::Set,
        _    => panic!("unrecognized")
    };

    Metric {
        metric_type : metric_type,
        value: value,
        name: name
    }
}
