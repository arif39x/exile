use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{info, error};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct NodeConfig {
    pub heartbeat_interval: u64,
    pub config_poll_interval: u64,
    pub workloads: Vec<WorkloadConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WorkloadConfig {
    pub id: String,
    pub binary: String,
    pub args: Vec<String>,
    pub env: std::collections::HashMap<String, String>,
}

pub struct ConfigPoller {
    vault_address: String,
    node_id: String,
    token: Arc<RwLock<Option<String>>>,
    current_config: Arc<RwLock<NodeConfig>>,
    client: Client,
}

impl ConfigPoller {
    pub fn new(vault_address: String, node_id: String, token: Arc<RwLock<Option<String>>>) -> Self {
        Self {
            vault_address,
            node_id,
            token,
            current_config: Arc::new(RwLock::new(NodeConfig::default())),
            client: Client::new(),
        }
    }

    pub async fn start_loop(self: Arc<Self>) {
        info!("Starting config polling loop");
        let mut interval = tokio::time::interval(Duration::from_secs(30));
        
        loop {
            interval.tick().await;
            if let Err(e) = self.poll_config().await {
                error!("Failed to poll config: {}", e);
            }
        }
    }

    async fn poll_config(&self) -> Result<()> {
        let token = self.token.read().await;
        let token = token.as_ref().ok_or_else(|| anyhow::anyhow!("No Vault token available"))?;

        let url = format!("{}/v1/secret/data/nodes/{}", self.vault_address, self.node_id);
        
        let resp = self.client.get(&url)
            .header("X-Vault-Token", token)
            .send()
            .await?;

        if resp.status().is_success() {
            let body: serde_json::Value = resp.json().await?;
            let data = body["data"]["data"].clone();
            let config: NodeConfig = serde_json::from_value(data)?;
            
            let mut current = self.current_config.write().await;
            *current = config;
            info!("Updated node configuration from Vault");
        }
        
        Ok(())
    }

    pub async fn get_config(&self) -> NodeConfig {
        self.current_config.read().await.clone()
    }
}
