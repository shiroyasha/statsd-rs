use std::str;
use std::net::UdpSocket;

mod metric;
mod stats;

fn server(url : &str) {
    let socket = match UdpSocket::bind(&url) {
        Ok(s) => s,
        Err(e) => panic!("couldn't bind socket: {}", e)
    };

    println!("Listening on {}", url);

    let mut stats = stats::Stats::new();
    let mut buf = [0; 2048];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, _)) => {
                let metric = metric::parse(&str::from_utf8(&buf[..amt]).unwrap_or(""));

                stats.process_metric(&metric);
            },
            Err(e) => {
                println!("couldn't recieve a datagram: {}", e);
            }
        }
    }
}

fn main() {
    let port = "55123";
    let host = "0.0.0.0";
    let url  = format!("{}:{}", host, port);

    server(&url);
}
