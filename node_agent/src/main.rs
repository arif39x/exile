mod adapters;
mod config;
mod heartbeat;
mod registry;
mod vault;
mod workload;

use crate::adapters::get_adapter;
use crate::config::ConfigPoller;
use crate::heartbeat::HeartbeatManager;
use crate::registry::RegistryClient;
use crate::vault::{VaultConfig, VaultSidecar};
use crate::workload::WorkloadSupervisor;
use anyhow::Result;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    info!("Starting Exile Node Agent");

    // Load bootstrap configuration
    let vault_config = VaultConfig {
        address: std::env::var("VAULT_ADDR")
            .unwrap_or_else(|_| "http://127.0.0.1:8200".to_string()),
        role_id: std::env::var("VAULT_ROLE_ID").unwrap_or_else(|_| "node-role".to_string()),
        secret_id: std::env::var("VAULT_SECRET_ID").ok(),
    };

    let registry_endpoint =
        std::env::var("REGISTRY_ADDR").unwrap_or_else(|_| "http://127.0.0.1:50051".to_string());

    let nats_url =
        std::env::var("NATS_URL").unwrap_or_else(|_| "nats://127.0.0.1:4222".to_string());
    let nats_client = async_nats::connect(nats_url).await?;

    // Initi Vault Sidecar
    let vault = Arc::new(VaultSidecar::new(vault_config));

    // Authenticate with Vault
    vault.authenticate().await?;

    // Request Node Certificate
    let hostname = hostname::get()?.to_string_lossy().to_string();
    vault.request_certificate(&hostname).await?;

    // Register with Control Plane
    let registry = RegistryClient::new(registry_endpoint);
    let node_id = registry
        .register(
            hostname.clone(),
            "127.0.0.1".to_string(), // detect IP
            std::env::consts::OS.to_string(),
            std::env::consts::ARCH.to_string(),
        )
        .await?;

    //Initialize Components
    let adapter = get_adapter();
    let supervisor = Arc::new(WorkloadSupervisor::new(adapter));
    let heartbeat = Arc::new(HeartbeatManager::new(
        node_id.clone(),
        "http://control-plane".to_string(),
    ));
    let config_poller = Arc::new(ConfigPoller::new(
        vault.config.address.clone(),
        node_id.clone(),
        vault.get_token_lock(), // I need to expose this
    ));

    // Start Concurrent Loops
    info!("All components initialized, starting loops");

    let vault_clone = vault.clone();
    let heartbeat_clone = heartbeat.clone();
    let supervisor_clone = supervisor.clone();
    let config_poller_clone = config_poller.clone();
    let node_id_clone = node_id.clone();
    let nats_client_clone = nats_client.clone();

    tokio::select! {
        _ = vault_clone.start_renewal_loop() => {},
        _ = heartbeat_clone.start_loop() => {},
        _ = config_poller_clone.start_loop() => {},
        _ = supervisor_clone.monitor_workloads() => {},
        _ = supervisor_clone.listen_for_intents(nats_client_clone, node_id_clone) => {},
    }

    Ok(())
}
