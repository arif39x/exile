use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn};
use crate::adapters::{ProcessSupervisor, ProcessHandle};

pub struct Workload {
    pub id: String,
    pub binary: PathBuf,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
    pub handle: Option<ProcessHandle>,
}

pub struct WorkloadSupervisor {
    adapter: Box<dyn ProcessSupervisor>,
    workloads: Arc<Mutex<HashMap<String, Workload>>>,
}

impl WorkloadSupervisor {
    pub fn new(adapter: Box<dyn ProcessSupervisor>) -> Self {
        Self {
            adapter,
            workloads: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn run_workload(&self, id: String, binary: PathBuf, args: Vec<String>, env: HashMap<String, String>) -> Result<()> {
        info!("Starting workload: {}", id);
        
        let handle = self.adapter.start(binary.clone(), args.clone(), env.clone()).await?;
        
        let mut workloads = self.workloads.lock().await;
        workloads.insert(id.clone(), Workload {
            id,
            binary,
            args,
            env,
            handle: Some(handle),
        });
        
        Ok(())
    }

    pub async fn stop_workload(&self, id: &str) -> Result<()> {
        info!("Stopping workload: {}", id);
        
        let mut workloads = self.workloads.lock().await;
        if let Some(workload) = workloads.remove(id) {
            if let Some(handle) = workload.handle {
                self.adapter.stop(handle).await?;
            }
        }
        
        Ok(())
    }

    pub async fn monitor_workloads(&self) {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            let mut workloads = self.workloads.lock().await;
            
            for (id, workload) in workloads.iter_mut() {
                if let Some(handle) = workload.handle {
                    if !self.adapter.is_running(handle).await {
                        warn!("Workload {} is not running, attempting restart", id);
                        // Restart logic with backoff
                    }
                }
            }
        }
    }
}
