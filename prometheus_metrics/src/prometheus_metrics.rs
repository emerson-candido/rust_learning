use std::error::Error;
use prometheus::{Gauge, GaugeVec, Opts, Registry};
use prometheus::core::{AtomicF64, GenericGauge, GenericGaugeVec};
use prometheus_hyper::RegistryFn;

pub struct Metrics {
    pub ssh_logins_total: Gauge,
    pub ssh_logins_details: GaugeVec,
}

impl Metrics {
    pub fn new() -> Result<(Self, RegistryFn), Box<dyn Error>> {
        let ssh_logins_total :GenericGauge<AtomicF64> = Gauge::with_opts(Opts::new(
            "ssh_logins_total",
            "Number of active SSH connections"
        ))?;

        let ssh_logins_total_clone :GenericGauge<AtomicF64> = ssh_logins_total.clone();

        let ssh_logins_details :GenericGaugeVec<AtomicF64> = GaugeVec::new(
            Opts::new(
                "ssh_logins_details",
                "Details of active SSH connections"
            ),
            &["username", "remote_ip"]
        ).unwrap();

        let ssh_logins_details_clone:GenericGaugeVec<AtomicF64> = ssh_logins_details.clone();

        let f = |registry: &Registry| {
            registry.register(Box::new(ssh_logins_total_clone))?;
            registry.register(Box::new(ssh_logins_details_clone))?;
            Ok(())
        };

        Ok((Self {
            ssh_logins_total,
            ssh_logins_details
        }, Box::new(f)))
    }
}
