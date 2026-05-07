use crate::adapters::{ProcessHandle, ProcessSupervisor};
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;

pub struct LinuxAdapter {}

impl LinuxAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl ProcessSupervisor for LinuxAdapter {
    async fn start(
        &self,
        binary: PathBuf,
        args: Vec<String>,
        env: HashMap<String, String>,
    ) -> Result<ProcessHandle> {
        let child = Command::new(binary)
            .args(args)
            .envs(env)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let pid = child
            .id()
            .ok_or_else(|| anyhow::anyhow!("Failed to get PID"))?;

        Ok(pid)
    }

    async fn stop(&self, handle: ProcessHandle) -> Result<()> {
        let pid = nix::unistd::Pid::from_raw(handle as i32);
        nix::sys::signal::kill(pid, nix::sys::signal::Signal::SIGTERM)?;
        Ok(())
    }

    async fn is_running(&self, handle: ProcessHandle) -> bool {
        let path = format!("/proc/{}", handle);
        std::path::Path::new(&path).exists()
    }

    async fn pid(&self, handle: ProcessHandle) -> Option<u32> {
        if self.is_running(handle).await {
            Some(handle)
        } else {
            None
        }
    }

    async fn set_cpu_affinity(&self, _handle: ProcessHandle, _cpus: Vec<u32>) -> Result<()> {
        // Implementation using sched_setaffinity
        Ok(())
    }

    async fn set_memory_limit(&self, _handle: ProcessHandle, _bytes: u64) -> Result<()> {
        // Implementation using cgroups v2
        Ok(())
    }
}
