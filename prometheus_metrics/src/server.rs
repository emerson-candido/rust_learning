use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use prometheus::Registry;
use prometheus_hyper::Server;
use tracing::info;
use crate::ssh;
use crate::prometheus_metrics::Metrics;

pub async fn run(refresh_interval: u64, port: u16){
    let registry :Arc<Registry> = Arc::new(Registry::new());
    let (metrics, f) = Metrics::new().expect("failed prometheus");
    f(&registry).expect("problem registering");

    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();
    let running :Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
    tokio::spawn(Server::run(
        Arc::clone(&registry),
        SocketAddr::from(([0; 4], port)),
        shutdown_signal(Arc::clone(&running)),
    ));

    let mut sec_int = tokio::time::interval(Duration::from_secs(refresh_interval));

    let mut previous_connections:HashMap<String,String> = HashMap::new();
    while running.load(Ordering::Relaxed) {
        sec_int.tick().await;
        info!("Getting ssh connections");
        let connections:HashMap<String, String> = ssh::ssh_connections();
        if connections.is_empty() {
            metrics.ssh_logins_total.set(0.0);
            info!("Detected the total of 0 ssh connections");
        }
        else{
            metrics.ssh_logins_total.set(connections.len() as f64);
            info!("Detected the total of {} ssh connections", connections.len());
        }

        let mut inactive_connections :Vec<String> = Vec::new();

        for (username, ip_address) in &previous_connections{
            match connections.get(username) {
                Some(ip_address) if ip_address != ip_address => println!("{}: {}", username, ip_address),
                None => {
                    metrics.ssh_logins_details.with_label_values(&[username, ip_address]).set(0.00);
                    inactive_connections.push(String::from(username));
                    info!("Ssh connection from {} by user {} has been closed", ip_address, username);
                },
                _ => (),
            }
        }
        for inactive_connection in inactive_connections{
            previous_connections.remove(&inactive_connection);
        }

        for (username, ip_address) in &connections {
            metrics.ssh_logins_details.with_label_values(&[username, ip_address]).set(1.00);
            previous_connections.insert(String::from(username), String::from(ip_address));
            info!("Detected ssh connection from {} authenticated by user {}", ip_address, username);
        }
    }
    info!("Stopping metrics");
}

async fn shutdown_signal(running: Arc<AtomicBool>) {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
    running.store(false, Ordering::Relaxed);
}
