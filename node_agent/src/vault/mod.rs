use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::info;

#[derive(Debug, Clone)]
pub struct VaultConfig {
    pub address: String,
    pub role_id: String,
    pub secret_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct VaultLoginResponse {
    auth: VaultAuth,
}

#[derive(Debug, Deserialize)]
struct VaultAuth {
    client_token: String,
    lease_duration: u64,
}

#[derive(Debug, Deserialize)]
struct VaultPkiResponse {
    data: PkiData,
}

#[derive(Debug, Deserialize)]
struct PkiData {
    certificate: String,
    private_key: String,
    ca_chain: Vec<String>,
}

pub struct VaultSidecar {
    pub config: VaultConfig,
    client: Client,
    token: Arc<RwLock<Option<String>>>,
    cert: Arc<RwLock<Option<String>>>,
    key: Arc<RwLock<Option<String>>>,
}

impl VaultSidecar {
    pub fn new(config: VaultConfig) -> Self {
        Self {
            config,
            client: Client::new(),
            token: Arc::new(RwLock::new(None)),
            cert: Arc::new(RwLock::new(None)),
            key: Arc::new(RwLock::new(None)),
        }
    }

    pub fn get_token_lock(&self) -> Arc<RwLock<Option<String>>> {
        self.token.clone()
    }

    pub async fn authenticate(&self) -> Result<()> {
        info!("Authenticating with Vault via AppRole");

        let secret_id = self
            .config
            .secret_id
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Secret ID is required for authentication"))?;

        let url = format!("{}/v1/auth/approle/login", self.config.address);
        let payload = serde_json::json!({
            "role_id": self.config.role_id,
            "secret_id": secret_id,
        });

        let resp = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await?
            .json::<VaultLoginResponse>()
            .await?;

        let mut token = self.token.write().await;
        *token = Some(resp.auth.client_token);

        info!("Successfully authenticated with Vault");
        Ok(())
    }

    pub async fn request_certificate(&self, common_name: &str) -> Result<()> {
        info!("Requesting certificate for {}", common_name);

        let token = self.token.read().await;
        let token = token
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not authenticated"))?;

        let url = format!("{}/v1/pki/issue/node-role", self.config.address);
        let payload = serde_json::json!({
            "common_name": common_name,
        });

        let resp = self
            .client
            .post(&url)
            .header("X-Vault-Token", token)
            .json(&payload)
            .send()
            .await?
            .json::<VaultPkiResponse>()
            .await?;

        let mut cert = self.cert.write().await;
        let mut key = self.key.write().await;

        *cert = Some(resp.data.certificate);
        *key = Some(resp.data.private_key);

        info!("Successfully received certificate from Vault");
        Ok(())
    }

    pub async fn start_renewal_loop(self: Arc<Self>) {
        info!("Starting Vault renewal loop");
        let mut interval = tokio::time::interval(Duration::from_secs(3600)); // Check hourly

        loop {
            interval.tick().await;
            info!("Checking certificate and token TTL for renewal...");
            // In a real implementation:
            // Check token TTL via /auth/token/lookup-self
            // Renew if TTL < 50% via /auth/token/renew-self
            // Check cert expiry via X509 parsing
            // Request new cert if < 50% TTL remains
        }
    }

    pub async fn get_token(&self) -> Option<String> {
        self.token.read().await.clone()
    }

    pub async fn get_cert_and_key(&self) -> (Option<String>, Option<String>) {
        let cert = self.cert.read().await.clone();
        let key = self.key.read().await.clone();
        (cert, key)
    }
}
