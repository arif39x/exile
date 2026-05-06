# Exile Platform: Vault Server Configuration
# This is the base config for our secrets and identity backend.

storage "postgresql" {
  connection_url = "postgres://vault_user:{{vault_db_password}}@localhost:5432/vault_db?sslmode=disable"
  table          = "vault_kv_store"
}

listener "tcp" {
  address     = "0.0.0.0:8200"
  tls_disable = "true" # TLS is terminated at the edge/load balancer or handled via mTLS internally
}

# Enable the UI for easier administration (optional but helpful)
ui = true

# Root token TTL should be short if used, but we prefer AppRole/Policies
max_lease_ttl = "768h"
default_lease_ttl = "768h"

# Enable audit logging to a local file
audit_device "file" {
  path = "/var/log/vault/audit.log"
}
