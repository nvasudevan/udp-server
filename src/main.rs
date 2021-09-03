use std::{env, fmt, io};
use chrono::{DateTime, Utc};
use tokio::net::UdpSocket;
use std::net::SocketAddr;

struct AppMetric {
    at_time: DateTime<Utc>,
    ip_address: String,
    app_env: String,
    app_version: f32,
}

impl fmt::Display for AppMetric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{} {} {} ({})",
                        self.at_time,
                        self.app_env,
                        self.app_version,
                        self.ip_address
        );
        write!(f, "{}", s)
    }
}

/// message format: app_env=val;app_version=x.y
fn process_msg(s: &str, src_addr: &SocketAddr) -> Option<AppMetric> {
    let key_values: Vec<&str> = s.trim_end().split(";").collect();
    let app_env_k_v: Vec<&str> = key_values[0].split("=").collect();
    let app_version_k_v: Vec<&str> = key_values[1].split("=").collect();
    let app_env_key = app_env_k_v.get(0)?;
    if (*app_env_key).eq("app_env") {
        let app_version_key = app_version_k_v.get(0)?;
        if (*app_version_key).eq("app_version") {
            if let Ok(app_version_val) = app_version_k_v[1].parse::<f32>() {
                return Some(AppMetric {
                    at_time: Utc::now(),
                    ip_address: src_addr.to_string(),
                    app_env: app_env_k_v[1].to_owned(),
                    app_version: app_version_val,
                });
            }
        }
    }

    None
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let bind_addr = env::var("BIND_ADDR")
        .expect("BIND_ADDR environment variable is not set!");
    println!("=> UDP server starting on {}", bind_addr);
    let skt = UdpSocket::bind(bind_addr).await?;
    let mut metrics: Vec<AppMetric> = vec![];

    loop {
        let mut buf = [0; 64];
        match skt.recv_from(&mut buf).await {
            Ok((no_bytes, src_addr)) => {
                let msg = String::from_utf8_lossy(&buf[..no_bytes]).to_string();
                if let Some(app_metric) = process_msg(&msg, &src_addr) {
                    eprint!("=> {}", app_metric);
                    metrics.push(app_metric);
                }
                eprintln!(" [c]");
            }
            Err(e) => {
                eprintln!("[error] read failed from client : {}", e.to_string());
            }
        }
    }
}