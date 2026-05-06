#!/usr/bin/env bash

# Exile Platform: PKI Bootstrapping Script
# This script initializes the Root and Intermediate CAs in Vault.
# It assumes Vault is unsealed and you have a valid VAULT_TOKEN env var.

set -euo pipefail

# Configuration
ROOT_MOUNT="pki_root"
INTERMEDIATE_MOUNT="pki_int"
COMMON_NAME="exile.internal"
TTL="87600h" # 10 years for root

echo "--- Starting PKI Bootstrap for $COMMON_NAME ---"

# 1. Enable and Setup Root CA
if ! vault auth list | grep -q "${ROOT_MOUNT}/"; then
    echo "[*] Enabling Root CA mount at ${ROOT_MOUNT}..."
    vault secrets enable -path="${ROOT_MOUNT}" pki
    vault secrets tune -max-lease-ttl="${TTL}" "${ROOT_MOUNT}"

    echo "[*] Generating Root CA certificate..."
    vault write -field=certificate "${ROOT_MOUNT}/root/generate/internal" \
        common_name="${COMMON_NAME} Root CA" \
        ttl="${TTL}" > root_ca.crt
else
    echo "[!] Root CA mount already exists. Skipping."
fi

# 2. Enable and Setup Intermediate CA
if ! vault auth list | grep -q "${INTERMEDIATE_MOUNT}/"; then
    echo "[*] Enabling Intermediate CA mount at ${INTERMEDIATE_MOUNT}..."
    vault secrets enable -path="${INTERMEDIATE_MOUNT}" pki
    vault secrets tune -max-lease-ttl="43800h" "${INTERMEDIATE_MOUNT}"

    echo "[*] Generating CSR for Intermediate CA..."
    vault write -format=json "${INTERMEDIATE_MOUNT}/intermediate/generate/internal" \
        common_name="${COMMON_NAME} Intermediate Authority" \
        | jq -r '.data.csr' > pki_int.csr

    echo "[*] Signing Intermediate CA with Root..."
    vault write -format=json "${ROOT_MOUNT}/root/sign-intermediate" \
        csr=@pki_int.csr \
        format=pem_bundle \
        ttl="43800h" \
        | jq -r '.data.certificate' > pki_int.crt

    echo "[*] Importing signed Intermediate certificate back into Vault..."
    vault write "${INTERMEDIATE_MOUNT}/intermediate/set-signed" certificate=@pki_int.crt
else
    echo "[!] Intermediate CA mount already exists. Skipping."
fi

# 3. Create a Role for Issuing Certificates
echo "[*] Creating 'node-agent' role for certificate issuance..."
vault write "${INTERMEDIATE_MOUNT}/roles/node-agent" \
    allowed_domains="${COMMON_NAME}" \
    allow_subdomains=true \
    max_ttl="720h" # 30 days

echo "--- PKI Bootstrap Complete ---"
echo "Root Certificate saved to: root_ca.crt"
echo "Intermediate Certificate saved to: pki_int.crt"
