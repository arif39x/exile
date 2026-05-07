use std::collections::HashMap;
use std::path::PathBuf;
use async_trait::async_trait;
use anyhow::Result;
use crate::adapters::{ProcessSupervisor, ProcessHandle};

pub struct WindowsAdapter {}

impl WindowsAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl ProcessSupervisor for WindowsAdapter {
    async fn start(
        &self,
        _binary: PathBuf,
        _args: Vec<String>,
        _env: HashMap<String, String>,
    ) -> Result<ProcessHandle> {
        anyhow::bail!("Windows adapter not fully implemented")
    }

    async fn stop(&self, _handle: ProcessHandle) -> Result<()> {
        anyhow::bail!("Windows adapter not fully implemented")
    }

    async fn is_running(&self, _handle: ProcessHandle) -> bool {
        false
    }

    async fn pid(&self, _handle: ProcessHandle) -> Option<u32> {
        None
    }

    async fn set_cpu_affinity(&self, _handle: ProcessHandle, _cpus: Vec<u32>) -> Result<()> {
        anyhow::bail!("Windows adapter not fully implemented")
    }

    async fn set_memory_limit(&self, _handle: ProcessHandle, _bytes: u64) -> Result<()> {
        anyhow::bail!("Windows adapter not fully implemented")
    }
}
