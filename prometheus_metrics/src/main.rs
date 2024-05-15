use std::env;

mod common;
mod ssh;
mod prometheus_metrics;
mod server;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let refresh_interval :u64 = env::var("REFRESH_INTERVAL")
        .unwrap_or("60".to_string())
        .parse::<u64>()
        .unwrap_or(60);

    let port :u16 = env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);

    server::run(
        refresh_interval,
        port
    ).await;
}
