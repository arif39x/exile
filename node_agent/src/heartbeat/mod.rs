use anyhow::Result;
use serde::Serialize;
use sysinfo::{System, Disks, Components};
use std::time::Duration;
use tracing::{info, error};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Serialize, Clone)]
pub struct HeartbeatPayload {
    pub node_id: String,
    pub workload_states: Vec<WorkloadState>,
    pub metrics: HealthMetrics,
    pub token_expiry: u64,
    pub cert_expiry: u64,
}

#[derive(Debug, Serialize, Clone)]
pub struct WorkloadState {
    pub id: String,
    pub state: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct HealthMetrics {
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub disk_usage: u64,
    pub temperature: Option<f32>,
}

pub struct HeartbeatManager {
    node_id: String,
    endpoint: String,
    system: Arc<RwLock<System>>,
}

impl HeartbeatManager {
    pub fn new(node_id: String, endpoint: String) -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        Self {
            node_id,
            endpoint,
            system: Arc::new(RwLock::new(system)),
        }
    }

    pub async fn start_loop(self: Arc<Self>) {
        info!("Starting heartbeat loop");
        let mut interval = tokio::time::interval(Duration::from_secs(10));
        
        loop {
            interval.tick().await;
            if let Err(e) = self.send_heartbeat().await {
                error!("Failed to send heartbeat: {}", e);
            }
        }
    }

    async fn send_heartbeat(&self) -> Result<()> {
        let mut sys = self.system.write().await;
        sys.refresh_all();

        let disks = Disks::new_with_refreshed_list();
        let components = Components::new_with_refreshed_list();

        let metrics = HealthMetrics {
            cpu_usage: sys.global_cpu_usage(),
            memory_usage: sys.used_memory(),
            disk_usage: disks.iter().map(|d| d.total_space() - d.available_space()).sum(),
            temperature: components.iter().find(|c| c.label().contains("CPU")).map(|c| c.temperature()),
        };

        let _payload = HeartbeatPayload {
            node_id: self.node_id.clone(),
            workload_states: vec![], // TODO: Get from workload supervisor
            metrics,
            token_expiry: 0, // TODO: Get from vault sidecar
            cert_expiry: 0,  // TODO: Get from vault sidecar
        };

        // info!("Sending heartbeat to {}: {:?}", self.endpoint, payload);
        
        Ok(())
    }
}
