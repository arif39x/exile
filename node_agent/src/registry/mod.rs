pub mod exile {
    pub mod registry {
        pub mod v1 {
            tonic::include_proto!("exile.registry.v1");
        }
    }
}

use anyhow::Result;
use exile::registry::v1::node_registry_client::NodeRegistryClient;
use exile::registry::v1::RegisterNodeRequest;
use tracing::info;

pub struct RegistryClient {
    endpoint: String,
}

impl RegistryClient {
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }

    pub async fn register(
        &self,
        hostname: String,
        ip_address: String,
        os_family: String,
        os_arch: String,
    ) -> Result<String> {
        info!("Registering node with registry at {}", self.endpoint);
        
        let mut client = NodeRegistryClient::connect(self.endpoint.clone()).await?;
        
        let request = tonic::Request::new(RegisterNodeRequest {
            hostname,
            ip_address,
            os_family,
            os_arch,
        });

        let response = client.register_node(request).await?;
        let node = response.into_inner();
        
        info!("Successfully registered node with ID: {}", node.id);
        Ok(node.id)
    }
}
