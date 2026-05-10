pub mod exile {
    pub mod workload {
        pub mod v1 {
            tonic::include_proto!("exile.workload.v1");
        }
    }
}

use crate::adapters::{ProcessHandle, ProcessSupervisor};
use anyhow::{Context, Result};
use exile::workload::v1::WorkloadDefinition;
use futures_util::StreamExt;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info, warn};

pub struct Workload {
    pub definition: WorkloadDefinition,
    pub binary_path: PathBuf,
    pub handle: Option<ProcessHandle>,
}

pub struct WorkloadSupervisor {
    adapter: Box<dyn ProcessSupervisor>,
    workloads: Arc<Mutex<HashMap<String, Workload>>>,
    base_dir: PathBuf,
}

impl WorkloadSupervisor {
    pub fn new(adapter: Box<dyn ProcessSupervisor>) -> Self {
        let base_dir = std::env::current_dir()
            .unwrap_or_default()
            .join("workloads");
        if !base_dir.exists() {
            std::fs::create_dir_all(&base_dir).unwrap_or_default();
        }
        Self {
            adapter,
            workloads: Arc::new(Mutex::new(HashMap::new())),
            base_dir,
        }
    }

    pub async fn listen_for_intents(
        &self,
        nats: async_nats::Client,
        node_id: String,
    ) -> Result<()> {
        let subject = format!("platform.workloads.{}.*.*", node_id);
        let mut subscriber = nats
            .subscribe(subject.clone())
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        info!("Listening for workload intents on {}", subject);

        while let Some(message) = subscriber.next().await {
            let subject = message.subject.clone();
            let parts: Vec<&str> = subject.split('.').collect();
            if parts.len() < 5 {
                continue;
            }

            let operation = parts[4];
            let definition: WorkloadDefinition = serde_json::from_slice(&message.payload)
                .context("Failed to deserialize WorkloadDefinition")?;

            if let Err(e) = self.handle_intent(operation, definition).await {
                error!("Failed to handle intent {}: {}", operation, e);
            }
        }
        Ok(())
    }

    pub async fn handle_intent(
        &self,
        operation: &str,
        definition: WorkloadDefinition,
    ) -> Result<()> {
        info!(
            "Handling workload intent: {} for {}",
            operation, definition.id
        );
        match operation {
            "deploy" => self.deploy(definition).await,
            "start" => self.start(&definition.id).await,
            "stop" => self.stop(&definition.id).await,
            "restart" => self.restart(&definition.id).await,
            "reconfigure" => self.reconfigure(definition).await,
            _ => Err(anyhow::anyhow!("Unknown operation: {}", operation)),
        }
    }

    pub async fn deploy(&self, definition: WorkloadDefinition) -> Result<()> {
        info!(
            "Deploying workload: {} ({})",
            definition.name, definition.id
        );

        let workload_dir = self.base_dir.join(&definition.id);
        std::fs::create_dir_all(&workload_dir)?;

        // In a real implementation, i would download the artifact from definition.artifact_ref
        // For now as placeholder, simulate it by creating a dummy script if it doesn't exist
        let binary_path = workload_dir.join(&definition.name);

        if !binary_path.exists() {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::write(
                    &binary_path,
                    "#!/bin/sh\necho \"Starting workload\"\nwhile true; do sleep 10; done",
                )?;
                let mut perms = std::fs::metadata(&binary_path)?.permissions();
                perms.set_mode(0o755);
                std::fs::set_permissions(&binary_path, perms)?;
            }
            #[cfg(windows)]
            {
                std::fs::write(
                    &binary_path,
                    "echo Starting workload\n:loop\ntimeout /t 10\ngoto loop",
                )?;
            }
        }

        let mut workloads = self.workloads.lock().await;
        workloads.insert(
            definition.id.clone(),
            Workload {
                definition,
                binary_path,
                handle: None,
            },
        );

        Ok(())
    }

    pub async fn start(&self, id: &str) -> Result<()> {
        let mut workloads = self.workloads.lock().await;
        let workload = workloads.get_mut(id).context("Workload not found")?;

        if workload.handle.is_some() {
            return Ok(()); // Already running
        }

        info!("Starting workload: {}", id);
        let handle = self
            .adapter
            .start(workload.binary_path.clone(), vec![], HashMap::new())
            .await?;
        workload.handle = Some(handle);

        Ok(())
    }

    pub async fn stop(&self, id: &str) -> Result<()> {
        let mut workloads = self.workloads.lock().await;
        let workload = workloads.get_mut(id).context("Workload not found")?;

        if let Some(handle) = workload.handle.take() {
            info!("Stopping workload: {}", id);
            self.adapter.stop(handle).await?;
        }

        Ok(())
    }

    pub async fn restart(&self, id: &str) -> Result<()> {
        self.stop(id).await?;
        self.start(id).await
    }

    pub async fn reconfigure(&self, definition: WorkloadDefinition) -> Result<()> {
        info!("Reconfiguring workload: {}", definition.id);
        // Apply configuration changes
        // For now, just update the definition and restart
        {
            let mut workloads = self.workloads.lock().await;
            if let Some(workload) = workloads.get_mut(&definition.id) {
                workload.definition = definition.clone();
            }
        }
        self.restart(&definition.id).await
    }

    pub async fn monitor_workloads(&self) {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            let mut workloads = self.workloads.lock().await;

            for (id, workload) in workloads.iter_mut() {
                if let Some(handle) = workload.handle.as_mut() {
                    if !self.adapter.is_running(handle.clone()).await {
                        warn!("Workload {} is not running, attempting restart", id);
                        if let Ok(new_handle) = self
                            .adapter
                            .start(workload.binary_path.clone(), vec![], HashMap::new())
                            .await
                        {
                            *handle = new_handle;
                        }
                    }
                }
            }
        }
    }
}
