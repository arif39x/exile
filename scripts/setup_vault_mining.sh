#!/bin/bash

# Exit on error
set -e

echo "Populating Vault with mining secrets..."

# Ensure Vault is reachable
export VAULT_ADDR='http://127.0.0.1:8200'
export VAULT_TOKEN='root' # In a real scenario, use a proper token/auth

# Enable KV engine if not enabled
vault secrets enable -path=secret kv || true

# Store wallet addresses
vault kv put secret/mining/wallets/primary \
    address="44AFFq5kSiGBo3SBYnrYcyEC2UWXG6EGE7TJWAn9ZjrR63rZ7o6q4XmCjvY6pP2uG6EGE7TJWAn9ZjrR63rZ7o6q4Xm"

# Store pool addresses
vault kv put secret/mining/pools/primary \
    url="gulf.moneroocean.stream:10128"

vault kv put secret/mining/pools/secondary \
    url="pool.supportxmr.com:443"

echo "Mining secrets successfully stored in Vault."

# Create policy for xmrig-proxy
vault policy write xmrig-proxy - <<EOF
path "secret/data/mining/*" {
  capabilities = ["read"]
}
EOF

echo "Vault policy 'xmrig-proxy' created."
