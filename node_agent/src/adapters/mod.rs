use std::collections::HashMap;
use std::path::PathBuf;
use async_trait::async_trait;
use anyhow::Result;

pub mod linux;
pub mod windows;
pub mod macos;

pub type ProcessHandle = u32;

#[async_trait]
pub trait ProcessSupervisor: Send + Sync {
    async fn start(
        &self,
        binary: PathBuf,
        args: Vec<String>,
        env: HashMap<String, String>,
    ) -> Result<ProcessHandle>;
    
    async fn stop(&self, handle: ProcessHandle) -> Result<()>;
    
    async fn is_running(&self, handle: ProcessHandle) -> bool;
    
    async fn pid(&self, handle: ProcessHandle) -> Option<u32>;
    
    async fn set_cpu_affinity(&self, handle: ProcessHandle, cpus: Vec<u32>) -> Result<()>;
    
    async fn set_memory_limit(&self, handle: ProcessHandle, bytes: u64) -> Result<()>;
}

pub fn get_adapter() -> Box<dyn ProcessSupervisor> {
    #[cfg(target_os = "linux")]
    {
        Box::new(linux::LinuxAdapter::new())
    }
    #[cfg(target_os = "windows")]
    {
        Box::new(windows::WindowsAdapter::new())
    }
    #[cfg(target_os = "macos")]
    {
        Box::new(macos::MacosAdapter::new())
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        panic!("Unsupported operating system");
    }
}
